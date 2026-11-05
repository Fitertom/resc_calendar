//! The calendar and its note tip, drawn with GDI into a bitmap that goes onto the window as is.
//!
//! **GDI and not a font atlas**, as in Glint: the system's own text renderer is already in
//! memory, draws Cyrillic, and adds nothing to the exe.
//!
//! **Transparent pixels from GDI**: the window floats over a blur, so every pixel needs its
//! alpha — and GDI writes none. Each picture is drawn twice, over black and over white: where
//! nothing was drawn the two differ by 255, where something opaque was they agree, and every
//! antialiased edge in between gives its own coverage. `alpha = 1 − (white − black)`, and the
//! black pass already is the colour premultiplied by it. Then the tint goes under.
//!
//! Proportions and helpers (`round_cover`, the fonts per DPI) are Glint's.

use crate::cal::{Date, MONTHS, WEEKDAYS_SHORT};
use std::ffi::c_void;
use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::Graphics::Gdi::*;

type Rgb = (u8, u8, u8);
const WHITE: Rgb = (255, 255, 255);
const TEXT: Rgb = (0xe8, 0xe8, 0xec);
const TEXT_DIM: Rgb = (0x9a, 0x9a, 0xa2);
const TEXT_OUT: Rgb = (0x6a, 0x6a, 0x72);
const WEEKEND: Rgb = (0xf0, 0x9a, 0x90);
const WEEKEND_OUT: Rgb = (0x7a, 0x5c, 0x58);
const ACCENT: Rgb = (0x3d, 0x8b, 0xd4);
const ACCENT_HOT: Rgb = (0x4a, 0x9a, 0xe4);
const MARK: Rgb = (0xe8, 0x45, 0x45);
const NOTE: Rgb = (0xf2, 0xb8, 0x3a);
/// The window's 1-px rim, as DWM draws it.
pub const BORDER: Rgb = (0x3a, 0x3a, 0x3e);

const PAD: i32 = 12;
const HEAD_H: i32 = 40;
const WEEK_H: i32 = 24;
const CELL_W: i32 = 40;
const CELL_H: i32 = 36;
const DAY: i32 = 30;
const EDIT_H: i32 = 36;
const TIP_W: i32 = 260;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Hit {
    Prev,
    Next,
    Title,
    Day(Date),
}

/// What the calendar shows.
pub struct View<'a> {
    pub month: Date,
    pub today: Date,
    /// For a day: its red circle, and whether it has a note.
    pub day: &'a dyn Fn(Date) -> (bool, bool),
    pub hot: Option<Hit>,
    /// A note being written: for which day, the text so far, the caret's blink.
    pub editing: Option<(Date, &'a str, bool)>,
    pub weekends: bool,
    pub tint: (u8, u8, u8, u8),
}

pub struct Painter {
    dc: HDC,
    bmp: HBITMAP,
    bits: *mut u8,
    bw: i32,
    bh: i32,
    dpi: u32,
    title: HFONT,
    body: HFONT,
    small: HFONT,
    icons: HFONT,
}

pub fn scaled(v: i32, dpi: u32) -> i32 {
    (v * dpi as i32 + 48) / 96
}

pub fn colorref(c: Rgb) -> u32 {
    c.0 as u32 | (c.1 as u32) << 8 | (c.2 as u32) << 16
}

fn rect(l: i32, t: i32, r: i32, b: i32) -> RECT {
    RECT { left: l, top: t, right: r, bottom: b }
}

impl Painter {
    pub fn new() -> Painter {
        // SAFETY: a memory DC of our own.
        let dc = unsafe { CreateCompatibleDC(std::ptr::null_mut()) };
        let mut p = Painter {
            dc,
            bmp: std::ptr::null_mut(),
            bits: std::ptr::null_mut(),
            bw: 0,
            bh: 0,
            dpi: 0,
            title: std::ptr::null_mut(),
            body: std::ptr::null_mut(),
            small: std::ptr::null_mut(),
            icons: std::ptr::null_mut(),
        };
        p.set_dpi(96);
        p
    }

