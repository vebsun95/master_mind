use super::PlatformState;
#[cfg(target_os = "windows")]
use winapi::{
    shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM},
    um::winuser::{
        GET_WHEEL_DELTA_WPARAM, WM_LBUTTONDOWN, WM_MBUTTONDOWN, WM_RBUTTONDOWN, WM_RBUTTONUP,
    },
};
#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{LRESULT, RECT},
    UI::WindowsAndMessaging::{
        DefWindowProcW, GetClientRect, WM_DESTROY, WM_ERASEBKGND, WM_KEYDOWN, WM_KEYUP,
        WM_LBUTTONUP, WM_MBUTTONUP, WM_MOUSEHWHEEL, WM_MOUSEMOVE, WM_SIZE, WM_SYSKEYDOWN,
        WM_SYSKEYUP, WM_CLOSE,
    },
};

#[cfg(target_os = "windows")]
pub struct InternalState {
    h_instance: windows::Win32::Foundation::HINSTANCE,
    hwnd: windows::Win32::Foundation::HWND,
}

impl PlatformState {
    #[cfg(target_os = "windows")]
    pub fn new(
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        application_name: &String,
    ) -> ApplicationState {
        use windows::{
            core::*,
            Win32::Foundation::*,
            Win32::System::LibraryLoader::*,
            Win32::{
                Graphics::Gdi::{GetSysColorBrush, COLOR_3DFACE},
                UI::WindowsAndMessaging::*,
            },
        };

        let module_handle =
            unsafe { GetModuleHandleA(None).expect("Failed to get windows module handle.") };
        let icon =
            unsafe { LoadIconW(module_handle, w!("my-icon")).expect("Failed to load icon.") };
        let cursor = unsafe { LoadCursorW(None, IDC_ARROW).expect("Failed to load cursor.") };

        let mut wc = WNDCLASSA::default();
        wc.style = CS_DBLCLKS;
        wc.lpfnWndProc = Some(windowproc);
        wc.hInstance = module_handle.into();
        wc.hIcon = icon;
        wc.hCursor = cursor;
        wc.lpszClassName = s!("master_mind_class");

        if unsafe { RegisterClassA(&wc) } == 0 {
            panic!("Failed to register windows class.");
        }

        let client_x: i32 = i32::from(x);
        let client_y: i32 = i32::from(y);
        let client_width: i32 = i32::from(width);
        let client_height: i32 = i32::from(height);

        let mut window_x: i32 = client_x;
        let mut window_y: i32 = client_y;
        let mut window_width: i32 = client_width;
        let mut window_height: i32 = client_height;

        let window_style = WS_OVERLAPPED
            | WS_SYSMENU
            | WS_CAPTION
            | WS_MAXIMIZEBOX
            | WS_MINIMIZEBOX
            | WS_THICKFRAME;
        let window_ex_style = WS_EX_APPWINDOW;

        let mut border_rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        unsafe {
            AdjustWindowRectEx(&mut border_rect, window_style, None, window_ex_style)
                .expect("Failed to get window rectangle.")
        };

        window_x += border_rect.left;
        window_y += border_rect.top;

        window_width += border_rect.right - border_rect.left;
        window_height += border_rect.bottom - border_rect.top;

        let handle = unsafe {
            CreateWindowExA(
                window_ex_style,
                s!("master_mind_class"),
                PCSTR::from_raw(format!("{application_name}\0").as_ptr()),
                window_style,
                window_x,
                window_y,
                window_width,
                window_height,
                None,
                None,
                module_handle,
                None,
            )
        };

        let show_window_command_flags = SW_SHOW;

        unsafe { ShowWindow(handle, show_window_command_flags) };

        return ApplicationState {
            internal_state: PlatformState {
                h_instance: module_handle.into(),
                hwnd: handle,
            },
        };
    }
    #[cfg(target_os = "windows")]
    pub fn pump_messages(self: &ApplicationState) -> bool {
        use windows::Win32::{
            Foundation::HWND,
            UI::WindowsAndMessaging::{
                DispatchMessageA, PeekMessageA, PeekMessageW, TranslateMessage, MSG, PM_REMOVE,
            },
        };

        let mut message = MSG::default();

        unsafe {
            while PeekMessageA(&mut message, HWND::default(), 0, 0, PM_REMOVE).into() {
                TranslateMessage(&mut message);
                DispatchMessageA(&mut message);
            }
        }

        return true;
    }
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn windowproc(
    handle: windows::Win32::Foundation::HWND,
    msg: u32,
    wparam: windows::Win32::Foundation::WPARAM,
    lparam: windows::Win32::Foundation::LPARAM,
) -> windows::Win32::Foundation::LRESULT {
    println!("message: {msg}");

    let match_result: Option<LRESULT> = match msg {
        WM_ERASEBKGND => {
            // Notify the os that erasing will be handled by the application.
            Some(LRESULT(1))
        }
        WM_CLOSE => {
            // TODO: fire an event for the application to close.
            Some(LRESULT(0))
        }
        WM_SIZE => {
            //let mut rect = RECT::default();

            //GetClientRect(handle, &mut rect).ok();

            //let width = rect.right - rect.left;
            //let height = rect.top - rect.bottom;

            // TODO: Fire an event for window resize.
            None
        }
        WM_KEYDOWN | WM_SYSKEYDOWN | WM_KEYUP | WM_SYSKEYUP => {
            //let pressed = msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN;
            None
        }
        WM_MOUSEMOVE => {
            let x_position = GET_X_LPARAM(lparam.0);
            let y_position = GET_Y_LPARAM(lparam.0);
            // TODO: Input procssing

            None
        }
        WM_MOUSEHWHEEL => {
            let z_delta = GET_WHEEL_DELTA_WPARAM(wparam.0);

            None
        }
        WM_LBUTTONDOWN | WM_MBUTTONDOWN | WM_RBUTTONDOWN | WM_LBUTTONUP | WM_MBUTTONUP
        | WM_RBUTTONUP => {
            let pressed = msg == WM_LBUTTONDOWN || msg == WM_MBUTTONDOWN || msg == WM_RBUTTONDOWN;
            println!("Mouse button pressed.");

            None
        }
        WM_DESTROY => {
            println!("Received quit message");
            windows::Win32::UI::WindowsAndMessaging::PostQuitMessage(0);

            None
        }
        _ => None,
    };

    if match_result.is_some() {
        return  match_result.unwrap();
    }
    else {
        return DefWindowProcW(handle, msg, wparam, lparam);
    }
}
