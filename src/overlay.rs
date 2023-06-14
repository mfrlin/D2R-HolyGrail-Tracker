use windows::{
    core::*, 
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::Graphics::Gdi::*,
};

pub fn show_overlay() {
    unsafe {
        let instance = GetModuleHandleA(None).unwrap();
        let window_class = s!("window");
        let wc = WNDCLASSA {
            style: WNDCLASS_STYLES::default(),
            lpfnWndProc: Some(wndproc),
            lpszClassName: window_class,
            hInstance: instance,
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hbrBackground: CreateSolidBrush(COLORREF(0xFF00FF)),
            ..Default::default()
        };

        RegisterClassA(&wc);

        let overlay_window = CreateWindowExA(
            WS_EX_LAYERED | WS_EX_DLGMODALFRAME,
            window_class,
            None, // title
            WS_VISIBLE | WS_POPUP | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            200,
            200,
            None,
            None,
            instance,
            None,
        );

        SetLayeredWindowAttributes(
            overlay_window,
            COLORREF(0x000000),
            60,
            LWA_ALPHA,
        );

        let mut message = MSG::default();

        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}