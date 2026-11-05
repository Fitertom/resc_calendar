#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ButtonBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ButtonBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ButtonBase,
    IAnimationObject,
    IVisualElement,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ButtonBase {
    pub fn IsPointerOver(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPointerOver)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsPressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPressed)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CommandParameter(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommandParameter)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCommandParameter<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetCommandParameter)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Click<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Click)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveClick(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveClick)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn new() -> windows_core::Result<ButtonBase> {
        Self::IButtonBaseFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                &mut core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Content(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContent)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTemplateRoot(&self) -> windows_core::Result<UIElement> {
        let this = &windows_core::Interface::cast::<IContentControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplateRoot)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnContentChanged<P0, P1>(
        &self,
        oldcontent: P0,
        newcontent: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IContentControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnContentChanged)(
                windows_core::Interface::as_raw(this),
                oldcontent.param().abi(),
                newcontent.param().abi(),
            )
            .ok()
        }
    }
    pub fn FontSize(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontSize)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSpacing)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCharacterSpacing)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTabStop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTabIndex)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveIsEnabledChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveIsEnabledChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ApplyTemplate(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyTemplate)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsFocusEngagementEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngagementEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsFocusEngagementEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsFocusEngagementEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsFocusEngaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngaged)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsFocusEngaged(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsFocusEngaged)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusLeft)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusRight)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusUp)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusDown)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveFocusEngaged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusEngaged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveFocusDisengaged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusDisengaged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveFocusEngagement(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusEngagement)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn OnPointerEntered<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerEntered)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerPressed<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerPressed)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerMoved<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerMoved)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerReleased<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerReleased)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerExited<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerExited)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCaptureLost<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCanceled<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerCanceled)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerWheelChanged<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnGotFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnGotFocus)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnLostFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnLostFocus)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn DefaultStyleKey(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IControlProtected>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDefaultStyleKey)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn GetTemplateChild(
        &self,
        childname: &windows_core::HSTRING,
    ) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateChild)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(childname),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTag)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetLanguage)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDataContext)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Parent(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Loaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLoaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Unloaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unloaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveUnloaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveUnloaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveSizeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveSizeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveLayoutUpdated(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLayoutUpdated)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(name),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveDataContextChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDataContextChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveLoading(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoading)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveActualThemeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveActualThemeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveEffectiveViewportChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnApplyTemplate)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn GoToElementStateCore(
        &self,
        statename: &windows_core::HSTRING,
        usetransitions: bool,
    ) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElementOverrides2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(statename),
                usetransitions,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElementProtected7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateViewport)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowDrop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetOpacity)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GotFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GotFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveGotFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGotFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LostFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLostFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLostFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDragEnter(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragEnter)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDragLeave(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragLeave)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDragOver(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragOver)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDrop(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDrop)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerPressed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerMoved)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerReleased)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerEntered)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerExited)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCaptureLost<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCaptureLost(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCanceled<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCanceled)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerWheelChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerWheelChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDoubleTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDoubleTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveHolding(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHolding)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveRightTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveRightTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationInertiaStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationStarted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationDelta(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationDelta)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ReleasePointerCaptures)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn AddHandler<P0, P1>(
        &self,
        routedevent: P0,
        handler: P1,
        handledeventstoo: bool,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).AddHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
                handledeventstoo,
            )
            .ok()
        }
    }
    pub fn RemoveHandler<P0, P1>(&self, routedevent: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateMeasure)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateArrange)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanDrag)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveDragStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDropCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDropCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKey)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn RemoveContextRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveContextCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveAccessKeyInvoked(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveGettingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGettingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveLosingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLosingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveNoFocusCandidateFound(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartBringIntoView)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RemoveCharacterReceived(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveCharacterReceived)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveProcessKeyboardAccelerators(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemovePreviewKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemovePreviewKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveBringIntoViewRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveBringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTranslation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetScale)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTransformMatrix(
        &self,
        value: windows_numerics::Matrix4x4,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransformMatrix)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCenterPoint)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotationAxis)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn GetChildrenInTabFocusOrder(
        &self,
    ) -> windows_core::Result<windows_collections::IIterable<DependencyObject>> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    fn IButtonBaseFactory<R, F: FnOnce(&IButtonBaseFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ButtonBase, IButtonBaseFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IButtonBaseStatics<R, F: FnOnce(&IButtonBaseStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ButtonBase, IButtonBaseStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ButtonBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IButtonBase>();
}
unsafe impl windows_core::Interface for ButtonBase {
    type Vtable = <IButtonBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IButtonBase as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ButtonBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ButtonBase";
}
unsafe impl Send for ButtonBase {}
unsafe impl Sync for ButtonBase {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ContentControl,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ContentControl,
    IAnimationObject,
    IVisualElement,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ContentControl {
    pub fn Content(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetContent)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTemplateRoot(&self) -> windows_core::Result<UIElement> {
        let this = &windows_core::Interface::cast::<IContentControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplateRoot)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn new() -> windows_core::Result<ContentControl> {
        Self::IContentControlFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                &mut core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn OnContentChanged<P0, P1>(
        &self,
        oldcontent: P0,
        newcontent: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IContentControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnContentChanged)(
                windows_core::Interface::as_raw(this),
                oldcontent.param().abi(),
                newcontent.param().abi(),
            )
            .ok()
        }
    }
    pub fn FontSize(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontSize)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSpacing)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCharacterSpacing)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTabStop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTabIndex)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveIsEnabledChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveIsEnabledChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ApplyTemplate(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyTemplate)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsFocusEngagementEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngagementEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsFocusEngagementEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsFocusEngagementEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsFocusEngaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngaged)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsFocusEngaged(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsFocusEngaged)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusLeft)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusRight)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusUp)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusDown)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveFocusEngaged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusEngaged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveFocusDisengaged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusDisengaged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveFocusEngagement(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusEngagement)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn OnPointerEntered<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerEntered)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerPressed<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerPressed)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerMoved<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerMoved)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerReleased<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerReleased)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerExited<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerExited)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCaptureLost<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCanceled<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerCanceled)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerWheelChanged<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnGotFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnGotFocus)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnLostFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnLostFocus)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn DefaultStyleKey(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IControlProtected>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDefaultStyleKey)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn GetTemplateChild(
        &self,
        childname: &windows_core::HSTRING,
    ) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateChild)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(childname),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTag)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetLanguage)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDataContext)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Parent(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Loaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLoaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Unloaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unloaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveUnloaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveUnloaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveSizeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveSizeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveLayoutUpdated(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLayoutUpdated)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(name),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveDataContextChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDataContextChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveLoading(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoading)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveActualThemeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveActualThemeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveEffectiveViewportChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnApplyTemplate)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn GoToElementStateCore(
        &self,
        statename: &windows_core::HSTRING,
        usetransitions: bool,
    ) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElementOverrides2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(statename),
                usetransitions,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElementProtected7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateViewport)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowDrop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetOpacity)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GotFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GotFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveGotFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGotFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LostFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLostFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLostFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDragEnter(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragEnter)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDragLeave(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragLeave)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDragOver(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragOver)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDrop(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDrop)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerPressed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerMoved)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerReleased)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerEntered)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerExited)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCaptureLost<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCaptureLost(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCanceled<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCanceled)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerWheelChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerWheelChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDoubleTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDoubleTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveHolding(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHolding)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveRightTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveRightTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationInertiaStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationStarted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationDelta(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationDelta)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ReleasePointerCaptures)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn AddHandler<P0, P1>(
        &self,
        routedevent: P0,
        handler: P1,
        handledeventstoo: bool,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).AddHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
                handledeventstoo,
            )
            .ok()
        }
    }
    pub fn RemoveHandler<P0, P1>(&self, routedevent: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateMeasure)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateArrange)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanDrag)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveDragStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDropCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDropCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKey)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn RemoveContextRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveContextCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveAccessKeyInvoked(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveGettingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGettingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveLosingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLosingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveNoFocusCandidateFound(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartBringIntoView)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RemoveCharacterReceived(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveCharacterReceived)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveProcessKeyboardAccelerators(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemovePreviewKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemovePreviewKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveBringIntoViewRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveBringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTranslation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetScale)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTransformMatrix(
        &self,
        value: windows_numerics::Matrix4x4,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransformMatrix)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCenterPoint)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotationAxis)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn GetChildrenInTabFocusOrder(
        &self,
    ) -> windows_core::Result<windows_collections::IIterable<DependencyObject>> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    fn IContentControlFactory<R, F: FnOnce(&IContentControlFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ContentControl, IContentControlFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IContentControlStatics<R, F: FnOnce(&IContentControlStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ContentControl, IContentControlStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ContentControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IContentControl>();
}
unsafe impl windows_core::Interface for ContentControl {
    type Vtable = <IContentControl as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContentControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContentControl {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ContentControl";
}
unsafe impl Send for ContentControl {}
unsafe impl Sync for ContentControl {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Control(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Control,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    Control,
    IAnimationObject,
    IVisualElement,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Control {
    pub fn FontSize(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontSize)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSpacing)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetCharacterSpacing)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTabStop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetTabIndex)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveIsEnabledChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveIsEnabledChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ApplyTemplate(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyTemplate)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsFocusEngagementEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngagementEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsFocusEngagementEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsFocusEngagementEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsFocusEngaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngaged)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsFocusEngaged(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsFocusEngaged)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusLeft)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusRight)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusUp)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusDown)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveFocusEngaged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusEngaged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveFocusDisengaged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusDisengaged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveFocusEngagement(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusEngagement)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn new() -> windows_core::Result<Control> {
        Self::IControlFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                &mut core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn OnPointerEntered<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerEntered)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerPressed<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerPressed)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerMoved<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerMoved)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerReleased<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerReleased)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerExited<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerExited)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCaptureLost<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCanceled<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerCanceled)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerWheelChanged<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnGotFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnGotFocus)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnLostFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnLostFocus)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn DefaultStyleKey(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IControlProtected>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDefaultStyleKey)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn GetTemplateChild(
        &self,
        childname: &windows_core::HSTRING,
    ) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateChild)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(childname),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIsTemplateFocusTarget<P0>(element: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IControlStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsTemplateFocusTarget)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn SetIsTemplateFocusTarget<P0>(element: P0, value: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IControlStatics3(|this| unsafe {
            (windows_core::Interface::vtable(this).SetIsTemplateFocusTarget)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn GetIsTemplateKeyTipTarget<P0>(element: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IControlStatics5(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsTemplateKeyTipTarget)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn SetIsTemplateKeyTipTarget<P0>(element: P0, value: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IControlStatics5(|this| unsafe {
            (windows_core::Interface::vtable(this).SetIsTemplateKeyTipTarget)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTag)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetLanguage)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDataContext)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Parent(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Loaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLoaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Unloaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unloaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveUnloaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveUnloaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveSizeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveSizeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveLayoutUpdated(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLayoutUpdated)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(name),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveDataContextChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDataContextChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveLoading(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoading)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveActualThemeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveActualThemeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveEffectiveViewportChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnApplyTemplate)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn GoToElementStateCore(
        &self,
        statename: &windows_core::HSTRING,
        usetransitions: bool,
    ) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElementOverrides2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(statename),
                usetransitions,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElementProtected7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateViewport)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowDrop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetOpacity)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GotFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GotFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveGotFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGotFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LostFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLostFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLostFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDragEnter(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragEnter)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDragLeave(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragLeave)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDragOver(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragOver)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDrop(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDrop)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerPressed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerMoved)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerReleased)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerEntered)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerExited)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCaptureLost<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCaptureLost(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCanceled<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCanceled)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerWheelChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerWheelChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDoubleTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDoubleTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveHolding(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHolding)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveRightTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveRightTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationInertiaStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationStarted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationDelta(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationDelta)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ReleasePointerCaptures)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn AddHandler<P0, P1>(
        &self,
        routedevent: P0,
        handler: P1,
        handledeventstoo: bool,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).AddHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
                handledeventstoo,
            )
            .ok()
        }
    }
    pub fn RemoveHandler<P0, P1>(&self, routedevent: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateMeasure)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateArrange)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanDrag)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveDragStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDropCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDropCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKey)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn RemoveContextRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveContextCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveAccessKeyInvoked(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveGettingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGettingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveLosingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLosingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveNoFocusCandidateFound(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartBringIntoView)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RemoveCharacterReceived(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveCharacterReceived)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveProcessKeyboardAccelerators(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemovePreviewKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemovePreviewKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveBringIntoViewRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveBringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTranslation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetScale)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTransformMatrix(
        &self,
        value: windows_numerics::Matrix4x4,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransformMatrix)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCenterPoint)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotationAxis)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn GetChildrenInTabFocusOrder(
        &self,
    ) -> windows_core::Result<windows_collections::IIterable<DependencyObject>> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    fn IControlFactory<R, F: FnOnce(&IControlFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Control, IControlFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IControlStatics<R, F: FnOnce(&IControlStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Control, IControlStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IControlStatics2<R, F: FnOnce(&IControlStatics2) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Control, IControlStatics2> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IControlStatics3<R, F: FnOnce(&IControlStatics3) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Control, IControlStatics3> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IControlStatics4<R, F: FnOnce(&IControlStatics4) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Control, IControlStatics4> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IControlStatics5<R, F: FnOnce(&IControlStatics5) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Control, IControlStatics5> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IControlStatics7<R, F: FnOnce(&IControlStatics7) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Control, IControlStatics7> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Control {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IControl>();
}
unsafe impl windows_core::Interface for Control {
    type Vtable = <IControl as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Control {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Control";
}
unsafe impl Send for Control {}
unsafe impl Sync for Control {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DependencyObject,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl DependencyObject {
    pub fn new() -> windows_core::Result<DependencyObject> {
        Self::IDependencyObjectFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                &mut core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IDependencyObjectFactory<
        R,
        F: FnOnce(&IDependencyObjectFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DependencyObject, IDependencyObjectFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DependencyObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDependencyObject>();
}
unsafe impl windows_core::Interface for DependencyObject {
    type Vtable = <IDependencyObject as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDependencyObject as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DependencyObject {
    const NAME: &'static str = "Windows.UI.Xaml.DependencyObject";
}
unsafe impl Send for DependencyObject {}
unsafe impl Sync for DependencyObject {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameworkElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FrameworkElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    FrameworkElement,
    IAnimationObject,
    IVisualElement,
    UIElement,
    DependencyObject
);
impl FrameworkElement {
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetTag)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetLanguage)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetDataContext)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Parent(&self) -> windows_core::Result<DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Loaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLoaded(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Unloaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unloaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveUnloaded(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveUnloaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveSizeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveSizeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveLayoutUpdated(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLayoutUpdated)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(name),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveDataContextChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDataContextChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveLoading(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoading)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveActualThemeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveActualThemeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveEffectiveViewportChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn new() -> windows_core::Result<FrameworkElement> {
        Self::IFrameworkElementFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                &mut core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnApplyTemplate)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn GoToElementStateCore(
        &self,
        statename: &windows_core::HSTRING,
        usetransitions: bool,
    ) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElementOverrides2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(statename),
                usetransitions,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElementProtected7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateViewport)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn DeferTree<P0>(element: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IFrameworkElementStatics5(|this| unsafe {
            (windows_core::Interface::vtable(this).DeferTree)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
            )
            .ok()
        })
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowDrop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetOpacity)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GotFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GotFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveGotFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGotFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LostFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLostFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLostFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDragEnter(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragEnter)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDragLeave(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragLeave)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDragOver(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragOver)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDrop(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDrop)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerPressed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerMoved)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerReleased)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerEntered)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerExited)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCaptureLost<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCaptureLost(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCanceled<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCanceled)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerWheelChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerWheelChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDoubleTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDoubleTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveHolding(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHolding)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveRightTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveRightTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationInertiaStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationStarted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationDelta(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationDelta)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ReleasePointerCaptures)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn AddHandler<P0, P1>(
        &self,
        routedevent: P0,
        handler: P1,
        handledeventstoo: bool,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).AddHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
                handledeventstoo,
            )
            .ok()
        }
    }
    pub fn RemoveHandler<P0, P1>(&self, routedevent: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateMeasure)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateArrange)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanDrag)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveDragStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDropCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDropCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKey)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn RemoveContextRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveContextCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveAccessKeyInvoked(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveGettingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGettingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveLosingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLosingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveNoFocusCandidateFound(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartBringIntoView)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RemoveCharacterReceived(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveCharacterReceived)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveProcessKeyboardAccelerators(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemovePreviewKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemovePreviewKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveBringIntoViewRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveBringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTranslation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetScale)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTransformMatrix(
        &self,
        value: windows_numerics::Matrix4x4,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransformMatrix)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCenterPoint)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotationAxis)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn GetChildrenInTabFocusOrder(
        &self,
    ) -> windows_core::Result<windows_collections::IIterable<DependencyObject>> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    fn IFrameworkElementFactory<
        R,
        F: FnOnce(&IFrameworkElementFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FrameworkElement, IFrameworkElementFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IFrameworkElementStatics<
        R,
        F: FnOnce(&IFrameworkElementStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FrameworkElement, IFrameworkElementStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IFrameworkElementStatics2<
        R,
        F: FnOnce(&IFrameworkElementStatics2) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            FrameworkElement,
            IFrameworkElementStatics2,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IFrameworkElementStatics4<
        R,
        F: FnOnce(&IFrameworkElementStatics4) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            FrameworkElement,
            IFrameworkElementStatics4,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IFrameworkElementStatics5<
        R,
        F: FnOnce(&IFrameworkElementStatics5) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            FrameworkElement,
            IFrameworkElementStatics5,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IFrameworkElementStatics6<
        R,
        F: FnOnce(&IFrameworkElementStatics6) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            FrameworkElement,
            IFrameworkElementStatics6,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FrameworkElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFrameworkElement>();
}
unsafe impl windows_core::Interface for FrameworkElement {
    type Vtable = <IFrameworkElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFrameworkElement as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FrameworkElement {
    const NAME: &'static str = "Windows.UI.Xaml.FrameworkElement";
}
unsafe impl Send for FrameworkElement {}
unsafe impl Sync for FrameworkElement {}
windows_core::imp::define_interface!(
    IAnimationObject,
    IAnimationObject_Vtbl,
    0xe7141e0a_04b8_4fc5_a4dc_195392e57807
);
impl windows_core::RuntimeType for IAnimationObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IAnimationObject,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for IAnimationObject {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationObject";
}
pub trait IAnimationObject_Impl: windows_core::IUnknownImpl {}
impl IAnimationObject_Vtbl {
    pub const fn new<Identity: IAnimationObject_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAnimationObject, OFFSET>(),
            PopulatePropertyInfo: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAnimationObject as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnimationObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PopulatePropertyInfo: usize,
}
windows_core::imp::define_interface!(
    IButtonBase,
    IButtonBase_Vtbl,
    0xfa002c1a_494e_46cf_91d4_e14a8d798674
);
impl windows_core::RuntimeType for IButtonBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IButtonBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IButtonBase";
}
pub trait IButtonBase_Impl: windows_core::IUnknownImpl {
    fn IsPointerOver(&self) -> windows_core::Result<bool>;
    fn IsPressed(&self) -> windows_core::Result<bool>;
    fn CommandParameter(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn SetCommandParameter(
        &self,
        value: windows_core::Ref<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<()>;
    fn Click(
        &self,
        handler: windows_core::Ref<'_, RoutedEventHandler>,
    ) -> windows_core::Result<i64>;
    fn RemoveClick(&self, token: i64) -> windows_core::Result<()>;
}
impl IButtonBase_Vtbl {
    pub const fn new<Identity: IButtonBase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsPointerOver<Identity: IButtonBase_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IButtonBase_Impl::IsPointerOver(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPressed<Identity: IButtonBase_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IButtonBase_Impl::IsPressed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommandParameter<
            Identity: IButtonBase_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IButtonBase_Impl::CommandParameter(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCommandParameter<
            Identity: IButtonBase_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IButtonBase_Impl::SetCommandParameter(this, core::mem::transmute_copy(&value))
                    .into()
            }
        }
        unsafe extern "system" fn Click<Identity: IButtonBase_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IButtonBase_Impl::Click(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveClick<Identity: IButtonBase_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IButtonBase_Impl::RemoveClick(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IButtonBase, OFFSET>(),
            ClickMode: 0,
            SetClickMode: 0,
            IsPointerOver: IsPointerOver::<Identity, OFFSET>,
            IsPressed: IsPressed::<Identity, OFFSET>,
            Command: 0,
            SetCommand: 0,
            CommandParameter: CommandParameter::<Identity, OFFSET>,
            SetCommandParameter: SetCommandParameter::<Identity, OFFSET>,
            Click: Click::<Identity, OFFSET>,
            RemoveClick: RemoveClick::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IButtonBase as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ClickMode: usize,
    SetClickMode: usize,
    pub IsPointerOver:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    Command: usize,
    SetCommand: usize,
    pub CommandParameter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetCommandParameter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Click: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveClick:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IButtonBaseFactory,
    IButtonBaseFactory_Vtbl,
    0x389b7c71_5220_42b2_9992_2690c1a6702f
);
impl windows_core::RuntimeType for IButtonBaseFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IButtonBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IButtonBaseFactory";
}
pub trait IButtonBaseFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(
        &self,
        baseInterface: windows_core::Ref<'_, windows_core::IInspectable>,
        innerInterface: windows_core::OutRef<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<ButtonBase>;
}
impl IButtonBaseFactory_Vtbl {
    pub const fn new<Identity: IButtonBaseFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<
            Identity: IButtonBaseFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            baseinterface: *mut core::ffi::c_void,
            innerinterface: *mut *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IButtonBaseFactory_Impl::CreateInstance(
                    this,
                    core::mem::transmute_copy(&baseinterface),
                    core::mem::transmute_copy(&innerinterface),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IButtonBaseFactory, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IButtonBaseFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBaseFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IButtonBaseStatics,
    IButtonBaseStatics_Vtbl,
    0x67ef17e1_fe37_474f_9e97_3b5e0b30f2df
);
impl windows_core::RuntimeType for IButtonBaseStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IButtonBaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IButtonBaseStatics";
}
pub trait IButtonBaseStatics_Impl: windows_core::IUnknownImpl {}
impl IButtonBaseStatics_Vtbl {
    pub const fn new<Identity: IButtonBaseStatics_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IButtonBaseStatics, OFFSET>(),
            ClickModeProperty: 0,
            IsPointerOverProperty: 0,
            IsPressedProperty: 0,
            CommandProperty: 0,
            CommandParameterProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IButtonBaseStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBaseStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ClickModeProperty: usize,
    IsPointerOverProperty: usize,
    IsPressedProperty: usize,
    CommandProperty: usize,
    CommandParameterProperty: usize,
}
windows_core::imp::define_interface!(
    IClassFactory,
    IClassFactory_Vtbl,
    0x00000001_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IClassFactory, windows_core::IUnknown);
impl IClassFactory {
    pub unsafe fn CreateInstance<P0, T>(&self, punkouter: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).CreateInstance)(
                windows_core::Interface::as_raw(self),
                punkouter.param().abi(),
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn LockServer(&self, flock: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).LockServer)(
                windows_core::Interface::as_raw(self),
                flock.into(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub LockServer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
pub trait IClassFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(
        &self,
        punkouter: windows_core::Ref<'_, windows_core::IUnknown>,
        riid: *const windows_core::GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn LockServer(&self, flock: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IClassFactory_Vtbl {
    pub const fn new<Identity: IClassFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<
            Identity: IClassFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            punkouter: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            ppvobject: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClassFactory_Impl::CreateInstance(
                    this,
                    core::mem::transmute_copy(&punkouter),
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppvobject),
                )
                .into()
            }
        }
        unsafe extern "system" fn LockServer<Identity: IClassFactory_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            flock: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClassFactory_Impl::LockServer(this, core::mem::transmute_copy(&flock)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            LockServer: LockServer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClassFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IClassFactory {}
windows_core::imp::define_interface!(
    IContentControl,
    IContentControl_Vtbl,
    0xa26dd1dc_cd44_435c_be94_01d6241c231c
);
impl windows_core::RuntimeType for IContentControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IContentControl {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IContentControl";
}
pub trait IContentControl_Impl: windows_core::IUnknownImpl {
    fn Content(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn SetContent(
        &self,
        value: windows_core::Ref<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<()>;
}
impl IContentControl_Vtbl {
    pub const fn new<Identity: IContentControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Content<Identity: IContentControl_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContentControl_Impl::Content(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContent<
            Identity: IContentControl_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContentControl_Impl::SetContent(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContentControl, OFFSET>(),
            Content: Content::<Identity, OFFSET>,
            SetContent: SetContent::<Identity, OFFSET>,
            ContentTemplate: 0,
            SetContentTemplate: 0,
            ContentTemplateSelector: 0,
            SetContentTemplateSelector: 0,
            ContentTransitions: 0,
            SetContentTransitions: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContentControl as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ContentTemplate: usize,
    SetContentTemplate: usize,
    ContentTemplateSelector: usize,
    SetContentTemplateSelector: usize,
    ContentTransitions: usize,
    SetContentTransitions: usize,
}
windows_core::imp::define_interface!(
    IContentControl2,
    IContentControl2_Vtbl,
    0x652c208c_8ccf_4399_bd3e_5a015aa1bc03
);
impl windows_core::RuntimeType for IContentControl2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IContentControl2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IContentControl2";
}
pub trait IContentControl2_Impl: windows_core::IUnknownImpl {
    fn ContentTemplateRoot(&self) -> windows_core::Result<UIElement>;
}
impl IContentControl2_Vtbl {
    pub const fn new<Identity: IContentControl2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ContentTemplateRoot<
            Identity: IContentControl2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContentControl2_Impl::ContentTemplateRoot(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContentControl2, OFFSET>(),
            ContentTemplateRoot: ContentTemplateRoot::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContentControl2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentControl2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentTemplateRoot: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContentControlFactory,
    IContentControlFactory_Vtbl,
    0xab017b19_196e_4ed5_a27f_55c58fb1673f
);
impl windows_core::RuntimeType for IContentControlFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IContentControlFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IContentControlFactory";
}
pub trait IContentControlFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(
        &self,
        baseInterface: windows_core::Ref<'_, windows_core::IInspectable>,
        innerInterface: windows_core::OutRef<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<ContentControl>;
}
impl IContentControlFactory_Vtbl {
    pub const fn new<Identity: IContentControlFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<
            Identity: IContentControlFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            baseinterface: *mut core::ffi::c_void,
            innerinterface: *mut *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContentControlFactory_Impl::CreateInstance(
                    this,
                    core::mem::transmute_copy(&baseinterface),
                    core::mem::transmute_copy(&innerinterface),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContentControlFactory, OFFSET>(
            ),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContentControlFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentControlFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContentControlOverrides,
    IContentControlOverrides_Vtbl,
    0x5f4c0b10_e38e_4b5d_be1a_5ed04246a635
);
impl windows_core::RuntimeType for IContentControlOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IContentControlOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IContentControlOverrides";
}
pub trait IContentControlOverrides_Impl: windows_core::IUnknownImpl {
    fn OnContentChanged(
        &self,
        oldContent: windows_core::Ref<'_, windows_core::IInspectable>,
        newContent: windows_core::Ref<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<()>;
}
impl IContentControlOverrides_Vtbl {
    pub const fn new<Identity: IContentControlOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnContentChanged<
            Identity: IContentControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            oldcontent: *mut core::ffi::c_void,
            newcontent: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContentControlOverrides_Impl::OnContentChanged(
                    this,
                    core::mem::transmute_copy(&oldcontent),
                    core::mem::transmute_copy(&newcontent),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IContentControlOverrides,
                OFFSET,
            >(),
            OnContentChanged: OnContentChanged::<Identity, OFFSET>,
            OnContentTemplateChanged: 0,
            OnContentTemplateSelectorChanged: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContentControlOverrides as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentControlOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnContentChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    OnContentTemplateChanged: usize,
    OnContentTemplateSelectorChanged: usize,
}
windows_core::imp::define_interface!(
    IContentControlStatics,
    IContentControlStatics_Vtbl,
    0x116982a4_5cf5_4c05_a53f_15e4876c85ec
);
impl windows_core::RuntimeType for IContentControlStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IContentControlStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IContentControlStatics";
}
pub trait IContentControlStatics_Impl: windows_core::IUnknownImpl {}
impl IContentControlStatics_Vtbl {
    pub const fn new<Identity: IContentControlStatics_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContentControlStatics, OFFSET>(
            ),
            ContentProperty: 0,
            ContentTemplateProperty: 0,
            ContentTemplateSelectorProperty: 0,
            ContentTransitionsProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContentControlStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentControlStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ContentProperty: usize,
    ContentTemplateProperty: usize,
    ContentTemplateSelectorProperty: usize,
    ContentTransitionsProperty: usize,
}
windows_core::imp::define_interface!(
    IControl,
    IControl_Vtbl,
    0xa8912263_2951_4f58_a9c5_5a134eaa7f07
);
impl windows_core::RuntimeType for IControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControl {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControl";
}
pub trait IControl_Impl: windows_core::IUnknownImpl {
    fn FontSize(&self) -> windows_core::Result<f64>;
    fn SetFontSize(&self, value: f64) -> windows_core::Result<()>;
    fn CharacterSpacing(&self) -> windows_core::Result<i32>;
    fn SetCharacterSpacing(&self, value: i32) -> windows_core::Result<()>;
    fn IsTabStop(&self) -> windows_core::Result<bool>;
    fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()>;
    fn IsEnabled(&self) -> windows_core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()>;
    fn TabIndex(&self) -> windows_core::Result<i32>;
    fn SetTabIndex(&self, value: i32) -> windows_core::Result<()>;
    fn RemoveIsEnabledChanged(&self, token: i64) -> windows_core::Result<()>;
    fn ApplyTemplate(&self) -> windows_core::Result<bool>;
}
impl IControl_Vtbl {
    pub const fn new<Identity: IControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FontSize<Identity: IControl_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControl_Impl::FontSize(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFontSize<Identity: IControl_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl_Impl::SetFontSize(this, value).into()
            }
        }
        unsafe extern "system" fn CharacterSpacing<Identity: IControl_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControl_Impl::CharacterSpacing(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCharacterSpacing<
            Identity: IControl_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl_Impl::SetCharacterSpacing(this, value).into()
            }
        }
        unsafe extern "system" fn IsTabStop<Identity: IControl_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControl_Impl::IsTabStop(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsTabStop<Identity: IControl_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl_Impl::SetIsTabStop(this, value).into()
            }
        }
        unsafe extern "system" fn IsEnabled<Identity: IControl_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControl_Impl::IsEnabled(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsEnabled<Identity: IControl_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl_Impl::SetIsEnabled(this, value).into()
            }
        }
        unsafe extern "system" fn TabIndex<Identity: IControl_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControl_Impl::TabIndex(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTabIndex<Identity: IControl_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl_Impl::SetTabIndex(this, value).into()
            }
        }
        unsafe extern "system" fn RemoveIsEnabledChanged<
            Identity: IControl_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl_Impl::RemoveIsEnabledChanged(this, token).into()
            }
        }
        unsafe extern "system" fn ApplyTemplate<Identity: IControl_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControl_Impl::ApplyTemplate(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControl, OFFSET>(),
            FontSize: FontSize::<Identity, OFFSET>,
            SetFontSize: SetFontSize::<Identity, OFFSET>,
            FontFamily: 0,
            SetFontFamily: 0,
            FontWeight: 0,
            SetFontWeight: 0,
            FontStyle: 0,
            SetFontStyle: 0,
            FontStretch: 0,
            SetFontStretch: 0,
            CharacterSpacing: CharacterSpacing::<Identity, OFFSET>,
            SetCharacterSpacing: SetCharacterSpacing::<Identity, OFFSET>,
            Foreground: 0,
            SetForeground: 0,
            IsTabStop: IsTabStop::<Identity, OFFSET>,
            SetIsTabStop: SetIsTabStop::<Identity, OFFSET>,
            IsEnabled: IsEnabled::<Identity, OFFSET>,
            SetIsEnabled: SetIsEnabled::<Identity, OFFSET>,
            TabIndex: TabIndex::<Identity, OFFSET>,
            SetTabIndex: SetTabIndex::<Identity, OFFSET>,
            TabNavigation: 0,
            SetTabNavigation: 0,
            Template: 0,
            SetTemplate: 0,
            Padding: 0,
            SetPadding: 0,
            HorizontalContentAlignment: 0,
            SetHorizontalContentAlignment: 0,
            VerticalContentAlignment: 0,
            SetVerticalContentAlignment: 0,
            Background: 0,
            SetBackground: 0,
            BorderThickness: 0,
            SetBorderThickness: 0,
            BorderBrush: 0,
            SetBorderBrush: 0,
            FocusState: 0,
            IsEnabledChanged: 0,
            RemoveIsEnabledChanged: RemoveIsEnabledChanged::<Identity, OFFSET>,
            ApplyTemplate: ApplyTemplate::<Identity, OFFSET>,
            Focus: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControl as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FontSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetFontSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    FontFamily: usize,
    SetFontFamily: usize,
    FontWeight: usize,
    SetFontWeight: usize,
    FontStyle: usize,
    SetFontStyle: usize,
    FontStretch: usize,
    SetFontStretch: usize,
    pub CharacterSpacing:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCharacterSpacing:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    Foreground: usize,
    SetForeground: usize,
    pub IsTabStop:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsTabStop:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub TabIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTabIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    TabNavigation: usize,
    SetTabNavigation: usize,
    Template: usize,
    SetTemplate: usize,
    Padding: usize,
    SetPadding: usize,
    HorizontalContentAlignment: usize,
    SetHorizontalContentAlignment: usize,
    VerticalContentAlignment: usize,
    SetVerticalContentAlignment: usize,
    Background: usize,
    SetBackground: usize,
    BorderThickness: usize,
    SetBorderThickness: usize,
    BorderBrush: usize,
    SetBorderBrush: usize,
    FocusState: usize,
    IsEnabledChanged: usize,
    pub RemoveIsEnabledChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ApplyTemplate:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    Focus: usize,
}
windows_core::imp::define_interface!(
    IControl2,
    IControl2_Vtbl,
    0x43e0fe10_14ec_427e_8c57_dee60df60aa8
);
impl windows_core::RuntimeType for IControl2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControl2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControl2";
}
pub trait IControl2_Impl: windows_core::IUnknownImpl {
    fn IsTextScaleFactorEnabled(&self) -> windows_core::Result<bool>;
    fn SetIsTextScaleFactorEnabled(&self, value: bool) -> windows_core::Result<()>;
}
impl IControl2_Vtbl {
    pub const fn new<Identity: IControl2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsTextScaleFactorEnabled<
            Identity: IControl2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControl2_Impl::IsTextScaleFactorEnabled(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsTextScaleFactorEnabled<
            Identity: IControl2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl2_Impl::SetIsTextScaleFactorEnabled(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControl2, OFFSET>(),
            IsTextScaleFactorEnabled: IsTextScaleFactorEnabled::<Identity, OFFSET>,
            SetIsTextScaleFactorEnabled: SetIsTextScaleFactorEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControl2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControl2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsTextScaleFactorEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsTextScaleFactorEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IControl3,
    IControl3_Vtbl,
    0x077b29e4_dade_4f55_9b96_09e21b28eb1c
);
impl windows_core::RuntimeType for IControl3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControl3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControl3";
}
pub trait IControl3_Impl: windows_core::IUnknownImpl {
    fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool>;
    fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()>;
}
impl IControl3_Vtbl {
    pub const fn new<Identity: IControl3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UseSystemFocusVisuals<
            Identity: IControl3_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControl3_Impl::UseSystemFocusVisuals(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseSystemFocusVisuals<
            Identity: IControl3_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl3_Impl::SetUseSystemFocusVisuals(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControl3, OFFSET>(),
            UseSystemFocusVisuals: UseSystemFocusVisuals::<Identity, OFFSET>,
            SetUseSystemFocusVisuals: SetUseSystemFocusVisuals::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControl3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControl3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UseSystemFocusVisuals:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetUseSystemFocusVisuals:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IControl4,
    IControl4_Vtbl,
    0x0e18aeee_5f2e_44ea_8513_d3875c0a7513
);
impl windows_core::RuntimeType for IControl4 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControl4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControl4";
}
pub trait IControl4_Impl: windows_core::IUnknownImpl {
    fn IsFocusEngagementEnabled(&self) -> windows_core::Result<bool>;
    fn SetIsFocusEngagementEnabled(&self, value: bool) -> windows_core::Result<()>;
    fn IsFocusEngaged(&self) -> windows_core::Result<bool>;
    fn SetIsFocusEngaged(&self, value: bool) -> windows_core::Result<()>;
    fn XYFocusLeft(&self) -> windows_core::Result<DependencyObject>;
    fn SetXYFocusLeft(
        &self,
        value: windows_core::Ref<'_, DependencyObject>,
    ) -> windows_core::Result<()>;
    fn XYFocusRight(&self) -> windows_core::Result<DependencyObject>;
    fn SetXYFocusRight(
        &self,
        value: windows_core::Ref<'_, DependencyObject>,
    ) -> windows_core::Result<()>;
    fn XYFocusUp(&self) -> windows_core::Result<DependencyObject>;
    fn SetXYFocusUp(
        &self,
        value: windows_core::Ref<'_, DependencyObject>,
    ) -> windows_core::Result<()>;
    fn XYFocusDown(&self) -> windows_core::Result<DependencyObject>;
    fn SetXYFocusDown(
        &self,
        value: windows_core::Ref<'_, DependencyObject>,
    ) -> windows_core::Result<()>;
    fn RemoveFocusEngaged(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveFocusDisengaged(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveFocusEngagement(&self) -> windows_core::Result<()>;
}
impl IControl4_Vtbl {
    pub const fn new<Identity: IControl4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsFocusEngagementEnabled<
            Identity: IControl4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControl4_Impl::IsFocusEngagementEnabled(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsFocusEngagementEnabled<
            Identity: IControl4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl4_Impl::SetIsFocusEngagementEnabled(this, value).into()
            }
        }
        unsafe extern "system" fn IsFocusEngaged<Identity: IControl4_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControl4_Impl::IsFocusEngaged(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsFocusEngaged<
            Identity: IControl4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl4_Impl::SetIsFocusEngaged(this, value).into()
            }
        }
        unsafe extern "system" fn XYFocusLeft<Identity: IControl4_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControl4_Impl::XYFocusLeft(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetXYFocusLeft<Identity: IControl4_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl4_Impl::SetXYFocusLeft(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn XYFocusRight<Identity: IControl4_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControl4_Impl::XYFocusRight(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetXYFocusRight<Identity: IControl4_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl4_Impl::SetXYFocusRight(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn XYFocusUp<Identity: IControl4_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControl4_Impl::XYFocusUp(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetXYFocusUp<Identity: IControl4_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl4_Impl::SetXYFocusUp(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn XYFocusDown<Identity: IControl4_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControl4_Impl::XYFocusDown(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetXYFocusDown<Identity: IControl4_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl4_Impl::SetXYFocusDown(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn RemoveFocusEngaged<
            Identity: IControl4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl4_Impl::RemoveFocusEngaged(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveFocusDisengaged<
            Identity: IControl4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl4_Impl::RemoveFocusDisengaged(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveFocusEngagement<
            Identity: IControl4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControl4_Impl::RemoveFocusEngagement(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControl4, OFFSET>(),
            IsFocusEngagementEnabled: IsFocusEngagementEnabled::<Identity, OFFSET>,
            SetIsFocusEngagementEnabled: SetIsFocusEngagementEnabled::<Identity, OFFSET>,
            IsFocusEngaged: IsFocusEngaged::<Identity, OFFSET>,
            SetIsFocusEngaged: SetIsFocusEngaged::<Identity, OFFSET>,
            RequiresPointer: 0,
            SetRequiresPointer: 0,
            XYFocusLeft: XYFocusLeft::<Identity, OFFSET>,
            SetXYFocusLeft: SetXYFocusLeft::<Identity, OFFSET>,
            XYFocusRight: XYFocusRight::<Identity, OFFSET>,
            SetXYFocusRight: SetXYFocusRight::<Identity, OFFSET>,
            XYFocusUp: XYFocusUp::<Identity, OFFSET>,
            SetXYFocusUp: SetXYFocusUp::<Identity, OFFSET>,
            XYFocusDown: XYFocusDown::<Identity, OFFSET>,
            SetXYFocusDown: SetXYFocusDown::<Identity, OFFSET>,
            ElementSoundMode: 0,
            SetElementSoundMode: 0,
            FocusEngaged: 0,
            RemoveFocusEngaged: RemoveFocusEngaged::<Identity, OFFSET>,
            FocusDisengaged: 0,
            RemoveFocusDisengaged: RemoveFocusDisengaged::<Identity, OFFSET>,
            RemoveFocusEngagement: RemoveFocusEngagement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControl4 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControl4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsFocusEngagementEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsFocusEngagementEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsFocusEngaged:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsFocusEngaged:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    RequiresPointer: usize,
    SetRequiresPointer: usize,
    pub XYFocusLeft: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetXYFocusLeft: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub XYFocusRight: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetXYFocusRight: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub XYFocusUp: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetXYFocusUp: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub XYFocusDown: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetXYFocusDown: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ElementSoundMode: usize,
    SetElementSoundMode: usize,
    FocusEngaged: usize,
    pub RemoveFocusEngaged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    FocusDisengaged: usize,
    pub RemoveFocusDisengaged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub RemoveFocusEngagement:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IControl5,
    IControl5_Vtbl,
    0x5ebb51d5_9e58_49b7_bc2d_0155ff118664
);
impl windows_core::RuntimeType for IControl5 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControl5 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControl5";
}
pub trait IControl5_Impl: windows_core::IUnknownImpl {}
impl IControl5_Vtbl {
    pub const fn new<Identity: IControl5_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControl5, OFFSET>(),
            DefaultStyleResourceUri: 0,
            SetDefaultStyleResourceUri: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControl5 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControl5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    DefaultStyleResourceUri: usize,
    SetDefaultStyleResourceUri: usize,
}
windows_core::imp::define_interface!(
    IControl7,
    IControl7_Vtbl,
    0x3550dbe1_e561_5934_a3ef_7db8bf6e3b45
);
impl windows_core::RuntimeType for IControl7 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControl7 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControl7";
}
pub trait IControl7_Impl: windows_core::IUnknownImpl {}
impl IControl7_Vtbl {
    pub const fn new<Identity: IControl7_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControl7, OFFSET>(),
            BackgroundSizing: 0,
            SetBackgroundSizing: 0,
            CornerRadius: 0,
            SetCornerRadius: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControl7 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControl7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    BackgroundSizing: usize,
    SetBackgroundSizing: usize,
    CornerRadius: usize,
    SetCornerRadius: usize,
}
windows_core::imp::define_interface!(
    IControlFactory,
    IControlFactory_Vtbl,
    0xb12b1d5a_6419_4e16_b212_a45336b75778
);
impl windows_core::RuntimeType for IControlFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControlFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControlFactory";
}
pub trait IControlFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(
        &self,
        baseInterface: windows_core::Ref<'_, windows_core::IInspectable>,
        innerInterface: windows_core::OutRef<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<Control>;
}
impl IControlFactory_Vtbl {
    pub const fn new<Identity: IControlFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<
            Identity: IControlFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            baseinterface: *mut core::ffi::c_void,
            innerinterface: *mut *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControlFactory_Impl::CreateInstance(
                    this,
                    core::mem::transmute_copy(&baseinterface),
                    core::mem::transmute_copy(&innerinterface),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControlFactory, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IControlOverrides,
    IControlOverrides_Vtbl,
    0xa09691df_9824_41fe_b530_b0d8990e64c1
);
impl windows_core::RuntimeType for IControlOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControlOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControlOverrides";
}
pub trait IControlOverrides_Impl: windows_core::IUnknownImpl {
    fn OnPointerEntered(
        &self,
        e: windows_core::Ref<'_, PointerRoutedEventArgs>,
    ) -> windows_core::Result<()>;
    fn OnPointerPressed(
        &self,
        e: windows_core::Ref<'_, PointerRoutedEventArgs>,
    ) -> windows_core::Result<()>;
    fn OnPointerMoved(
        &self,
        e: windows_core::Ref<'_, PointerRoutedEventArgs>,
    ) -> windows_core::Result<()>;
    fn OnPointerReleased(
        &self,
        e: windows_core::Ref<'_, PointerRoutedEventArgs>,
    ) -> windows_core::Result<()>;
    fn OnPointerExited(
        &self,
        e: windows_core::Ref<'_, PointerRoutedEventArgs>,
    ) -> windows_core::Result<()>;
    fn OnPointerCaptureLost(
        &self,
        e: windows_core::Ref<'_, PointerRoutedEventArgs>,
    ) -> windows_core::Result<()>;
    fn OnPointerCanceled(
        &self,
        e: windows_core::Ref<'_, PointerRoutedEventArgs>,
    ) -> windows_core::Result<()>;
    fn OnPointerWheelChanged(
        &self,
        e: windows_core::Ref<'_, PointerRoutedEventArgs>,
    ) -> windows_core::Result<()>;
    fn OnGotFocus(&self, e: windows_core::Ref<'_, RoutedEventArgs>) -> windows_core::Result<()>;
    fn OnLostFocus(&self, e: windows_core::Ref<'_, RoutedEventArgs>) -> windows_core::Result<()>;
}
impl IControlOverrides_Vtbl {
    pub const fn new<Identity: IControlOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnPointerEntered<
            Identity: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            e: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlOverrides_Impl::OnPointerEntered(this, core::mem::transmute_copy(&e)).into()
            }
        }
        unsafe extern "system" fn OnPointerPressed<
            Identity: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            e: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlOverrides_Impl::OnPointerPressed(this, core::mem::transmute_copy(&e)).into()
            }
        }
        unsafe extern "system" fn OnPointerMoved<
            Identity: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            e: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlOverrides_Impl::OnPointerMoved(this, core::mem::transmute_copy(&e)).into()
            }
        }
        unsafe extern "system" fn OnPointerReleased<
            Identity: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            e: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlOverrides_Impl::OnPointerReleased(this, core::mem::transmute_copy(&e))
                    .into()
            }
        }
        unsafe extern "system" fn OnPointerExited<
            Identity: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            e: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlOverrides_Impl::OnPointerExited(this, core::mem::transmute_copy(&e)).into()
            }
        }
        unsafe extern "system" fn OnPointerCaptureLost<
            Identity: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            e: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlOverrides_Impl::OnPointerCaptureLost(this, core::mem::transmute_copy(&e))
                    .into()
            }
        }
        unsafe extern "system" fn OnPointerCanceled<
            Identity: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            e: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlOverrides_Impl::OnPointerCanceled(this, core::mem::transmute_copy(&e))
                    .into()
            }
        }
        unsafe extern "system" fn OnPointerWheelChanged<
            Identity: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            e: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlOverrides_Impl::OnPointerWheelChanged(this, core::mem::transmute_copy(&e))
                    .into()
            }
        }
        unsafe extern "system" fn OnGotFocus<
            Identity: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            e: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlOverrides_Impl::OnGotFocus(this, core::mem::transmute_copy(&e)).into()
            }
        }
        unsafe extern "system" fn OnLostFocus<
            Identity: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            e: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlOverrides_Impl::OnLostFocus(this, core::mem::transmute_copy(&e)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControlOverrides, OFFSET>(),
            OnPointerEntered: OnPointerEntered::<Identity, OFFSET>,
            OnPointerPressed: OnPointerPressed::<Identity, OFFSET>,
            OnPointerMoved: OnPointerMoved::<Identity, OFFSET>,
            OnPointerReleased: OnPointerReleased::<Identity, OFFSET>,
            OnPointerExited: OnPointerExited::<Identity, OFFSET>,
            OnPointerCaptureLost: OnPointerCaptureLost::<Identity, OFFSET>,
            OnPointerCanceled: OnPointerCanceled::<Identity, OFFSET>,
            OnPointerWheelChanged: OnPointerWheelChanged::<Identity, OFFSET>,
            OnTapped: 0,
            OnDoubleTapped: 0,
            OnHolding: 0,
            OnRightTapped: 0,
            OnManipulationStarting: 0,
            OnManipulationInertiaStarting: 0,
            OnManipulationStarted: 0,
            OnManipulationDelta: 0,
            OnManipulationCompleted: 0,
            OnKeyUp: 0,
            OnKeyDown: 0,
            OnGotFocus: OnGotFocus::<Identity, OFFSET>,
            OnLostFocus: OnLostFocus::<Identity, OFFSET>,
            OnDragEnter: 0,
            OnDragLeave: 0,
            OnDragOver: 0,
            OnDrop: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlOverrides as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnPointerEntered: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPointerPressed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPointerMoved: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPointerReleased: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPointerExited: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPointerCaptureLost: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPointerCanceled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPointerWheelChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    OnTapped: usize,
    OnDoubleTapped: usize,
    OnHolding: usize,
    OnRightTapped: usize,
    OnManipulationStarting: usize,
    OnManipulationInertiaStarting: usize,
    OnManipulationStarted: usize,
    OnManipulationDelta: usize,
    OnManipulationCompleted: usize,
    OnKeyUp: usize,
    OnKeyDown: usize,
    pub OnGotFocus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnLostFocus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    OnDragEnter: usize,
    OnDragLeave: usize,
    OnDragOver: usize,
    OnDrop: usize,
}
windows_core::imp::define_interface!(
    IControlOverrides6,
    IControlOverrides6_Vtbl,
    0xe194c96a_d8cc_4339_a0e1_2e9c420879ec
);
impl windows_core::RuntimeType for IControlOverrides6 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControlOverrides6 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControlOverrides6";
}
pub trait IControlOverrides6_Impl: windows_core::IUnknownImpl {}
impl IControlOverrides6_Vtbl {
    pub const fn new<Identity: IControlOverrides6_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControlOverrides6, OFFSET>(),
            OnPreviewKeyDown: 0,
            OnPreviewKeyUp: 0,
            OnCharacterReceived: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlOverrides6 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlOverrides6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    OnPreviewKeyDown: usize,
    OnPreviewKeyUp: usize,
    OnCharacterReceived: usize,
}
windows_core::imp::define_interface!(
    IControlProtected,
    IControlProtected_Vtbl,
    0x1791587a_2a7c_4981_a2d8_ab99fffc65b9
);
impl windows_core::RuntimeType for IControlProtected {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControlProtected {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControlProtected";
}
pub trait IControlProtected_Impl: windows_core::IUnknownImpl {
    fn DefaultStyleKey(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn SetDefaultStyleKey(
        &self,
        value: windows_core::Ref<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<()>;
    fn GetTemplateChild(
        &self,
        childName: &windows_core::HSTRING,
    ) -> windows_core::Result<DependencyObject>;
}
impl IControlProtected_Vtbl {
    pub const fn new<Identity: IControlProtected_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DefaultStyleKey<
            Identity: IControlProtected_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControlProtected_Impl::DefaultStyleKey(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultStyleKey<
            Identity: IControlProtected_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlProtected_Impl::SetDefaultStyleKey(this, core::mem::transmute_copy(&value))
                    .into()
            }
        }
        unsafe extern "system" fn GetTemplateChild<
            Identity: IControlProtected_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            childname: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControlProtected_Impl::GetTemplateChild(
                    this,
                    core::mem::transmute(&childname),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControlProtected, OFFSET>(),
            DefaultStyleKey: DefaultStyleKey::<Identity, OFFSET>,
            SetDefaultStyleKey: SetDefaultStyleKey::<Identity, OFFSET>,
            GetTemplateChild: GetTemplateChild::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlProtected as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlProtected_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DefaultStyleKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetDefaultStyleKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetTemplateChild: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IControlStatics,
    IControlStatics_Vtbl,
    0x473b305b_877d_4c35_8fca_46a05f96607a
);
impl windows_core::RuntimeType for IControlStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControlStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControlStatics";
}
pub trait IControlStatics_Impl: windows_core::IUnknownImpl {}
impl IControlStatics_Vtbl {
    pub const fn new<Identity: IControlStatics_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControlStatics, OFFSET>(),
            FontSizeProperty: 0,
            FontFamilyProperty: 0,
            FontWeightProperty: 0,
            FontStyleProperty: 0,
            FontStretchProperty: 0,
            CharacterSpacingProperty: 0,
            ForegroundProperty: 0,
            IsTabStopProperty: 0,
            IsEnabledProperty: 0,
            TabIndexProperty: 0,
            TabNavigationProperty: 0,
            TemplateProperty: 0,
            PaddingProperty: 0,
            HorizontalContentAlignmentProperty: 0,
            VerticalContentAlignmentProperty: 0,
            BackgroundProperty: 0,
            BorderThicknessProperty: 0,
            BorderBrushProperty: 0,
            DefaultStyleKeyProperty: 0,
            FocusStateProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    FontSizeProperty: usize,
    FontFamilyProperty: usize,
    FontWeightProperty: usize,
    FontStyleProperty: usize,
    FontStretchProperty: usize,
    CharacterSpacingProperty: usize,
    ForegroundProperty: usize,
    IsTabStopProperty: usize,
    IsEnabledProperty: usize,
    TabIndexProperty: usize,
    TabNavigationProperty: usize,
    TemplateProperty: usize,
    PaddingProperty: usize,
    HorizontalContentAlignmentProperty: usize,
    VerticalContentAlignmentProperty: usize,
    BackgroundProperty: usize,
    BorderThicknessProperty: usize,
    BorderBrushProperty: usize,
    DefaultStyleKeyProperty: usize,
    FocusStateProperty: usize,
}
windows_core::imp::define_interface!(
    IControlStatics2,
    IControlStatics2_Vtbl,
    0x32233e36_ab74_4a25_a556_45293135b2c5
);
impl windows_core::RuntimeType for IControlStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControlStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControlStatics2";
}
pub trait IControlStatics2_Impl: windows_core::IUnknownImpl {}
impl IControlStatics2_Vtbl {
    pub const fn new<Identity: IControlStatics2_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControlStatics2, OFFSET>(),
            IsTextScaleFactorEnabledProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlStatics2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsTextScaleFactorEnabledProperty: usize,
}
windows_core::imp::define_interface!(
    IControlStatics3,
    IControlStatics3_Vtbl,
    0xaad15e61_e4b9_4d72_b653_934149aa5003
);
impl windows_core::RuntimeType for IControlStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControlStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControlStatics3";
}
pub trait IControlStatics3_Impl: windows_core::IUnknownImpl {
    fn GetIsTemplateFocusTarget(
        &self,
        element: windows_core::Ref<'_, FrameworkElement>,
    ) -> windows_core::Result<bool>;
    fn SetIsTemplateFocusTarget(
        &self,
        element: windows_core::Ref<'_, FrameworkElement>,
        value: bool,
    ) -> windows_core::Result<()>;
}
impl IControlStatics3_Vtbl {
    pub const fn new<Identity: IControlStatics3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIsTemplateFocusTarget<
            Identity: IControlStatics3_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControlStatics3_Impl::GetIsTemplateFocusTarget(
                    this,
                    core::mem::transmute_copy(&element),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsTemplateFocusTarget<
            Identity: IControlStatics3_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlStatics3_Impl::SetIsTemplateFocusTarget(
                    this,
                    core::mem::transmute_copy(&element),
                    value,
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControlStatics3, OFFSET>(),
            UseSystemFocusVisualsProperty: 0,
            IsTemplateFocusTargetProperty: 0,
            GetIsTemplateFocusTarget: GetIsTemplateFocusTarget::<Identity, OFFSET>,
            SetIsTemplateFocusTarget: SetIsTemplateFocusTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlStatics3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    UseSystemFocusVisualsProperty: usize,
    IsTemplateFocusTargetProperty: usize,
    pub GetIsTemplateFocusTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub SetIsTemplateFocusTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IControlStatics4,
    IControlStatics4_Vtbl,
    0x59fda728_c05a_4ec2_8a4b_06ad69b66e15
);
impl windows_core::RuntimeType for IControlStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControlStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControlStatics4";
}
pub trait IControlStatics4_Impl: windows_core::IUnknownImpl {}
impl IControlStatics4_Vtbl {
    pub const fn new<Identity: IControlStatics4_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControlStatics4, OFFSET>(),
            IsFocusEngagementEnabledProperty: 0,
            IsFocusEngagedProperty: 0,
            RequiresPointerProperty: 0,
            XYFocusLeftProperty: 0,
            XYFocusRightProperty: 0,
            XYFocusUpProperty: 0,
            XYFocusDownProperty: 0,
            ElementSoundModeProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlStatics4 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsFocusEngagementEnabledProperty: usize,
    IsFocusEngagedProperty: usize,
    RequiresPointerProperty: usize,
    XYFocusLeftProperty: usize,
    XYFocusRightProperty: usize,
    XYFocusUpProperty: usize,
    XYFocusDownProperty: usize,
    ElementSoundModeProperty: usize,
}
windows_core::imp::define_interface!(
    IControlStatics5,
    IControlStatics5_Vtbl,
    0xf635b5a3_ef8e_404f_877b_5580473710a6
);
impl windows_core::RuntimeType for IControlStatics5 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControlStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControlStatics5";
}
pub trait IControlStatics5_Impl: windows_core::IUnknownImpl {
    fn GetIsTemplateKeyTipTarget(
        &self,
        element: windows_core::Ref<'_, DependencyObject>,
    ) -> windows_core::Result<bool>;
    fn SetIsTemplateKeyTipTarget(
        &self,
        element: windows_core::Ref<'_, DependencyObject>,
        value: bool,
    ) -> windows_core::Result<()>;
}
impl IControlStatics5_Vtbl {
    pub const fn new<Identity: IControlStatics5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIsTemplateKeyTipTarget<
            Identity: IControlStatics5_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControlStatics5_Impl::GetIsTemplateKeyTipTarget(
                    this,
                    core::mem::transmute_copy(&element),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsTemplateKeyTipTarget<
            Identity: IControlStatics5_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlStatics5_Impl::SetIsTemplateKeyTipTarget(
                    this,
                    core::mem::transmute_copy(&element),
                    value,
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControlStatics5, OFFSET>(),
            DefaultStyleResourceUriProperty: 0,
            IsTemplateKeyTipTargetProperty: 0,
            GetIsTemplateKeyTipTarget: GetIsTemplateKeyTipTarget::<Identity, OFFSET>,
            SetIsTemplateKeyTipTarget: SetIsTemplateKeyTipTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlStatics5 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlStatics5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    DefaultStyleResourceUriProperty: usize,
    IsTemplateKeyTipTargetProperty: usize,
    pub GetIsTemplateKeyTipTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub SetIsTemplateKeyTipTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IControlStatics7,
    IControlStatics7_Vtbl,
    0xaee1ca45_c910_5dab_a266_fdf50c0bedd2
);
impl windows_core::RuntimeType for IControlStatics7 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IControlStatics7 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControlStatics7";
}
pub trait IControlStatics7_Impl: windows_core::IUnknownImpl {}
impl IControlStatics7_Vtbl {
    pub const fn new<Identity: IControlStatics7_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControlStatics7, OFFSET>(),
            BackgroundSizingProperty: 0,
            CornerRadiusProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlStatics7 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlStatics7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    BackgroundSizingProperty: usize,
    CornerRadiusProperty: usize,
}
windows_core::imp::define_interface!(
    IDependencyObject,
    IDependencyObject_Vtbl,
    0x5c526665_f60e_4912_af59_5fe0680f089d
);
impl windows_core::RuntimeType for IDependencyObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IDependencyObject {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyObject";
}
pub trait IDependencyObject_Impl: windows_core::IUnknownImpl {}
impl IDependencyObject_Vtbl {
    pub const fn new<Identity: IDependencyObject_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDependencyObject, OFFSET>(),
            GetValue: 0,
            SetValue: 0,
            ClearValue: 0,
            ReadLocalValue: 0,
            GetAnimationBaseValue: 0,
            Dispatcher: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDependencyObject as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDependencyObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    GetValue: usize,
    SetValue: usize,
    ClearValue: usize,
    ReadLocalValue: usize,
    GetAnimationBaseValue: usize,
    Dispatcher: usize,
}
windows_core::imp::define_interface!(
    IDependencyObject2,
    IDependencyObject2_Vtbl,
    0x29fed85d_3d22_43a1_add0_17027c08b212
);
impl windows_core::RuntimeType for IDependencyObject2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IDependencyObject2 {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyObject2";
}
pub trait IDependencyObject2_Impl: windows_core::IUnknownImpl {}
impl IDependencyObject2_Vtbl {
    pub const fn new<Identity: IDependencyObject2_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDependencyObject2, OFFSET>(),
            RegisterPropertyChangedCallback: 0,
            UnregisterPropertyChangedCallback: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDependencyObject2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDependencyObject2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    RegisterPropertyChangedCallback: usize,
    UnregisterPropertyChangedCallback: usize,
}
windows_core::imp::define_interface!(
    IDependencyObjectFactory,
    IDependencyObjectFactory_Vtbl,
    0x9a03af92_7d8a_4937_884f_ecf34fe02acb
);
impl windows_core::RuntimeType for IDependencyObjectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IDependencyObjectFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyObjectFactory";
}
pub trait IDependencyObjectFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(
        &self,
        baseInterface: windows_core::Ref<'_, windows_core::IInspectable>,
        innerInterface: windows_core::OutRef<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<DependencyObject>;
}
impl IDependencyObjectFactory_Vtbl {
    pub const fn new<Identity: IDependencyObjectFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<
            Identity: IDependencyObjectFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            baseinterface: *mut core::ffi::c_void,
            innerinterface: *mut *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDependencyObjectFactory_Impl::CreateInstance(
                    this,
                    core::mem::transmute_copy(&baseinterface),
                    core::mem::transmute_copy(&innerinterface),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IDependencyObjectFactory,
                OFFSET,
            >(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDependencyObjectFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDependencyObjectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElement,
    IFrameworkElement_Vtbl,
    0xa391d09b_4a99_4b7c_9d8d_6fa5d01f6fbf
);
impl windows_core::RuntimeType for IFrameworkElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IFrameworkElement {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement";
}
pub trait IFrameworkElement_Impl: windows_core::IUnknownImpl {
    fn Tag(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn SetTag(
        &self,
        value: windows_core::Ref<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<()>;
    fn Language(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn ActualWidth(&self) -> windows_core::Result<f64>;
    fn ActualHeight(&self) -> windows_core::Result<f64>;
    fn Width(&self) -> windows_core::Result<f64>;
    fn SetWidth(&self, value: f64) -> windows_core::Result<()>;
    fn Height(&self) -> windows_core::Result<f64>;
    fn SetHeight(&self, value: f64) -> windows_core::Result<()>;
    fn MinWidth(&self) -> windows_core::Result<f64>;
    fn SetMinWidth(&self, value: f64) -> windows_core::Result<()>;
    fn MaxWidth(&self) -> windows_core::Result<f64>;
    fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()>;
    fn MinHeight(&self) -> windows_core::Result<f64>;
    fn SetMinHeight(&self, value: f64) -> windows_core::Result<()>;
    fn MaxHeight(&self) -> windows_core::Result<f64>;
    fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn SetDataContext(
        &self,
        value: windows_core::Ref<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<()>;
    fn Parent(&self) -> windows_core::Result<DependencyObject>;
    fn Loaded(
        &self,
        handler: windows_core::Ref<'_, RoutedEventHandler>,
    ) -> windows_core::Result<i64>;
    fn RemoveLoaded(&self, token: i64) -> windows_core::Result<()>;
    fn Unloaded(
        &self,
        handler: windows_core::Ref<'_, RoutedEventHandler>,
    ) -> windows_core::Result<i64>;
    fn RemoveUnloaded(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveSizeChanged(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveLayoutUpdated(&self, token: i64) -> windows_core::Result<()>;
    fn FindName(
        &self,
        name: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::IInspectable>;
}
impl IFrameworkElement_Vtbl {
    pub const fn new<Identity: IFrameworkElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Tag<Identity: IFrameworkElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::Tag(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTag<Identity: IFrameworkElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement_Impl::SetTag(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Language<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::Language(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLanguage<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement_Impl::SetLanguage(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ActualWidth<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::ActualWidth(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActualHeight<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::ActualHeight(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Width<Identity: IFrameworkElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::Width(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWidth<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement_Impl::SetWidth(this, value).into()
            }
        }
        unsafe extern "system" fn Height<Identity: IFrameworkElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::Height(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHeight<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement_Impl::SetHeight(this, value).into()
            }
        }
        unsafe extern "system" fn MinWidth<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::MinWidth(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinWidth<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement_Impl::SetMinWidth(this, value).into()
            }
        }
        unsafe extern "system" fn MaxWidth<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::MaxWidth(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxWidth<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement_Impl::SetMaxWidth(this, value).into()
            }
        }
        unsafe extern "system" fn MinHeight<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::MinHeight(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinHeight<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement_Impl::SetMinHeight(this, value).into()
            }
        }
        unsafe extern "system" fn MaxHeight<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::MaxHeight(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxHeight<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement_Impl::SetMaxHeight(this, value).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IFrameworkElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::Name(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IFrameworkElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement_Impl::SetName(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn DataContext<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::DataContext(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDataContext<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement_Impl::SetDataContext(this, core::mem::transmute_copy(&value))
                    .into()
            }
        }
        unsafe extern "system" fn Parent<Identity: IFrameworkElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::Parent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Loaded<Identity: IFrameworkElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::Loaded(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveLoaded<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement_Impl::RemoveLoaded(this, token).into()
            }
        }
        unsafe extern "system" fn Unloaded<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::Unloaded(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveUnloaded<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement_Impl::RemoveUnloaded(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveSizeChanged<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement_Impl::RemoveSizeChanged(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveLayoutUpdated<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement_Impl::RemoveLayoutUpdated(this, token).into()
            }
        }
        unsafe extern "system" fn FindName<
            Identity: IFrameworkElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            name: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement_Impl::FindName(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFrameworkElement, OFFSET>(),
            Triggers: 0,
            Resources: 0,
            SetResources: 0,
            Tag: Tag::<Identity, OFFSET>,
            SetTag: SetTag::<Identity, OFFSET>,
            Language: Language::<Identity, OFFSET>,
            SetLanguage: SetLanguage::<Identity, OFFSET>,
            ActualWidth: ActualWidth::<Identity, OFFSET>,
            ActualHeight: ActualHeight::<Identity, OFFSET>,
            Width: Width::<Identity, OFFSET>,
            SetWidth: SetWidth::<Identity, OFFSET>,
            Height: Height::<Identity, OFFSET>,
            SetHeight: SetHeight::<Identity, OFFSET>,
            MinWidth: MinWidth::<Identity, OFFSET>,
            SetMinWidth: SetMinWidth::<Identity, OFFSET>,
            MaxWidth: MaxWidth::<Identity, OFFSET>,
            SetMaxWidth: SetMaxWidth::<Identity, OFFSET>,
            MinHeight: MinHeight::<Identity, OFFSET>,
            SetMinHeight: SetMinHeight::<Identity, OFFSET>,
            MaxHeight: MaxHeight::<Identity, OFFSET>,
            SetMaxHeight: SetMaxHeight::<Identity, OFFSET>,
            HorizontalAlignment: 0,
            SetHorizontalAlignment: 0,
            VerticalAlignment: 0,
            SetVerticalAlignment: 0,
            Margin: 0,
            SetMargin: 0,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            BaseUri: 0,
            DataContext: DataContext::<Identity, OFFSET>,
            SetDataContext: SetDataContext::<Identity, OFFSET>,
            Style: 0,
            SetStyle: 0,
            Parent: Parent::<Identity, OFFSET>,
            FlowDirection: 0,
            SetFlowDirection: 0,
            Loaded: Loaded::<Identity, OFFSET>,
            RemoveLoaded: RemoveLoaded::<Identity, OFFSET>,
            Unloaded: Unloaded::<Identity, OFFSET>,
            RemoveUnloaded: RemoveUnloaded::<Identity, OFFSET>,
            SizeChanged: 0,
            RemoveSizeChanged: RemoveSizeChanged::<Identity, OFFSET>,
            LayoutUpdated: 0,
            RemoveLayoutUpdated: RemoveLayoutUpdated::<Identity, OFFSET>,
            FindName: FindName::<Identity, OFFSET>,
            SetBinding: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElement as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Triggers: usize,
    Resources: usize,
    SetResources: usize,
    pub Tag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ActualWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ActualHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Height:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub MinWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMinWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub MaxWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMaxWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub MinHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMinHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub MaxHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMaxHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    HorizontalAlignment: usize,
    SetHorizontalAlignment: usize,
    VerticalAlignment: usize,
    SetVerticalAlignment: usize,
    Margin: usize,
    SetMargin: usize,
    pub Name: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    BaseUri: usize,
    pub DataContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetDataContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Style: usize,
    SetStyle: usize,
    pub Parent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    FlowDirection: usize,
    SetFlowDirection: usize,
    pub Loaded: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveLoaded:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Unloaded: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveUnloaded:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    SizeChanged: usize,
    pub RemoveSizeChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    LayoutUpdated: usize,
    pub RemoveLayoutUpdated:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub FindName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SetBinding: usize,
}
windows_core::imp::define_interface!(
    IFrameworkElement2,
    IFrameworkElement2_Vtbl,
    0xf19104be_422a_4904_a52f_ee72010429e5
);
impl windows_core::RuntimeType for IFrameworkElement2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IFrameworkElement2 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement2";
}
pub trait IFrameworkElement2_Impl: windows_core::IUnknownImpl {
    fn RemoveDataContextChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl IFrameworkElement2_Vtbl {
    pub const fn new<Identity: IFrameworkElement2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RemoveDataContextChanged<
            Identity: IFrameworkElement2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement2_Impl::RemoveDataContextChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFrameworkElement2, OFFSET>(),
            RequestedTheme: 0,
            SetRequestedTheme: 0,
            DataContextChanged: 0,
            RemoveDataContextChanged: RemoveDataContextChanged::<Identity, OFFSET>,
            GetBindingExpression: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElement2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElement2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    RequestedTheme: usize,
    SetRequestedTheme: usize,
    DataContextChanged: usize,
    pub RemoveDataContextChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    GetBindingExpression: usize,
}
windows_core::imp::define_interface!(
    IFrameworkElement3,
    IFrameworkElement3_Vtbl,
    0xc81c2720_5c52_4bbe_a199_2b1e34f00f70
);
impl windows_core::RuntimeType for IFrameworkElement3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IFrameworkElement3 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement3";
}
pub trait IFrameworkElement3_Impl: windows_core::IUnknownImpl {
    fn RemoveLoading(&self, token: i64) -> windows_core::Result<()>;
}
impl IFrameworkElement3_Vtbl {
    pub const fn new<Identity: IFrameworkElement3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RemoveLoading<
            Identity: IFrameworkElement3_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement3_Impl::RemoveLoading(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFrameworkElement3, OFFSET>(),
            Loading: 0,
            RemoveLoading: RemoveLoading::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElement3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElement3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Loading: usize,
    pub RemoveLoading:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElement4,
    IFrameworkElement4_Vtbl,
    0x6b765bb3_fba3_4404_bdee_1a45d1ca5f21
);
impl windows_core::RuntimeType for IFrameworkElement4 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IFrameworkElement4 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement4";
}
pub trait IFrameworkElement4_Impl: windows_core::IUnknownImpl {
    fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool>;
    fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()>;
    fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool>;
    fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()>;
}
impl IFrameworkElement4_Vtbl {
    pub const fn new<Identity: IFrameworkElement4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AllowFocusOnInteraction<
            Identity: IFrameworkElement4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement4_Impl::AllowFocusOnInteraction(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowFocusOnInteraction<
            Identity: IFrameworkElement4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement4_Impl::SetAllowFocusOnInteraction(this, value).into()
            }
        }
        unsafe extern "system" fn AllowFocusWhenDisabled<
            Identity: IFrameworkElement4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement4_Impl::AllowFocusWhenDisabled(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowFocusWhenDisabled<
            Identity: IFrameworkElement4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement4_Impl::SetAllowFocusWhenDisabled(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFrameworkElement4, OFFSET>(),
            AllowFocusOnInteraction: AllowFocusOnInteraction::<Identity, OFFSET>,
            SetAllowFocusOnInteraction: SetAllowFocusOnInteraction::<Identity, OFFSET>,
            FocusVisualMargin: 0,
            SetFocusVisualMargin: 0,
            FocusVisualSecondaryThickness: 0,
            SetFocusVisualSecondaryThickness: 0,
            FocusVisualPrimaryThickness: 0,
            SetFocusVisualPrimaryThickness: 0,
            FocusVisualSecondaryBrush: 0,
            SetFocusVisualSecondaryBrush: 0,
            FocusVisualPrimaryBrush: 0,
            SetFocusVisualPrimaryBrush: 0,
            AllowFocusWhenDisabled: AllowFocusWhenDisabled::<Identity, OFFSET>,
            SetAllowFocusWhenDisabled: SetAllowFocusWhenDisabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElement4 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElement4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowFocusOnInteraction:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowFocusOnInteraction:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    FocusVisualMargin: usize,
    SetFocusVisualMargin: usize,
    FocusVisualSecondaryThickness: usize,
    SetFocusVisualSecondaryThickness: usize,
    FocusVisualPrimaryThickness: usize,
    SetFocusVisualPrimaryThickness: usize,
    FocusVisualSecondaryBrush: usize,
    SetFocusVisualSecondaryBrush: usize,
    FocusVisualPrimaryBrush: usize,
    SetFocusVisualPrimaryBrush: usize,
    pub AllowFocusWhenDisabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowFocusWhenDisabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElement6,
    IFrameworkElement6_Vtbl,
    0x792a5d91_62a1_40bf_a0ce_f9c131fcb7a7
);
impl windows_core::RuntimeType for IFrameworkElement6 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IFrameworkElement6 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement6";
}
pub trait IFrameworkElement6_Impl: windows_core::IUnknownImpl {
    fn RemoveActualThemeChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl IFrameworkElement6_Vtbl {
    pub const fn new<Identity: IFrameworkElement6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RemoveActualThemeChanged<
            Identity: IFrameworkElement6_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement6_Impl::RemoveActualThemeChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFrameworkElement6, OFFSET>(),
            ActualTheme: 0,
            ActualThemeChanged: 0,
            RemoveActualThemeChanged: RemoveActualThemeChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElement6 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElement6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ActualTheme: usize,
    ActualThemeChanged: usize,
    pub RemoveActualThemeChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElement7,
    IFrameworkElement7_Vtbl,
    0x2263886c_c069_570f_b9cc_9e21dd028d8e
);
impl windows_core::RuntimeType for IFrameworkElement7 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IFrameworkElement7 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement7";
}
pub trait IFrameworkElement7_Impl: windows_core::IUnknownImpl {
    fn IsLoaded(&self) -> windows_core::Result<bool>;
    fn RemoveEffectiveViewportChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl IFrameworkElement7_Vtbl {
    pub const fn new<Identity: IFrameworkElement7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsLoaded<
            Identity: IFrameworkElement7_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElement7_Impl::IsLoaded(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveEffectiveViewportChanged<
            Identity: IFrameworkElement7_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElement7_Impl::RemoveEffectiveViewportChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFrameworkElement7, OFFSET>(),
            IsLoaded: IsLoaded::<Identity, OFFSET>,
            EffectiveViewportChanged: 0,
            RemoveEffectiveViewportChanged: RemoveEffectiveViewportChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElement7 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElement7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsLoaded:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    EffectiveViewportChanged: usize,
    pub RemoveEffectiveViewportChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElementFactory,
    IFrameworkElementFactory_Vtbl,
    0xdeaee126_03ca_4966_b576_604cce93b5e8
);
impl windows_core::RuntimeType for IFrameworkElementFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IFrameworkElementFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementFactory";
}
pub trait IFrameworkElementFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(
        &self,
        baseInterface: windows_core::Ref<'_, windows_core::IInspectable>,
        innerInterface: windows_core::OutRef<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<FrameworkElement>;
}
impl IFrameworkElementFactory_Vtbl {
    pub const fn new<Identity: IFrameworkElementFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<
            Identity: IFrameworkElementFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            baseinterface: *mut core::ffi::c_void,
            innerinterface: *mut *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElementFactory_Impl::CreateInstance(
                    this,
                    core::mem::transmute_copy(&baseinterface),
                    core::mem::transmute_copy(&innerinterface),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IFrameworkElementFactory,
                OFFSET,
            >(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElementFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElementOverrides,
    IFrameworkElementOverrides_Vtbl,
    0xda007e54_b3c2_4b9a_aa8e_d3f071262b97
);
impl windows_core::RuntimeType for IFrameworkElementOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IFrameworkElementOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementOverrides";
}
pub trait IFrameworkElementOverrides_Impl: windows_core::IUnknownImpl {
    fn OnApplyTemplate(&self) -> windows_core::Result<()>;
}
impl IFrameworkElementOverrides_Vtbl {
    pub const fn new<Identity: IFrameworkElementOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnApplyTemplate<
            Identity: IFrameworkElementOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElementOverrides_Impl::OnApplyTemplate(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IFrameworkElementOverrides,
                OFFSET,
            >(),
            MeasureOverride: 0,
            ArrangeOverride: 0,
            OnApplyTemplate: OnApplyTemplate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElementOverrides as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    MeasureOverride: usize,
    ArrangeOverride: usize,
    pub OnApplyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElementOverrides2,
    IFrameworkElementOverrides2_Vtbl,
    0xcb5cd2b9_e3b4_458c_b64e_1434fd1bd88a
);
impl windows_core::RuntimeType for IFrameworkElementOverrides2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IFrameworkElementOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementOverrides2";
}
pub trait IFrameworkElementOverrides2_Impl: windows_core::IUnknownImpl {
    fn GoToElementStateCore(
        &self,
        stateName: &windows_core::HSTRING,
        useTransitions: bool,
    ) -> windows_core::Result<bool>;
}
impl IFrameworkElementOverrides2_Vtbl {
    pub const fn new<Identity: IFrameworkElementOverrides2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GoToElementStateCore<
            Identity: IFrameworkElementOverrides2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            statename: *mut core::ffi::c_void,
            usetransitions: bool,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElementOverrides2_Impl::GoToElementStateCore(
                    this,
                    core::mem::transmute(&statename),
                    usetransitions,
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IFrameworkElementOverrides2,
                OFFSET,
            >(),
            GoToElementStateCore: GoToElementStateCore::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElementOverrides2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementOverrides2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GoToElementStateCore: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
        *mut bool,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElementProtected7,
    IFrameworkElementProtected7_Vtbl,
    0x65aa0480_22e3_5103_ad2a_b626f88ca5ae
);
impl windows_core::RuntimeType for IFrameworkElementProtected7 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IFrameworkElementProtected7 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementProtected7";
}
pub trait IFrameworkElementProtected7_Impl: windows_core::IUnknownImpl {
    fn InvalidateViewport(&self) -> windows_core::Result<()>;
}
impl IFrameworkElementProtected7_Vtbl {
    pub const fn new<Identity: IFrameworkElementProtected7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InvalidateViewport<
            Identity: IFrameworkElementProtected7_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElementProtected7_Impl::InvalidateViewport(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IFrameworkElementProtected7,
                OFFSET,
            >(),
            InvalidateViewport: InvalidateViewport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElementProtected7 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementProtected7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InvalidateViewport:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElementStatics,
    IFrameworkElementStatics_Vtbl,
    0x48383032_fbeb_4f8a_aed2_ee21fb27a57b
);
impl windows_core::RuntimeType for IFrameworkElementStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IFrameworkElementStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementStatics";
}
pub trait IFrameworkElementStatics_Impl: windows_core::IUnknownImpl {}
impl IFrameworkElementStatics_Vtbl {
    pub const fn new<Identity: IFrameworkElementStatics_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IFrameworkElementStatics,
                OFFSET,
            >(),
            TagProperty: 0,
            LanguageProperty: 0,
            ActualWidthProperty: 0,
            ActualHeightProperty: 0,
            WidthProperty: 0,
            HeightProperty: 0,
            MinWidthProperty: 0,
            MaxWidthProperty: 0,
            MinHeightProperty: 0,
            MaxHeightProperty: 0,
            HorizontalAlignmentProperty: 0,
            VerticalAlignmentProperty: 0,
            MarginProperty: 0,
            NameProperty: 0,
            DataContextProperty: 0,
            StyleProperty: 0,
            FlowDirectionProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElementStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    TagProperty: usize,
    LanguageProperty: usize,
    ActualWidthProperty: usize,
    ActualHeightProperty: usize,
    WidthProperty: usize,
    HeightProperty: usize,
    MinWidthProperty: usize,
    MaxWidthProperty: usize,
    MinHeightProperty: usize,
    MaxHeightProperty: usize,
    HorizontalAlignmentProperty: usize,
    VerticalAlignmentProperty: usize,
    MarginProperty: usize,
    NameProperty: usize,
    DataContextProperty: usize,
    StyleProperty: usize,
    FlowDirectionProperty: usize,
}
windows_core::imp::define_interface!(
    IFrameworkElementStatics2,
    IFrameworkElementStatics2_Vtbl,
    0x9695db02_c0d8_4fa2_b100_3fa2df8b9538
);
impl windows_core::RuntimeType for IFrameworkElementStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IFrameworkElementStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementStatics2";
}
pub trait IFrameworkElementStatics2_Impl: windows_core::IUnknownImpl {}
impl IFrameworkElementStatics2_Vtbl {
    pub const fn new<Identity: IFrameworkElementStatics2_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IFrameworkElementStatics2,
                OFFSET,
            >(),
            RequestedThemeProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElementStatics2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    RequestedThemeProperty: usize,
}
windows_core::imp::define_interface!(
    IFrameworkElementStatics4,
    IFrameworkElementStatics4_Vtbl,
    0x9c41b155_c5d8_4663_bff2_d8d54fb5dbb3
);
impl windows_core::RuntimeType for IFrameworkElementStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IFrameworkElementStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementStatics4";
}
pub trait IFrameworkElementStatics4_Impl: windows_core::IUnknownImpl {}
impl IFrameworkElementStatics4_Vtbl {
    pub const fn new<Identity: IFrameworkElementStatics4_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IFrameworkElementStatics4,
                OFFSET,
            >(),
            AllowFocusOnInteractionProperty: 0,
            FocusVisualMarginProperty: 0,
            FocusVisualSecondaryThicknessProperty: 0,
            FocusVisualPrimaryThicknessProperty: 0,
            FocusVisualSecondaryBrushProperty: 0,
            FocusVisualPrimaryBrushProperty: 0,
            AllowFocusWhenDisabledProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElementStatics4 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    AllowFocusOnInteractionProperty: usize,
    FocusVisualMarginProperty: usize,
    FocusVisualSecondaryThicknessProperty: usize,
    FocusVisualPrimaryThicknessProperty: usize,
    FocusVisualSecondaryBrushProperty: usize,
    FocusVisualPrimaryBrushProperty: usize,
    AllowFocusWhenDisabledProperty: usize,
}
windows_core::imp::define_interface!(
    IFrameworkElementStatics5,
    IFrameworkElementStatics5_Vtbl,
    0x525d3941_0b3c_4be6_9978_19a8025c09d8
);
impl windows_core::RuntimeType for IFrameworkElementStatics5 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IFrameworkElementStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementStatics5";
}
pub trait IFrameworkElementStatics5_Impl: windows_core::IUnknownImpl {
    fn DeferTree(
        &self,
        element: windows_core::Ref<'_, DependencyObject>,
    ) -> windows_core::Result<()>;
}
impl IFrameworkElementStatics5_Vtbl {
    pub const fn new<Identity: IFrameworkElementStatics5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeferTree<
            Identity: IFrameworkElementStatics5_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElementStatics5_Impl::DeferTree(this, core::mem::transmute_copy(&element))
                    .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IFrameworkElementStatics5,
                OFFSET,
            >(),
            DeferTree: DeferTree::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElementStatics5 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementStatics5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeferTree: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElementStatics6,
    IFrameworkElementStatics6_Vtbl,
    0xfcc1529a_69db_4582_a7be_cf6a1cfdacd0
);
impl windows_core::RuntimeType for IFrameworkElementStatics6 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IFrameworkElementStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementStatics6";
}
pub trait IFrameworkElementStatics6_Impl: windows_core::IUnknownImpl {}
impl IFrameworkElementStatics6_Vtbl {
    pub const fn new<Identity: IFrameworkElementStatics6_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IFrameworkElementStatics6,
                OFFSET,
            >(),
            ActualThemeProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElementStatics6 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementStatics6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ActualThemeProperty: usize,
}
windows_core::imp::define_interface!(
    IObjectWithSite,
    IObjectWithSite_Vtbl,
    0xfc4801a3_2ba9_11cf_a229_00aa003d7352
);
windows_core::imp::interface_hierarchy!(IObjectWithSite, windows_core::IUnknown);
impl IObjectWithSite {
    pub unsafe fn SetSite<P0>(&self, punksite: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetSite)(
                windows_core::Interface::as_raw(self),
                punksite.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn GetSite<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetSite)(
                windows_core::Interface::as_raw(self),
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectWithSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSite: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetSite: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IObjectWithSite_Impl: windows_core::IUnknownImpl {
    fn SetSite(
        &self,
        punksite: windows_core::Ref<'_, windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
    fn GetSite(
        &self,
        riid: *const windows_core::GUID,
        ppvsite: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
}
impl IObjectWithSite_Vtbl {
    pub const fn new<Identity: IObjectWithSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSite<Identity: IObjectWithSite_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            punksite: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectWithSite_Impl::SetSite(this, core::mem::transmute_copy(&punksite)).into()
            }
        }
        unsafe extern "system" fn GetSite<Identity: IObjectWithSite_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            ppvsite: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectWithSite_Impl::GetSite(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppvsite),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSite: SetSite::<Identity, OFFSET>,
            GetSite: GetSite::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectWithSite as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IObjectWithSite {}
windows_core::imp::define_interface!(
    IPointerPoint,
    IPointerPoint_Vtbl,
    0xe995317d_7296_42d9_8233_c5be73b74a4a
);
impl windows_core::RuntimeType for IPointerPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IPointerPoint {
    const NAME: &'static str = "Windows.UI.Input.IPointerPoint";
}
pub trait IPointerPoint_Impl: windows_core::IUnknownImpl {
    fn PointerId(&self) -> windows_core::Result<u32>;
    fn FrameId(&self) -> windows_core::Result<u32>;
    fn Timestamp(&self) -> windows_core::Result<u64>;
    fn IsInContact(&self) -> windows_core::Result<bool>;
    fn Properties(&self) -> windows_core::Result<PointerPointProperties>;
}
impl IPointerPoint_Vtbl {
    pub const fn new<Identity: IPointerPoint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PointerId<Identity: IPointerPoint_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPoint_Impl::PointerId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FrameId<Identity: IPointerPoint_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPoint_Impl::FrameId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Timestamp<Identity: IPointerPoint_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPoint_Impl::Timestamp(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInContact<Identity: IPointerPoint_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPoint_Impl::IsInContact(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IPointerPoint_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPoint_Impl::Properties(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPointerPoint, OFFSET>(),
            PointerDevice: 0,
            Position: 0,
            RawPosition: 0,
            PointerId: PointerId::<Identity, OFFSET>,
            FrameId: FrameId::<Identity, OFFSET>,
            Timestamp: Timestamp::<Identity, OFFSET>,
            IsInContact: IsInContact::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerPoint as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPoint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PointerDevice: usize,
    Position: usize,
    RawPosition: usize,
    pub PointerId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FrameId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Timestamp:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub IsInContact:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPointerPointPhysicalPosition,
    IPointerPointPhysicalPosition_Vtbl,
    0x003185a3_a5e7_4859_9c0b_89340204806c
);
impl windows_core::RuntimeType for IPointerPointPhysicalPosition {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IPointerPointPhysicalPosition {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointPhysicalPosition";
}
pub trait IPointerPointPhysicalPosition_Impl: windows_core::IUnknownImpl {
    fn IsPhysicalPositionSupported(&self) -> windows_core::Result<bool>;
}
impl IPointerPointPhysicalPosition_Vtbl {
    pub const fn new<Identity: IPointerPointPhysicalPosition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsPhysicalPositionSupported<
            Identity: IPointerPointPhysicalPosition_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointPhysicalPosition_Impl::IsPhysicalPositionSupported(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IPointerPointPhysicalPosition,
                OFFSET,
            >(),
            IsPhysicalPositionSupported: IsPhysicalPositionSupported::<Identity, OFFSET>,
            PhysicalPosition: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerPointPhysicalPosition as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointPhysicalPosition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPhysicalPositionSupported:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    PhysicalPosition: usize,
}
windows_core::imp::define_interface!(
    IPointerPointProperties,
    IPointerPointProperties_Vtbl,
    0xc79d8a4b_c163_4ee7_803f_67ce79f9972d
);
impl windows_core::RuntimeType for IPointerPointProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IPointerPointProperties {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointProperties";
}
pub trait IPointerPointProperties_Impl: windows_core::IUnknownImpl {
    fn Pressure(&self) -> windows_core::Result<f32>;
    fn IsInverted(&self) -> windows_core::Result<bool>;
    fn IsEraser(&self) -> windows_core::Result<bool>;
    fn Orientation(&self) -> windows_core::Result<f32>;
    fn XTilt(&self) -> windows_core::Result<f32>;
    fn YTilt(&self) -> windows_core::Result<f32>;
    fn Twist(&self) -> windows_core::Result<f32>;
    fn TouchConfidence(&self) -> windows_core::Result<bool>;
    fn IsLeftButtonPressed(&self) -> windows_core::Result<bool>;
    fn IsRightButtonPressed(&self) -> windows_core::Result<bool>;
    fn IsMiddleButtonPressed(&self) -> windows_core::Result<bool>;
    fn MouseWheelDelta(&self) -> windows_core::Result<i32>;
    fn IsHorizontalMouseWheel(&self) -> windows_core::Result<bool>;
    fn IsPrimary(&self) -> windows_core::Result<bool>;
    fn IsInRange(&self) -> windows_core::Result<bool>;
    fn IsCanceled(&self) -> windows_core::Result<bool>;
    fn IsBarrelButtonPressed(&self) -> windows_core::Result<bool>;
    fn IsXButton1Pressed(&self) -> windows_core::Result<bool>;
    fn IsXButton2Pressed(&self) -> windows_core::Result<bool>;
    fn PointerUpdateKind(&self) -> windows_core::Result<PointerUpdateKind>;
    fn HasUsage(&self, usagePage: u32, usageId: u32) -> windows_core::Result<bool>;
    fn GetUsageValue(&self, usagePage: u32, usageId: u32) -> windows_core::Result<i32>;
}
impl IPointerPointProperties_Vtbl {
    pub const fn new<Identity: IPointerPointProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pressure<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::Pressure(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInverted<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsInverted(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEraser<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsEraser(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Orientation<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::Orientation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn XTilt<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::XTilt(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn YTilt<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::YTilt(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Twist<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::Twist(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TouchConfidence<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::TouchConfidence(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLeftButtonPressed<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsLeftButtonPressed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsRightButtonPressed<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsRightButtonPressed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsMiddleButtonPressed<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsMiddleButtonPressed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MouseWheelDelta<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::MouseWheelDelta(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsHorizontalMouseWheel<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsHorizontalMouseWheel(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPrimary<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsPrimary(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInRange<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsInRange(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCanceled<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsCanceled(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsBarrelButtonPressed<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsBarrelButtonPressed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsXButton1Pressed<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsXButton1Pressed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsXButton2Pressed<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsXButton2Pressed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerUpdateKind<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut PointerUpdateKind,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::PointerUpdateKind(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HasUsage<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            usagepage: u32,
            usageid: u32,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::HasUsage(this, usagepage, usageid) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUsageValue<
            Identity: IPointerPointProperties_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            usagepage: u32,
            usageid: u32,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::GetUsageValue(this, usagepage, usageid) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPointerPointProperties, OFFSET>(
            ),
            Pressure: Pressure::<Identity, OFFSET>,
            IsInverted: IsInverted::<Identity, OFFSET>,
            IsEraser: IsEraser::<Identity, OFFSET>,
            Orientation: Orientation::<Identity, OFFSET>,
            XTilt: XTilt::<Identity, OFFSET>,
            YTilt: YTilt::<Identity, OFFSET>,
            Twist: Twist::<Identity, OFFSET>,
            ContactRect: 0,
            ContactRectRaw: 0,
            TouchConfidence: TouchConfidence::<Identity, OFFSET>,
            IsLeftButtonPressed: IsLeftButtonPressed::<Identity, OFFSET>,
            IsRightButtonPressed: IsRightButtonPressed::<Identity, OFFSET>,
            IsMiddleButtonPressed: IsMiddleButtonPressed::<Identity, OFFSET>,
            MouseWheelDelta: MouseWheelDelta::<Identity, OFFSET>,
            IsHorizontalMouseWheel: IsHorizontalMouseWheel::<Identity, OFFSET>,
            IsPrimary: IsPrimary::<Identity, OFFSET>,
            IsInRange: IsInRange::<Identity, OFFSET>,
            IsCanceled: IsCanceled::<Identity, OFFSET>,
            IsBarrelButtonPressed: IsBarrelButtonPressed::<Identity, OFFSET>,
            IsXButton1Pressed: IsXButton1Pressed::<Identity, OFFSET>,
            IsXButton2Pressed: IsXButton2Pressed::<Identity, OFFSET>,
            PointerUpdateKind: PointerUpdateKind::<Identity, OFFSET>,
            HasUsage: HasUsage::<Identity, OFFSET>,
            GetUsageValue: GetUsageValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerPointProperties as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Pressure:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub IsInverted:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsEraser:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Orientation:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub XTilt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub YTilt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Twist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    ContactRect: usize,
    ContactRectRaw: usize,
    pub TouchConfidence:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsLeftButtonPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsRightButtonPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMiddleButtonPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MouseWheelDelta:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsHorizontalMouseWheel:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPrimary:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInRange:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCanceled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsBarrelButtonPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsXButton1Pressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsXButton2Pressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PointerUpdateKind: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PointerUpdateKind,
    ) -> windows_core::HRESULT,
    pub HasUsage: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub GetUsageValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut i32,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPointerPointProperties2,
    IPointerPointProperties2_Vtbl,
    0x22c3433a_c83b_41c0_a296_5e232d64d6af
);
impl windows_core::RuntimeType for IPointerPointProperties2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IPointerPointProperties2 {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointProperties2";
}
pub trait IPointerPointProperties2_Impl: windows_core::IUnknownImpl {}
impl IPointerPointProperties2_Vtbl {
    pub const fn new<Identity: IPointerPointProperties2_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IPointerPointProperties2,
                OFFSET,
            >(),
            ZDistance: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerPointProperties2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointProperties2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ZDistance: usize,
}
windows_core::imp::define_interface!(
    IPointerPointStatics,
    IPointerPointStatics_Vtbl,
    0xa506638d_2a1a_413e_bc75_9f38381cc069
);
impl windows_core::RuntimeType for IPointerPointStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IPointerPointStatics {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointStatics";
}
pub trait IPointerPointStatics_Impl: windows_core::IUnknownImpl {
    fn GetCurrentPoint(&self, pointerId: u32) -> windows_core::Result<PointerPoint>;
    fn GetIntermediatePoints(
        &self,
        pointerId: u32,
    ) -> windows_core::Result<windows_collections::IVector<PointerPoint>>;
}
impl IPointerPointStatics_Vtbl {
    pub const fn new<Identity: IPointerPointStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentPoint<
            Identity: IPointerPointStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pointerid: u32,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointStatics_Impl::GetCurrentPoint(this, pointerid) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIntermediatePoints<
            Identity: IPointerPointStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pointerid: u32,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointStatics_Impl::GetIntermediatePoints(this, pointerid) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPointerPointStatics, OFFSET>(
            ),
            GetCurrentPoint: GetCurrentPoint::<Identity, OFFSET>,
            GetIntermediatePoints: GetIntermediatePoints::<Identity, OFFSET>,
            GetCurrentPointTransformed: 0,
            GetIntermediatePointsTransformed: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerPointStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetIntermediatePoints: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetCurrentPointTransformed: usize,
    GetIntermediatePointsTransformed: usize,
}
windows_core::imp::define_interface!(
    IPointerRoutedEventArgs,
    IPointerRoutedEventArgs_Vtbl,
    0xda628f0a_9752_49e2_bde2_49eccab9194d
);
impl windows_core::RuntimeType for IPointerRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IPointerRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IPointerRoutedEventArgs";
}
pub trait IPointerRoutedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn GetCurrentPoint(
        &self,
        relativeTo: windows_core::Ref<'_, UIElement>,
    ) -> windows_core::Result<PointerPoint>;
    fn GetIntermediatePoints(
        &self,
        relativeTo: windows_core::Ref<'_, UIElement>,
    ) -> windows_core::Result<windows_collections::IVector<PointerPoint>>;
}
impl IPointerRoutedEventArgs_Vtbl {
    pub const fn new<Identity: IPointerRoutedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Handled<
            Identity: IPointerRoutedEventArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerRoutedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<
            Identity: IPointerRoutedEventArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPointerRoutedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn GetCurrentPoint<
            Identity: IPointerRoutedEventArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            relativeto: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerRoutedEventArgs_Impl::GetCurrentPoint(
                    this,
                    core::mem::transmute_copy(&relativeto),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIntermediatePoints<
            Identity: IPointerRoutedEventArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            relativeto: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerRoutedEventArgs_Impl::GetIntermediatePoints(
                    this,
                    core::mem::transmute_copy(&relativeto),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPointerRoutedEventArgs, OFFSET>(
            ),
            Pointer: 0,
            KeyModifiers: 0,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            GetCurrentPoint: GetCurrentPoint::<Identity, OFFSET>,
            GetIntermediatePoints: GetIntermediatePoints::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerRoutedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Pointer: usize,
    KeyModifiers: usize,
    pub Handled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetCurrentPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetIntermediatePoints: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPointerRoutedEventArgs2,
    IPointerRoutedEventArgs2_Vtbl,
    0x0821f294_1de6_4711_ba7c_8d4b8b0911d0
);
impl windows_core::RuntimeType for IPointerRoutedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IPointerRoutedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IPointerRoutedEventArgs2";
}
pub trait IPointerRoutedEventArgs2_Impl: windows_core::IUnknownImpl {
    fn IsGenerated(&self) -> windows_core::Result<bool>;
}
impl IPointerRoutedEventArgs2_Vtbl {
    pub const fn new<Identity: IPointerRoutedEventArgs2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsGenerated<
            Identity: IPointerRoutedEventArgs2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerRoutedEventArgs2_Impl::IsGenerated(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IPointerRoutedEventArgs2,
                OFFSET,
            >(),
            IsGenerated: IsGenerated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerRoutedEventArgs2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsGenerated:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRoutedEvent,
    IRoutedEvent_Vtbl,
    0xa6b25818_43c1_4c70_865c_7bdd5a32e327
);
impl windows_core::RuntimeType for IRoutedEvent {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IRoutedEvent {
    const NAME: &'static str = "Windows.UI.Xaml.IRoutedEvent";
}
pub trait IRoutedEvent_Impl: windows_core::IUnknownImpl {}
impl IRoutedEvent_Vtbl {
    pub const fn new<Identity: IRoutedEvent_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRoutedEvent, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRoutedEvent as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoutedEvent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IRoutedEventArgs,
    IRoutedEventArgs_Vtbl,
    0x5c985ac6_d802_4b38_a223_bf070c43fedf
);
impl windows_core::RuntimeType for IRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IRoutedEventArgs";
}
pub trait IRoutedEventArgs_Impl: windows_core::IUnknownImpl {
    fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable>;
}
impl IRoutedEventArgs_Vtbl {
    pub const fn new<Identity: IRoutedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OriginalSource<
            Identity: IRoutedEventArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRoutedEventArgs_Impl::OriginalSource(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRoutedEventArgs, OFFSET>(),
            OriginalSource: OriginalSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRoutedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OriginalSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRoutedEventArgsFactory,
    IRoutedEventArgsFactory_Vtbl,
    0xb61c4d87_70e5_412e_b520_1a41ee76bbf4
);
impl windows_core::RuntimeType for IRoutedEventArgsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IRoutedEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IRoutedEventArgsFactory";
}
pub trait IRoutedEventArgsFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(
        &self,
        baseInterface: windows_core::Ref<'_, windows_core::IInspectable>,
        innerInterface: windows_core::OutRef<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<RoutedEventArgs>;
}
impl IRoutedEventArgsFactory_Vtbl {
    pub const fn new<Identity: IRoutedEventArgsFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<
            Identity: IRoutedEventArgsFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            baseinterface: *mut core::ffi::c_void,
            innerinterface: *mut *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRoutedEventArgsFactory_Impl::CreateInstance(
                    this,
                    core::mem::transmute_copy(&baseinterface),
                    core::mem::transmute_copy(&innerinterface),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRoutedEventArgsFactory, OFFSET>(
            ),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRoutedEventArgsFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoutedEventArgsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUIElement,
    IUIElement_Vtbl,
    0x676d0be9_b65c_41c6_ba40_58cf87f201c1
);
impl windows_core::RuntimeType for IUIElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElement {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement";
}
pub trait IUIElement_Impl: windows_core::IUnknownImpl {
    fn AllowDrop(&self) -> windows_core::Result<bool>;
    fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()>;
    fn Opacity(&self) -> windows_core::Result<f64>;
    fn SetOpacity(&self, value: f64) -> windows_core::Result<()>;
    fn IsHitTestVisible(&self) -> windows_core::Result<bool>;
    fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()>;
    fn UseLayoutRounding(&self) -> windows_core::Result<bool>;
    fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()>;
    fn IsTapEnabled(&self) -> windows_core::Result<bool>;
    fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()>;
    fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool>;
    fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()>;
    fn IsRightTapEnabled(&self) -> windows_core::Result<bool>;
    fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()>;
    fn IsHoldingEnabled(&self) -> windows_core::Result<bool>;
    fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()>;
    fn RemoveKeyUp(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveKeyDown(&self, token: i64) -> windows_core::Result<()>;
    fn GotFocus(
        &self,
        handler: windows_core::Ref<'_, RoutedEventHandler>,
    ) -> windows_core::Result<i64>;
    fn RemoveGotFocus(&self, token: i64) -> windows_core::Result<()>;
    fn LostFocus(
        &self,
        handler: windows_core::Ref<'_, RoutedEventHandler>,
    ) -> windows_core::Result<i64>;
    fn RemoveLostFocus(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveDragEnter(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveDragLeave(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveDragOver(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveDrop(&self, token: i64) -> windows_core::Result<()>;
    fn PointerPressed(
        &self,
        handler: windows_core::Ref<'_, PointerEventHandler>,
    ) -> windows_core::Result<i64>;
    fn RemovePointerPressed(&self, token: i64) -> windows_core::Result<()>;
    fn PointerMoved(
        &self,
        handler: windows_core::Ref<'_, PointerEventHandler>,
    ) -> windows_core::Result<i64>;
    fn RemovePointerMoved(&self, token: i64) -> windows_core::Result<()>;
    fn PointerReleased(
        &self,
        handler: windows_core::Ref<'_, PointerEventHandler>,
    ) -> windows_core::Result<i64>;
    fn RemovePointerReleased(&self, token: i64) -> windows_core::Result<()>;
    fn PointerEntered(
        &self,
        handler: windows_core::Ref<'_, PointerEventHandler>,
    ) -> windows_core::Result<i64>;
    fn RemovePointerEntered(&self, token: i64) -> windows_core::Result<()>;
    fn PointerExited(
        &self,
        handler: windows_core::Ref<'_, PointerEventHandler>,
    ) -> windows_core::Result<i64>;
    fn RemovePointerExited(&self, token: i64) -> windows_core::Result<()>;
    fn PointerCaptureLost(
        &self,
        handler: windows_core::Ref<'_, PointerEventHandler>,
    ) -> windows_core::Result<i64>;
    fn RemovePointerCaptureLost(&self, token: i64) -> windows_core::Result<()>;
    fn PointerCanceled(
        &self,
        handler: windows_core::Ref<'_, PointerEventHandler>,
    ) -> windows_core::Result<i64>;
    fn RemovePointerCanceled(&self, token: i64) -> windows_core::Result<()>;
    fn PointerWheelChanged(
        &self,
        handler: windows_core::Ref<'_, PointerEventHandler>,
    ) -> windows_core::Result<i64>;
    fn RemovePointerWheelChanged(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveTapped(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveDoubleTapped(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveHolding(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveRightTapped(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveManipulationStarting(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveManipulationInertiaStarting(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveManipulationStarted(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveManipulationDelta(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveManipulationCompleted(&self, token: i64) -> windows_core::Result<()>;
    fn ReleasePointerCaptures(&self) -> windows_core::Result<()>;
    fn AddHandler(
        &self,
        routedEvent: windows_core::Ref<'_, RoutedEvent>,
        handler: windows_core::Ref<'_, windows_core::IInspectable>,
        handledEventsToo: bool,
    ) -> windows_core::Result<()>;
    fn RemoveHandler(
        &self,
        routedEvent: windows_core::Ref<'_, RoutedEvent>,
        handler: windows_core::Ref<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<()>;
    fn InvalidateMeasure(&self) -> windows_core::Result<()>;
    fn InvalidateArrange(&self) -> windows_core::Result<()>;
    fn UpdateLayout(&self) -> windows_core::Result<()>;
}
impl IUIElement_Vtbl {
    pub const fn new<Identity: IUIElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AllowDrop<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::AllowDrop(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowDrop<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::SetAllowDrop(this, value).into()
            }
        }
        unsafe extern "system" fn Opacity<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::Opacity(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOpacity<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::SetOpacity(this, value).into()
            }
        }
        unsafe extern "system" fn IsHitTestVisible<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::IsHitTestVisible(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsHitTestVisible<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::SetIsHitTestVisible(this, value).into()
            }
        }
        unsafe extern "system" fn UseLayoutRounding<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::UseLayoutRounding(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseLayoutRounding<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::SetUseLayoutRounding(this, value).into()
            }
        }
        unsafe extern "system" fn IsTapEnabled<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::IsTapEnabled(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsTapEnabled<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::SetIsTapEnabled(this, value).into()
            }
        }
        unsafe extern "system" fn IsDoubleTapEnabled<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::IsDoubleTapEnabled(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsDoubleTapEnabled<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::SetIsDoubleTapEnabled(this, value).into()
            }
        }
        unsafe extern "system" fn IsRightTapEnabled<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::IsRightTapEnabled(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsRightTapEnabled<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::SetIsRightTapEnabled(this, value).into()
            }
        }
        unsafe extern "system" fn IsHoldingEnabled<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::IsHoldingEnabled(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsHoldingEnabled<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::SetIsHoldingEnabled(this, value).into()
            }
        }
        unsafe extern "system" fn RemoveKeyUp<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveKeyUp(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveKeyDown<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveKeyDown(this, token).into()
            }
        }
        unsafe extern "system" fn GotFocus<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::GotFocus(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveGotFocus<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveGotFocus(this, token).into()
            }
        }
        unsafe extern "system" fn LostFocus<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::LostFocus(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveLostFocus<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveLostFocus(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveDragEnter<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveDragEnter(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveDragLeave<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveDragLeave(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveDragOver<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveDragOver(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveDrop<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveDrop(this, token).into()
            }
        }
        unsafe extern "system" fn PointerPressed<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::PointerPressed(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerPressed<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemovePointerPressed(this, token).into()
            }
        }
        unsafe extern "system" fn PointerMoved<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::PointerMoved(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerMoved<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemovePointerMoved(this, token).into()
            }
        }
        unsafe extern "system" fn PointerReleased<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::PointerReleased(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerReleased<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemovePointerReleased(this, token).into()
            }
        }
        unsafe extern "system" fn PointerEntered<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::PointerEntered(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerEntered<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemovePointerEntered(this, token).into()
            }
        }
        unsafe extern "system" fn PointerExited<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::PointerExited(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerExited<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemovePointerExited(this, token).into()
            }
        }
        unsafe extern "system" fn PointerCaptureLost<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::PointerCaptureLost(this, core::mem::transmute_copy(&handler))
                {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerCaptureLost<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemovePointerCaptureLost(this, token).into()
            }
        }
        unsafe extern "system" fn PointerCanceled<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::PointerCanceled(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerCanceled<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemovePointerCanceled(this, token).into()
            }
        }
        unsafe extern "system" fn PointerWheelChanged<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement_Impl::PointerWheelChanged(
                    this,
                    core::mem::transmute_copy(&handler),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerWheelChanged<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemovePointerWheelChanged(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveTapped<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveTapped(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveDoubleTapped<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveDoubleTapped(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveHolding<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveHolding(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveRightTapped<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveRightTapped(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveManipulationStarting<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveManipulationStarting(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveManipulationInertiaStarting<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveManipulationInertiaStarting(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveManipulationStarted<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveManipulationStarted(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveManipulationDelta<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveManipulationDelta(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveManipulationCompleted<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveManipulationCompleted(this, token).into()
            }
        }
        unsafe extern "system" fn ReleasePointerCaptures<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::ReleasePointerCaptures(this).into()
            }
        }
        unsafe extern "system" fn AddHandler<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            routedevent: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            handledeventstoo: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::AddHandler(
                    this,
                    core::mem::transmute_copy(&routedevent),
                    core::mem::transmute_copy(&handler),
                    handledeventstoo,
                )
                .into()
            }
        }
        unsafe extern "system" fn RemoveHandler<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            routedevent: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::RemoveHandler(
                    this,
                    core::mem::transmute_copy(&routedevent),
                    core::mem::transmute_copy(&handler),
                )
                .into()
            }
        }
        unsafe extern "system" fn InvalidateMeasure<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::InvalidateMeasure(this).into()
            }
        }
        unsafe extern "system" fn InvalidateArrange<
            Identity: IUIElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::InvalidateArrange(this).into()
            }
        }
        unsafe extern "system" fn UpdateLayout<Identity: IUIElement_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement_Impl::UpdateLayout(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElement, OFFSET>(),
            DesiredSize: 0,
            AllowDrop: AllowDrop::<Identity, OFFSET>,
            SetAllowDrop: SetAllowDrop::<Identity, OFFSET>,
            Opacity: Opacity::<Identity, OFFSET>,
            SetOpacity: SetOpacity::<Identity, OFFSET>,
            Clip: 0,
            SetClip: 0,
            RenderTransform: 0,
            SetRenderTransform: 0,
            Projection: 0,
            SetProjection: 0,
            RenderTransformOrigin: 0,
            SetRenderTransformOrigin: 0,
            IsHitTestVisible: IsHitTestVisible::<Identity, OFFSET>,
            SetIsHitTestVisible: SetIsHitTestVisible::<Identity, OFFSET>,
            Visibility: 0,
            SetVisibility: 0,
            RenderSize: 0,
            UseLayoutRounding: UseLayoutRounding::<Identity, OFFSET>,
            SetUseLayoutRounding: SetUseLayoutRounding::<Identity, OFFSET>,
            Transitions: 0,
            SetTransitions: 0,
            CacheMode: 0,
            SetCacheMode: 0,
            IsTapEnabled: IsTapEnabled::<Identity, OFFSET>,
            SetIsTapEnabled: SetIsTapEnabled::<Identity, OFFSET>,
            IsDoubleTapEnabled: IsDoubleTapEnabled::<Identity, OFFSET>,
            SetIsDoubleTapEnabled: SetIsDoubleTapEnabled::<Identity, OFFSET>,
            IsRightTapEnabled: IsRightTapEnabled::<Identity, OFFSET>,
            SetIsRightTapEnabled: SetIsRightTapEnabled::<Identity, OFFSET>,
            IsHoldingEnabled: IsHoldingEnabled::<Identity, OFFSET>,
            SetIsHoldingEnabled: SetIsHoldingEnabled::<Identity, OFFSET>,
            ManipulationMode: 0,
            SetManipulationMode: 0,
            PointerCaptures: 0,
            KeyUp: 0,
            RemoveKeyUp: RemoveKeyUp::<Identity, OFFSET>,
            KeyDown: 0,
            RemoveKeyDown: RemoveKeyDown::<Identity, OFFSET>,
            GotFocus: GotFocus::<Identity, OFFSET>,
            RemoveGotFocus: RemoveGotFocus::<Identity, OFFSET>,
            LostFocus: LostFocus::<Identity, OFFSET>,
            RemoveLostFocus: RemoveLostFocus::<Identity, OFFSET>,
            DragEnter: 0,
            RemoveDragEnter: RemoveDragEnter::<Identity, OFFSET>,
            DragLeave: 0,
            RemoveDragLeave: RemoveDragLeave::<Identity, OFFSET>,
            DragOver: 0,
            RemoveDragOver: RemoveDragOver::<Identity, OFFSET>,
            Drop: 0,
            RemoveDrop: RemoveDrop::<Identity, OFFSET>,
            PointerPressed: PointerPressed::<Identity, OFFSET>,
            RemovePointerPressed: RemovePointerPressed::<Identity, OFFSET>,
            PointerMoved: PointerMoved::<Identity, OFFSET>,
            RemovePointerMoved: RemovePointerMoved::<Identity, OFFSET>,
            PointerReleased: PointerReleased::<Identity, OFFSET>,
            RemovePointerReleased: RemovePointerReleased::<Identity, OFFSET>,
            PointerEntered: PointerEntered::<Identity, OFFSET>,
            RemovePointerEntered: RemovePointerEntered::<Identity, OFFSET>,
            PointerExited: PointerExited::<Identity, OFFSET>,
            RemovePointerExited: RemovePointerExited::<Identity, OFFSET>,
            PointerCaptureLost: PointerCaptureLost::<Identity, OFFSET>,
            RemovePointerCaptureLost: RemovePointerCaptureLost::<Identity, OFFSET>,
            PointerCanceled: PointerCanceled::<Identity, OFFSET>,
            RemovePointerCanceled: RemovePointerCanceled::<Identity, OFFSET>,
            PointerWheelChanged: PointerWheelChanged::<Identity, OFFSET>,
            RemovePointerWheelChanged: RemovePointerWheelChanged::<Identity, OFFSET>,
            Tapped: 0,
            RemoveTapped: RemoveTapped::<Identity, OFFSET>,
            DoubleTapped: 0,
            RemoveDoubleTapped: RemoveDoubleTapped::<Identity, OFFSET>,
            Holding: 0,
            RemoveHolding: RemoveHolding::<Identity, OFFSET>,
            RightTapped: 0,
            RemoveRightTapped: RemoveRightTapped::<Identity, OFFSET>,
            ManipulationStarting: 0,
            RemoveManipulationStarting: RemoveManipulationStarting::<Identity, OFFSET>,
            ManipulationInertiaStarting: 0,
            RemoveManipulationInertiaStarting: RemoveManipulationInertiaStarting::<Identity, OFFSET>,
            ManipulationStarted: 0,
            RemoveManipulationStarted: RemoveManipulationStarted::<Identity, OFFSET>,
            ManipulationDelta: 0,
            RemoveManipulationDelta: RemoveManipulationDelta::<Identity, OFFSET>,
            ManipulationCompleted: 0,
            RemoveManipulationCompleted: RemoveManipulationCompleted::<Identity, OFFSET>,
            Measure: 0,
            Arrange: 0,
            CapturePointer: 0,
            ReleasePointerCapture: 0,
            ReleasePointerCaptures: ReleasePointerCaptures::<Identity, OFFSET>,
            AddHandler: AddHandler::<Identity, OFFSET>,
            RemoveHandler: RemoveHandler::<Identity, OFFSET>,
            TransformToVisual: 0,
            InvalidateMeasure: InvalidateMeasure::<Identity, OFFSET>,
            InvalidateArrange: InvalidateArrange::<Identity, OFFSET>,
            UpdateLayout: UpdateLayout::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElement as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    DesiredSize: usize,
    pub AllowDrop:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowDrop:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Opacity:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    Clip: usize,
    SetClip: usize,
    RenderTransform: usize,
    SetRenderTransform: usize,
    Projection: usize,
    SetProjection: usize,
    RenderTransformOrigin: usize,
    SetRenderTransformOrigin: usize,
    pub IsHitTestVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsHitTestVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    Visibility: usize,
    SetVisibility: usize,
    RenderSize: usize,
    pub UseLayoutRounding:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetUseLayoutRounding:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    Transitions: usize,
    SetTransitions: usize,
    CacheMode: usize,
    SetCacheMode: usize,
    pub IsTapEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsTapEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsDoubleTapEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsDoubleTapEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsRightTapEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsRightTapEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsHoldingEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsHoldingEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    ManipulationMode: usize,
    SetManipulationMode: usize,
    PointerCaptures: usize,
    KeyUp: usize,
    pub RemoveKeyUp:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    KeyDown: usize,
    pub RemoveKeyDown:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GotFocus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveGotFocus:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub LostFocus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveLostFocus:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    DragEnter: usize,
    pub RemoveDragEnter:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    DragLeave: usize,
    pub RemoveDragLeave:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    DragOver: usize,
    pub RemoveDragOver:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Drop: usize,
    pub RemoveDrop: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerPressed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePointerPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerMoved: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePointerMoved:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerReleased: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePointerReleased:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerEntered: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePointerEntered:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerExited: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePointerExited:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerCaptureLost: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePointerCaptureLost:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerCanceled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePointerCanceled:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerWheelChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePointerWheelChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Tapped: usize,
    pub RemoveTapped:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    DoubleTapped: usize,
    pub RemoveDoubleTapped:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Holding: usize,
    pub RemoveHolding:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    RightTapped: usize,
    pub RemoveRightTapped:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ManipulationStarting: usize,
    pub RemoveManipulationStarting:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ManipulationInertiaStarting: usize,
    pub RemoveManipulationInertiaStarting:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ManipulationStarted: usize,
    pub RemoveManipulationStarted:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ManipulationDelta: usize,
    pub RemoveManipulationDelta:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ManipulationCompleted: usize,
    pub RemoveManipulationCompleted:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Measure: usize,
    Arrange: usize,
    CapturePointer: usize,
    ReleasePointerCapture: usize,
    pub ReleasePointerCaptures:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddHandler: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    pub RemoveHandler: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    TransformToVisual: usize,
    pub InvalidateMeasure:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvalidateArrange:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateLayout: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUIElement10,
    IUIElement10_Vtbl,
    0xd531c629_ad2c_5f6b_adcf_fb87287d18d7
);
impl windows_core::RuntimeType for IUIElement10 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElement10 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement10";
}
pub trait IUIElement10_Impl: windows_core::IUnknownImpl {
    fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3>;
    fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2>;
}
impl IUIElement10_Vtbl {
    pub const fn new<Identity: IUIElement10_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActualOffset<Identity: IUIElement10_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut windows_numerics::Vector3,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement10_Impl::ActualOffset(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActualSize<Identity: IUIElement10_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut windows_numerics::Vector2,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement10_Impl::ActualSize(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElement10, OFFSET>(),
            ActualOffset: ActualOffset::<Identity, OFFSET>,
            ActualSize: ActualSize::<Identity, OFFSET>,
            XamlRoot: 0,
            SetXamlRoot: 0,
            UIContext: 0,
            Shadow: 0,
            SetShadow: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElement10 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement10_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActualOffset: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    pub ActualSize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_numerics::Vector2,
    ) -> windows_core::HRESULT,
    XamlRoot: usize,
    SetXamlRoot: usize,
    UIContext: usize,
    Shadow: usize,
    SetShadow: usize,
}
windows_core::imp::define_interface!(
    IUIElement2,
    IUIElement2_Vtbl,
    0x676d0bf9_b66c_41d6_ba50_58cf87f201d1
);
impl windows_core::RuntimeType for IUIElement2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElement2 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement2";
}
pub trait IUIElement2_Impl: windows_core::IUnknownImpl {
    fn CancelDirectManipulations(&self) -> windows_core::Result<bool>;
}
impl IUIElement2_Vtbl {
    pub const fn new<Identity: IUIElement2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CancelDirectManipulations<
            Identity: IUIElement2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement2_Impl::CancelDirectManipulations(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElement2, OFFSET>(),
            CompositeMode: 0,
            SetCompositeMode: 0,
            CancelDirectManipulations: CancelDirectManipulations::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElement2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CompositeMode: usize,
    SetCompositeMode: usize,
    pub CancelDirectManipulations:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUIElement3,
    IUIElement3_Vtbl,
    0xbc2b28f1_26f2_4aab_b256_3b5350881e37
);
impl windows_core::RuntimeType for IUIElement3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElement3 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement3";
}
pub trait IUIElement3_Impl: windows_core::IUnknownImpl {
    fn CanDrag(&self) -> windows_core::Result<bool>;
    fn SetCanDrag(&self, value: bool) -> windows_core::Result<()>;
    fn RemoveDragStarting(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveDropCompleted(&self, token: i64) -> windows_core::Result<()>;
}
impl IUIElement3_Vtbl {
    pub const fn new<Identity: IUIElement3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CanDrag<Identity: IUIElement3_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement3_Impl::CanDrag(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCanDrag<Identity: IUIElement3_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement3_Impl::SetCanDrag(this, value).into()
            }
        }
        unsafe extern "system" fn RemoveDragStarting<
            Identity: IUIElement3_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement3_Impl::RemoveDragStarting(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveDropCompleted<
            Identity: IUIElement3_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement3_Impl::RemoveDropCompleted(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElement3, OFFSET>(),
            Transform3D: 0,
            SetTransform3D: 0,
            CanDrag: CanDrag::<Identity, OFFSET>,
            SetCanDrag: SetCanDrag::<Identity, OFFSET>,
            DragStarting: 0,
            RemoveDragStarting: RemoveDragStarting::<Identity, OFFSET>,
            DropCompleted: 0,
            RemoveDropCompleted: RemoveDropCompleted::<Identity, OFFSET>,
            StartDragAsync: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElement3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Transform3D: usize,
    SetTransform3D: usize,
    pub CanDrag:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanDrag:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    DragStarting: usize,
    pub RemoveDragStarting:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    DropCompleted: usize,
    pub RemoveDropCompleted:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    StartDragAsync: usize,
}
windows_core::imp::define_interface!(
    IUIElement4,
    IUIElement4_Vtbl,
    0x69145cd4_199a_4657_9e57_e99e8f136712
);
impl windows_core::RuntimeType for IUIElement4 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElement4 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement4";
}
pub trait IUIElement4_Impl: windows_core::IUnknownImpl {
    fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool>;
    fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()>;
    fn IsAccessKeyScope(&self) -> windows_core::Result<bool>;
    fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()>;
    fn AccessKeyScopeOwner(&self) -> windows_core::Result<DependencyObject>;
    fn SetAccessKeyScopeOwner(
        &self,
        value: windows_core::Ref<'_, DependencyObject>,
    ) -> windows_core::Result<()>;
    fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn RemoveContextRequested(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveContextCanceled(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveAccessKeyDisplayRequested(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveAccessKeyDisplayDismissed(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveAccessKeyInvoked(&self, token: i64) -> windows_core::Result<()>;
}
impl IUIElement4_Vtbl {
    pub const fn new<Identity: IUIElement4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ExitDisplayModeOnAccessKeyInvoked<
            Identity: IUIElement4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement4_Impl::ExitDisplayModeOnAccessKeyInvoked(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExitDisplayModeOnAccessKeyInvoked<
            Identity: IUIElement4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement4_Impl::SetExitDisplayModeOnAccessKeyInvoked(this, value).into()
            }
        }
        unsafe extern "system" fn IsAccessKeyScope<
            Identity: IUIElement4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement4_Impl::IsAccessKeyScope(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsAccessKeyScope<
            Identity: IUIElement4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement4_Impl::SetIsAccessKeyScope(this, value).into()
            }
        }
        unsafe extern "system" fn AccessKeyScopeOwner<
            Identity: IUIElement4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement4_Impl::AccessKeyScopeOwner(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccessKeyScopeOwner<
            Identity: IUIElement4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement4_Impl::SetAccessKeyScopeOwner(this, core::mem::transmute_copy(&value))
                    .into()
            }
        }
        unsafe extern "system" fn AccessKey<Identity: IUIElement4_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement4_Impl::AccessKey(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccessKey<Identity: IUIElement4_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement4_Impl::SetAccessKey(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn RemoveContextRequested<
            Identity: IUIElement4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement4_Impl::RemoveContextRequested(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveContextCanceled<
            Identity: IUIElement4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement4_Impl::RemoveContextCanceled(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveAccessKeyDisplayRequested<
            Identity: IUIElement4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement4_Impl::RemoveAccessKeyDisplayRequested(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveAccessKeyDisplayDismissed<
            Identity: IUIElement4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement4_Impl::RemoveAccessKeyDisplayDismissed(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveAccessKeyInvoked<
            Identity: IUIElement4_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement4_Impl::RemoveAccessKeyInvoked(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElement4, OFFSET>(),
            ContextFlyout: 0,
            SetContextFlyout: 0,
            ExitDisplayModeOnAccessKeyInvoked: ExitDisplayModeOnAccessKeyInvoked::<Identity, OFFSET>,
            SetExitDisplayModeOnAccessKeyInvoked: SetExitDisplayModeOnAccessKeyInvoked::<
                Identity,
                OFFSET,
            >,
            IsAccessKeyScope: IsAccessKeyScope::<Identity, OFFSET>,
            SetIsAccessKeyScope: SetIsAccessKeyScope::<Identity, OFFSET>,
            AccessKeyScopeOwner: AccessKeyScopeOwner::<Identity, OFFSET>,
            SetAccessKeyScopeOwner: SetAccessKeyScopeOwner::<Identity, OFFSET>,
            AccessKey: AccessKey::<Identity, OFFSET>,
            SetAccessKey: SetAccessKey::<Identity, OFFSET>,
            ContextRequested: 0,
            RemoveContextRequested: RemoveContextRequested::<Identity, OFFSET>,
            ContextCanceled: 0,
            RemoveContextCanceled: RemoveContextCanceled::<Identity, OFFSET>,
            AccessKeyDisplayRequested: 0,
            RemoveAccessKeyDisplayRequested: RemoveAccessKeyDisplayRequested::<Identity, OFFSET>,
            AccessKeyDisplayDismissed: 0,
            RemoveAccessKeyDisplayDismissed: RemoveAccessKeyDisplayDismissed::<Identity, OFFSET>,
            AccessKeyInvoked: 0,
            RemoveAccessKeyInvoked: RemoveAccessKeyInvoked::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElement4 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ContextFlyout: usize,
    SetContextFlyout: usize,
    pub ExitDisplayModeOnAccessKeyInvoked:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetExitDisplayModeOnAccessKeyInvoked:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsAccessKeyScope:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsAccessKeyScope:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AccessKeyScopeOwner: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetAccessKeyScopeOwner: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AccessKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ContextRequested: usize,
    pub RemoveContextRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ContextCanceled: usize,
    pub RemoveContextCanceled:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    AccessKeyDisplayRequested: usize,
    pub RemoveAccessKeyDisplayRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    AccessKeyDisplayDismissed: usize,
    pub RemoveAccessKeyDisplayDismissed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    AccessKeyInvoked: usize,
    pub RemoveAccessKeyInvoked:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUIElement5,
    IUIElement5_Vtbl,
    0x8eed9bc2_a58c_4453_af0f_a92ee06d0317
);
impl windows_core::RuntimeType for IUIElement5 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElement5 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement5";
}
pub trait IUIElement5_Impl: windows_core::IUnknownImpl {
    fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64>;
    fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()>;
    fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64>;
    fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()>;
    fn RemoveGettingFocus(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveLosingFocus(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveNoFocusCandidateFound(&self, token: i64) -> windows_core::Result<()>;
    fn StartBringIntoView(&self) -> windows_core::Result<()>;
}
impl IUIElement5_Vtbl {
    pub const fn new<Identity: IUIElement5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KeyTipHorizontalOffset<
            Identity: IUIElement5_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement5_Impl::KeyTipHorizontalOffset(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeyTipHorizontalOffset<
            Identity: IUIElement5_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement5_Impl::SetKeyTipHorizontalOffset(this, value).into()
            }
        }
        unsafe extern "system" fn KeyTipVerticalOffset<
            Identity: IUIElement5_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement5_Impl::KeyTipVerticalOffset(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeyTipVerticalOffset<
            Identity: IUIElement5_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement5_Impl::SetKeyTipVerticalOffset(this, value).into()
            }
        }
        unsafe extern "system" fn RemoveGettingFocus<
            Identity: IUIElement5_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement5_Impl::RemoveGettingFocus(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveLosingFocus<
            Identity: IUIElement5_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement5_Impl::RemoveLosingFocus(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveNoFocusCandidateFound<
            Identity: IUIElement5_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement5_Impl::RemoveNoFocusCandidateFound(this, token).into()
            }
        }
        unsafe extern "system" fn StartBringIntoView<
            Identity: IUIElement5_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement5_Impl::StartBringIntoView(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElement5, OFFSET>(),
            Lights: 0,
            KeyTipPlacementMode: 0,
            SetKeyTipPlacementMode: 0,
            KeyTipHorizontalOffset: KeyTipHorizontalOffset::<Identity, OFFSET>,
            SetKeyTipHorizontalOffset: SetKeyTipHorizontalOffset::<Identity, OFFSET>,
            KeyTipVerticalOffset: KeyTipVerticalOffset::<Identity, OFFSET>,
            SetKeyTipVerticalOffset: SetKeyTipVerticalOffset::<Identity, OFFSET>,
            XYFocusKeyboardNavigation: 0,
            SetXYFocusKeyboardNavigation: 0,
            XYFocusUpNavigationStrategy: 0,
            SetXYFocusUpNavigationStrategy: 0,
            XYFocusDownNavigationStrategy: 0,
            SetXYFocusDownNavigationStrategy: 0,
            XYFocusLeftNavigationStrategy: 0,
            SetXYFocusLeftNavigationStrategy: 0,
            XYFocusRightNavigationStrategy: 0,
            SetXYFocusRightNavigationStrategy: 0,
            HighContrastAdjustment: 0,
            SetHighContrastAdjustment: 0,
            TabFocusNavigation: 0,
            SetTabFocusNavigation: 0,
            GettingFocus: 0,
            RemoveGettingFocus: RemoveGettingFocus::<Identity, OFFSET>,
            LosingFocus: 0,
            RemoveLosingFocus: RemoveLosingFocus::<Identity, OFFSET>,
            NoFocusCandidateFound: 0,
            RemoveNoFocusCandidateFound: RemoveNoFocusCandidateFound::<Identity, OFFSET>,
            StartBringIntoView: StartBringIntoView::<Identity, OFFSET>,
            StartBringIntoViewWithOptions: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElement5 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Lights: usize,
    KeyTipPlacementMode: usize,
    SetKeyTipPlacementMode: usize,
    pub KeyTipHorizontalOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetKeyTipHorizontalOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub KeyTipVerticalOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetKeyTipVerticalOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    XYFocusKeyboardNavigation: usize,
    SetXYFocusKeyboardNavigation: usize,
    XYFocusUpNavigationStrategy: usize,
    SetXYFocusUpNavigationStrategy: usize,
    XYFocusDownNavigationStrategy: usize,
    SetXYFocusDownNavigationStrategy: usize,
    XYFocusLeftNavigationStrategy: usize,
    SetXYFocusLeftNavigationStrategy: usize,
    XYFocusRightNavigationStrategy: usize,
    SetXYFocusRightNavigationStrategy: usize,
    HighContrastAdjustment: usize,
    SetHighContrastAdjustment: usize,
    TabFocusNavigation: usize,
    SetTabFocusNavigation: usize,
    GettingFocus: usize,
    pub RemoveGettingFocus:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    LosingFocus: usize,
    pub RemoveLosingFocus:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    NoFocusCandidateFound: usize,
    pub RemoveNoFocusCandidateFound:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub StartBringIntoView:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    StartBringIntoViewWithOptions: usize,
}
windows_core::imp::define_interface!(
    IUIElement7,
    IUIElement7_Vtbl,
    0xcafc4968_6369_4249_80f9_3d656319e811
);
impl windows_core::RuntimeType for IUIElement7 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElement7 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement7";
}
pub trait IUIElement7_Impl: windows_core::IUnknownImpl {
    fn RemoveCharacterReceived(&self, token: i64) -> windows_core::Result<()>;
    fn RemoveProcessKeyboardAccelerators(&self, token: i64) -> windows_core::Result<()>;
    fn RemovePreviewKeyDown(&self, token: i64) -> windows_core::Result<()>;
    fn RemovePreviewKeyUp(&self, token: i64) -> windows_core::Result<()>;
}
impl IUIElement7_Vtbl {
    pub const fn new<Identity: IUIElement7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RemoveCharacterReceived<
            Identity: IUIElement7_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement7_Impl::RemoveCharacterReceived(this, token).into()
            }
        }
        unsafe extern "system" fn RemoveProcessKeyboardAccelerators<
            Identity: IUIElement7_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement7_Impl::RemoveProcessKeyboardAccelerators(this, token).into()
            }
        }
        unsafe extern "system" fn RemovePreviewKeyDown<
            Identity: IUIElement7_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement7_Impl::RemovePreviewKeyDown(this, token).into()
            }
        }
        unsafe extern "system" fn RemovePreviewKeyUp<
            Identity: IUIElement7_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement7_Impl::RemovePreviewKeyUp(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElement7, OFFSET>(),
            KeyboardAccelerators: 0,
            CharacterReceived: 0,
            RemoveCharacterReceived: RemoveCharacterReceived::<Identity, OFFSET>,
            ProcessKeyboardAccelerators: 0,
            RemoveProcessKeyboardAccelerators: RemoveProcessKeyboardAccelerators::<Identity, OFFSET>,
            PreviewKeyDown: 0,
            RemovePreviewKeyDown: RemovePreviewKeyDown::<Identity, OFFSET>,
            PreviewKeyUp: 0,
            RemovePreviewKeyUp: RemovePreviewKeyUp::<Identity, OFFSET>,
            TryInvokeKeyboardAccelerator: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElement7 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    KeyboardAccelerators: usize,
    CharacterReceived: usize,
    pub RemoveCharacterReceived:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ProcessKeyboardAccelerators: usize,
    pub RemoveProcessKeyboardAccelerators:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PreviewKeyDown: usize,
    pub RemovePreviewKeyDown:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PreviewKeyUp: usize,
    pub RemovePreviewKeyUp:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    TryInvokeKeyboardAccelerator: usize,
}
windows_core::imp::define_interface!(
    IUIElement8,
    IUIElement8_Vtbl,
    0x3ab70e85_d508_4477_b6f8_0e435701c836
);
impl windows_core::RuntimeType for IUIElement8 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElement8 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement8";
}
pub trait IUIElement8_Impl: windows_core::IUnknownImpl {
    fn KeyTipTarget(&self) -> windows_core::Result<DependencyObject>;
    fn SetKeyTipTarget(
        &self,
        value: windows_core::Ref<'_, DependencyObject>,
    ) -> windows_core::Result<()>;
    fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<DependencyObject>;
    fn SetKeyboardAcceleratorPlacementTarget(
        &self,
        value: windows_core::Ref<'_, DependencyObject>,
    ) -> windows_core::Result<()>;
    fn RemoveBringIntoViewRequested(&self, token: i64) -> windows_core::Result<()>;
}
impl IUIElement8_Vtbl {
    pub const fn new<Identity: IUIElement8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KeyTipTarget<Identity: IUIElement8_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement8_Impl::KeyTipTarget(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeyTipTarget<
            Identity: IUIElement8_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement8_Impl::SetKeyTipTarget(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn KeyboardAcceleratorPlacementTarget<
            Identity: IUIElement8_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement8_Impl::KeyboardAcceleratorPlacementTarget(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeyboardAcceleratorPlacementTarget<
            Identity: IUIElement8_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement8_Impl::SetKeyboardAcceleratorPlacementTarget(
                    this,
                    core::mem::transmute_copy(&value),
                )
                .into()
            }
        }
        unsafe extern "system" fn RemoveBringIntoViewRequested<
            Identity: IUIElement8_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement8_Impl::RemoveBringIntoViewRequested(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElement8, OFFSET>(),
            KeyTipTarget: KeyTipTarget::<Identity, OFFSET>,
            SetKeyTipTarget: SetKeyTipTarget::<Identity, OFFSET>,
            KeyboardAcceleratorPlacementTarget: KeyboardAcceleratorPlacementTarget::<
                Identity,
                OFFSET,
            >,
            SetKeyboardAcceleratorPlacementTarget: SetKeyboardAcceleratorPlacementTarget::<
                Identity,
                OFFSET,
            >,
            KeyboardAcceleratorPlacementMode: 0,
            SetKeyboardAcceleratorPlacementMode: 0,
            BringIntoViewRequested: 0,
            RemoveBringIntoViewRequested: RemoveBringIntoViewRequested::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElement8 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement8_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KeyTipTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetKeyTipTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub KeyboardAcceleratorPlacementTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetKeyboardAcceleratorPlacementTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
    KeyboardAcceleratorPlacementMode: usize,
    SetKeyboardAcceleratorPlacementMode: usize,
    BringIntoViewRequested: usize,
    pub RemoveBringIntoViewRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUIElement9,
    IUIElement9_Vtbl,
    0xb4a04776_4e88_50ca_8f2b_08940d6c5f94
);
impl windows_core::RuntimeType for IUIElement9 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElement9 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement9";
}
pub trait IUIElement9_Impl: windows_core::IUnknownImpl {
    fn CanBeScrollAnchor(&self) -> windows_core::Result<bool>;
    fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()>;
    fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3>;
    fn SetTranslation(&self, value: &windows_numerics::Vector3) -> windows_core::Result<()>;
    fn Rotation(&self) -> windows_core::Result<f32>;
    fn SetRotation(&self, value: f32) -> windows_core::Result<()>;
    fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3>;
    fn SetScale(&self, value: &windows_numerics::Vector3) -> windows_core::Result<()>;
    fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4>;
    fn SetTransformMatrix(&self, value: &windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3>;
    fn SetCenterPoint(&self, value: &windows_numerics::Vector3) -> windows_core::Result<()>;
    fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3>;
    fn SetRotationAxis(&self, value: &windows_numerics::Vector3) -> windows_core::Result<()>;
}
impl IUIElement9_Vtbl {
    pub const fn new<Identity: IUIElement9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CanBeScrollAnchor<
            Identity: IUIElement9_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement9_Impl::CanBeScrollAnchor(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCanBeScrollAnchor<
            Identity: IUIElement9_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement9_Impl::SetCanBeScrollAnchor(this, value).into()
            }
        }
        unsafe extern "system" fn Translation<Identity: IUIElement9_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut windows_numerics::Vector3,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement9_Impl::Translation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTranslation<
            Identity: IUIElement9_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: windows_numerics::Vector3,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement9_Impl::SetTranslation(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Rotation<Identity: IUIElement9_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement9_Impl::Rotation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRotation<Identity: IUIElement9_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement9_Impl::SetRotation(this, value).into()
            }
        }
        unsafe extern "system" fn Scale<Identity: IUIElement9_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut windows_numerics::Vector3,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement9_Impl::Scale(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScale<Identity: IUIElement9_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: windows_numerics::Vector3,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement9_Impl::SetScale(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn TransformMatrix<
            Identity: IUIElement9_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut windows_numerics::Matrix4x4,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement9_Impl::TransformMatrix(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTransformMatrix<
            Identity: IUIElement9_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: windows_numerics::Matrix4x4,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement9_Impl::SetTransformMatrix(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn CenterPoint<Identity: IUIElement9_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut windows_numerics::Vector3,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement9_Impl::CenterPoint(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCenterPoint<
            Identity: IUIElement9_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: windows_numerics::Vector3,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement9_Impl::SetCenterPoint(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn RotationAxis<Identity: IUIElement9_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut windows_numerics::Vector3,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElement9_Impl::RotationAxis(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRotationAxis<
            Identity: IUIElement9_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: windows_numerics::Vector3,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElement9_Impl::SetRotationAxis(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElement9, OFFSET>(),
            CanBeScrollAnchor: CanBeScrollAnchor::<Identity, OFFSET>,
            SetCanBeScrollAnchor: SetCanBeScrollAnchor::<Identity, OFFSET>,
            OpacityTransition: 0,
            SetOpacityTransition: 0,
            Translation: Translation::<Identity, OFFSET>,
            SetTranslation: SetTranslation::<Identity, OFFSET>,
            TranslationTransition: 0,
            SetTranslationTransition: 0,
            Rotation: Rotation::<Identity, OFFSET>,
            SetRotation: SetRotation::<Identity, OFFSET>,
            RotationTransition: 0,
            SetRotationTransition: 0,
            Scale: Scale::<Identity, OFFSET>,
            SetScale: SetScale::<Identity, OFFSET>,
            ScaleTransition: 0,
            SetScaleTransition: 0,
            TransformMatrix: TransformMatrix::<Identity, OFFSET>,
            SetTransformMatrix: SetTransformMatrix::<Identity, OFFSET>,
            CenterPoint: CenterPoint::<Identity, OFFSET>,
            SetCenterPoint: SetCenterPoint::<Identity, OFFSET>,
            RotationAxis: RotationAxis::<Identity, OFFSET>,
            SetRotationAxis: SetRotationAxis::<Identity, OFFSET>,
            StartAnimation: 0,
            StopAnimation: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElement9 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement9_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanBeScrollAnchor:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanBeScrollAnchor:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    OpacityTransition: usize,
    SetOpacityTransition: usize,
    pub Translation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    pub SetTranslation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    TranslationTransition: usize,
    SetTranslationTransition: usize,
    pub Rotation:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetRotation:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    RotationTransition: usize,
    SetRotationTransition: usize,
    pub Scale: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    pub SetScale: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    ScaleTransition: usize,
    SetScaleTransition: usize,
    pub TransformMatrix: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_numerics::Matrix4x4,
    ) -> windows_core::HRESULT,
    pub SetTransformMatrix: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Matrix4x4,
    ) -> windows_core::HRESULT,
    pub CenterPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    pub SetCenterPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    pub RotationAxis: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    pub SetRotationAxis: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    StartAnimation: usize,
    StopAnimation: usize,
}
windows_core::imp::define_interface!(
    IUIElementFactory,
    IUIElementFactory_Vtbl,
    0xb9ee93fe_a338_419f_ac32_91dcaadf5d08
);
impl windows_core::RuntimeType for IUIElementFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElementFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementFactory";
}
pub trait IUIElementFactory_Impl: windows_core::IUnknownImpl {}
impl IUIElementFactory_Vtbl {
    pub const fn new<Identity: IUIElementFactory_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementFactory, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IUIElementOverrides,
    IUIElementOverrides_Vtbl,
    0x608d2f1d_7858_4aeb_89e4_b54e2c7ed3d3
);
impl windows_core::RuntimeType for IUIElementOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElementOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides";
}
pub trait IUIElementOverrides_Impl: windows_core::IUnknownImpl {
    fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()>;
}
impl IUIElementOverrides_Vtbl {
    pub const fn new<Identity: IUIElementOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDisconnectVisualChildren<
            Identity: IUIElementOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElementOverrides_Impl::OnDisconnectVisualChildren(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementOverrides, OFFSET>(),
            OnCreateAutomationPeer: 0,
            OnDisconnectVisualChildren: OnDisconnectVisualChildren::<Identity, OFFSET>,
            FindSubElementsForTouchTargeting: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementOverrides as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    OnCreateAutomationPeer: usize,
    pub OnDisconnectVisualChildren:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    FindSubElementsForTouchTargeting: usize,
}
windows_core::imp::define_interface!(
    IUIElementOverrides7,
    IUIElementOverrides7_Vtbl,
    0xb97f7f68_c29b_4c99_a1c3_952619d6e720
);
impl windows_core::RuntimeType for IUIElementOverrides7 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElementOverrides7 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides7";
}
pub trait IUIElementOverrides7_Impl: windows_core::IUnknownImpl {
    fn GetChildrenInTabFocusOrder(
        &self,
    ) -> windows_core::Result<windows_collections::IIterable<DependencyObject>>;
}
impl IUIElementOverrides7_Vtbl {
    pub const fn new<Identity: IUIElementOverrides7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetChildrenInTabFocusOrder<
            Identity: IUIElementOverrides7_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementOverrides7_Impl::GetChildrenInTabFocusOrder(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementOverrides7, OFFSET>(
            ),
            GetChildrenInTabFocusOrder: GetChildrenInTabFocusOrder::<Identity, OFFSET>,
            OnProcessKeyboardAccelerators: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementOverrides7 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementOverrides7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetChildrenInTabFocusOrder: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    OnProcessKeyboardAccelerators: usize,
}
windows_core::imp::define_interface!(
    IUIElementOverrides8,
    IUIElementOverrides8_Vtbl,
    0x4a5a645c_548d_48cf_b998_7844d6e235a1
);
impl windows_core::RuntimeType for IUIElementOverrides8 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElementOverrides8 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides8";
}
pub trait IUIElementOverrides8_Impl: windows_core::IUnknownImpl {}
impl IUIElementOverrides8_Vtbl {
    pub const fn new<Identity: IUIElementOverrides8_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementOverrides8, OFFSET>(
            ),
            OnKeyboardAcceleratorInvoked: 0,
            OnBringIntoViewRequested: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementOverrides8 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementOverrides8_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    OnKeyboardAcceleratorInvoked: usize,
    OnBringIntoViewRequested: usize,
}
windows_core::imp::define_interface!(
    IUIElementOverrides9,
    IUIElementOverrides9_Vtbl,
    0x9a6e5973_6d63_54f2_90fa_62813b20b7b9
);
impl windows_core::RuntimeType for IUIElementOverrides9 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElementOverrides9 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides9";
}
pub trait IUIElementOverrides9_Impl: windows_core::IUnknownImpl {}
impl IUIElementOverrides9_Vtbl {
    pub const fn new<Identity: IUIElementOverrides9_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementOverrides9, OFFSET>(
            ),
            PopulatePropertyInfoOverride: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementOverrides9 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementOverrides9_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PopulatePropertyInfoOverride: usize,
}
windows_core::imp::define_interface!(
    IUIElementStatics,
    IUIElementStatics_Vtbl,
    0x58d3573b_f52c_45be_988b_a5869564873c
);
impl windows_core::RuntimeType for IUIElementStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElementStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics";
}
pub trait IUIElementStatics_Impl: windows_core::IUnknownImpl {
    fn KeyDownEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn KeyUpEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn PointerEnteredEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn PointerPressedEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn PointerMovedEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn PointerReleasedEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn PointerExitedEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn PointerCaptureLostEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn PointerCanceledEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn PointerWheelChangedEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn TappedEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn DoubleTappedEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn HoldingEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn RightTappedEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn ManipulationStartingEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn ManipulationInertiaStartingEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn ManipulationStartedEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn ManipulationDeltaEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn ManipulationCompletedEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn DragEnterEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn DragLeaveEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn DragOverEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn DropEvent(&self) -> windows_core::Result<RoutedEvent>;
}
impl IUIElementStatics_Vtbl {
    pub const fn new<Identity: IUIElementStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KeyDownEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::KeyDownEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeyUpEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::KeyUpEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerEnteredEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::PointerEnteredEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerPressedEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::PointerPressedEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerMovedEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::PointerMovedEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerReleasedEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::PointerReleasedEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerExitedEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::PointerExitedEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerCaptureLostEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::PointerCaptureLostEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerCanceledEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::PointerCanceledEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerWheelChangedEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::PointerWheelChangedEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TappedEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::TappedEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DoubleTappedEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::DoubleTappedEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HoldingEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::HoldingEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RightTappedEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::RightTappedEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ManipulationStartingEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::ManipulationStartingEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ManipulationInertiaStartingEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::ManipulationInertiaStartingEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ManipulationStartedEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::ManipulationStartedEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ManipulationDeltaEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::ManipulationDeltaEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ManipulationCompletedEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::ManipulationCompletedEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DragEnterEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::DragEnterEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DragLeaveEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::DragLeaveEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DragOverEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::DragOverEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DropEvent<
            Identity: IUIElementStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics_Impl::DropEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementStatics, OFFSET>(),
            KeyDownEvent: KeyDownEvent::<Identity, OFFSET>,
            KeyUpEvent: KeyUpEvent::<Identity, OFFSET>,
            PointerEnteredEvent: PointerEnteredEvent::<Identity, OFFSET>,
            PointerPressedEvent: PointerPressedEvent::<Identity, OFFSET>,
            PointerMovedEvent: PointerMovedEvent::<Identity, OFFSET>,
            PointerReleasedEvent: PointerReleasedEvent::<Identity, OFFSET>,
            PointerExitedEvent: PointerExitedEvent::<Identity, OFFSET>,
            PointerCaptureLostEvent: PointerCaptureLostEvent::<Identity, OFFSET>,
            PointerCanceledEvent: PointerCanceledEvent::<Identity, OFFSET>,
            PointerWheelChangedEvent: PointerWheelChangedEvent::<Identity, OFFSET>,
            TappedEvent: TappedEvent::<Identity, OFFSET>,
            DoubleTappedEvent: DoubleTappedEvent::<Identity, OFFSET>,
            HoldingEvent: HoldingEvent::<Identity, OFFSET>,
            RightTappedEvent: RightTappedEvent::<Identity, OFFSET>,
            ManipulationStartingEvent: ManipulationStartingEvent::<Identity, OFFSET>,
            ManipulationInertiaStartingEvent: ManipulationInertiaStartingEvent::<Identity, OFFSET>,
            ManipulationStartedEvent: ManipulationStartedEvent::<Identity, OFFSET>,
            ManipulationDeltaEvent: ManipulationDeltaEvent::<Identity, OFFSET>,
            ManipulationCompletedEvent: ManipulationCompletedEvent::<Identity, OFFSET>,
            DragEnterEvent: DragEnterEvent::<Identity, OFFSET>,
            DragLeaveEvent: DragLeaveEvent::<Identity, OFFSET>,
            DragOverEvent: DragOverEvent::<Identity, OFFSET>,
            DropEvent: DropEvent::<Identity, OFFSET>,
            AllowDropProperty: 0,
            OpacityProperty: 0,
            ClipProperty: 0,
            RenderTransformProperty: 0,
            ProjectionProperty: 0,
            RenderTransformOriginProperty: 0,
            IsHitTestVisibleProperty: 0,
            VisibilityProperty: 0,
            UseLayoutRoundingProperty: 0,
            TransitionsProperty: 0,
            CacheModeProperty: 0,
            IsTapEnabledProperty: 0,
            IsDoubleTapEnabledProperty: 0,
            IsRightTapEnabledProperty: 0,
            IsHoldingEnabledProperty: 0,
            ManipulationModeProperty: 0,
            PointerCapturesProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KeyDownEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub KeyUpEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PointerEnteredEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PointerPressedEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PointerMovedEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PointerReleasedEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PointerExitedEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PointerCaptureLostEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PointerCanceledEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PointerWheelChangedEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub TappedEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DoubleTappedEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub HoldingEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RightTappedEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ManipulationStartingEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ManipulationInertiaStartingEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ManipulationStartedEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ManipulationDeltaEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ManipulationCompletedEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DragEnterEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DragLeaveEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DragOverEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DropEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    AllowDropProperty: usize,
    OpacityProperty: usize,
    ClipProperty: usize,
    RenderTransformProperty: usize,
    ProjectionProperty: usize,
    RenderTransformOriginProperty: usize,
    IsHitTestVisibleProperty: usize,
    VisibilityProperty: usize,
    UseLayoutRoundingProperty: usize,
    TransitionsProperty: usize,
    CacheModeProperty: usize,
    IsTapEnabledProperty: usize,
    IsDoubleTapEnabledProperty: usize,
    IsRightTapEnabledProperty: usize,
    IsHoldingEnabledProperty: usize,
    ManipulationModeProperty: usize,
    PointerCapturesProperty: usize,
}
windows_core::imp::define_interface!(
    IUIElementStatics10,
    IUIElementStatics10_Vtbl,
    0x60d25362_4b3e_53da_8b78_38db94ae8f26
);
impl windows_core::RuntimeType for IUIElementStatics10 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElementStatics10 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics10";
}
pub trait IUIElementStatics10_Impl: windows_core::IUnknownImpl {}
impl IUIElementStatics10_Vtbl {
    pub const fn new<Identity: IUIElementStatics10_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementStatics10, OFFSET>(),
            ShadowProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementStatics10 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics10_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ShadowProperty: usize,
}
windows_core::imp::define_interface!(
    IUIElementStatics2,
    IUIElementStatics2_Vtbl,
    0x58d3574b_f53c_45be_989b_a5869564874c
);
impl windows_core::RuntimeType for IUIElementStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElementStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics2";
}
pub trait IUIElementStatics2_Impl: windows_core::IUnknownImpl {}
impl IUIElementStatics2_Vtbl {
    pub const fn new<Identity: IUIElementStatics2_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementStatics2, OFFSET>(),
            CompositeModeProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementStatics2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CompositeModeProperty: usize,
}
windows_core::imp::define_interface!(
    IUIElementStatics3,
    IUIElementStatics3_Vtbl,
    0xd1f87ade_eca1_4561_a32b_64601b4e0597
);
impl windows_core::RuntimeType for IUIElementStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElementStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics3";
}
pub trait IUIElementStatics3_Impl: windows_core::IUnknownImpl {}
impl IUIElementStatics3_Vtbl {
    pub const fn new<Identity: IUIElementStatics3_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementStatics3, OFFSET>(),
            Transform3DProperty: 0,
            CanDragProperty: 0,
            TryStartDirectManipulation: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementStatics3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Transform3DProperty: usize,
    CanDragProperty: usize,
    TryStartDirectManipulation: usize,
}
windows_core::imp::define_interface!(
    IUIElementStatics4,
    IUIElementStatics4_Vtbl,
    0x1d157d61_16af_411f_b774_272375a4ac2c
);
impl windows_core::RuntimeType for IUIElementStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElementStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics4";
}
pub trait IUIElementStatics4_Impl: windows_core::IUnknownImpl {}
impl IUIElementStatics4_Vtbl {
    pub const fn new<Identity: IUIElementStatics4_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementStatics4, OFFSET>(),
            ContextFlyoutProperty: 0,
            ExitDisplayModeOnAccessKeyInvokedProperty: 0,
            IsAccessKeyScopeProperty: 0,
            AccessKeyScopeOwnerProperty: 0,
            AccessKeyProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementStatics4 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ContextFlyoutProperty: usize,
    ExitDisplayModeOnAccessKeyInvokedProperty: usize,
    IsAccessKeyScopeProperty: usize,
    AccessKeyScopeOwnerProperty: usize,
    AccessKeyProperty: usize,
}
windows_core::imp::define_interface!(
    IUIElementStatics5,
    IUIElementStatics5_Vtbl,
    0x59bd7d91_8fa3_4c65_ba1b_40df38556cbb
);
impl windows_core::RuntimeType for IUIElementStatics5 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElementStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics5";
}
pub trait IUIElementStatics5_Impl: windows_core::IUnknownImpl {}
impl IUIElementStatics5_Vtbl {
    pub const fn new<Identity: IUIElementStatics5_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementStatics5, OFFSET>(),
            LightsProperty: 0,
            KeyTipPlacementModeProperty: 0,
            KeyTipHorizontalOffsetProperty: 0,
            KeyTipVerticalOffsetProperty: 0,
            XYFocusKeyboardNavigationProperty: 0,
            XYFocusUpNavigationStrategyProperty: 0,
            XYFocusDownNavigationStrategyProperty: 0,
            XYFocusLeftNavigationStrategyProperty: 0,
            XYFocusRightNavigationStrategyProperty: 0,
            HighContrastAdjustmentProperty: 0,
            TabFocusNavigationProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementStatics5 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    LightsProperty: usize,
    KeyTipPlacementModeProperty: usize,
    KeyTipHorizontalOffsetProperty: usize,
    KeyTipVerticalOffsetProperty: usize,
    XYFocusKeyboardNavigationProperty: usize,
    XYFocusUpNavigationStrategyProperty: usize,
    XYFocusDownNavigationStrategyProperty: usize,
    XYFocusLeftNavigationStrategyProperty: usize,
    XYFocusRightNavigationStrategyProperty: usize,
    HighContrastAdjustmentProperty: usize,
    TabFocusNavigationProperty: usize,
}
windows_core::imp::define_interface!(
    IUIElementStatics6,
    IUIElementStatics6_Vtbl,
    0x647e03b7_036a_4dea_9540_1dd7fd1266f1
);
impl windows_core::RuntimeType for IUIElementStatics6 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElementStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics6";
}
pub trait IUIElementStatics6_Impl: windows_core::IUnknownImpl {
    fn GettingFocusEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn LosingFocusEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn NoFocusCandidateFoundEvent(&self) -> windows_core::Result<RoutedEvent>;
}
impl IUIElementStatics6_Vtbl {
    pub const fn new<Identity: IUIElementStatics6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GettingFocusEvent<
            Identity: IUIElementStatics6_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics6_Impl::GettingFocusEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LosingFocusEvent<
            Identity: IUIElementStatics6_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics6_Impl::LosingFocusEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NoFocusCandidateFoundEvent<
            Identity: IUIElementStatics6_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics6_Impl::NoFocusCandidateFoundEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementStatics6, OFFSET>(),
            GettingFocusEvent: GettingFocusEvent::<Identity, OFFSET>,
            LosingFocusEvent: LosingFocusEvent::<Identity, OFFSET>,
            NoFocusCandidateFoundEvent: NoFocusCandidateFoundEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementStatics6 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GettingFocusEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub LosingFocusEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub NoFocusCandidateFoundEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUIElementStatics7,
    IUIElementStatics7_Vtbl,
    0xda9b4493_a695_4145_ae93_888024396a0f
);
impl windows_core::RuntimeType for IUIElementStatics7 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElementStatics7 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics7";
}
pub trait IUIElementStatics7_Impl: windows_core::IUnknownImpl {
    fn PreviewKeyDownEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn CharacterReceivedEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn PreviewKeyUpEvent(&self) -> windows_core::Result<RoutedEvent>;
}
impl IUIElementStatics7_Vtbl {
    pub const fn new<Identity: IUIElementStatics7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PreviewKeyDownEvent<
            Identity: IUIElementStatics7_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics7_Impl::PreviewKeyDownEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CharacterReceivedEvent<
            Identity: IUIElementStatics7_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics7_Impl::CharacterReceivedEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PreviewKeyUpEvent<
            Identity: IUIElementStatics7_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics7_Impl::PreviewKeyUpEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementStatics7, OFFSET>(),
            PreviewKeyDownEvent: PreviewKeyDownEvent::<Identity, OFFSET>,
            CharacterReceivedEvent: CharacterReceivedEvent::<Identity, OFFSET>,
            PreviewKeyUpEvent: PreviewKeyUpEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementStatics7 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PreviewKeyDownEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CharacterReceivedEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PreviewKeyUpEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUIElementStatics8,
    IUIElementStatics8_Vtbl,
    0x17be3487_4875_4915_b0b1_a4c0f851df3f
);
impl windows_core::RuntimeType for IUIElementStatics8 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElementStatics8 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics8";
}
pub trait IUIElementStatics8_Impl: windows_core::IUnknownImpl {
    fn BringIntoViewRequestedEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn ContextRequestedEvent(&self) -> windows_core::Result<RoutedEvent>;
    fn RegisterAsScrollPort(
        &self,
        element: windows_core::Ref<'_, UIElement>,
    ) -> windows_core::Result<()>;
}
impl IUIElementStatics8_Vtbl {
    pub const fn new<Identity: IUIElementStatics8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BringIntoViewRequestedEvent<
            Identity: IUIElementStatics8_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics8_Impl::BringIntoViewRequestedEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ContextRequestedEvent<
            Identity: IUIElementStatics8_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementStatics8_Impl::ContextRequestedEvent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterAsScrollPort<
            Identity: IUIElementStatics8_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElementStatics8_Impl::RegisterAsScrollPort(
                    this,
                    core::mem::transmute_copy(&element),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementStatics8, OFFSET>(),
            BringIntoViewRequestedEvent: BringIntoViewRequestedEvent::<Identity, OFFSET>,
            ContextRequestedEvent: ContextRequestedEvent::<Identity, OFFSET>,
            KeyTipTargetProperty: 0,
            KeyboardAcceleratorPlacementTargetProperty: 0,
            KeyboardAcceleratorPlacementModeProperty: 0,
            RegisterAsScrollPort: RegisterAsScrollPort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementStatics8 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics8_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BringIntoViewRequestedEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ContextRequestedEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    KeyTipTargetProperty: usize,
    KeyboardAcceleratorPlacementTargetProperty: usize,
    KeyboardAcceleratorPlacementModeProperty: usize,
    pub RegisterAsScrollPort: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUIElementStatics9,
    IUIElementStatics9_Vtbl,
    0x71467e77_8ca3_5ed7_95db_d51cdad77f81
);
impl windows_core::RuntimeType for IUIElementStatics9 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IUIElementStatics9 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics9";
}
pub trait IUIElementStatics9_Impl: windows_core::IUnknownImpl {}
impl IUIElementStatics9_Vtbl {
    pub const fn new<Identity: IUIElementStatics9_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementStatics9, OFFSET>(),
            CanBeScrollAnchorProperty: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementStatics9 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics9_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CanBeScrollAnchorProperty: usize,
}
windows_core::imp::define_interface!(
    IVisualElement,
    IVisualElement_Vtbl,
    0x01e64612_1d82_42f4_8e3f_a722ded33fc7
);
impl windows_core::RuntimeType for IVisualElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IVisualElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for IVisualElement {
    const NAME: &'static str = "Windows.UI.Composition.IVisualElement";
}
pub trait IVisualElement_Impl: windows_core::IUnknownImpl {}
impl IVisualElement_Vtbl {
    pub const fn new<Identity: IVisualElement_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVisualElement, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualElement as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IVisualTreeService,
    IVisualTreeService_Vtbl,
    0xa593b11a_d17f_48bb_8f66_83910731c8a5
);
windows_core::imp::interface_hierarchy!(IVisualTreeService, windows_core::IUnknown);
impl IVisualTreeService {
    pub unsafe fn AdviseVisualTreeChange<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IVisualTreeServiceCallback>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AdviseVisualTreeChange)(
                windows_core::Interface::as_raw(self),
                pcallback.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn UnadviseVisualTreeChange<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IVisualTreeServiceCallback>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UnadviseVisualTreeChange)(
                windows_core::Interface::as_raw(self),
                pcallback.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn CreateInstance(
        &self,
        typename: &windows_core::BSTR,
        value: &windows_core::BSTR,
    ) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(typename),
                core::mem::transmute_copy(value),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetProperty(
        &self,
        instancehandle: u64,
        value: u64,
        propertyindex: u32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetProperty)(
                windows_core::Interface::as_raw(self),
                instancehandle,
                value,
                propertyindex,
            )
            .ok()
        }
    }
    pub unsafe fn ClearProperty(
        &self,
        instancehandle: u64,
        propertyindex: u32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ClearProperty)(
                windows_core::Interface::as_raw(self),
                instancehandle,
                propertyindex,
            )
            .ok()
        }
    }
    pub unsafe fn GetCollectionCount(&self, instancehandle: u64) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCollectionCount)(
                windows_core::Interface::as_raw(self),
                instancehandle,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn AddChild(&self, parent: u64, child: u64, index: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).AddChild)(
                windows_core::Interface::as_raw(self),
                parent,
                child,
                index,
            )
            .ok()
        }
    }
    pub unsafe fn RemoveChild(&self, parent: u64, index: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveChild)(
                windows_core::Interface::as_raw(self),
                parent,
                index,
            )
            .ok()
        }
    }
    pub unsafe fn ClearChildren(&self, parent: u64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ClearChildren)(
                windows_core::Interface::as_raw(self),
                parent,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseVisualTreeChange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub UnadviseVisualTreeChange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetEnums: usize,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut u64,
    ) -> windows_core::HRESULT,
    GetPropertyValuesChain: usize,
    pub SetProperty:
        unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> windows_core::HRESULT,
    pub ClearProperty:
        unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32) -> windows_core::HRESULT,
    pub GetCollectionCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut u32) -> windows_core::HRESULT,
    GetCollectionElements: usize,
    pub AddChild:
        unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> windows_core::HRESULT,
    pub RemoveChild:
        unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32) -> windows_core::HRESULT,
    pub ClearChildren:
        unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
pub trait IVisualTreeService_Impl: windows_core::IUnknownImpl {
    fn AdviseVisualTreeChange(
        &self,
        pcallback: windows_core::Ref<'_, IVisualTreeServiceCallback>,
    ) -> windows_core::Result<()>;
    fn UnadviseVisualTreeChange(
        &self,
        pcallback: windows_core::Ref<'_, IVisualTreeServiceCallback>,
    ) -> windows_core::Result<()>;
    fn CreateInstance(
        &self,
        typename: &windows_core::BSTR,
        value: &windows_core::BSTR,
    ) -> windows_core::Result<u64>;
    fn SetProperty(
        &self,
        instancehandle: u64,
        value: u64,
        propertyindex: u32,
    ) -> windows_core::Result<()>;
    fn ClearProperty(&self, instancehandle: u64, propertyindex: u32) -> windows_core::Result<()>;
    fn GetCollectionCount(&self, instancehandle: u64) -> windows_core::Result<u32>;
    fn AddChild(&self, parent: u64, child: u64, index: u32) -> windows_core::Result<()>;
    fn RemoveChild(&self, parent: u64, index: u32) -> windows_core::Result<()>;
    fn ClearChildren(&self, parent: u64) -> windows_core::Result<()>;
}
impl IVisualTreeService_Vtbl {
    pub const fn new<Identity: IVisualTreeService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseVisualTreeChange<
            Identity: IVisualTreeService_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcallback: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::AdviseVisualTreeChange(
                    this,
                    core::mem::transmute_copy(&pcallback),
                )
                .into()
            }
        }
        unsafe extern "system" fn UnadviseVisualTreeChange<
            Identity: IVisualTreeService_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcallback: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::UnadviseVisualTreeChange(
                    this,
                    core::mem::transmute_copy(&pcallback),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateInstance<
            Identity: IVisualTreeService_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            typename: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
            pinstancehandle: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisualTreeService_Impl::CreateInstance(
                    this,
                    core::mem::transmute(&typename),
                    core::mem::transmute(&value),
                ) {
                    Ok(ok__) => {
                        pinstancehandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<
            Identity: IVisualTreeService_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            instancehandle: u64,
            value: u64,
            propertyindex: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::SetProperty(
                    this,
                    core::mem::transmute_copy(&instancehandle),
                    core::mem::transmute_copy(&value),
                    core::mem::transmute_copy(&propertyindex),
                )
                .into()
            }
        }
        unsafe extern "system" fn ClearProperty<
            Identity: IVisualTreeService_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            instancehandle: u64,
            propertyindex: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::ClearProperty(
                    this,
                    core::mem::transmute_copy(&instancehandle),
                    core::mem::transmute_copy(&propertyindex),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetCollectionCount<
            Identity: IVisualTreeService_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            instancehandle: u64,
            pcollectionsize: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisualTreeService_Impl::GetCollectionCount(
                    this,
                    core::mem::transmute_copy(&instancehandle),
                ) {
                    Ok(ok__) => {
                        pcollectionsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddChild<
            Identity: IVisualTreeService_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: u64,
            child: u64,
            index: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::AddChild(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&child),
                    core::mem::transmute_copy(&index),
                )
                .into()
            }
        }
        unsafe extern "system" fn RemoveChild<
            Identity: IVisualTreeService_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: u64,
            index: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::RemoveChild(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&index),
                )
                .into()
            }
        }
        unsafe extern "system" fn ClearChildren<
            Identity: IVisualTreeService_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::ClearChildren(this, core::mem::transmute_copy(&parent))
                    .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseVisualTreeChange: AdviseVisualTreeChange::<Identity, OFFSET>,
            UnadviseVisualTreeChange: UnadviseVisualTreeChange::<Identity, OFFSET>,
            GetEnums: 0,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            GetPropertyValuesChain: 0,
            SetProperty: SetProperty::<Identity, OFFSET>,
            ClearProperty: ClearProperty::<Identity, OFFSET>,
            GetCollectionCount: GetCollectionCount::<Identity, OFFSET>,
            GetCollectionElements: 0,
            AddChild: AddChild::<Identity, OFFSET>,
            RemoveChild: RemoveChild::<Identity, OFFSET>,
            ClearChildren: ClearChildren::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualTreeService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVisualTreeService {}
windows_core::imp::define_interface!(
    IVisualTreeServiceCallback,
    IVisualTreeServiceCallback_Vtbl,
    0xaa7a8931_80e4_4fec_8f3b_553f87b4966e
);
windows_core::imp::interface_hierarchy!(IVisualTreeServiceCallback, windows_core::IUnknown);
impl IVisualTreeServiceCallback {
    pub unsafe fn OnVisualTreeChange(
        &self,
        relation: ParentChildRelation,
        element: &VisualElement,
        mutationtype: VisualMutationType,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).OnVisualTreeChange)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(relation),
                core::mem::transmute_copy(element),
                mutationtype,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeServiceCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnVisualTreeChange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ParentChildRelation,
        VisualElement,
        VisualMutationType,
    ) -> windows_core::HRESULT,
}
pub trait IVisualTreeServiceCallback_Impl: windows_core::IUnknownImpl {
    fn OnVisualTreeChange(
        &self,
        relation: &ParentChildRelation,
        element: &VisualElement,
        mutationtype: VisualMutationType,
    ) -> windows_core::Result<()>;
}
impl IVisualTreeServiceCallback_Vtbl {
    pub const fn new<Identity: IVisualTreeServiceCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnVisualTreeChange<
            Identity: IVisualTreeServiceCallback_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            relation: ParentChildRelation,
            element: VisualElement,
            mutationtype: VisualMutationType,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeServiceCallback_Impl::OnVisualTreeChange(
                    this,
                    core::mem::transmute(&relation),
                    core::mem::transmute(&element),
                    core::mem::transmute_copy(&mutationtype),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnVisualTreeChange: OnVisualTreeChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualTreeServiceCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVisualTreeServiceCallback {}
windows_core::imp::define_interface!(
    IVisualTreeServiceCallback2,
    IVisualTreeServiceCallback2_Vtbl,
    0xbad9eb88_ae77_4397_b948_5fa2db0a19ea
);
impl core::ops::Deref for IVisualTreeServiceCallback2 {
    type Target = IVisualTreeServiceCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IVisualTreeServiceCallback2,
    windows_core::IUnknown,
    IVisualTreeServiceCallback
);
impl IVisualTreeServiceCallback2 {
    pub unsafe fn OnElementStateChanged<P2>(
        &self,
        element: u64,
        elementstate: VisualElementState,
        context: P2,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnElementStateChanged)(
                windows_core::Interface::as_raw(self),
                element,
                elementstate,
                context.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeServiceCallback2_Vtbl {
    pub base__: IVisualTreeServiceCallback_Vtbl,
    pub OnElementStateChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        VisualElementState,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
}
pub trait IVisualTreeServiceCallback2_Impl: IVisualTreeServiceCallback_Impl {
    fn OnElementStateChanged(
        &self,
        element: u64,
        elementstate: VisualElementState,
        context: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
}
impl IVisualTreeServiceCallback2_Vtbl {
    pub const fn new<Identity: IVisualTreeServiceCallback2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnElementStateChanged<
            Identity: IVisualTreeServiceCallback2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: u64,
            elementstate: VisualElementState,
            context: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeServiceCallback2_Impl::OnElementStateChanged(
                    this,
                    core::mem::transmute_copy(&element),
                    core::mem::transmute_copy(&elementstate),
                    core::mem::transmute(&context),
                )
                .into()
            }
        }
        Self {
            base__: IVisualTreeServiceCallback_Vtbl::new::<Identity, OFFSET>(),
            OnElementStateChanged: OnElementStateChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualTreeServiceCallback2 as windows_core::Interface>::IID
            || iid == &<IVisualTreeServiceCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVisualTreeServiceCallback2 {}
windows_core::imp::define_interface!(
    IXamlDiagnostics,
    IXamlDiagnostics_Vtbl,
    0x18c9e2b6_3f43_4116_9f2b_ff935d7770d2
);
windows_core::imp::interface_hierarchy!(IXamlDiagnostics, windows_core::IUnknown);
impl IXamlDiagnostics {
    pub unsafe fn GetDispatcher(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDispatcher)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetUiLayer(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUiLayer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetApplication(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetApplication)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetIInspectableFromHandle(
        &self,
        instancehandle: u64,
    ) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIInspectableFromHandle)(
                windows_core::Interface::as_raw(self),
                instancehandle,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHandleFromIInspectable<P0>(&self, pinstance: P0) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHandleFromIInspectable)(
                windows_core::Interface::as_raw(self),
                pinstance.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn RegisterInstance<P0>(&self, pinstance: P0) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterInstance)(
                windows_core::Interface::as_raw(self),
                pinstance.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetInitializationData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInitializationData)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlDiagnostics_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDispatcher: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetUiLayer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetApplication: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetIInspectableFromHandle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetHandleFromIInspectable: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut u64,
    ) -> windows_core::HRESULT,
    HitTest: usize,
    pub RegisterInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut u64,
    ) -> windows_core::HRESULT,
    pub GetInitializationData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IXamlDiagnostics_Impl: windows_core::IUnknownImpl {
    fn GetDispatcher(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn GetUiLayer(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn GetApplication(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn GetIInspectableFromHandle(
        &self,
        instancehandle: u64,
    ) -> windows_core::Result<windows_core::IInspectable>;
    fn GetHandleFromIInspectable(
        &self,
        pinstance: windows_core::Ref<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<u64>;
    fn RegisterInstance(
        &self,
        pinstance: windows_core::Ref<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<u64>;
    fn GetInitializationData(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IXamlDiagnostics_Vtbl {
    pub const fn new<Identity: IXamlDiagnostics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDispatcher<
            Identity: IXamlDiagnostics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppdispatcher: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::GetDispatcher(this) {
                    Ok(ok__) => {
                        ppdispatcher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUiLayer<
            Identity: IXamlDiagnostics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pplayer: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::GetUiLayer(this) {
                    Ok(ok__) => {
                        pplayer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetApplication<
            Identity: IXamlDiagnostics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppapplication: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::GetApplication(this) {
                    Ok(ok__) => {
                        ppapplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIInspectableFromHandle<
            Identity: IXamlDiagnostics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            instancehandle: u64,
            ppinstance: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::GetIInspectableFromHandle(
                    this,
                    core::mem::transmute_copy(&instancehandle),
                ) {
                    Ok(ok__) => {
                        ppinstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHandleFromIInspectable<
            Identity: IXamlDiagnostics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pinstance: *mut core::ffi::c_void,
            phandle: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::GetHandleFromIInspectable(
                    this,
                    core::mem::transmute_copy(&pinstance),
                ) {
                    Ok(ok__) => {
                        phandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterInstance<
            Identity: IXamlDiagnostics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pinstance: *mut core::ffi::c_void,
            pinstancehandle: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::RegisterInstance(
                    this,
                    core::mem::transmute_copy(&pinstance),
                ) {
                    Ok(ok__) => {
                        pinstancehandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInitializationData<
            Identity: IXamlDiagnostics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pinitializationdata: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::GetInitializationData(this) {
                    Ok(ok__) => {
                        pinitializationdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDispatcher: GetDispatcher::<Identity, OFFSET>,
            GetUiLayer: GetUiLayer::<Identity, OFFSET>,
            GetApplication: GetApplication::<Identity, OFFSET>,
            GetIInspectableFromHandle: GetIInspectableFromHandle::<Identity, OFFSET>,
            GetHandleFromIInspectable: GetHandleFromIInspectable::<Identity, OFFSET>,
            HitTest: 0,
            RegisterInstance: RegisterInstance::<Identity, OFFSET>,
            GetInitializationData: GetInitializationData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlDiagnostics as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXamlDiagnostics {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ParentChildRelation {
    pub Parent: u64,
    pub Child: u64,
    pub ChildIndex: u32,
}
windows_core::imp::define_interface!(
    PointerEventHandler,
    PointerEventHandler_Vtbl,
    0xe4385929_c004_4bcf_8970_359486e39f88
);
impl windows_core::RuntimeType for PointerEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl PointerEventHandler {
    pub fn new<
        F: FnMut(
                windows_core::Ref<'_, windows_core::IInspectable>,
                windows_core::Ref<'_, PointerRoutedEventArgs>,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = PointerEventHandlerBox {
            vtable: &PointerEventHandlerBox::<F>::VTABLE,
            count: windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<PointerRoutedEventArgs>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Invoke)(
                windows_core::Interface::as_raw(this),
                sender.param().abi(),
                e.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct PointerEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
#[repr(C)]
struct PointerEventHandlerBox<
    F: FnMut(
            windows_core::Ref<'_, windows_core::IInspectable>,
            windows_core::Ref<'_, PointerRoutedEventArgs>,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
> {
    vtable: *const PointerEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                windows_core::Ref<'_, windows_core::IInspectable>,
                windows_core::Ref<'_, PointerRoutedEventArgs>,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    > PointerEventHandlerBox<F>
{
    const VTABLE: PointerEventHandler_Vtbl = PointerEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut core::ffi::c_void,
        iid: *const windows_core::GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <PointerEventHandler as windows_core::Interface>::IID
                || *iid == <windows_core::IUnknown as windows_core::Interface>::IID
                || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID
            {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(
                    core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void),
                    interface,
                );
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            )
            .into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerPoint(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PointerPoint,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl PointerPoint {
    pub fn PointerId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerId)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn FrameId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameId)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsInContact(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInContact)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Properties(&self) -> windows_core::Result<PointerPointProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPhysicalPositionSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPointerPointPhysicalPosition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPhysicalPositionSupported)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetCurrentPoint(pointerid: u32) -> windows_core::Result<PointerPoint> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentPoint)(
                windows_core::Interface::as_raw(this),
                pointerid,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetIntermediatePoints(
        pointerid: u32,
    ) -> windows_core::Result<windows_collections::IVector<PointerPoint>> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIntermediatePoints)(
                windows_core::Interface::as_raw(this),
                pointerid,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPointerPointStatics<R, F: FnOnce(&IPointerPointStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PointerPoint, IPointerPointStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PointerPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPointerPoint>();
}
unsafe impl windows_core::Interface for PointerPoint {
    type Vtable = <IPointerPoint as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerPoint as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PointerPoint {
    const NAME: &'static str = "Windows.UI.Input.PointerPoint";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerPointProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PointerPointProperties,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl PointerPointProperties {
    pub fn Pressure(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pressure)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsInverted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInverted)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsEraser(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEraser)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Orientation(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn XTilt(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XTilt)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn YTilt(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YTilt)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Twist(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Twist)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn TouchConfidence(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TouchConfidence)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsLeftButtonPressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLeftButtonPressed)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsRightButtonPressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightButtonPressed)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsMiddleButtonPressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMiddleButtonPressed)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MouseWheelDelta(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MouseWheelDelta)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsHorizontalMouseWheel(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHorizontalMouseWheel)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsPrimary(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPrimary)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsInRange(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInRange)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsCanceled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCanceled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsBarrelButtonPressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBarrelButtonPressed)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsXButton1Pressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsXButton1Pressed)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsXButton2Pressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsXButton2Pressed)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn PointerUpdateKind(&self) -> windows_core::Result<PointerUpdateKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerUpdateKind)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn HasUsage(&self, usagepage: u32, usageid: u32) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasUsage)(
                windows_core::Interface::as_raw(this),
                usagepage,
                usageid,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetUsageValue(&self, usagepage: u32, usageid: u32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUsageValue)(
                windows_core::Interface::as_raw(this),
                usagepage,
                usageid,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PointerPointProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPointerPointProperties>();
}
unsafe impl windows_core::Interface for PointerPointProperties {
    type Vtable = <IPointerPointProperties as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerPointProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PointerPointProperties {
    const NAME: &'static str = "Windows.UI.Input.PointerPointProperties";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PointerRoutedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(PointerRoutedEventArgs, RoutedEventArgs);
impl PointerRoutedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetHandled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn GetCurrentPoint<P0>(&self, relativeto: P0) -> windows_core::Result<PointerPoint>
    where
        P0: windows_core::Param<UIElement>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentPoint)(
                windows_core::Interface::as_raw(this),
                relativeto.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIntermediatePoints<P0>(
        &self,
        relativeto: P0,
    ) -> windows_core::Result<windows_collections::IVector<PointerPoint>>
    where
        P0: windows_core::Param<UIElement>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIntermediatePoints)(
                windows_core::Interface::as_raw(this),
                relativeto.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsGenerated(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPointerRoutedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGenerated)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PointerRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPointerRoutedEventArgs>();
}
unsafe impl windows_core::Interface for PointerRoutedEventArgs {
    type Vtable = <IPointerRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerRoutedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PointerRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.PointerRoutedEventArgs";
}
unsafe impl Send for PointerRoutedEventArgs {}
unsafe impl Sync for PointerRoutedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PointerUpdateKind(pub i32);
impl PointerUpdateKind {
    pub const Other: Self = Self(0i32);
    pub const LeftButtonPressed: Self = Self(1i32);
    pub const LeftButtonReleased: Self = Self(2i32);
    pub const RightButtonPressed: Self = Self(3i32);
    pub const RightButtonReleased: Self = Self(4i32);
    pub const MiddleButtonPressed: Self = Self(5i32);
    pub const MiddleButtonReleased: Self = Self(6i32);
    pub const XButton1Pressed: Self = Self(7i32);
    pub const XButton1Released: Self = Self(8i32);
    pub const XButton2Pressed: Self = Self(9i32);
    pub const XButton2Released: Self = Self(10i32);
}
impl windows_core::TypeKind for PointerUpdateKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PointerUpdateKind {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.PointerUpdateKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutedEvent(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RoutedEvent,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl RoutedEvent {}
impl windows_core::RuntimeType for RoutedEvent {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRoutedEvent>();
}
unsafe impl windows_core::Interface for RoutedEvent {
    type Vtable = <IRoutedEvent as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRoutedEvent as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RoutedEvent {
    const NAME: &'static str = "Windows.UI.Xaml.RoutedEvent";
}
unsafe impl Send for RoutedEvent {}
unsafe impl Sync for RoutedEvent {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RoutedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl RoutedEventArgs {
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn new() -> windows_core::Result<RoutedEventArgs> {
        Self::IRoutedEventArgsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                &mut core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IRoutedEventArgsFactory<
        R,
        F: FnOnce(&IRoutedEventArgsFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RoutedEventArgs, IRoutedEventArgsFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRoutedEventArgs>();
}
unsafe impl windows_core::Interface for RoutedEventArgs {
    type Vtable = <IRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRoutedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.RoutedEventArgs";
}
unsafe impl Send for RoutedEventArgs {}
unsafe impl Sync for RoutedEventArgs {}
windows_core::imp::define_interface!(
    RoutedEventHandler,
    RoutedEventHandler_Vtbl,
    0xa856e674_b0b6_4bc3_bba8_1ba06e40d4b5
);
impl windows_core::RuntimeType for RoutedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl RoutedEventHandler {
    pub fn new<
        F: FnMut(
                windows_core::Ref<'_, windows_core::IInspectable>,
                windows_core::Ref<'_, RoutedEventArgs>,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = RoutedEventHandlerBox {
            vtable: &RoutedEventHandlerBox::<F>::VTABLE,
            count: windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<RoutedEventArgs>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Invoke)(
                windows_core::Interface::as_raw(this),
                sender.param().abi(),
                e.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct RoutedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
#[repr(C)]
struct RoutedEventHandlerBox<
    F: FnMut(
            windows_core::Ref<'_, windows_core::IInspectable>,
            windows_core::Ref<'_, RoutedEventArgs>,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
> {
    vtable: *const RoutedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                windows_core::Ref<'_, windows_core::IInspectable>,
                windows_core::Ref<'_, RoutedEventArgs>,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    > RoutedEventHandlerBox<F>
{
    const VTABLE: RoutedEventHandler_Vtbl = RoutedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut core::ffi::c_void,
        iid: *const windows_core::GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <RoutedEventHandler as windows_core::Interface>::IID
                || *iid == <windows_core::IUnknown as windows_core::Interface>::IID
                || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID
            {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(
                    core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void),
                    interface,
                );
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            )
            .into()
        }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SourceInfo {
    pub FileName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub LineNumber: u32,
    pub ColumnNumber: u32,
    pub CharPosition: u32,
    pub Hash: core::mem::ManuallyDrop<windows_core::BSTR>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UIElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    UIElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    UIElement,
    IAnimationObject,
    IVisualElement,
    DependencyObject
);
impl UIElement {
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowDrop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetOpacity)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GotFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GotFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveGotFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGotFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<RoutedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LostFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLostFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLostFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDragEnter(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragEnter)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDragLeave(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragLeave)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDragOver(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragOver)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDrop(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDrop)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerPressed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerMoved)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerReleased)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerEntered)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerExited)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCaptureLost<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCaptureLost(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCanceled<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCanceled)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerWheelChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<PointerEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerWheelChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDoubleTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDoubleTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveHolding(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHolding)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveRightTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveRightTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationInertiaStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationStarted(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationDelta(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationDelta)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveManipulationCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).ReleasePointerCaptures)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn AddHandler<P0, P1>(
        &self,
        routedevent: P0,
        handler: P1,
        handledeventstoo: bool,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).AddHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
                handledeventstoo,
            )
            .ok()
        }
    }
    pub fn RemoveHandler<P0, P1>(&self, routedevent: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateMeasure)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateArrange)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanDrag)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveDragStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDropCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDropCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKey)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn RemoveContextRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveContextCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveAccessKeyInvoked(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveGettingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGettingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveLosingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLosingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveNoFocusCandidateFound(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartBringIntoView)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RemoveCharacterReceived(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveCharacterReceived)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveProcessKeyboardAccelerators(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemovePreviewKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemovePreviewKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveBringIntoViewRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveBringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTranslation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetScale)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTransformMatrix(
        &self,
        value: windows_numerics::Matrix4x4,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransformMatrix)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCenterPoint)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotationAxis)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn GetChildrenInTabFocusOrder(
        &self,
    ) -> windows_core::Result<windows_collections::IIterable<DependencyObject>> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KeyDownEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyDownEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn KeyUpEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyUpEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PointerEnteredEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEnteredEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PointerPressedEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressedEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PointerMovedEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMovedEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PointerReleasedEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleasedEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PointerExitedEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExitedEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PointerCaptureLostEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLostEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PointerCanceledEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCanceledEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PointerWheelChangedEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChangedEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TappedEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TappedEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DoubleTappedEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DoubleTappedEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HoldingEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HoldingEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RightTappedEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RightTappedEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ManipulationStartingEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationStartingEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ManipulationInertiaStartingEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationInertiaStartingEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ManipulationStartedEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationStartedEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ManipulationDeltaEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationDeltaEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ManipulationCompletedEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationCompletedEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DragEnterEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragEnterEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DragLeaveEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragLeaveEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DragOverEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragOverEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DropEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DropEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GettingFocusEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics6(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GettingFocusEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LosingFocusEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics6(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LosingFocusEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn NoFocusCandidateFoundEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics6(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NoFocusCandidateFoundEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PreviewKeyDownEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics7(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviewKeyDownEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CharacterReceivedEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics7(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterReceivedEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PreviewKeyUpEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics7(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviewKeyUpEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn BringIntoViewRequestedEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics8(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BringIntoViewRequestedEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ContextRequestedEvent() -> windows_core::Result<RoutedEvent> {
        Self::IUIElementStatics8(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContextRequestedEvent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RegisterAsScrollPort<P0>(element: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IUIElementStatics8(|this| unsafe {
            (windows_core::Interface::vtable(this).RegisterAsScrollPort)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
            )
            .ok()
        })
    }
    fn IUIElementStatics<R, F: FnOnce(&IUIElementStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UIElement, IUIElementStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IUIElementStatics10<R, F: FnOnce(&IUIElementStatics10) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UIElement, IUIElementStatics10> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IUIElementStatics2<R, F: FnOnce(&IUIElementStatics2) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UIElement, IUIElementStatics2> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IUIElementStatics3<R, F: FnOnce(&IUIElementStatics3) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UIElement, IUIElementStatics3> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IUIElementStatics4<R, F: FnOnce(&IUIElementStatics4) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UIElement, IUIElementStatics4> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IUIElementStatics5<R, F: FnOnce(&IUIElementStatics5) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UIElement, IUIElementStatics5> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IUIElementStatics6<R, F: FnOnce(&IUIElementStatics6) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UIElement, IUIElementStatics6> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IUIElementStatics7<R, F: FnOnce(&IUIElementStatics7) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UIElement, IUIElementStatics7> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IUIElementStatics8<R, F: FnOnce(&IUIElementStatics8) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UIElement, IUIElementStatics8> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IUIElementStatics9<R, F: FnOnce(&IUIElementStatics9) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UIElement, IUIElementStatics9> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UIElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IUIElement>();
}
unsafe impl windows_core::Interface for UIElement {
    type Vtable = <IUIElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUIElement as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UIElement {
    const NAME: &'static str = "Windows.UI.Xaml.UIElement";
}
unsafe impl Send for UIElement {}
unsafe impl Sync for UIElement {}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VisualElement {
    pub Handle: u64,
    pub SrcInfo: SourceInfo,
    pub Type: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub Name: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub NumChildren: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VisualElementState(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VisualMutationType(pub i32);
