use bindings::Windows::Win32::UI::WindowsAndMessaging::{VK_ESCAPE, VK_SPACE, VK_RETURN, WM_CLOSE, SendMessageW};
use bindings::Windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use bindings::Windows::Win32::UI::KeyboardAndMouseInput::GetAsyncKeyState;

#[allow(overflowing_literals)]
pub fn game_main(main_window_handle: HWND) {
  unsafe {
    let es_state = GetAsyncKeyState(VK_ESCAPE as i32) ^ (0x8000 as i16);

    if es_state == 1 {
      println!("VK_ESCAPED");
      SendMessageW(main_window_handle, WM_CLOSE, WPARAM::default(), LPARAM::default());
    }
  }
}
