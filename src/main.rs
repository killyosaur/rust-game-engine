use bindings::{
    Windows::{
        Win32::{
            Foundation::*,
            System::{
                LibraryLoader::GetModuleHandleW
            },
            UI::{
                WindowsAndMessaging::*,
            },
            Graphics::Gdi::*,
        }
    }
};

use std::mem::transmute;
use windows::*;

fn main() -> Result<()> {
    run_app("GAME_WINDOW".to_string())?;

    Ok(())
}

#[derive(Clone)]
struct Window {
    hwnd: HWND
}

fn run_app(name: String) -> Result<()> {
    unsafe {
        let instance = GetModuleHandleW(None);
        let mut window = Window {
            hwnd: HWND(0)
        };
        debug_assert!(instance.0 != 0);

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            hCursor: LoadCursorW(None, IDC_ARROW),
            hInstance: instance,
            lpszClassName: PWSTR(name.as_ptr() as _),
            hbrBackground: CreateSolidBrush(0),
            hIconSm: LoadIconW(None, IDI_APPLICATION),
            cbClsExtra: 0,
            cbWndExtra: 0,
            lpfnWndProc: Some(wndproc),
            style: CS_DBLCLKS | CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            ..Default::default() 
        };

        let atom = RegisterClassExW(&wc);

        debug_assert_ne!(atom, 0);

        let handle = CreateWindowExW(
            Default::default(),               // extended style
            PWSTR(name.as_ptr() as _),        // class
            "Your Basic Window, in Rust",     // title
            WS_OVERLAPPEDWINDOW | WS_VISIBLE, 
            CW_USEDEFAULT, CW_USEDEFAULT,     // initial x, y
            400, 400,                         // initial width, height
            None,                             // handled to parent
            None,                             // handle to menu
            instance,                         // instance of this application
            &mut window as *mut _ as _,       // extra creation params
        );

        debug_assert!(!handle.is_null());

        loop {
            let mut message = MSG::default();

            if PeekMessageW(&mut message, None, 0, 0, PM_REMOVE).into() {
                // translate any accelerator keys
                TranslateMessage(&message);

                // send the message to the window proc
                DispatchMessageW(&message);

                if message.message == WM_QUIT {
                    break;
                }
            }
        }

        // return to Windows like this
        Ok(())
    }
}

extern "system" fn wndproc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
        let mut hdc: HDC = HDC::default();

        match message {
            WM_CREATE => {
                // do initialization stuff here
                let create_struct: &CREATESTRUCTW = transmute(lparam);
                SetWindowLongW(window, GWLP_USERDATA, create_struct.lpCreateParams as _);

                // return success 
                LRESULT(0)
            },
            WM_PAINT => {
                // simply validate the window
                hdc = BeginPaint(window, &mut ps);
                // you would do all your painting here
                EndPaint(window, &mut ps);

                // return success
                LRESULT(0)
            },
            WM_DESTROY => {
                // kill the application, this sends a WM_QUIT message
                PostQuitMessage(0);

                // return success
                LRESULT(0)
            },
            // process any message that we didn't take care of
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}