    fn set_dpi(&mut self, dpi: u32) {
        if dpi == self.dpi {
            return;
        }
        self.dpi = dpi;
        // Grayscale antialiasing: ClearType's coloured fringes assume an opaque ground, and the
        // ground here is a blur.
        let q = ANTIALIASED_QUALITY as u32;
        // SAFETY: fonts of ours, deleted once.
        unsafe {
            for f in [self.title, self.body, self.small, self.icons] {
                if !f.is_null() {
                    DeleteObject(f);
                }
            }
            self.title = font(scaled(15, dpi), "Segoe UI Semibold", q);
            self.body = font(scaled(13, dpi), "Segoe UI", q);
            self.small = font(scaled(12, dpi), "Segoe UI", q);
            self.icons = font(scaled(12, dpi), "Segoe Fluent Icons", q);
            SelectObject(self.dc, self.icons);
            let mut face = [0u16; 64];
            let n = GetTextFaceW(self.dc, face.len() as i32, face.as_mut_ptr());
            if String::from_utf16_lossy(&face[..n.max(1) as usize - 1]) != "Segoe Fluent Icons" {
                DeleteObject(self.icons);
                self.icons = font(scaled(12, dpi), "Segoe MDL2 Assets", q);
            }
        }
    }

    /// The bitmap at least `w × h`.
    fn canvas(&mut self, w: i32, h: i32) {
        if w <= self.bw && h <= self.bh {
            return;
        }
        let (w, h) = (w.max(self.bw), h.max(self.bh));
        // SAFETY: the header is filled here; the old bitmap is deleted after it is deselected.
        unsafe {
            let mut bmi: BITMAPINFO = std::mem::zeroed();
            bmi.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
            bmi.bmiHeader.biWidth = w;
            bmi.bmiHeader.biHeight = -h; // top-down
            bmi.bmiHeader.biPlanes = 1;
            bmi.bmiHeader.biBitCount = 32;
            let mut bits: *mut c_void = std::ptr::null_mut();
            let bmp = CreateDIBSection(self.dc, &bmi, DIB_RGB_COLORS, &mut bits, std::ptr::null_mut(), 0);
            SelectObject(self.dc, bmp);
            if !self.bmp.is_null() {
                DeleteObject(self.bmp);
            }
            self.bmp = bmp;
            self.bits = bits as *mut u8;
        }
        self.bw = w;
        self.bh = h;
    }

    fn fill(&self, r: RECT, c: Rgb) {
        // SAFETY: a brush made and deleted around one call on our DC.
        unsafe {
            let b = CreateSolidBrush(colorref(c));
            FillRect(self.dc, &r, b);
            DeleteObject(b);
        }
    }

    /// One line of `s` in `r` with DrawText flags.
    fn text(&self, s: &str, f: HFONT, c: Rgb, mut r: RECT, flags: u32) {
        // NOTHING FOR AN EMPTY STRING: DrawTextW reads through the pointer even at length zero.
        if s.is_empty() {
            return;
        }
        let w: Vec<u16> = s.encode_utf16().collect();
        // SAFETY: our DC and font; the string lives through the calls.
        unsafe {
            SelectObject(self.dc, f);
            SetTextColor(self.dc, colorref(c));
            SetBkMode(self.dc, TRANSPARENT as i32);
            DrawTextW(self.dc, w.as_ptr(), w.len() as i32, &mut r, flags | DT_SINGLELINE | DT_NOPREFIX | DT_VCENTER);
        }
    }

