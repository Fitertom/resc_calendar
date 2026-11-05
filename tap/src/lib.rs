//! The part of Glint Calendar that lives inside explorer.exe.
//!
//! **How it gets there**: the calendar calls `InitializeXamlDiagnosticsEx` on the explorer
//! process — the API Visual Studio's Live Visual Tree uses. XAML in explorer loads this DLL,
//! creates our class through `DllGetClassObject`, and hands it its diagnostics site; from there
//! `AdviseVisualTreeChange` reports every element of the taskbar's tree, with a handle that turns
//! into the live object. No code of explorer is patched or hooked.
//!
//! **What it does**: subscribes to `PointerReleased` on the clock (with `handledEventsToo`, as
//! the button marks its own events handled) and posts one message to the calendar's window.
//! Nothing else: the less runs inside explorer, the less can go wrong there.
//!
//! **Never a panic across the boundary**: every entry point runs under `catch_unwind`, the crate
//! is built with `panic = "unwind"`, and a failure only ends up in the log.

#![allow(non_snake_case)]

#[allow(clippy::all, unused)]
mod bindings;

use bindings::*;
use std::ffi::c_void;
use std::io::Write;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Mutex;
use windows_core::{GUID, HRESULT, IInspectable, IUnknown, Interface, PCWSTR, Ref, implement};

/// The class XAML is asked to create; the calendar passes the same id.
pub const CLSID_TAP: GUID = GUID::from_u128(0x7c9a2f4e_6b1d_4e8a_9f35_2d7b1c4a8e61);
/// Posted to the calendar's window: the clock was clicked (the pointer says where).
const WM_CLOCK: u32 = 0x8000 + 10;
const CALENDAR_CLASS: &str = "GlintCalendar";

const S_OK: HRESULT = HRESULT(0);
const S_FALSE: HRESULT = HRESULT(1);
const E_POINTER: HRESULT = HRESULT(0x8000_4003_u32 as i32);
const E_UNEXPECTED: HRESULT = HRESULT(0x8000_FFFF_u32 as i32);
const CLASS_E_CLASSNOTAVAILABLE: HRESULT = HRESULT(0x8004_0111_u32 as i32);
const CLASS_E_NOAGGREGATION: HRESULT = HRESULT(0x8004_0110_u32 as i32);

fn log(line: &str) {
    static LOCK: Mutex<()> = Mutex::new(());
    let _g = LOCK.lock();
    let path = std::env::temp_dir().join("glint_calendar_tap.txt");
    // Kept small: explorer runs for weeks.
    if std::fs::metadata(&path).is_ok_and(|m| m.len() > 512 * 1024) {
        let _ = std::fs::remove_file(&path);
    }
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(f, "[{}] {line}", std::process::id());
    }
}

fn guarded<T>(what: &str, f: impl FnOnce() -> windows_core::Result<T>) -> windows_core::Result<T> {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(r) => {
            if let Err(e) = &r {
                log(&format!("{what}: {e}"));
            }
            r
        }
        Err(_) => {
            log(&format!("{what}: panic caught"));
            Err(E_UNEXPECTED.into())
        }
    }
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

windows_link::link!("user32.dll" "system" fn FindWindowW(class: *const u16, title: *const u16) -> *mut c_void);
windows_link::link!("user32.dll" "system" fn PostMessageW(hwnd: *mut c_void, msg: u32, wparam: usize, lparam: isize) -> i32);
windows_link::link!("user32.dll" "system" fn GetWindowThreadProcessId(hwnd: *mut c_void, pid: *mut u32) -> u32);
windows_link::link!("user32.dll" "system" fn AllowSetForegroundWindow(pid: u32) -> i32);
windows_link::link!("kernel32.dll" "system" fn CreateEventW(attrs: *const c_void, manual: i32, initial: i32, name: *const u16) -> *mut c_void);

fn notify_calendar() {
    let class = wide(CALENDAR_CLASS);
    // SAFETY: a lookup and two calls on a window handle; the string lives through the call.
    unsafe {
        let h = FindWindowW(class.as_ptr(), std::ptr::null());
        if h.is_null() {
            return;
        }
        // EXPLORER HOLDS THE FOREGROUND RIGHT NOW (the user just clicked it) and passes it on:
        // without this the calendar would open behind, unable to take the keyboard.
        let mut pid = 0;
        GetWindowThreadProcessId(h, &mut pid);
        AllowSetForegroundWindow(pid);
        PostMessageW(h, WM_CLOCK, 0, 0);
    }
}

/// A named event per explorer process, so the calendar knows not to load a second copy.
fn mark_loaded() {
    let name = wide(&format!("Local\\GlintCalendarTap.{}", std::process::id()));
    // SAFETY: a named event; the handle is kept for the life of the process on purpose.
    unsafe { CreateEventW(std::ptr::null(), 1, 0, name.as_ptr()) };
}

// ---- COM entry points -------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllGetClassObject(clsid: *const GUID, iid: *const GUID, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() || clsid.is_null() || iid.is_null() {
        return E_POINTER;
    }
    // SAFETY: checked above; the pointers are the caller's.
    unsafe {
        *out = std::ptr::null_mut();
        if *clsid != CLSID_TAP {
            return CLASS_E_CLASSNOTAVAILABLE;
        }
        let r = catch_unwind(AssertUnwindSafe(|| {
            let f: IClassFactory = Factory.into();
            f.query(iid, out)
        }));
        r.unwrap_or(E_UNEXPECTED)
    }
}

