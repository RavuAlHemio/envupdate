use std::sync::{Mutex, OnceLock};

use windows::core::w;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, TRUE, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, SendMessageTimeoutW, SMTO_NORMAL, WM_SETTINGCHANGE,
};


static WINDOW_LIST: OnceLock<Mutex<Vec<HWND>>> = OnceLock::new();


unsafe extern "system" fn window_callback(window: HWND, _parameter: LPARAM) -> BOOL {
    let mut list_guard = WINDOW_LIST
        .get().expect("WINDOW_LIST not set?!")
        .lock().expect("WINDOW_LIST poisoned?!");
    list_guard.push(window);

    // continue enumerating
    TRUE
}


fn main() {
    let window_list = Mutex::new(Vec::new());
    WINDOW_LIST.set(window_list)
        .expect("WINDOW_LIST already set?!");

    let enum_result = unsafe { EnumWindows(Some(window_callback), LPARAM(0)) };
    enum_result.expect("failed to enumerate windows");

    let window_list_guard = WINDOW_LIST
        .get().expect("WINDOW_LIST not set?!")
        .lock().expect("WINDOW_LIST poisoned?!");
    let env_string = w!("Environment");
    let env_string_lparam = LPARAM(env_string.0 as isize);
    for window in &*window_list_guard {
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
            println!("window 0x{:X}: send result 0x{:X}, message result {}, error {}", window.0, send_result.0, message_result, io_error);
        } else {
            println!("window 0x{:X}: send result 0x{:X}, message result {}", window.0, send_result.0, message_result);
        }
    }
}
