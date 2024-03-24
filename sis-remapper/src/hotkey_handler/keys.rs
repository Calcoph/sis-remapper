use sis_core::VirtualKey;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

#[derive(Debug)]
pub(crate) enum Input {
    Keyboard(VirtualKey, KeyDirection),
    Mouse,
    Hardware
}

impl From<Input> for INPUT {
    fn from(value: Input) -> Self {
        match value {
            Input::Keyboard(key, dir) => {
                let flags = dir.to_flag();
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: key.to_vk(),
                            wScan: key.to_scan(),
                            dwFlags: flags,
                            time: 0,
                            dwExtraInfo: 0,
                        }
                    },
                }
            },
            _ => unimplemented!()
        }
    }
}

#[derive(Debug)]
pub(crate) enum KeyDirection {
    Press,
    Release
}

impl KeyDirection {
    fn to_flag(&self) -> KEYBD_EVENT_FLAGS {
        match self {
            KeyDirection::Press => KEYBD_EVENT_FLAGS::default(),
            KeyDirection::Release => KEYEVENTF_KEYUP,
        }
    }
}
