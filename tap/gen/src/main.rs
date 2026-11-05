//! Regenerates `tap/src/bindings.rs` from the metadata of the Windows it runs on:
//! `cargo run --manifest-path tap/gen/Cargo.toml`.
fn main() {
    let out = concat!(env!("CARGO_MANIFEST_DIR"), "/../src/bindings.rs");
    let xaml = r"C:\Windows\System32\WinMetadata\Windows.UI.Xaml.winmd";
    let types = [
        // XAML: the elements and the one event used
        "Windows.UI.Xaml.DependencyObject",
        "Windows.UI.Xaml.UIElement",
        "Windows.UI.Xaml.FrameworkElement",
        "Windows.UI.Xaml.RoutedEvent",
        "Windows.UI.Xaml.RoutedEventArgs",
        "Windows.UI.Xaml.RoutedEventHandler",
        "Windows.UI.Xaml.Controls.Control",
        "Windows.UI.Xaml.Controls.ContentControl",
        "Windows.UI.Xaml.Controls.Primitives.ButtonBase",
        "Windows.UI.Xaml.Input.PointerEventHandler",
        "Windows.UI.Xaml.Input.PointerRoutedEventArgs",
        "Windows.UI.Input.PointerPoint",
        "Windows.UI.Input.PointerPointProperties",
        "Windows.UI.Input.PointerUpdateKind",
        // XAML diagnostics: how the DLL gets the taskbar's tree
        "Windows.Win32.UI.Xaml.Diagnostics.IXamlDiagnostics",
        "Windows.Win32.UI.Xaml.Diagnostics.IVisualTreeService",
        "Windows.Win32.UI.Xaml.Diagnostics.IVisualTreeServiceCallback",
        "Windows.Win32.UI.Xaml.Diagnostics.IVisualTreeServiceCallback2",
        "Windows.Win32.UI.Xaml.Diagnostics.VisualElement",
        "Windows.Win32.UI.Xaml.Diagnostics.VisualMutationType",
        "Windows.Win32.UI.Xaml.Diagnostics.VisualElementState",
        "Windows.Win32.UI.Xaml.Diagnostics.ParentChildRelation",
        // COM plumbing
        "Windows.Win32.System.Com.IClassFactory",
        "Windows.Win32.System.Ole.IObjectWithSite",
    ];
    let mut args = vec!["--in", "default", xaml, "--out", out, "--implement", "--no-comment", "--flat", "--filter"];
    args.extend(types);
    let _ = windows_bindgen::bindgen(args);
    println!("written {out}");
}
