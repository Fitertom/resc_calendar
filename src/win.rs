//! The windows: the calendar and its note tip, frameless, on a blurred backdrop.
//!
//! **The window procedure never calls the application re-entrantly; it queues** (as Glint and
//! `resc_window` do). Win32 calls it from inside our own calls — `ShowWindow` delivers
//! `WM_ACTIVATE` before it returns — and the application would be borrowed twice.
//!
//! **The backdrop is the one ExplorerBlurMica gives Explorer**: the accent policy's blur behind
//! (`SetWindowCompositionAttribute`, what the shell uses for its own flyouts), with the frame
//! extended over the whole client area, so the compositor reads our pixels' alpha. What we draw
//! is premultiplied BGRA: transparent where the blur shows.

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::ffi::c_void;
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Dwm::{DwmExtendFrameIntoClientArea, DwmSetWindowAttribute};
use windows_sys::Win32::Graphics::Gdi::{
    BeginPaint, EndPaint, GetDC, GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromPoint, PAINTSTRUCT, ReleaseDC, ScreenToClient,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
use windows_sys::Win32::UI::Controls::{MARGINS, WM_MOUSELEAVE};
use windows_sys::Win32::UI::HiDpi::{DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, GetDpiForMonitor, MDT_EFFECTIVE_DPI, SetProcessDpiAwarenessContext};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyState, INPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP, SendInput, SetFocus, TME_LEAVE, TRACKMOUSEEVENT, TrackMouseEvent, VK_CONTROL, VK_MENU,
};
use windows_sys::Win32::UI::WindowsAndMessaging::*;

#[derive(Debug)]
pub enum Ev {
    Paint,
    MouseMove(i32, i32),
    MouseLeave,
    /// A button went down over the calendar: 0 left, 1 right, 2 middle.
    Press { button: u8, x: i32, y: i32 },
    /// Wheel notches (positive away from the user).
    Wheel(f32),
    Key { vk: u16, ctrl: bool },
    /// One UTF-16 unit of typed text.
    Char(u16),
    /// The calendar lost the foreground: a click elsewhere, Alt+Tab, Win.
    Deactivate,
    /// The taskbar clock was clicked (from the part of us inside explorer).
    Clock,
    /// Explorer restarted: a new taskbar, to be joined again.
    TaskbarCreated,
    /// A command from another launch: `toggle`, `quit`.
    Command(String),
    Timer(usize),
    Quit,
}

pub trait Handler {
    fn event(&mut self, e: Ev);
    /// After every batch of events: draw if anything changed.
    fn frame(&mut self);
}

thread_local! {
    static QUEUE: RefCell<VecDeque<Ev>> = const { RefCell::new(VecDeque::new()) };
    static BUSY: Cell<bool> = const { Cell::new(false) };
    static APP: Cell<Option<*mut dyn Handler>> = const { Cell::new(None) };
    static TRACKING: Cell<bool> = const { Cell::new(false) };
}

pub const CLASS: &str = "GlintCalendar";
const TIP_CLASS: &str = "GlintCalendar.Tip";
/// From the DLL in explorer: the clock was clicked. The same number is in `tap/src/lib.rs`.
pub const WM_CLOCK: u32 = WM_APP + 10;
pub const WM_QUIT_HOST: u32 = WM_APP + 2;
const COPYDATA_COMMAND: usize = 0x4743_414C; // "GCAL"

fn taskbar_created() -> u32 {
    static ID: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    // SAFETY: registers a name, idempotent.
    *ID.get_or_init(|| unsafe { RegisterWindowMessageW(wide("TaskbarCreated").as_ptr()) })
}

pub fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn dpi_aware() {
    // SAFETY: plain call with a constant.
    unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
}

fn post(e: Ev) {
    QUEUE.with(|q| q.borrow_mut().push_back(e));
    drain();
}

fn drain() {
    if BUSY.get() {
        return;
    }
    let Some(app) = APP.get() else { return };
    BUSY.set(true);
    loop {
        while let Some(e) = QUEUE.with(|q| q.borrow_mut().pop_front()) {
            // SAFETY: the pointer is set by `run` for the length of the loop, and BUSY keeps
            // this the only borrow.
            unsafe { (*app).event(e) };
        }
        // SAFETY: as above.
        unsafe { (*app).frame() };
        if QUEUE.with(|q| q.borrow().is_empty()) {
            break;
        }
    }
    BUSY.set(false);
}

