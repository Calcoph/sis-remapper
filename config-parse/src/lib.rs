use std::{cell::RefCell, collections::HashMap, time::Duration};

use cgmath::Deg;
use parser::token_parse;
use sis_core::{ColorAnimation, HotkeySlot, RippleAnimation, VirtualKey, WaveAnimation};
use statement::{Color, Statement, Value};
use token::{Spanned, Token};

mod lexer;
mod parser;
mod token;
mod combinators;
mod statement;

pub struct Config {
    pub profiles: Vec<Profile>,
    pub macros: Vec<Macro>,
    pub color_animations: Vec<ColorAnimation>
}

pub fn get_config() -> Config {
    let config = std::fs::read_to_string("config.txt").unwrap();
    let errors = RefCell::new(Vec::new());
    let tokens = lexer::lex(&config, &errors)
        .into_iter()
        .filter(|tok| match tok.fragment() {
            Token::Comment { content: _ } => false,
            _ => true
        }).collect::<Vec<_>>();

    let statements = token_parse(tokens).0;

    let mut functions = HashMap::new();
    let mut macros = HashMap::new();
    let mut profiles = HashMap::new();
    let mut color_animations = HashMap::new();

    for (statement, _) in statements {
        match statement {
            Statement::Profile {
                name,
                body
            } => {
                let profile_actions = get_calls(body);
                if let Some(_) = profiles.insert(name.clone(), profile_actions) {
                    eprint!("Redefinition of profile {name}");
                    break;
                };
            },
            Statement::Func { name, body } => {
                let func_actions = get_calls(body);
                if let Some(_) = functions.insert(name.clone(), MaybeExpanded::NotExpanded(func_actions)) {
                    eprint!("Redefinition of function {name}");
                    break;
                }
            },
            Statement::Call { .. } => {
                eprint!("Invalid AST");
                break;
            },
            Statement::Macro { name, body } => {
                let macro_actions = get_calls(body);
                if let Some(_) = macros.insert(name.clone(), macro_actions) {
                    eprint!("Redefinition of macro {name}");
                    break;
                }
            },
            Statement::ColorAnimation {
                name,
                body
            } => {
                let color_animation = ColorAnimation{
                    name: name.clone(),
                    keyframes: body.into_iter().map(|(keyf, _)| keyf.into()).collect()
                };
                if let Some(_) = color_animations.insert(name.clone(), color_animation) {
                    eprint!("Redefinition of macro {name}");
                    break;
                }
            },
        }
    }

    let profile_names = profiles.iter().map(|(name, _)| name.as_str()).collect();
    expand_functions(&mut functions, &mut Vec::new(), &color_animations, &profile_names).unwrap();
    let functions = functions.into_iter()
        .map(|(name, actions)| {
            let actions = match actions {
                MaybeExpanded::Expanded(actions) => actions,
                MaybeExpanded::NotExpanded(_) => panic!("WTF"),
            };

            (name, actions)
        }).collect();

    let mut expanded_macros = Vec::new();
    for (macro_name, profile_actions) in macros {
        let actions = create_macro(macro_name, profile_actions, &profile_names).unwrap();
        expanded_macros.push(actions)
    }

    let mut expanded_profiles = Vec::new();
    for (profile_name, profile_actions) in profiles {
        let profile = create_profile(profile_name, profile_actions, &functions, &color_animations).unwrap();
        expanded_profiles.push(profile)
    }

    Config {
        profiles: expanded_profiles,
        macros: expanded_macros,
        color_animations: color_animations.into_iter().map(|(_name, animation)| animation).collect()
    }
}

pub struct Profile {
    pub name: String,
    pub actions: Vec<Action>
}

fn create_profile(profile_name: String, profile_actions: Vec<(Spanned<String>, Vec<Spanned<Value>>)>, functions: &HashMap<String, Vec<Action>>, animations: &HashMap<String, ColorAnimation>) -> Result<Profile, ()> {
    let mut actions = Vec::new();
    for ((action_name, _), args) in profile_actions {
        match get_action(&action_name, &args, animations)? {
            Some(action) => actions.push(action),
            None => {
                if args.len() != 0 {
                    return Err(())
                }
                let function = functions.get(&action_name);
                if let Some(function) = function {
                    actions.extend(function.iter().map(|a| a.clone()))
                } else {
                    return Err(());
                }
            },
        }
    }

    Ok(Profile{ name: profile_name, actions })
}

pub struct Macro {
    pub name: String,
    pub actions: Vec<Action>
}

fn create_macro(macro_name: String, profile_actions: Vec<(Spanned<String>, Vec<Spanned<Value>>)>, profile_names: &[&str]) -> Result<Macro, ()> {
    let mut actions = Vec::new();
    for ((action_name, _), args) in profile_actions {
        match action_name.as_str() {
            "set_hotkey" => {
                return Err(())
            }
            "press_key" => {
                let press_key_action = get_press_key_action(&args)?;
                actions.push(press_key_action);
            },
            "release_key" => {
                let release_key_action = get_release_key_action(&args)?;
                actions.push(release_key_action);
            },
            "switch_profile" => {
                let switch_profile_action = get_switch_profile_action(&args, profile_names)?;
                actions.push(switch_profile_action);
            }
            "wave_effect" => {
                return Err(())
            },
            "ripple_effect" => {
                return Err(())
            },
            "static_color" => {
                return Err(())
            },
            _ => return Err(())
        }
    }

    Ok(Macro{ name: macro_name, actions })
}

