use std::io::BufRead;

use windows::core::{PCWSTR, PWSTR};
use windows::Win32::System::Environment::{FreeEnvironmentStringsW, GetEnvironmentStringsW};


struct EnvironmentStringBlock {
    pub strings: PWSTR,
}
impl Drop for EnvironmentStringBlock {
    fn drop(&mut self) {
        if !self.strings.0.is_null() {
            let _ = unsafe { FreeEnvironmentStringsW(PCWSTR(self.strings.0)) };
        }
    }
}


fn read_nul_terminated_u16_string(ptr: *const u16) -> (Option<String>, *const u16) {
    let slice_start = ptr;
    let mut slice_length: usize = 0;
    loop {
        let current_ptr = unsafe { slice_start.add(slice_length) };
        let ptr_value = unsafe { *current_ptr };
        if ptr_value == 0 {
            // end of string!
            break;
        }
        slice_length += 1;
    }

    let utf16_slice = unsafe { std::slice::from_raw_parts(slice_start, slice_length) };
    let string = String::from_utf16(utf16_slice).ok();
    let new_slice_start = unsafe { slice_start.add(slice_length + 1) };
    (string, new_slice_start)
}


fn main() {
    {
        let string_block_raw = unsafe { GetEnvironmentStringsW() };
        let string_block = EnvironmentStringBlock { strings: string_block_raw };

        // decode environment variables
        let mut ptr = string_block.strings.0 as *const u16;
        loop {
            let (s_opt, new_ptr) = read_nul_terminated_u16_string(ptr);
            if let Some(s) = s_opt {
                if s.len() == 0 {
                    break;
                }
                println!("{}", s);
            }
            ptr = new_ptr;
        }
    }

    {
        eprintln!("(Press Enter to exit.)");

        let stdin = std::io::stdin();
        let mut stdin_guard = stdin.lock();
        let mut never_mind = String::new();
        let _ = stdin_guard.read_line(&mut never_mind);
    }
}
