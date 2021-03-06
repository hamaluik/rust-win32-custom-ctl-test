pub fn win32_string(value: &str) -> Vec<u16> {
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(once(0))
        .collect()
}

pub fn default_rect() -> winapi::shared::windef::RECT {
    winapi::shared::windef::RECT { left: 0, top: 0, right: 0, bottom: 0 }
}

pub fn register_custom_classes() {
    crate::close_button::register_close_button();
    crate::button::register_button();
    crate::progress::register_progress();
}

pub fn unregister_custom_classes() {
    crate::close_button::unregister_close_button();
    crate::button::unregister_button();
    crate::progress::unregister_progress();
}
