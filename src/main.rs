use bindings::Windows::{
    Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
    Win32::System::WindowsProgramming::CloseHandle,
    Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK, MB_ICONEXCLAMATION},
};

fn main() -> windows::Result<()> {
    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), true, false, None);
        SetEvent(event).ok()?;
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok()?;

        MessageBoxA(None, "THERE CAN BE ONLY ONE!!!", "MY FIRST RUST-BASED WINDOWS PROGRAM", MB_OK | MB_ICONEXCLAMATION);
    }

    Ok(())
}
