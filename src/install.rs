//! Installing like a normal Windows program, per user and without admin rights (Glint's way):
//! the exe and its DLL go to `%LOCALAPPDATA%\Programs\Glint Calendar`, a shortcut to the Start
//! menu, an entry to Settings ▸ Apps, and a line to the user's autostart.

use crate::com::{Com, check};
use crate::win::wide;
use std::path::{Path, PathBuf};
use windows_sys::Win32::System::Com::{CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx};
use windows_sys::Win32::System::Registry::{HKEY_CURRENT_USER, REG_DWORD, REG_SZ, RegDeleteKeyValueW, RegDeleteTreeW, RegSetKeyValueW};
use windows_sys::core::GUID;

const CLSID_SHELL_LINK: GUID = GUID::from_u128(0x00021401_0000_0000_c000_000000000046);
const IID_SHELL_LINK_W: GUID = GUID::from_u128(0x000214f9_0000_0000_c000_000000000046);
const IID_PERSIST_FILE: GUID = GUID::from_u128(0x0000010b_0000_0000_c000_000000000046);
const UNINSTALL: &str = r"Software\Microsoft\Windows\CurrentVersion\Uninstall\GlintCalendar";
const RUN: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const RUN_NAME: &str = "Glint Calendar";
const EXE: &str = "glint-calendar.exe";

pub fn dir() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
    base.join("Programs").join("Glint Calendar")
}

fn shortcut_path() -> PathBuf {
    let base = std::env::var_os("APPDATA").map(PathBuf::from).unwrap_or_default();
    base.join(r"Microsoft\Windows\Start Menu\Programs\Glint Calendar.lnk")
}

fn same(a: &Path, b: &Path) -> bool {
    a.to_string_lossy().to_lowercase() == b.to_string_lossy().to_lowercase()
}

fn set(key: &str, name: Option<&str>, value: &str) -> Result<(), String> {
    let (k, v) = (wide(key), wide(value));
    let n = name.map(wide);
    // SAFETY: all strings are NUL-terminated and live through the call.
    let r = unsafe { RegSetKeyValueW(HKEY_CURRENT_USER, k.as_ptr(), n.as_ref().map_or(std::ptr::null(), |n| n.as_ptr()), REG_SZ, v.as_ptr().cast(), (v.len() * 2) as u32) };
    if r == 0 { Ok(()) } else { Err(format!("registry {key}: error {r}")) }
}

fn set_dword(key: &str, name: &str, value: u32) -> Result<(), String> {
    let (k, n) = (wide(key), wide(name));
    // SAFETY: NUL-terminated strings and a 4-byte value living through the call.
    let r = unsafe { RegSetKeyValueW(HKEY_CURRENT_USER, k.as_ptr(), n.as_ptr(), REG_DWORD, (&value as *const u32).cast(), 4) };
    if r == 0 { Ok(()) } else { Err(format!("registry {key}: error {r}")) }
}