pub fn run(app: &mut dyn Handler) {
    // SAFETY: the pointer is cleared below, before `app`'s borrow ends.
    let p: *mut (dyn Handler + '_) = app;
    APP.set(Some(unsafe { std::mem::transmute::<*mut (dyn Handler + '_), *mut (dyn Handler + 'static)>(p) }));
    drain();
    // SAFETY: the message is ours and handed straight back.
    unsafe {
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    APP.set(None);
}

// ---- other launches ---------------------------------------------------------------------

/// The resident calendar's window, if one is running.
pub fn find_host() -> Option<HWND> {
    let c = wide(CLASS);
    // SAFETY: a lookup by class name.
    let h = unsafe { FindWindowW(c.as_ptr(), std::ptr::null()) };
    (!h.is_null()).then_some(h)
}

/// Hand a command to the resident calendar, passing the foreground on with it.
pub fn send_command(host: HWND, cmd: &str) -> bool {
    use windows_sys::Win32::System::DataExchange::COPYDATASTRUCT;
    let data: Vec<u16> = cmd.encode_utf16().collect();
    // SAFETY: the struct and its data live through the synchronous send.
    unsafe {
        let mut pid = 0;
        GetWindowThreadProcessId(host, &mut pid);
        AllowSetForegroundWindow(pid);
        let cds = COPYDATASTRUCT { dwData: COPYDATA_COMMAND, cbData: (data.len() * 2) as u32, lpData: data.as_ptr() as *mut _ };
        let mut res = 0;
        SendMessageTimeoutW(host, WM_COPYDATA, 0, &cds as *const _ as isize, SMTO_ABORTIFHUNG, 1000, &mut res) != 0 && res == 1
    }
}

pub fn post_quit(host: HWND) {
    // SAFETY: posting to a window handle.
    unsafe { PostMessageW(host, WM_QUIT_HOST, 0, 0) };
}

// ---- windows ----------------------------------------------------------------------------

#[repr(C)]
struct AccentPolicy {
    state: i32,
    flags: i32,
    gradient: u32,
    animation: i32,
}

#[repr(C)]
struct CompositionData {
    attribute: i32,
    data: *mut c_void,
    size: usize,
}

windows_link::link!("user32.dll" "system" fn SetWindowCompositionAttribute(hwnd: HWND, data: *mut CompositionData) -> i32);

/// The blur behind, as ExplorerBlurMica's "Blur" effect: ACCENT_ENABLE_BLURBEHIND through
/// WCA_ACCENT_POLICY. The tint is ours, drawn into the pixels.
///
/// **FLAGS ZERO, NO GRADIENT**: with flag 2 (use the gradient colour) the compositor filled the
/// window black instead of blurring — checked against a plain window on this machine.
fn blur_behind(hwnd: HWND) {
    let mut accent = AccentPolicy { state: 3, flags: 0, gradient: 0, animation: 0 };
    let mut data = CompositionData { attribute: 19, data: (&mut accent as *mut AccentPolicy).cast(), size: size_of::<AccentPolicy>() };
    // SAFETY: both structs live through the call.
    unsafe { SetWindowCompositionAttribute(hwnd, &mut data) };
}

/// Make one of our two windows: the calendar takes the keyboard, the tip never does.
fn make(class_name: &str, tip: bool, border: u32) -> Result<HWND, String> {
    // SAFETY: every struct is filled here and every string outlives its call.
    unsafe {
        let hinst = GetModuleHandleW(std::ptr::null());
        let class = wide(class_name);
        let icon = |side| LoadImageW(hinst, 1 as _, IMAGE_ICON, side, side, LR_DEFAULTCOLOR) as HICON;
        let wc = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: 0,
            lpfnWndProc: Some(if tip { tip_proc } else { wndproc }),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinst,
            hIcon: icon(GetSystemMetrics(SM_CXICON)),
            hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
            hbrBackground: std::ptr::null_mut(),
            lpszMenuName: std::ptr::null(),
            lpszClassName: class.as_ptr(),
            hIconSm: icon(GetSystemMetrics(SM_CXSMICON)),
        };
        if RegisterClassExW(&wc) == 0 {
            return Err(format!("RegisterClassExW: {}", std::io::Error::last_os_error()));
        }
        // A PLAIN POPUP: with a thick frame (Glint's trick for a shadow) the blur came out black
        // and the frame showed as a light rim. The compositor still rounds a popup's corners
        // and draws its hairline border when asked to.
        let ex = WS_EX_TOOLWINDOW | WS_EX_TOPMOST | if tip { WS_EX_NOACTIVATE | WS_EX_TRANSPARENT } else { 0 };
        let hwnd = CreateWindowExW(ex, class.as_ptr(), class.as_ptr(), WS_POPUP, 0, 0, 100, 100, std::ptr::null_mut(), std::ptr::null_mut(), hinst, std::ptr::null());
        if hwnd.is_null() {
            return Err(format!("CreateWindowExW: {}", std::io::Error::last_os_error()));
        }
        let attr = |a: u32, v: &i32| DwmSetWindowAttribute(hwnd, a, (v as *const i32).cast(), 4);
        attr(20, &1); // DWMWA_USE_IMMERSIVE_DARK_MODE
        attr(33, &if tip { 3 } else { 2 }); // DWMWA_WINDOW_CORNER_PREFERENCE: round / round small
        attr(34, &(border as i32)); // DWMWA_BORDER_COLOR
        // THE WHOLE CLIENT AREA IS "FRAME": the compositor then takes our alpha as it is.
        let m = MARGINS { cxLeftWidth: -1, cxRightWidth: -1, cyTopHeight: -1, cyBottomHeight: -1 };
        DwmExtendFrameIntoClientArea(hwnd, &m);
        blur_behind(hwnd);
        SetWindowPos(hwnd, std::ptr::null_mut(), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED);
        Ok(hwnd)
    }
}

pub fn create(border: u32) -> Result<(HWND, HWND), String> {
    taskbar_created();
    Ok((make(CLASS, false, border)?, make(TIP_CLASS, true, border)?))
}

/// Move, size, and (with `show`) show a window above everything, without activating it.
pub fn place(hwnd: HWND, x: i32, y: i32, w: i32, h: i32, show: bool) {
    let flags = SWP_NOACTIVATE | if show { SWP_SHOWWINDOW } else { 0 };
    // SAFETY: a window of ours.
    unsafe { SetWindowPos(hwnd, HWND_TOPMOST, x, y, w, h, flags) };
}

/// Until the compositor's next frame: animation steps land one per screen refresh.
pub fn flush() {
    // SAFETY: plain call.
    unsafe { windows_sys::Win32::Graphics::Dwm::DwmFlush() };
}

pub fn hide(hwnd: HWND) {
    // SAFETY: a window of ours.
    unsafe { ShowWindow(hwnd, SW_HIDE) };
}

#[allow(dead_code)]
pub fn is_visible(hwnd: HWND) -> bool {
    // SAFETY: a window of ours.
    unsafe { IsWindowVisible(hwnd) != 0 }
}

/// Our pixels onto the window, alpha and all.
pub fn present(hwnd: HWND, painter: &crate::ui::Painter, w: i32, h: i32) {
    // SAFETY: a DC of our window, released right after.
    unsafe {
        let dc = GetDC(hwnd);
        painter.blit(dc, w, h);
        ReleaseDC(hwnd, dc);
    }
}

/// Into the foreground and keyboard focus. Explorer passed the right on when the clock was
/// clicked (`AllowSetForegroundWindow` in the DLL); joining the foreground thread's input is
/// for when it did not, and an Alt tap — which lifts the foreground lock — the last resort.
pub fn activate(hwnd: HWND) {
    // SAFETY: plain calls on windows and threads of this desktop.
    unsafe {
        SetForegroundWindow(hwnd);
        if GetForegroundWindow() == hwnd {
            SetFocus(hwnd);
            return;
        }
        let fg = GetForegroundWindow();
        let me = GetCurrentThreadId();
        let other = if fg.is_null() { 0 } else { GetWindowThreadProcessId(fg, std::ptr::null_mut()) };
        let joined = other != 0 && other != me && AttachThreadInput(me, other, 1) != 0;
        BringWindowToTop(hwnd);
        SetForegroundWindow(hwnd);
        SetFocus(hwnd);
        if joined {
            AttachThreadInput(me, other, 0);
        }
        if GetForegroundWindow() != hwnd {
            let mut keys: [INPUT; 2] = std::mem::zeroed();
            for (k, up) in keys.iter_mut().zip([false, true]) {
                k.r#type = INPUT_KEYBOARD;
                k.Anonymous.ki.wVk = VK_MENU;
                k.Anonymous.ki.dwFlags = if up { KEYEVENTF_KEYUP } else { 0 };
            }
            SendInput(2, keys.as_ptr(), size_of::<INPUT>() as i32);
            SetForegroundWindow(hwnd);
            SetFocus(hwnd);
        }
    }
}

/// Every `ms`, `Ev::Timer(id)`, until killed.
pub fn set_timer(hwnd: HWND, id: usize, ms: u32) {
    // SAFETY: a timer on a window of ours.
    unsafe { SetTimer(hwnd, id, ms, None) };
}

pub fn kill_timer(hwnd: HWND, id: usize) {
    // SAFETY: as above.
    unsafe { KillTimer(hwnd, id) };
}

pub fn destroy(hwnd: HWND) {
    // SAFETY: a window of ours.
    unsafe { DestroyWindow(hwnd) };
}

pub fn cursor() -> POINT {
    let mut p = POINT { x: 0, y: 0 };
    // SAFETY: fills a struct of ours.
    unsafe { GetCursorPos(&mut p) };
    p
}

/// The monitor under a point: its whole rectangle and its scale.
pub fn monitor_at(p: POINT) -> (RECT, u32) {
    // SAFETY: plain queries into structs of ours.
    unsafe {
        let mon = MonitorFromPoint(p, MONITOR_DEFAULTTONEAREST);
        let mut mi: MONITORINFO = std::mem::zeroed();
        mi.cbSize = size_of::<MONITORINFO>() as u32;
        GetMonitorInfoW(mon, &mut mi);
        let (mut dx, mut dy) = (96, 96);
        GetDpiForMonitor(mon, MDT_EFFECTIVE_DPI, &mut dx, &mut dy);
        (mi.rcMonitor, dx.max(96))
    }
}

/// The taskbar under a point — the main one or a secondary — else the main one.
pub fn taskbar_near(p: POINT) -> Option<RECT> {
    // SAFETY: lookups and queries on the shell's windows.
    unsafe {
        let mut class = [0u16; 64];
        let h = WindowFromPoint(p);
        if !h.is_null() {
            let root = GetAncestor(h, GA_ROOT);
            let n = GetClassNameW(root, class.as_mut_ptr(), class.len() as i32);
            let name = String::from_utf16_lossy(&class[..n.max(0) as usize]);
            if name == "Shell_TrayWnd" || name == "Shell_SecondaryTrayWnd" {
                let mut r = std::mem::zeroed();
                GetWindowRect(root, &mut r);
                return Some(r);
            }
        }
        let tray = FindWindowW(wide("Shell_TrayWnd").as_ptr(), std::ptr::null());
        if tray.is_null() {
            return None;
        }
        let mut r = std::mem::zeroed();
        GetWindowRect(tray, &mut r);
        Some(r)
    }
}

/// Text on the clipboard, if any.
pub fn clipboard_text(hwnd: HWND) -> Option<String> {
    use windows_sys::Win32::System::DataExchange::{CloseClipboard, GetClipboardData, OpenClipboard};
    use windows_sys::Win32::System::Memory::{GlobalLock, GlobalUnlock};
    const CF_UNICODETEXT: u32 = 13;
    // SAFETY: the clipboard is opened and closed here; the locked memory is read up to its NUL.
    unsafe {
        if OpenClipboard(hwnd) == 0 {
            return None;
        }
        let h = GetClipboardData(CF_UNICODETEXT);
        let mut out = None;
        if !h.is_null() {
            let p = GlobalLock(h) as *const u16;
            if !p.is_null() {
                let mut n = 0;
                while *p.add(n) != 0 {
                    n += 1;
                }
                out = Some(String::from_utf16_lossy(std::slice::from_raw_parts(p, n)));
                GlobalUnlock(h);
            }
        }
        CloseClipboard();
        out
    }
}

fn lo(l: LPARAM) -> i32 {
    (l & 0xffff) as u16 as i16 as i32
}
fn hi(l: LPARAM) -> i32 {
    ((l >> 16) & 0xffff) as u16 as i16 as i32
}

/// The frame shared by both windows: none, no edges to resize by, no system menu.
unsafe fn frame_proc(hwnd: HWND, msg: u32, wparam: WPARAM, _lparam: LPARAM) -> Option<LRESULT> {
    match msg {
        WM_NCCALCSIZE if wparam != 0 => Some(0),
        WM_NCHITTEST => Some(HTCLIENT as LRESULT),
        WM_ERASEBKGND => Some(1),
        // We size ourselves for the monitor we open on.
        WM_DPICHANGED => Some(0),
        WM_PAINT => {
            // SAFETY: the paint struct is ours.
            unsafe {
                let mut ps: PAINTSTRUCT = std::mem::zeroed();
                BeginPaint(hwnd, &mut ps);
                EndPaint(hwnd, &ps);
            }
            post(Ev::Paint);
            Some(0)
        }
        _ => None,
    }
}

unsafe extern "system" fn tip_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // SAFETY: the message's own arguments, on this window.
    unsafe {
        match msg {
            WM_MOUSEACTIVATE => MA_NOACTIVATE as LRESULT,
            _ => frame_proc(hwnd, msg, wparam, lparam).unwrap_or_else(|| DefWindowProcW(hwnd, msg, wparam, lparam)),
        }
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // SAFETY: every call below is on this window with the message's own arguments.
    unsafe {
        if let Some(r) = frame_proc(hwnd, msg, wparam, lparam) {
            return r;
        }
        match msg {
            // Alt+F4 hides, as Esc does: the calendar stays resident.
            WM_CLOSE => {
                post(Ev::Deactivate);
                0
            }
            WM_QUIT_HOST => {
                post(Ev::Quit);
                0
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            }
            WM_CLOCK => {
                post(Ev::Clock);
                0
            }
            WM_COPYDATA => {
                let cds = &*(lparam as *const windows_sys::Win32::System::DataExchange::COPYDATASTRUCT);
                if cds.dwData != COPYDATA_COMMAND {
                    return 0;
                }
                let units = std::slice::from_raw_parts(cds.lpData as *const u16, cds.cbData as usize / 2);
                post(Ev::Command(String::from_utf16_lossy(units)));
                1
            }
            WM_ACTIVATE => {
                if (wparam & 0xffff) as u32 == WA_INACTIVE {
                    post(Ev::Deactivate);
                }
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
            WM_MOUSEMOVE => {
                if !TRACKING.get() {
                    let mut t = TRACKMOUSEEVENT { cbSize: size_of::<TRACKMOUSEEVENT>() as u32, dwFlags: TME_LEAVE, hwndTrack: hwnd, dwHoverTime: 0 };
                    TrackMouseEvent(&mut t);
                    TRACKING.set(true);
                }
                post(Ev::MouseMove(lo(lparam), hi(lparam)));
                0
            }
            WM_MOUSELEAVE => {
                TRACKING.set(false);
                post(Ev::MouseLeave);
                0
            }
            WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN => {
                let button = match msg {
                    WM_LBUTTONDOWN => 0,
                    WM_RBUTTONDOWN => 1,
                    _ => 2,
                };
                post(Ev::Press { button, x: lo(lparam), y: hi(lparam) });
                0
            }
            // No context menu: the right button marks days.
            WM_RBUTTONUP | WM_CONTEXTMENU => 0,
            WM_MOUSEWHEEL => {
                let notches = ((wparam >> 16) & 0xffff) as u16 as i16 as f32 / WHEEL_DELTA as f32;
                let mut p = POINT { x: lo(lparam), y: hi(lparam) };
                ScreenToClient(hwnd, &mut p);
                post(Ev::Wheel(notches));
                0
            }
            WM_KEYDOWN => {
                post(Ev::Key { vk: wparam as u16, ctrl: GetKeyState(VK_CONTROL as i32) < 0 });
                0
            }
            WM_CHAR => {
                post(Ev::Char(wparam as u16));
                0
            }
            WM_TIMER => {
                post(Ev::Timer(wparam));
                0
            }
            m if m == taskbar_created() && m != 0 => {
                post(Ev::TaskbarCreated);
                0
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}
