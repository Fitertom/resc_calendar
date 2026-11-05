//! Getting the clock's clicks: the DLL next to the exe (`tap/`) is loaded into explorer through
//! XAML diagnostics — `InitializeXamlDiagnosticsEx`, what Visual Studio's Live Visual Tree and
//! TranslucentTB use. Explorer loads it, creates its class, and it subscribes to the clock.
//!
//! Once per explorer process: the DLL leaves a named event behind, and a second load is skipped.
//! Explorer restarted, the taskbar announces itself (`TaskbarCreated`) and this runs again.

use crate::win::wide;
use windows_sys::Win32::Foundation::CloseHandle;
use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows_sys::Win32::System::Threading::OpenEventW;
use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowThreadProcessId};
use windows_sys::core::GUID;

/// The DLL's class; the same id is in `tap/src/lib.rs`.
const CLSID_TAP: GUID = GUID::from_u128(0x7c9a2f4e_6b1d_4e8a_9f35_2d7b1c4a8e61);
pub const DLL: &str = "glint_calendar_tap.dll";
const SYNCHRONIZE: u32 = 0x0010_0000;
/// HRESULT_FROM_WIN32(ERROR_NOT_FOUND): that connection name is taken, try the next one.
const NAME_TAKEN: i32 = 0x8007_0490_u32 as i32;

type Initialize = unsafe extern "system" fn(*const u16, u32, *const u16, *const u16, GUID, *const u16) -> i32;

/// Make sure explorer's taskbar has the DLL. `Ok(true)`: loaded now, `Ok(false)`: it already was.
pub fn ensure() -> Result<bool, String> {
    // SAFETY: lookups, a library load, and a call through the documented signature.
    unsafe {
        let tray = FindWindowW(wide("Shell_TrayWnd").as_ptr(), std::ptr::null());
        if tray.is_null() {
            return Err("no taskbar yet".into());
        }
        let mut pid = 0;
        GetWindowThreadProcessId(tray, &mut pid);
        let marker = OpenEventW(SYNCHRONIZE, 0, wide(&format!("Local\\GlintCalendarTap.{pid}")).as_ptr());
        if !marker.is_null() {
            CloseHandle(marker);
            return Ok(false);
        }
        let dll = std::env::current_exe().map_err(|e| e.to_string())?.with_file_name(DLL);
        if !dll.exists() {
            return Err(format!("{} is missing", dll.display()));
        }
        let xaml = LoadLibraryW(wide("Windows.UI.Xaml.dll").as_ptr());
        if xaml.is_null() {
            return Err("Windows.UI.Xaml.dll did not load".into());
        }
        let Some(f) = GetProcAddress(xaml, c"InitializeXamlDiagnosticsEx".as_ptr().cast()) else {
            return Err("no InitializeXamlDiagnosticsEx".into());
        };
        let init: Initialize = std::mem::transmute(f);
        let path = wide(&dll.display().to_string());
        let mut hr = NAME_TAKEN;
        for i in 1..=20 {
            hr = init(wide(&format!("VisualDiagConnection{i}")).as_ptr(), pid, std::ptr::null(), path.as_ptr(), CLSID_TAP, std::ptr::null());
            if hr != NAME_TAKEN {
                break;
            }
        }
        crate::trace(&format!("InitializeXamlDiagnosticsEx into {pid}: 0x{:08X}", hr as u32));
        if hr < 0 { Err(format!("InitializeXamlDiagnosticsEx: 0x{:08X}", hr as u32)) } else { Ok(true) }
    }
}
