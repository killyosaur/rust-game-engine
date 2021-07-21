fn main() {
    windows::build!(
        Windows::{
            Win32::{
                Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, PWSTR, WPARAM},
                Graphics::{
                    Direct2D::*,
                    Gdi::{BeginPaint, EndPaint, GetStockObject, CreateSolidBrush, PAINTSTRUCT, HDC, HBRUSH},
                },
                System::{
                    LibraryLoader::GetModuleHandleW,
                    Performance::{QueryPerformanceCounter, QueryPerformanceFrequency},
                    SystemInformation::GetLocalTime,
                },
                UI::{
                    WindowsAndMessaging::{
                        CreateWindowExW, DefWindowProcW, DispatchMessageW,
                        GetWindowLongPtrW, LoadCursorW, PeekMessageW, PostQuitMessage, RegisterClassExW,
                        SetWindowLongW, SetWindowLongPtrW, TranslateMessage, LoadIconW, SendMessageW,
                        CREATESTRUCTW, CW_USEDEFAULT, IDC_ARROW, MSG, WM_DESTROY,
                        WM_CREATE, IDI_APPLICATION, WM_CLOSE, VK_ESCAPE, VK_SPACE, VK_RETURN,
                        WM_PAINT, WM_QUIT, WM_SIZE, WM_USER, WNDCLASSEXW,
                    },
                    KeyboardAndMouseInput::GetAsyncKeyState,
                },
            },
        },
    );  
}
