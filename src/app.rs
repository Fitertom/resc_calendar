//! The calendar's state and what every input does to it.
//!
//! Left click: months and days. Right click on a day: a red circle, again to take it away.
//! Middle click: write a note for the day (shown when the pointer rests on it), again to
//! delete it. Wheel: months. Esc or a click anywhere else: gone.

use crate::cal::Date;
use crate::config::Config;
use crate::notes::Notes;
use crate::ui::{self, Hit, Painter, View, scaled};
use crate::win::{self, Ev};
use windows_sys::Win32::Foundation::{HWND, RECT};
use windows_sys::Win32::System::SystemInformation::GetTickCount64;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{VK_BACK, VK_END, VK_ESCAPE, VK_HOME, VK_NEXT, VK_PRIOR, VK_RETURN};

const CARET: usize = 1;
const INJECT: usize = 2;
/// The pointer left the calendar: it hides unless the pointer is back by then.
const LEAVE: usize = 3;
const LEAVE_MS: u32 = 300;
/// One click on the clock can reach us twice (the button and its text both report it).
const SAME_CLICK_MS: u64 = 300;
/// A click on the clock while the calendar is open first takes the foreground away (and the
/// calendar hides), then arrives as a click: within this long, it was the closing click.
const REOPEN_MS: u64 = 400;
const MAX_NOTE: usize = 400;

/// **The Quick Settings flyout's own motion, measured** (Win+A filmed one screen column at a
/// time, ~200 frames/s, 100 Hz screen): it slides up out of the taskbar's edge, clipped by it,
/// with no fade. Open: share of the panel (and its gap to the taskbar) above the edge by ms —
/// a third on the very first frame, then a long settle. Close: share sunk back by ms, slow
/// at first and falling away. No standard easing curve has this shape, so the samples are
/// played back as they were.
const OPEN: [(f32, f32); 29] = [
    (0.0, 0.0), (9.0, 0.335), (16.0, 0.496), (25.0, 0.569), (33.0, 0.625), (41.0, 0.672), (49.0, 0.710), (58.0, 0.743),
    (66.0, 0.775), (79.0, 0.811), (87.0, 0.835), (96.0, 0.852), (104.0, 0.873), (114.0, 0.886), (124.0, 0.904), (133.0, 0.914),
    (143.0, 0.931), (150.0, 0.944), (158.0, 0.949), (167.0, 0.959), (175.0, 0.968), (183.0, 0.972), (187.0, 0.978), (197.0, 0.979),
    (204.0, 0.985), (212.0, 0.989), (220.0, 0.994), (233.0, 0.998), (241.0, 1.0),
];
const CLOSE: [(f32, f32); 31] = [
    (0.0, 0.0), (7.0, 0.002), (17.0, 0.007), (24.0, 0.011), (33.0, 0.015), (40.0, 0.022), (45.0, 0.024), (50.0, 0.030),
    (57.0, 0.043), (61.0, 0.051), (66.0, 0.060), (69.0, 0.067), (73.0, 0.077), (78.0, 0.090), (83.0, 0.100), (91.0, 0.127),
    (100.0, 0.157), (107.0, 0.189), (111.0, 0.212), (115.0, 0.230), (120.0, 0.253), (128.0, 0.307), (132.0, 0.333), (136.0, 0.369),
    (140.0, 0.403), (144.0, 0.442), (149.0, 0.485), (158.0, 0.590), (166.0, 0.734), (169.0, 0.848), (173.0, 1.0),
];

/// A table at `t` ms, straight lines between samples.
fn sample(table: &[(f32, f32)], t: f32) -> f32 {
    let last = table[table.len() - 1];
    if t >= last.0 {
        return last.1;
    }
    for w in table.windows(2) {
        let ((t0, v0), (t1, v1)) = (w[0], w[1]);
        if t < t1 {
            return v0 + (v1 - v0) * ((t - t0) / (t1 - t0)).clamp(0.0, 1.0);
        }
    }
    last.1
}

pub struct App {
    hwnd: HWND,
    tip: HWND,
    painter: Painter,
    tip_painter: Painter,
    notes: Notes,
    cfg: Config,
    visible: bool,
    need_show: bool,
    dirty: bool,
    today: Date,
    month: Date,
    hot: Option<Hit>,
    items: Vec<(Hit, RECT)>,
    editing: Option<Date>,
    input: String,
    high: Option<u16>,
    caret: bool,
    size: (i32, i32),
    origin: (i32, i32),
    /// Where the calendar's bottom-right corner goes: above the clock.
    anchor: (i32, i32),
    monitor: RECT,
    dpi: u32,
    hidden_at: u64,
    clock_at: u64,
    wheel: f32,
}

