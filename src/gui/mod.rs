use windows::{
    Win32::UI::WindowsAndMessaging::*,
    Win32::Foundation::*,
    Win32::UI::Controls::*,
};

pub fn create_main_window() {
    unsafe {
        let hinstance = GetModuleHandleW(None).unwrap();
        let class_name = to_wchar("MainWindow");
        
        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: hinstance,
            lpszClassName: class_name.as_ptr(),
            ..Default::default()
        };
        
        RegisterClassW(&wc);
        
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name.as_ptr(),
            to_wchar("NVIDIA Universal Driver").as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            100,
            100,
            400,
            300,
            None,
            None,
            hinstance,
            None,
        );
        
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            CreateWindowW(
                to_wchar("BUTTON").as_ptr(),
                to_wchar("Install Drivers").as_ptr(),
                WS_VISIBLE | WS_CHILD | BS_DEFPUSHBUTTON,
                20,
                20,
                150,
                30,
                hwnd,
                HMENU(1),
                GetModuleHandleW(None).unwrap(),
                None,
            );
            LRESULT(0)
        }
        WM_COMMAND => {
            if wparam.0 as u16 == 1 {
                MessageBoxW(hwnd, to_wchar("Starting installation...").as_ptr(), to_wchar("Status").as_ptr(), MB_OK);
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn to_wchar(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}