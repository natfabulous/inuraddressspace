use windows::{
    Win32::{ UI::WindowsAndMessaging::{ MessageBoxW, MB_OK }, Foundation::HWND },
    core::PCWSTR,
};
use u16cstr::u16cstr;

#[ctor::ctor]
unsafe fn ctor() {
    MessageBoxW(
        HWND::default(),
        PCWSTR(u16cstr!("I'm in ur address space, executing code!").as_ptr()),
        PCWSTR(u16cstr!("A message from rust").as_ptr()),
        MB_OK
    );
    std::process::exit(112)
}