fn now() -> u64 {
    // SAFETY: plain query.
    unsafe { GetTickCount64() }
}

impl App {
    pub fn new(hwnd: HWND, tip: HWND, cfg: Config) -> App {
        let today = Date::today();
        let mut app = App {
            hwnd,
            tip,
            painter: Painter::new(),
            tip_painter: Painter::new(),
            notes: Notes::load(),
            cfg,
            visible: false,
            need_show: false,
            dirty: false,
            today,
            month: today.first_of_month(),
            hot: None,
            items: Vec::new(),
            editing: None,
            input: String::new(),
            high: None,
            caret: true,
            size: (0, 0),
            origin: (0, 0),
            anchor: (0, 0),
            monitor: RECT { left: 0, top: 0, right: 0, bottom: 0 },
            dpi: 96,
            hidden_at: 0,
            clock_at: 0,
            wheel: 0.0,
        };
        app.join_taskbar();
        app
    }

    fn join_taskbar(&mut self) {
        match crate::inject::ensure() {
            Ok(_) => win::kill_timer(self.hwnd, INJECT),
            // Explorer may still be coming up: try again in a moment.
            Err(e) => {
                crate::trace(&format!("taskbar not joined: {e}"));
                win::set_timer(self.hwnd, INJECT, 2000);
            }
        }
    }

    fn tint(&self) -> (u8, u8, u8, u8) {
        (self.cfg.tint.0, self.cfg.tint.1, self.cfg.tint.2, self.cfg.tint_alpha)
    }

    pub fn toggle(&mut self) {
        if self.visible { self.hide() } else { self.show() }
    }

    /// Open above the clock of the taskbar the pointer is on, at today's month.
    fn show(&mut self) {
        let p = win::cursor();
        let (mon, dpi) = win::monitor_at(p);
        let s = |v| scaled(v, dpi);
        self.monitor = mon;
        self.dpi = dpi;
        self.anchor = match win::taskbar_near(p) {
            Some(bar) if bar.top > (mon.top + mon.bottom) / 2 => (bar.right - s(12), bar.top - s(12)),
            _ => (mon.right - s(12), mon.bottom - s(60)),
        };
        self.notes.refresh();
        self.today = Date::today();
        self.month = self.today.first_of_month();
        self.hot = None;
        self.visible = true;
        self.need_show = true;
        self.dirty = true;
    }

    /// The gap between the calendar and the taskbar, which the slide also crosses.
    fn gap(&self) -> i32 {
        scaled(12, self.dpi)
    }

    /// One step of the slide: `hidden` of the way (0 = in place, 1 = all below the taskbar's
    /// edge). The window is cut at that edge — moved down and made shorter — and shows the top
    /// of the picture. False while nothing is above the edge yet.
    fn slide_to(&self, hidden: f32, show: bool) -> bool {
        let ((w, h), (x, y)) = (self.size, self.origin);
        let travel = h + self.gap();
        let d = (hidden.clamp(0.0, 1.0) * travel as f32).round() as i32;
        let visible = (travel - d).min(h);
        if visible <= 0 {
            return false;
        }
        win::place(self.hwnd, x, y + d, w, visible, show);
        win::present(self.hwnd, &self.painter, w, visible);
        true
    }

    /// Up out of the taskbar, frame by frame with the compositor (a quarter of a second;
    /// input waits in the queue meanwhile).
    fn slide_in(&mut self) {
        let start = std::time::Instant::now();
        let mut shown = false;
        loop {
            let t = start.elapsed().as_secs_f32() * 1000.0;
            let done = t >= OPEN[OPEN.len() - 1].0;
            let p = if done { 1.0 } else { sample(&OPEN, t) };
            if self.slide_to(1.0 - p, !shown) && !shown {
                shown = true;
                win::activate(self.hwnd);
            }
            if done {
                break;
            }
            win::flush();
        }
    }

    fn slide_out(&mut self) {
        let start = std::time::Instant::now();
        loop {
            let t = start.elapsed().as_secs_f32() * 1000.0;
            let o = sample(&CLOSE, t);
            if o >= 1.0 || !self.slide_to(o, false) {
                break;
            }
            win::flush();
        }
    }

    fn hide(&mut self) {
        win::kill_timer(self.hwnd, LEAVE);
        win::hide(self.tip);
        if self.visible && self.size.0 > 0 && !self.need_show {
            self.slide_out();
        }
        win::hide(self.hwnd);
        win::hide(self.tip);
        self.visible = false;
        self.hidden_at = now();
        self.stop_editing();
    }