fn get_calls(body: Vec<Statement>) -> Vec<(Spanned<String>, Vec<Spanned<Value>>)> {
    let mut func_calls = Vec::new();
    for statement in body {
        match statement {
            Statement::Call { name, args } => func_calls.push((name, args)),
            Statement::Profile { .. } => unreachable!(),
            Statement::Func { .. } => unreachable!(),
            Statement::Macro { .. } => unreachable!(),
            Statement::ColorAnimation { .. } => unreachable!(),
        }
    }

    func_calls
}

fn expand_functions(functions_ptr: *mut HashMap<String, MaybeExpanded>, call_stack: &mut Vec<String>, animations: &HashMap<String, ColorAnimation>, profile_names: &Vec<&str>) -> Result<(), ()> {
    let functions = unsafe{
        functions_ptr.as_mut().unwrap()
    };

    let reserved_function_names = vec![
        "set_hotkey",
        "press_key",
        "release_key",
        "switch_profile",
        "wave_effect",
        "ripple_effect",
        "static_color",
    ];

    for (function_name, function_actions) in functions.iter_mut() {
        if reserved_function_names.contains(&function_name.as_str()) {
            return Err(())
        } else {
            if let MaybeExpanded::NotExpanded(not_expanded_function_actions) = function_actions {
                *function_actions = MaybeExpanded::Expanded(expand_function(functions_ptr, call_stack, function_name.clone(), &not_expanded_function_actions, animations, profile_names).unwrap());
            } else if let MaybeExpanded::Expanded(_) = function_actions {
                // Do nothing, it is already expanded
            }
        }
    }

    Ok(())
}

fn expand_function(
    functions_ptr: *mut HashMap<String, MaybeExpanded>,
    call_stack: &mut Vec<String>,
    function_name: String,
    function_actions: &[(Spanned<String>, Vec<Spanned<Value>>)],
    animations: &HashMap<String, ColorAnimation>,
    profile_names: &[&str]
) -> Result<Vec<Action>, ()> {
    let functions = unsafe{
        functions_ptr.as_mut().unwrap()
    };
    if call_stack.contains(&function_name) {
        return Err(())
    }
    call_stack.push(function_name.clone());

    let mut actions = Vec::new();

    for ((action_name, _), args) in function_actions {
        match get_action(action_name, &args, animations) {
            Ok(Some(action)) => actions.push(action),
            Ok(None) => {
                let function = functions.get_mut(action_name);
                if let Some(function) = function {
                    if let MaybeExpanded::NotExpanded(not_expanded_function) = function {
                        if let Ok(acts) = expand_function(functions_ptr, call_stack, action_name.clone(), not_expanded_function, animations, profile_names) {
                            *function = MaybeExpanded::Expanded(acts.clone());
                            actions.extend(acts)
                        } else {
                            call_stack.pop();
                            return Err(())
                        }
                    } else if let MaybeExpanded::Expanded(function) = function {
                        actions.extend(function.iter().map(|a| a.to_owned()))
                    }
                } else {
                    call_stack.pop();
                    return  Err(());
                }
            },
            Err(_) => {
                call_stack.pop();
                return Err(());
            },
        }
    }

    call_stack.pop();
    Ok(actions)
}

fn get_action(action_name: &str, args: &[Spanned<Value>], animations: &HashMap<String, ColorAnimation>) -> Result<Option<Action>, ()> {
    Ok(match action_name {
        "set_hotkey" => {
            Some(get_hotkey_action(args)?)
        }
        "press_key" => {
            return Err(())
        },
        "release_key" => {
            return Err(())
        },
        "switch_profile" => {
            return Err(())
        }
        "wave_effect" => {
            Some(get_wave_effect_action(args, animations)?)
        },
        "ripple_effect" => {
            Some(get_ripple_effect_action(args, animations)?)
        },
        "static_color" => {
            Some(get_static_color_action(args)?)
        },
        _ => None
    })
}

fn get_static_color_action(args: &[Spanned<Value>]) -> Result<Action, ()> {
    if args.len() != 1 {
        return Err(())
    }

    let color = &args[0].0;
    match color {
        Value::Color(c) => Ok(Action::StaticColor(c.clone())),
        _ => Err(())
    }
}

fn get_ripple_effect_action(args: &[Spanned<Value>], animations: &HashMap<String, ColorAnimation>) -> Result<Action, ()> {
    if args.len() != 4 {
        return Err(())
    }
    let animation = &args[0].0;
    let duration = &args[1].0;
    let speed = &args[2].0;
    let light_amount = &args[3].0;

    let animation = match animation {
        Value::Variable { name } => name,
        _ => return Err(())
    };
    let animation = animations.get(animation).ok_or(())?.clone();
    let duration = match duration {
        Value::Integer(millis) => Duration::from_millis(*millis as u64),
        _ => return Err(())
    };
    let speed = match speed {
        Value::Float(speed) => speed,
        _ => return Err(())
    };
    let light_amount = match light_amount {
        Value::Float(light_amount) => light_amount,
        _ => return Err(())
    };

    Ok(Action::RippleEffect(RippleAnimation {
        animation,
        duration,
        speed: *speed as f64,
        light_amount: *light_amount as f64,
    }))
}

