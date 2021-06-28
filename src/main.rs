use bindings::{
    Windows::{
        Win32::{
            Foundation::*,
            UI::{
                Animation::*,
                WindowsAndMessaging::*,
            }
        }
    }
};

use windows::*;

fn main() -> Result<()> {
    initialize_sta()?;
    let mut window = Window::new()?;
    window.run()
}

struct Window {
    handle: HWND,
    ps: PAINTSTRUCT,
    hdc: HDC,
}

impl Window {
    

    fn message_handler(&mut self, message u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match message {
                WM_CREATE => {
                    // do initialization stuff here

                    // return success
                    LRESULT(0)
                }
                WM_PAINT {
                    // simply validate the window
                    hdc = BeginPaint(self.handle, &self.ps);
                    // you would do all your painting here
                    EndPaint(self.handle, &self.ps)

                    // return success
                    LRESULT(0)
                }
                WM_DESTROY => {
                    // kill the application, this sends a WM_QUIT message
                    PostQuitMessage(0);

                    // return success
                    LRESULT(0)
                }
                // process any message that we didn't take care of
                _ => DefWindowProcA(self.handle, message, wparam, lparam),
            }
        }
    }
}