    fn stop_editing(&mut self) {
        self.editing = None;
        self.input.clear();
        self.high = None;
        win::kill_timer(self.hwnd, CARET);
        self.dirty = true;
    }

    fn start_editing(&mut self, d: Date) {
        self.editing = Some(d);
        self.input.clear();
        self.caret = true;
        win::set_timer(self.hwnd, CARET, 530);
        self.dirty = true;
    }

    fn commit(&mut self) {
        if let Some(d) = self.editing {
            if !self.input.trim().is_empty() {
                self.notes.set_note(d, Some(&self.input));
            }
        }
        self.stop_editing();
    }

    fn hit(&self, x: i32, y: i32) -> Option<Hit> {
        self.items.iter().find(|(_, r)| x >= r.left && x < r.right && y >= r.top && y < r.bottom).map(|&(h, _)| h)
    }

    fn go_month(&mut self, n: i32) {
        self.month = self.month.add_months(n).first_of_month();
        self.dirty = true;
    }

    fn press(&mut self, button: u8, x: i32, y: i32) {
        let Some(hit) = self.hit(x, y) else { return };
        match (button, hit) {
            (0, Hit::Prev) => self.go_month(-1),
            (0, Hit::Next) => self.go_month(1),
            (0, Hit::Title) => {
                self.month = self.today.first_of_month();
                self.dirty = true;
            }
            (0, Hit::Day(d)) if !d.same_month(self.month) => {
                self.month = d.first_of_month();
                self.dirty = true;
            }
            (1, Hit::Day(d)) => {
                self.notes.toggle_mark(d);
                self.dirty = true;
            }
            (2, Hit::Day(d)) => {
                if self.notes.day(d).is_some_and(|x| x.note.is_some()) {
                    self.notes.set_note(d, None);
                    if self.editing == Some(d) {
                        self.stop_editing();
                    }
                } else if self.editing == Some(d) {
                    self.stop_editing();
                } else {
                    self.start_editing(d);
                }
                self.dirty = true;
            }
            _ => {}
        }
    }

    fn key(&mut self, vk: u16, ctrl: bool) {
        if self.editing.is_some() {
            match vk {
                VK_ESCAPE => self.stop_editing(),
                VK_RETURN => self.commit(),
                VK_BACK if ctrl => {
                    let t = self.input.trim_end();
                    let cut = t.rfind(char::is_whitespace).map_or(0, |i| i + 1);
                    self.input.truncate(cut);
                }
                VK_BACK => {
                    self.input.pop();
                }
                0x56 if ctrl => {
                    if let Some(t) = win::clipboard_text(self.hwnd) {
                        let t = t.replace(['\r', '\n', '\t'], " ");
                        self.input.extend(t.chars().take(MAX_NOTE.saturating_sub(self.input.chars().count())));
                    }
                }
                _ => return,
            }
            self.caret = true;
            self.dirty = true;
            return;
        }
        match vk {
            VK_ESCAPE => self.hide(),
            VK_PRIOR => self.go_month(-1),
            VK_NEXT => self.go_month(1),
            VK_HOME | VK_END => {
                self.month = self.today.first_of_month();
                self.dirty = true;
            }
            _ => {}
        }
    }

    fn typed(&mut self, u: u16) {
        if self.editing.is_none() {
            return;
        }
        let c = match u {
            0xD800..=0xDBFF => {
                self.high = Some(u);
                return;
            }
            0xDC00..=0xDFFF => match self.high.take() {
                Some(h) => char::decode_utf16([h, u]).next().and_then(|r| r.ok()),
                None => None,
            },
            _ => char::from_u32(u as u32),
        };
        // Control characters come with their keys, which `key` already handled.
        if let Some(c) = c.filter(|c| !c.is_control()) {
            if self.input.chars().count() < MAX_NOTE {
                self.input.push(c);
                self.caret = true;
                self.dirty = true;
            }
        }
    }

