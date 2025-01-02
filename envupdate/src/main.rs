use windows::core::w;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, TRUE, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, SendMessageTimeoutW, SMTO_NORMAL, WM_SETTINGCHANGE,
};


unsafe extern "system" fn window_callback(window: HWND, parameter: LPARAM) -> BOOL {
    let window_list = parameter.0 as *mut Vec<HWND>;
    (*window_list).push(window);

    // continue enumerating
    TRUE
}


fn main() {
    let mut window_list: Vec<HWND> = Vec::new();
    let window_list_ptr = &mut window_list as *mut Vec<HWND> as isize;

    let enum_result = unsafe { EnumWindows(Some(window_callback), LPARAM(window_list_ptr)) };
    enum_result.expect("failed to enumerate windows");

    let env_string = w!("Environment");
    let env_string_lparam = LPARAM(env_string.0 as isize);
    for window in &window_list {
        let mut message_result = 0;
        let send_result = unsafe {
            SendMessageTimeoutW(
                *window,
                WM_SETTINGCHANGE,
                WPARAM(0),
                env_string_lparam,
                SMTO_NORMAL,
                500,
                Some(&mut message_result),
            )
        };
        if send_result.0 == 0 {
            let io_error = std::io::Error::last_os_error();
            println!("window 0x{:X}: send result 0x{:X}, message result {}, error {}", window.0 as usize, send_result.0, message_result, io_error);
        } else {
            println!("window 0x{:X}: send result 0x{:X}, message result {}", window.0 as usize, send_result.0, message_result);
        }
    }
}
