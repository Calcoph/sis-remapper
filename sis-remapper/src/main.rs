use std::process::exit;

use corsair::init_corsair;
use hotkey_handler::HotkeyHandler;
use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY};

mod corsair;
mod hotkey_handler;

fn main() {
    init();
    main_loop();
}

fn main_loop() -> !{
    loop {
        unsafe {
            let mut message = MSG::default();
            let _ = GetMessageW(&mut message, None, WM_HOTKEY, WM_HOTKEY+1);
            if message.message == WM_HOTKEY {
                HotkeyHandler::handle_hotkey(message.wParam.0 as i32)
            }
        }
    }
}

fn init() {
    ctrlc::set_handler(handle_ctrlc).unwrap();
    HotkeyHandler::init();
    let corsair_sender = init_corsair();
    HotkeyHandler::register_corsair(corsair_sender);
}

fn handle_ctrlc() {
    println!("Cleaning");
    HotkeyHandler::cleanup();

    println!("Exited correctly. Goodbye");
    exit(0)
}