/// Copy `src` over `dst`, which may be in use: a running exe, or the DLL explorer holds. Windows
/// lets a file in use be renamed, so the old one is moved aside and swept away next time.
fn replace(src: &Path, dst: &Path) -> Result<(), String> {
    if same(src, dst) {
        return Ok(());
    }
    if let Some(dir) = dst.parent() {
        for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            if e.file_name().to_string_lossy().ends_with(".old") {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
    if std::fs::copy(src, dst).is_ok() {
        return Ok(());
    }
    let aside = dst.with_extension(format!("{}.old", std::process::id()));
    std::fs::rename(dst, &aside).map_err(|e| format!("{}: {e}", dst.display()))?;
    std::fs::copy(src, dst).map(|_| ()).map_err(|e| format!("{}: {e}", dst.display()))
}

/// Stop the running calendar, if any, and wait for it to go.
fn stop_running() {
    if let Some(h) = crate::win::find_host() {
        crate::win::post_quit(h);
        for _ in 0..50 {
            if crate::win::find_host().is_none() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }
}

/// `--install`: put the program in place, make it start with Windows, and start it.
pub fn install() -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let here = exe.parent().ok_or("no folder")?;
    let dir = dir();
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    stop_running();
    let dst = dir.join(EXE);
    replace(&exe, &dst)?;
    let dll = here.join(crate::inject::DLL);
    if !dll.exists() {
        return Err(format!("{} не найден рядом с exe", crate::inject::DLL));
    }
    replace(&dll, &dir.join(crate::inject::DLL))?;
    let ini = here.join("glint-calendar.ini");
    if ini.exists() && !dir.join("glint-calendar.ini").exists() {
        let _ = std::fs::copy(&ini, dir.join("glint-calendar.ini"));
    }
    shortcut(&dst)?;
    let d = dst.display().to_string();
    set(UNINSTALL, Some("DisplayName"), "Glint Calendar")?;
    set(UNINSTALL, Some("DisplayIcon"), &format!("\"{d}\",0"))?;
    set(UNINSTALL, Some("DisplayVersion"), env!("CARGO_PKG_VERSION"))?;
    set(UNINSTALL, Some("Publisher"), "Glint")?;
    set(UNINSTALL, Some("InstallLocation"), &dir.display().to_string())?;
    set(UNINSTALL, Some("UninstallString"), &format!("\"{d}\" --uninstall"))?;
    set(UNINSTALL, Some("QuietUninstallString"), &format!("\"{d}\" --uninstall"))?;
    set_dword(UNINSTALL, "NoModify", 1)?;
    set_dword(UNINSTALL, "NoRepair", 1)?;
    set(RUN, Some(RUN_NAME), &format!("\"{d}\" --background"))?;
    std::process::Command::new(&dst).arg("--background").spawn().map_err(|e| e.to_string())?;
    Ok(())
}

/// The Start menu shortcut, through the shell's own IShellLink.
fn shortcut(exe: &Path) -> Result<(), String> {
    // SAFETY: COM on this thread, objects released by `Com`; slots as in shobjidl.h / objidl.h:
    // IShellLinkW SetDescription 7, SetWorkingDirectory 9, SetIconLocation 17, SetPath 20;
    // IPersistFile Save 6.
    unsafe {
        CoInitializeEx(std::ptr::null(), COINIT_APARTMENTTHREADED as u32);
        let mut p = std::ptr::null_mut();
        check(CoCreateInstance(&CLSID_SHELL_LINK, std::ptr::null_mut(), CLSCTX_INPROC_SERVER, &IID_SHELL_LINK_W, &mut p), "ShellLink")?;
        let link = Com(p);
        type SetStr = unsafe extern "system" fn(*mut std::ffi::c_void, *const u16) -> i32;
        let path = wide(&exe.display().to_string());
        let dir = wide(&exe.parent().unwrap_or(exe).display().to_string());
        link.slot::<SetStr>(20)(link.raw(), path.as_ptr());
        link.slot::<SetStr>(9)(link.raw(), dir.as_ptr());
        link.slot::<SetStr>(7)(link.raw(), wide("Календарь у часов").as_ptr());
        link.slot::<unsafe extern "system" fn(*mut std::ffi::c_void, *const u16, i32) -> i32>(17)(link.raw(), path.as_ptr(), 0);
        let file = link.query(&IID_PERSIST_FILE).ok_or("IPersistFile")?;
        let at = shortcut_path();
        if let Some(parent) = at.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let at = wide(&at.display().to_string());
        check(file.slot::<unsafe extern "system" fn(*mut std::ffi::c_void, *const u16, i32) -> i32>(6)(file.raw(), at.as_ptr(), 1), "shortcut")
    }
}

/// `--uninstall`, as Settings ▸ Apps runs it. The notes stay in %APPDATA%; the DLL leaves
/// explorer when explorer next restarts, and with it the folder is fully gone.
pub fn uninstall() -> Result<(), String> {
    stop_running();
    // SAFETY: NUL-terminated strings living through the calls; missing values are fine.
    unsafe {
        RegDeleteKeyValueW(HKEY_CURRENT_USER, wide(RUN).as_ptr(), wide(RUN_NAME).as_ptr());
        RegDeleteTreeW(HKEY_CURRENT_USER, wide(UNINSTALL).as_ptr());
    }
    let _ = std::fs::remove_file(shortcut_path());
    let dir = dir();
    // Only ever our own folder: the path is built here, never taken from outside.
    if dir.ends_with(r"Programs\Glint Calendar") && dir.exists() {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        let script = format!("ping -n 3 127.0.0.1 >nul & rmdir /s /q \"{}\"", dir.display());
        let _ = std::process::Command::new("cmd").args(["/c", &script]).creation_flags(CREATE_NO_WINDOW).spawn();
    }
    Ok(())
}
