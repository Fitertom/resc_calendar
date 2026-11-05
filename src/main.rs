//! Glint Calendar — the calendar that opens from the taskbar clock. Win32 and GDI on a blurred
//! backdrop, with a small DLL in explorer to hear the clock's clicks (`tap/`).
//!
//! It runs resident and hidden (`--background`, from autostart); the clock, or launching the
//! exe again, shows it.

#![windows_subsystem = "windows"]

mod app;
mod cal;
mod com;
mod config;
mod inject;
mod install;
mod notes;
mod ui;
mod win;

use windows_sys::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW};

static TRACE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();

/// A line to `%TEMP%\glint_calendar_trace.txt` when `GLINT_TRACE` is set.
pub fn trace(what: &str) {
    if !*TRACE.get_or_init(|| std::env::var_os("GLINT_TRACE").is_some()) {
        return;
    }
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(std::env::temp_dir().join("glint_calendar_trace.txt")) {
        let _ = writeln!(f, "{what}");
    }
}

fn main() {
    let flag = std::env::args().nth(1);
    match flag.as_deref() {
        Some("--install") => return report(install::install(), "Готово. Календарь открывается по клику на часы и запускается вместе с Windows."),
        Some("--uninstall") => return report(install::uninstall(), "Glint Calendar удалён. Заметки остались в %APPDATA%\\Glint Calendar."),
        Some("--quit") => {
            if let Some(h) = win::find_host() {
                win::post_quit(h);
            }
            return;
        }
        _ => {}
    }
    let background = flag.as_deref() == Some("--background");
    // Already running: a plain launch toggles it, the autostart one has nothing to do.
    if let Some(host) = win::find_host() {
        if !background {
            win::send_command(host, "toggle");
        }
        return;
    }
    win::dpi_aware();
    let cfg = config::Config::load();
    let (hwnd, tip) = match win::create(ui::colorref(ui::BORDER)) {
        Ok(h) => h,
        Err(e) => return fail(&e),
    };
    let mut app = app::App::new(hwnd, tip, cfg);
    if !background {
        app.toggle();
    }
    win::run(&mut app);
    std::process::exit(0);
}

fn message(text: &str, flags: u32) {
    let (t, c) = (win::wide(text), win::wide("Glint Calendar"));
    // SAFETY: both strings live through the call.
    unsafe { MessageBoxW(std::ptr::null_mut(), t.as_ptr(), c.as_ptr(), flags) };
}

fn fail(e: &str) {
    message(e, MB_OK | MB_ICONERROR);
}

fn report(r: Result<(), String>, ok: &str) {
    match r {
        Ok(()) => message(ok, MB_OK),
        Err(e) => fail(&e),
    }
}