fn get_wave_effect_action(args: &[Spanned<Value>], animations: &HashMap<String, ColorAnimation>) -> Result<Action, ()> {
    if args.len() != 6 {
        return Err(())
    }
    let animation = &args[0].0;
    let duration = &args[1].0;
    let speed = &args[2].0;
    let light_amount = &args[3].0;
    let rotation = &args[4].0;
    let two_sides = &args[5].0;

    let animation = match animation {
        Value::Variable { name } => name,
        _ => return Err(())
    };
    let animation = animations.get(animation).ok_or(())?.clone();
    let duration = match duration {
        Value::Integer(millis) => Duration::from_millis(*millis as u64),
        _ => return Err(())
    };
    let speed = match speed {
        Value::Float(speed) => *speed as f64,
        _ => return Err(())
    };
    let light_amount = match light_amount {
        Value::Float(light_amount) => *light_amount as f64,
        _ => return Err(())
    };
    let rotation = match rotation {
        Value::Float(f) => Deg(*f).into(),
        _ => return Err(())
    };
    let two_sides = match two_sides {
        Value::Bool(bool) => *bool,
        _ => return Err(())
    };

    Ok(Action::WaveEffect(WaveAnimation {
        animation,
        duration,
        speed,
        light_amount,
        rotation,
        two_sides,
    }))
}

fn get_switch_profile_action(args: &[Spanned<Value>], profile_names: &[&str]) -> Result<Action, ()> {
    if !args.len() == 1 {
        return Err(())
    }
    let profile_name = if let (Value::Variable { name }, _) = args.get(0).unwrap() {
        if profile_names.contains(&name.as_str()) {
            name.to_owned()
        } else {
            return Err(())
        }
    } else {
        return Err(());
    };

    Ok(Action::SwitchProfile(profile_name))
}

fn get_press_key_action(args: &[Spanned<Value>]) -> Result<Action, ()> {
    if !args.len() == 1 {
        return Err(())
    }
    let key = if let (Value::EnumVariant { enum_name, variant }, _) = args.get(0).unwrap() {
        if enum_name == "Key" {
            let key: VirtualKey = match  variant.to_owned().try_into() {
                Ok(key) => key,
                Err(_) => return Err(()),
            };
            key
        } else {
            return Err(())
        }
    } else {
        return Err(());
    };

    Ok(Action::PressKey(key))
}

fn get_release_key_action(args: &[Spanned<Value>]) -> Result<Action, ()> {
    if !args.len() == 1 {
        return Err(())
    }
    let key = if let (Value::EnumVariant { enum_name, variant }, _) = args.get(0).unwrap() {
        if enum_name == "Key" {
            let key: VirtualKey = match  variant.to_owned().try_into() {
                Ok(key) => key,
                Err(_) => return Err(()),
            };
            key
        } else {
            return Err(())
        }
    } else {
        return Err(());
    };

    Ok(Action::ReleaseKey(key))
}

fn get_hotkey_action(args: &[Spanned<Value>]) -> Result<Action, ()> {
    if !args.len() == 2 {
        return Err(());
    }
    let slot = if let (Value::EnumVariant { enum_name, variant }, _) = args.get(0).unwrap() {
        if enum_name == "Key" {
            match variant.as_str() {
                "F13" => HotkeySlot::S1,
                "F14" => HotkeySlot::S2,
                "F15" => HotkeySlot::S3,
                "F16" => HotkeySlot::S4,
                "F17" => HotkeySlot::S5,
                "F18" => HotkeySlot::S6,
                "F19" => HotkeySlot::S7,
                "F20" => HotkeySlot::S8,
                "F21" => HotkeySlot::S9,
                "F22" => HotkeySlot::S10,
                "F23" => HotkeySlot::S11,
                "F24" => HotkeySlot::S12,
                _ => return Err(())
            }
        } else {
            return Err(())
        }
    } else {
        return Err(());
    };
    let macro_name = if let (Value::Variable { name }, _) = args.get(1).unwrap() {
        name.to_owned()
    } else {
        return Err(());
    };

    Ok(Action::SetHotkey{ slot, macro_name })
}

#[derive(Debug, Clone)]
pub enum Action {
    SetHotkey {
        slot: HotkeySlot,
        macro_name: String
    },
    ReleaseKey(VirtualKey),
    PressKey(VirtualKey),
    SwitchProfile(String),
    StaticColor(Color),
    RippleEffect(RippleAnimation),
    WaveEffect(WaveAnimation),
}

enum MaybeExpanded {
    Expanded(Vec<Action>),
    NotExpanded(Vec<(Spanned<String>, Vec<Spanned<Value>>)>)
}