/// Never: handlers of ours are attached to explorer's elements for as long as it runs.
#[unsafe(no_mangle)]
pub extern "system" fn DllCanUnloadNow() -> HRESULT {
    S_FALSE
}

#[implement(IClassFactory)]
struct Factory;

impl IClassFactory_Impl for Factory_Impl {
    fn CreateInstance(&self, outer: Ref<'_, IUnknown>, iid: *const GUID, out: *mut *mut c_void) -> windows_core::Result<()> {
        guarded("CreateInstance", || {
            if outer.is_some() {
                return Err(CLASS_E_NOAGGREGATION.into());
            }
            let tap: IObjectWithSite = Tap { site: Mutex::new(None) }.into();
            // SAFETY: the caller's out-pointer and iid.
            unsafe { tap.query(iid, out).ok() }
        })
    }

    fn LockServer(&self, _lock: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }
}

/// What XAML creates: it is given the diagnostics site, and starts watching the tree.
#[implement(IObjectWithSite)]
struct Tap {
    site: Mutex<Option<IUnknown>>,
}

impl IObjectWithSite_Impl for Tap_Impl {
    fn SetSite(&self, site: Ref<'_, IUnknown>) -> windows_core::Result<()> {
        guarded("SetSite", || {
            let Some(site) = site.cloned() else {
                *self.site.lock().unwrap() = None;
                return Ok(());
            };
            *self.site.lock().unwrap() = Some(site.clone());
            let diag: IXamlDiagnostics = site.cast()?;
            let tree: IVisualTreeService = site.cast()?;
            mark_loaded();
            log("loaded into explorer, watching the taskbar tree");
            // ADVISE FROM ANOTHER THREAD: the call replays the whole existing tree into the
            // callback on the UI thread and waits for it — made from the UI thread, it deadlocks.
            let callback: IVisualTreeServiceCallback2 = Watcher { diag }.into();
            let (tree, callback) = (Agile(tree), Agile(callback));
            std::thread::spawn(move || {
                let (tree, callback) = (tree, callback);
                // SAFETY: both objects are alive, held by this closure.
                if let Err(e) = unsafe { tree.0.AdviseVisualTreeChange(&callback.0) } {
                    log(&format!("AdviseVisualTreeChange: {e}"));
                }
            });
            Ok(())
        })
    }

    fn GetSite(&self, iid: *const GUID, out: *mut *mut c_void) -> windows_core::Result<()> {
        let site = self.site.lock().unwrap();
        match site.as_ref() {
            // SAFETY: the caller's pointers.
            Some(s) => unsafe { s.query(iid, out).ok() },
            None => Err(E_POINTER.into()),
        }
    }
}

/// The diagnostics objects are free-threaded; the wrapper only tells Rust so.
struct Agile<T>(T);
// SAFETY: XAML's diagnostics service and our callback are callable from any thread.
unsafe impl<T> Send for Agile<T> {}

#[implement(IVisualTreeServiceCallback2)]
struct Watcher {
    diag: IXamlDiagnostics,
}

impl IVisualTreeServiceCallback_Impl for Watcher_Impl {
    fn OnVisualTreeChange(&self, relation: &ParentChildRelation, element: &VisualElement, mutation: VisualMutationType) -> windows_core::Result<()> {
        // NEVER AN ERROR BACK: one failed element would end the whole subscription.
        let _ = guarded("OnVisualTreeChange", || {
            if mutation != VisualMutationType(0) {
                return Ok(()); // 0 = Add; removals need nothing: the handler goes with the element
            }
            let _ = relation;
            // THE CLOCK IS THE BUTTON THE NOTIFICATION CENTRE HANGS ON: every click on it,
            // padding included, bubbles up to it (checked: the date and time texts alone only
            // see clicks right on the digits). A secondary taskbar's clock goes through its
            // `DateTimeIconContent`; the calendar drops the duplicate a click then makes.
            let ty = element.Type.to_string();
            let name = element.Name.to_string();
            let is_clock = (ty == "SystemTray.OmniButton" && name == "NotificationCenterButton") || ty == "SystemTray.DateTimeIconContent";
            if !is_clock {
                return Ok(());
            }
            // SAFETY: the handle is the one XAML just reported.
            let obj: IInspectable = unsafe { self.diag.GetIInspectableFromHandle(element.Handle)? };
            let el: UIElement = obj.cast()?;
            // A PLAIN SUBSCRIPTION, NOT `AddHandler`: that one takes the handler boxed as an
            // IReference. The button is not a ButtonBase and leaves the release unhandled.
            let handler = PointerEventHandler::new(move |_, args| {
                let _ = guarded("PointerReleased", || {
                    // Left button only: the right one keeps the taskbar's own menu.
                    if let Some(a) = args.as_ref() {
                        if let Ok(p) = a.GetCurrentPoint(None) {
                            if p.Properties().and_then(|pp| pp.PointerUpdateKind()).is_ok_and(|k| k != PointerUpdateKind::LeftButtonReleased) {
                                return Ok(());
                            }
                        }
                    }
                    notify_calendar();
                    Ok(())
                });
                Ok(())
            });
            el.PointerReleased(&handler)?;
            log(&format!("watching the clock: {ty} #{name}"));
            Ok(())
        });
        Ok(())
    }
}

impl IVisualTreeServiceCallback2_Impl for Watcher_Impl {
    fn OnElementStateChanged(&self, _element: u64, _state: VisualElementState, _context: &PCWSTR) -> windows_core::Result<()> {
        Ok(())
    }
}

#[allow(dead_code)]
const _: HRESULT = S_OK;