    /// Wrapped text in `r`; with `measure`, only how tall it comes out.
    fn paragraph(&self, s: &str, f: HFONT, c: Rgb, mut r: RECT, measure: bool) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let w: Vec<u16> = s.encode_utf16().collect();
        let flags = DT_WORDBREAK | DT_NOPREFIX | DT_EDITCONTROL | if measure { DT_CALCRECT } else { 0 };
        // SAFETY: as in `text`.
        unsafe {
            SelectObject(self.dc, f);
            SetTextColor(self.dc, colorref(c));
            SetBkMode(self.dc, TRANSPARENT as i32);
            DrawTextW(self.dc, w.as_ptr(), w.len() as i32, &mut r, flags);
        }
        r.bottom - r.top
    }

    fn measure(&self, s: &str, f: HFONT) -> (i32, i32) {
        if s.is_empty() {
            return (0, 0);
        }
        let w: Vec<u16> = s.encode_utf16().collect();
        let mut r = rect(0, 0, 0, 0);
        // SAFETY: as in `text`.
        unsafe {
            SelectObject(self.dc, f);
            DrawTextW(self.dc, w.as_ptr(), w.len() as i32, &mut r, DT_CALCRECT | DT_SINGLELINE | DT_NOPREFIX);
        }
        (r.right - r.left, r.bottom - r.top)
    }

    /// A filled rounded rectangle, antialiased, blended at `opacity` over what is there.
    fn round(&self, r: RECT, radius: i32, c: Rgb, opacity: f32) {
        self.shape(r, radius, 0.0, c, opacity);
    }

    /// The same shape as a ring `width` wide.
    fn ring(&self, r: RECT, width: f32, c: Rgb) {
        let rad = (r.right - r.left) / 2;
        self.shape(r, rad, width, c, 1.0);
    }

    fn shape(&self, r: RECT, radius: i32, ring: f32, c: Rgb, opacity: f32) {
        // SAFETY: GDI is flushed before the bits are touched; every write is inside the DIB.
        unsafe {
            GdiFlush();
            let (fw, fh, rad) = ((r.right - r.left) as f32, (r.bottom - r.top) as f32, radius as f32);
            for y in r.top.max(0)..r.bottom.min(self.bh) {
                for x in r.left.max(0)..r.right.min(self.bw) {
                    let (cx, cy) = ((x - r.left) as f32 + 0.5, (y - r.top) as f32 + 0.5);
                    let mut cov = round_cover(cx, cy, fw, fh, rad, 0.0);
                    if ring > 0.0 {
                        cov -= round_cover(cx, cy, fw, fh, rad, ring);
                    }
                    let cov = cov * opacity;
                    if cov <= 0.0 {
                        continue;
                    }
                    let p = std::slice::from_raw_parts_mut(self.bits.add(((y * self.bw + x) * 4) as usize), 4);
                    for (i, v) in [c.2, c.1, c.0].into_iter().enumerate() {
                        p[i] = (p[i] as f32 + (v as f32 - p[i] as f32) * cov).round() as u8;
                    }
                }
            }
        }
    }

    /// The top-left `w × h` as tight rows.
    fn take(&self, w: i32, h: i32) -> Vec<u8> {
        // SAFETY: GDI is flushed so the bits are final; the DIB is `bw × bh`, larger or equal.
        unsafe {
            GdiFlush();
            let mut out = Vec::with_capacity((w * h * 4) as usize);
            for y in 0..h {
                out.extend_from_slice(std::slice::from_raw_parts(self.bits.add((y * self.bw * 4) as usize), (w * 4) as usize));
            }
            out
        }
    }

    /// Draw a picture twice (see the module notes) and leave it in the bitmap premultiplied,
    /// over the tint.
    fn transparent(&mut self, w: i32, h: i32, tint: (u8, u8, u8, u8), mut draw: impl FnMut(&mut Painter)) {
        self.canvas(w, h);
        let all = rect(0, 0, w, h);
        self.fill(all, (0, 0, 0));
        draw(self);
        let black = self.take(w, h);
        self.fill(all, WHITE);
        draw(self);
        let white = self.take(w, h);
        let ta = tint.3 as f32 / 255.0;
        let t = [tint.2 as f32 * ta, tint.1 as f32 * ta, tint.0 as f32 * ta];
        // SAFETY: GDI is flushed by `take`; every write is inside the DIB.
        unsafe {
            for y in 0..h as usize {
                let row = std::slice::from_raw_parts_mut(self.bits.add(y * self.bw as usize * 4), w as usize * 4);
                for x in 0..w as usize {
                    let i = (y * w as usize + x) * 4;
                    let spread = (0..3).map(|c| white[i + c] as i32 - black[i + c] as i32).sum::<i32>() as f32 / 3.0;
                    let a = (1.0 - spread / 255.0).clamp(0.0, 1.0);
                    let p = &mut row[x * 4..x * 4 + 4];
                    for c in 0..3 {
                        p[c] = (black[i + c] as f32 + t[c] * (1.0 - a)).round().min(255.0) as u8;
                    }
                    p[3] = ((a + ta * (1.0 - a)) * 255.0).round() as u8;
                }
            }
        }
    }

    /// The calendar: its size and where its parts are, in its own pixels.
    pub fn calendar(&mut self, dpi: u32, v: &View) -> (i32, i32, Vec<(Hit, RECT)>) {
        self.set_dpi(dpi);
        let s = |x: i32| scaled(x, dpi);
        let w = s(PAD) * 2 + s(CELL_W) * 7;
        let grid_top = s(PAD) + s(HEAD_H) + s(WEEK_H);
        let grid_bottom = grid_top + s(CELL_H) * 6;
        let h = grid_bottom + s(PAD) + if v.editing.is_some() { s(EDIT_H) + s(PAD) / 2 } else { 0 };
        let mut items = Vec::new();
        let mut first = true;
        self.transparent(w, h, v.tint, |p| {
            let mut it = Vec::new();
            p.draw_calendar(v, w, grid_top, grid_bottom, &mut it);
            if first {
                items = it;
                first = false;
            }
        });
        (w, h, items)
    }

    fn draw_calendar(&self, v: &View, w: i32, grid_top: i32, grid_bottom: i32, items: &mut Vec<(Hit, RECT)>) {
        let s = |x: i32| scaled(x, self.dpi);
        let (left, right, pad) = (s(PAD), w - s(PAD), s(PAD));

        // Month, and the arrows the system calendar has: up for the one before.
        let bsz = s(32);
        let by = pad + (s(HEAD_H) - bsz) / 2;
        let next = rect(right - bsz, by, right, by + bsz);
        let prev = rect(next.left - bsz - s(2), by, next.left - s(2), by + bsz);
        let label = format!("{} {}", MONTHS[v.month.m as usize - 1], v.month.y);
        let (tw, _) = self.measure(&label, self.title);
        let title = rect(left, by, left + tw + s(16), by + bsz);
        if v.hot == Some(Hit::Title) {
            self.round(title, s(6), WHITE, 0.08);
        }
        self.text(&label, self.title, TEXT, rect(title.left + s(8), title.top, title.right, title.bottom), DT_LEFT);
        items.push((Hit::Title, title));
        for (hit, r, glyph) in [(Hit::Prev, prev, "\u{E70E}"), (Hit::Next, next, "\u{E70D}")] {
            if v.hot == Some(hit) {
                self.round(r, s(6), WHITE, 0.08);
            }
            self.text(glyph, self.icons, TEXT, r, DT_CENTER);
            items.push((hit, r));
        }

        let wy = pad + s(HEAD_H);
        for (k, name) in WEEKDAYS_SHORT.iter().enumerate() {
            let x = left + k as i32 * s(CELL_W);
            let c = if v.weekends && k >= 5 { WEEKEND_OUT } else { TEXT_DIM };
            self.text(name, self.small, c, rect(x, wy, x + s(CELL_W), wy + s(WEEK_H)), DT_CENTER);
        }

        // Six weeks from the Monday on or before the 1st.
        let start = v.month.add_days(-(v.month.weekday() as i64));
        let d = s(DAY);
        for i in 0..42 {
            let date = start.add_days(i);
            let (col, row) = ((i % 7) as i32, (i / 7) as i32);
            let cell = rect(left + col * s(CELL_W), grid_top + row * s(CELL_H), left + (col + 1) * s(CELL_W), grid_top + (row + 1) * s(CELL_H));
            let (cx, cy) = ((cell.left + cell.right) / 2, (cell.top + cell.bottom) / 2);
            let circle = rect(cx - d / 2, cy - d / 2, cx - d / 2 + d, cy - d / 2 + d);
            let inside = date.same_month(v.month);
            let today = date == v.today;
            let hot = v.hot == Some(Hit::Day(date));
            let editing = v.editing.is_some_and(|(e, _, _)| e == date);
            let (marked, noted) = (v.day)(date);
            if today {
                self.round(circle, d / 2, if hot { ACCENT_HOT } else { ACCENT }, 1.0);
            } else if hot || editing {
                self.round(circle, d / 2, WHITE, if editing { 0.16 } else { 0.08 });
            }
            if marked {
                // Just outside the day's own circle, so it rings today's blue too.
                let o = s(2);
                self.ring(rect(circle.left - o, circle.top - o, circle.right + o, circle.bottom + o), s(2) as f32, MARK);
            }
            let weekend = v.weekends && col >= 5;
            let c = match (today, inside, weekend) {
                (true, _, _) => WHITE,
                (_, true, true) => WEEKEND,
                (_, true, false) => TEXT,
                (_, false, true) => WEEKEND_OUT,
                _ => TEXT_OUT,
            };
            let num = if noted { rect(circle.left, circle.top - s(3), circle.right, circle.bottom - s(3)) } else { circle };
            self.text(&date.d.to_string(), self.body, c, num, DT_CENTER);
            if noted {
                let dot = s(4);
                let (dx, dy) = (cx - dot / 2, cy + s(7));
                self.round(rect(dx, dy, dx + dot, dy + dot), dot / 2, if today { WHITE } else { NOTE }, if inside || today { 1.0 } else { 0.5 });
            }
            items.push((Hit::Day(date), cell));
        }

        // The note being written: one line under the grid, Enter keeps it, Esc drops it.
        if let Some((day, text, caret)) = v.editing {
            let top = grid_bottom + s(PAD) / 2;
            let field = rect(left, top, right, top + s(EDIT_H));
            self.round(field, s(8), WHITE, 0.08);
            let prefix = day.spoken(v.today);
            let (pw, _) = self.measure(&prefix, self.small);
            self.text(&prefix, self.small, NOTE, rect(field.left + s(10), field.top, field.left + s(10) + pw, field.bottom), DT_LEFT);
            let tr = rect(field.left + s(18) + pw, field.top, field.right - s(10), field.bottom);
            let mut caret_x = tr.left;
            if text.is_empty() {
                self.text("заметка, Enter — сохранить", self.body, TEXT_OUT, tr, DT_LEFT | DT_END_ELLIPSIS);
            } else {
                let (tw, _) = self.measure(text, self.body);
                let over = tw > tr.right - tr.left;
                self.text(text, self.body, TEXT, tr, if over { DT_RIGHT } else { DT_LEFT });
                caret_x = (tr.left + tw).min(tr.right);
            }
            if caret {
                let cy = (field.top + field.bottom) / 2;
                self.fill(rect(caret_x, cy - s(8), caret_x + s(1).max(1), cy + s(8)), TEXT);
            }
        }
    }

    /// The tip over a day with a note: the note, wrapped. Its size.
    pub fn tip(&mut self, dpi: u32, note: &str, tint: (u8, u8, u8, u8)) -> (i32, i32) {
        self.set_dpi(dpi);
        let s = |x: i32| scaled(x, dpi);
        let (px, py) = (s(12), s(9));
        let max = s(TIP_W);
        let (one, _) = self.measure(note, self.body);
        let tw = one.min(max);
        let th = self.paragraph(note, self.body, TEXT, rect(0, 0, tw, 0), true);
        let (w, h) = (tw + px * 2, th + py * 2);
        // A little denser than the calendar: it floats over whatever is under the pointer.
        let tint = (tint.0, tint.1, tint.2, tint.3.saturating_add(60));
        self.transparent(w, h, tint, |p| {
            p.paragraph(note, p.body, TEXT, rect(px, py, px + tw, py + th), false);
        });
        (w, h)
    }

    pub fn blit(&self, hdc: HDC, w: i32, h: i32) {
        // SAFETY: both DCs are live; the bitmap is at least `w × h`.
        unsafe { BitBlt(hdc, 0, 0, w, h, self.dc, 0, 0, SRCCOPY) };
    }
}