    /// The tip over the day under the pointer, if it has a note.
    fn update_tip(&mut self) {
        let note = match self.hot {
            Some(Hit::Day(d)) if self.editing != Some(d) => self.notes.day(d).and_then(|x| x.note.clone()).map(|n| (d, n)),
            _ => None,
        };
        let Some((d, note)) = note else {
            win::hide(self.tip);
            return;
        };
        let Some(&(_, cell)) = self.items.iter().find(|(h, _)| *h == Hit::Day(d)) else { return };
        let (w, h) = self.tip_painter.tip(self.dpi, &note, self.tint());
        let s = |v| scaled(v, self.dpi);
        let m = self.monitor;
        let cx = self.origin.0 + (cell.left + cell.right) / 2;
        let x = (cx - w / 2).clamp(m.left + s(4), (m.right - w - s(4)).max(m.left));
        let mut y = self.origin.1 + cell.top - h - s(4);
        if y < m.top {
            y = self.origin.1 + cell.bottom + s(4);
        }
        win::place(self.tip, x, y, w, h, true);
        win::present(self.tip, &self.tip_painter, w, h);
    }
}

impl win::Handler for App {
    fn event(&mut self, e: Ev) {
        match e {
            Ev::Paint => {
                if self.visible && self.size.0 > 0 {
                    win::present(self.hwnd, &self.painter, self.size.0, self.size.1);
                }
            }
            Ev::Clock => {
                let t = now();
                if t.saturating_sub(self.clock_at) < SAME_CLICK_MS {
                    return;
                }
                self.clock_at = t;
                if self.visible {
                    self.hide();
                } else if t.saturating_sub(self.hidden_at) >= REOPEN_MS {
                    self.show();
                }
            }
            Ev::Command(c) => match c.as_str() {
                "quit" => win::destroy(self.hwnd),
                _ => self.toggle(),
            },
            Ev::Deactivate => {
                if self.visible && !self.need_show {
                    self.hide();
                }
            }
            Ev::MouseMove(x, y) => {
                win::kill_timer(self.hwnd, LEAVE);
                let h = self.hit(x, y);
                if h != self.hot {
                    self.hot = h;
                    self.dirty = true;
                }
            }
            Ev::MouseLeave => {
                if self.hot.is_some() {
                    self.hot = None;
                    self.dirty = true;
                }
                // Writing a note keeps it open: the text would go with it.
                if self.visible && self.editing.is_none() {
                    win::set_timer(self.hwnd, LEAVE, LEAVE_MS);
                }
            }
            Ev::Timer(LEAVE) => {
                win::kill_timer(self.hwnd, LEAVE);
                let p = win::cursor();
                let (x, y) = (p.x - self.origin.0, p.y - self.origin.1);
                let inside = x >= 0 && y >= 0 && x < self.size.0 && y < self.size.1;
                if self.visible && self.editing.is_none() && !inside {
                    self.hide();
                }
            }
            Ev::Press { button, x, y } => self.press(button, x, y),
            Ev::Wheel(n) => {
                self.wheel += n;
                while self.wheel >= 1.0 {
                    self.wheel -= 1.0;
                    self.go_month(-1);
                }
                while self.wheel <= -1.0 {
                    self.wheel += 1.0;
                    self.go_month(1);
                }
            }
            Ev::Key { vk, ctrl } => self.key(vk, ctrl),
            Ev::Char(u) => self.typed(u),
            Ev::Timer(CARET) => {
                self.caret = !self.caret;
                self.dirty = true;
            }
            Ev::Timer(INJECT) | Ev::TaskbarCreated => self.join_taskbar(),
            Ev::Timer(_) => {}
            Ev::Quit => win::destroy(self.hwnd),
        }
    }

    fn frame(&mut self) {
        if !self.visible || !self.dirty {
            return;
        }
        self.dirty = false;
        let notes = &self.notes;
        let day = |d: Date| notes.day(d).map_or((false, false), |x| (x.mark, x.note.is_some()));
        let view = View {
            month: self.month,
            today: self.today,
            day: &day,
            hot: self.hot,
            editing: self.editing.map(|d| (d, self.input.as_str(), self.caret)),
            weekends: self.cfg.weekends,
            tint: (self.cfg.tint.0, self.cfg.tint.1, self.cfg.tint.2, self.cfg.tint_alpha),
        };
        let (w, h, items) = self.painter.calendar(self.dpi, &view);
        self.items = items;
        let m = self.monitor;
        let (x, y) = ((self.anchor.0 - w).max(m.left), (self.anchor.1 - h).max(m.top));
        if self.need_show {
            self.size = (w, h);
            self.origin = (x, y);
            self.slide_in();
            self.need_show = false;
            self.update_tip();
            return;
        }
        if (w, h) != self.size || (x, y) != self.origin {
            win::place(self.hwnd, x, y, w, h, false);
            self.size = (w, h);
            self.origin = (x, y);
        }
        win::present(self.hwnd, &self.painter, w, h);
        self.update_tip();
    }
}

#[allow(dead_code)]
const _: (u8, u8, u8) = ui::BORDER;
