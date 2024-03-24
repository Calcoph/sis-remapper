use std::sync::mpsc::{self, Sender};

use config_parse::{get_config, Action, Config, Macro, Profile};
use sis_core::{ColorAnimation, HotkeySlot};
use windows::Win32::UI::Input::KeyboardAndMouse::*;

use crate::{corsair::{effects::Effect, CorsairMsg}, hotkey_handler::keys::{Input, KeyDirection}};

mod keys;
//mod macros;

static mut HOTKEY_HANDLER: HotkeyHandler = HotkeyHandler::new();

pub(crate) struct HotkeyHandler {
    hotkeys: Vec<(HotkeySlot, Vec<INPUT>, Option<String>)>,
    current_profile: String,
    corsair_sender: Option<Sender<CorsairMsg>>,
    profiles: Vec<Profile>,
    macros: Vec<Macro>,
    color_animations: Vec<ColorAnimation>
}

impl HotkeyHandler {
    const fn new() -> HotkeyHandler {
        HotkeyHandler {
            hotkeys: Vec::new(),
            current_profile: String::new(),
            corsair_sender: None,
            profiles: Vec::new(),
            macros: Vec::new(),
            color_animations: Vec::new(),
        }
    }

    pub(crate) fn init() {
        unsafe {
            let Config {
                profiles,
                macros,
                color_animations,
            } = get_config();
            HOTKEY_HANDLER.profiles = profiles;
            HOTKEY_HANDLER.macros = macros;
            HOTKEY_HANDLER.color_animations = color_animations;
            HotkeyHandler::switch_profile("default".into());
        };
    }

    fn register_hotkeys() {
        let hotkeys = unsafe {
            &HOTKEY_HANDLER.hotkeys
        };

        for (hotkey, _, _) in hotkeys {
            let hotkey_id = hotkey.to_id();
            unsafe {
                println!("Registering hotkey {}", hotkey_id);
                if let Err(err) = RegisterHotKey(None, hotkey_id, HOT_KEY_MODIFIERS::default(), hotkey.to_vkey()) {
                    panic!("Error registering hotkey {}: {}", hotkey_id, err)
                };
            }
        }
    }

    pub(crate) fn cleanup() {
        Self::unregister_hotkeys()
    }

    fn unregister_hotkeys() {
        let hotkeys = unsafe {
            &HOTKEY_HANDLER.hotkeys
        };
        for (hotkey, _, _) in hotkeys {
            let hotkey_id = hotkey.to_id();
            println!("Unregistering {hotkey_id}");
            unsafe {
                if let Err(err) = UnregisterHotKey(None, hotkey_id) {
                    println!("Error when unregistering hotkey {}: {}", hotkey_id, err.message())
                }
            }
        }
    }

    pub(crate) fn handle_hotkey(id: i32) {
        println!("Pressed {id}!");

        let hotkeys = unsafe {
            &HOTKEY_HANDLER.hotkeys
        };

        for (hotkey, inputs, after_profile) in hotkeys {
            if hotkey.to_id() == id {
                let res = unsafe { SendInput(inputs, std::mem::size_of::<INPUT>() as i32) };
                assert_eq!(inputs.len(), res as usize);

                if let Some(profile) = after_profile {
                    HotkeyHandler::switch_profile(profile.clone())
                }
            }
        }
    }

    pub(crate) fn switch_profile(profile: String) {
        println!("Switching to profile {profile:?}!");
        Self::unregister_hotkeys();

        let this = unsafe {
            &mut HOTKEY_HANDLER
        };
        this.current_profile = profile;
        let mut actions = Vec::new();
        for profile in this.profiles.iter() {
            if profile.name == this.current_profile {
                actions = profile.actions.clone();
                break;
            }
        }
        let mut hotkeys = Vec::new();
        for action in actions {
            match action {
                Action::SetHotkey { slot, macro_name } => hotkeys.push((slot, macro_name)),
                Action::StaticColor(_) | Action::RippleEffect(_) | Action::WaveEffect(_) => (), // These will be handled later in HotkeyHandler::set_profile_effects()
                _ => unimplemented!(),
            }
        }

        if let Some(_) = &this.corsair_sender {
            HotkeyHandler::set_profile_effects()
        }

        let hotkeys = hotkeys.into_iter().map(|(slot, macro_name)| {
            let mut actions = Vec::new();
            for macro_ in this.macros.iter() {
                if macro_.name == macro_name {
                    actions = macro_.actions.clone();
                    break
                }
            }
            let mut inputs = Vec::new();
            let mut after_profile = None;
            for action in actions {
                match action {
                    Action::ReleaseKey(key) => inputs.push(Input::Keyboard(key, KeyDirection::Release).into()),
                    Action::PressKey(key) => inputs.push(Input::Keyboard(key, KeyDirection::Press).into()),
                    Action::SwitchProfile(profile) => after_profile = Some(profile),
                    _ => unimplemented!()
                }
            }

            (slot, inputs, after_profile)
        }).collect();

        this.hotkeys = hotkeys;

        Self::register_hotkeys();
    }

    fn change_corsair_effects(corsair_sender: &Sender<CorsairMsg>, effects: Vec<Box<Effect>>) {
        corsair_sender.send(CorsairMsg::RemoveAllEffects).unwrap();
        for effect in effects {
            corsair_sender.send(CorsairMsg::AddEffect(effect)).unwrap();
        }
    }

    pub(crate) fn register_corsair(corsair_sender: mpsc::Sender<CorsairMsg>)  {
        let this = unsafe{&mut HOTKEY_HANDLER};
        this.corsair_sender = Some(corsair_sender);
        HotkeyHandler::set_profile_effects();
    }

    fn set_profile_effects() {
        let this = unsafe {
            &mut HOTKEY_HANDLER
        };

        let mut actions = Vec::new();
        for profile in this.profiles.iter() {
            if profile.name == this.current_profile {
                actions = profile.actions.clone();
                break;
            }
        }

        let mut light_effects = Vec::new();
        for action in actions {
            match action {
                Action::SetHotkey { .. } => (), // Not handled in this function
                Action::StaticColor(color) => light_effects.push(Box::new(Effect::Static(color.into()))),
                Action::RippleEffect(ripple) => light_effects.push(Box::new(Effect::Ripple(ripple))),
                Action::WaveEffect(wave) => light_effects.push(Box::new(Effect::Wave(wave))),
                _ => unimplemented!(),
            }
        }

        HotkeyHandler::change_corsair_effects(this.corsair_sender.as_ref().unwrap(), light_effects)
    }
}