impl Drop for Painter {
    fn drop(&mut self) {
        // SAFETY: objects of ours, deleted once.
        unsafe {
            for o in [self.title, self.body, self.small, self.icons, self.bmp] {
                if !o.is_null() {
                    DeleteObject(o);
                }
            }
            DeleteDC(self.dc);
        }
    }
}

/// How much of the pixel at (cx, cy) a `w × h` rectangle rounded by `radius` covers, the
/// rectangle shrunk by `inset` on every side.
fn round_cover(cx: f32, cy: f32, w: f32, h: f32, radius: f32, inset: f32) -> f32 {
    let rr = (radius - inset).max(0.0);
    let dx = (inset + rr - cx).max(cx - (w - inset - rr)).max(0.0);
    let dy = (inset + rr - cy).max(cy - (h - inset - rr)).max(0.0);
    if cx < inset - 0.5 || cy < inset - 0.5 || cx > w - inset + 0.5 || cy > h - inset + 0.5 {
        return 0.0;
    }
    if dx == 0.0 && dy == 0.0 {
        let edge = (cx - inset + 0.5).min(cy - inset + 0.5).min(w - inset - cx + 0.5).min(h - inset - cy + 0.5);
        return edge.clamp(0.0, 1.0);
    }
    (rr - (dx * dx + dy * dy).sqrt() + 0.5).clamp(0.0, 1.0)
}

unsafe fn font(px: i32, face: &str, quality: u32) -> HFONT {
    let f = crate::win::wide(face);
    // SAFETY: the face name lives through the call.
    unsafe { CreateFontW(-px, 0, 0, 0, FW_NORMAL as i32, 0, 0, 0, DEFAULT_CHARSET as u32, OUT_DEFAULT_PRECIS as u32, CLIP_DEFAULT_PRECIS as u32, quality, 0, f.as_ptr()) }
}
