use windows::Win32::{
  Foundation::{HWND, LPARAM, WPARAM},
  UI::{
    WindowsAndMessaging::{WM_CLOSE, SendMessageW},
    Input::{
      KeyboardAndMouse::{VK_ESCAPE, VK_SPACE, VK_RETURN, GetAsyncKeyState}
    }
  }
};

#[allow(overflowing_literals)]
pub fn game_main(main_window_handle: HWND) {
  unsafe {
    let es_state = GetAsyncKeyState(VK_ESCAPE.0 as i32) ^ (0x8000 as i16);

    if es_state == 1 {
      println!("VK_ESCAPED");
      SendMessageW(main_window_handle, WM_CLOSE, WPARAM::default(), LPARAM::default());
    }
  }
}
