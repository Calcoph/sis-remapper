use std::{cell::RefCell, collections::HashMap, time::Duration};

use cgmath::Deg;
use parser::token_parse;
use sis_core::{ColorAnimation, RippleAnimation, VirtualKey, WaveAnimation};
use statement::{Color, FuncName, Statement, Value};
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
                    eprintln!("Redefinition of profile {name}");
                    break;
                };
            },
            Statement::Func { name, body } => {
                let func_actions = get_calls(body);
                if let Some(_) = functions.insert(name.clone(), MaybeExpanded::NotExpanded(func_actions)) {
                    eprintln!("Redefinition of function {name}");
                    break;
                }
            },
            Statement::Call { .. } => {
                eprintln!("Invalid AST");
                break;
            },
            Statement::Macro { name, body } => {
                let macro_actions = get_calls(body);
                if let Some(_) = macros.insert(name.clone(), macro_actions) {
                    eprintln!("Redefinition of macro {name}");
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
            Statement::Loop { .. } => {
                eprintln!("Invalid AST");
                break;
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

            (FuncName::Other(name), actions)
        }).collect();

    let mut expanded_macros = Vec::new();
    for (macro_name, profile_actions) in macros {
        let Calls {
            loop_,
            one_time,
        } = profile_actions;
        if loop_.len() > 1 {
            eprintln!("Cannot have loops inside macros. All statements inside the loop will be ignored.")
        }
        let actions = create_macro(macro_name, one_time, &profile_names).unwrap();
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
    pub one_time_actions: Vec<Action>,
    pub loop_actions: Vec<Action>
}

fn create_profile(profile_name: String, profile_actions: Calls, functions: &HashMap<FuncName, Actions>, animations: &HashMap<String, ColorAnimation>) -> Result<Profile, ()> {
    let mut actions = Vec::new();
    let mut loop_ = Vec::new();
    for (in_loop, ((action_name, _), args)) in profile_actions.loop_.into_iter().map(|a| (true, a)).chain(profile_actions.one_time.into_iter().map(|a| (false, a))) {
        match get_action(&action_name, &args, animations)? {
            Some(action) => {
                if in_loop {
                    loop_.push(action)
                } else {
                    actions.push(action)
                }
            },
            None => {
                if args.len() != 0 {
                    return Err(())
                }
                let function = functions.get(&action_name);
                if let Some(function) = function {
                    let Actions {
                        loop_: l,
                        one_time: ot
                    } = function;
                    actions.extend(ot.iter().map(|a| a.clone()));
                    loop_.extend(l.iter().map(|a| a.clone()));
                } else {
                    return Err(());
                }
            },
        }
    }

    Ok(Profile{ name: profile_name, one_time_actions: actions, loop_actions: loop_ })
}

pub struct Macro {
    pub name: String,
    pub actions: Vec<Action>
}

fn create_macro(macro_name: String, profile_actions: Vec<(Spanned<FuncName>, Vec<Spanned<Value>>)>, profile_names: &[&str]) -> Result<Macro, ()> {
    let mut actions = Vec::new();
    for ((action_name, _), args) in profile_actions {
        match action_name {
            FuncName::SetHotkey => todo!(),
            FuncName::PressKey => {
                let press_key_action = get_press_key_action(&args)?;
                actions.push(press_key_action);
            },
            FuncName::ReleaseKey => {
                let release_key_action = get_release_key_action(&args)?;
                actions.push(release_key_action);
            },
            FuncName::SwitchProfile => {
                let switch_profile_action = get_switch_profile_action(&args, profile_names)?;
                actions.push(switch_profile_action);
            },
            _ => return Err(())
        }
    }

    Ok(Macro{ name: macro_name, actions })
}

struct Calls {
    loop_: Vec<(Spanned<FuncName>, Vec<Spanned<Value>>)>,
    one_time: Vec<(Spanned<FuncName>, Vec<Spanned<Value>>)>
}

fn get_calls(body: Vec<Statement>) -> Calls {
    let mut func_calls = Vec::new();
    let mut loop_ = Vec::new();
    for statement in body {
        match statement {
            Statement::Call { name, args } => func_calls.push((name, args)),
            Statement::Loop { body } => {
                let Calls {
                    loop_: l,
                    one_time: funcs,
                } = get_calls(body);
                if l.len() > 1 {
                    eprintln!("Cannot have nested loops. Statements inside the nested loops will be ignored.")
                }
                loop_.extend(funcs.into_iter());
            },
            Statement::Profile { .. } => unreachable!(),
            Statement::Func { .. } => unreachable!(),
            Statement::Macro { .. } => unreachable!(),
            Statement::ColorAnimation { .. } => unreachable!(),
        }
    }

    Calls {
        loop_,
        one_time: func_calls,
    }
}

fn expand_functions(functions_ptr: *mut HashMap<String, MaybeExpanded>, call_stack: &mut Vec<String>, animations: &HashMap<String, ColorAnimation>, profile_names: &Vec<&str>) -> Result<(), ()> {
    let functions = unsafe{
        functions_ptr.as_mut().unwrap()
    };

    for (function_name, function_actions) in functions.iter_mut() {
        if let MaybeExpanded::NotExpanded(not_expanded_function_actions) = function_actions {
            *function_actions = MaybeExpanded::Expanded(expand_function(functions_ptr, call_stack, function_name.clone(), &not_expanded_function_actions, animations, profile_names).unwrap());
        } else if let MaybeExpanded::Expanded(_) = function_actions {
            // Do nothing, it is already expanded
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct Actions {
    loop_: Vec<Action>,
    one_time: Vec<Action>
}

fn expand_function(
    functions_ptr: *mut HashMap<String, MaybeExpanded>,
    call_stack: &mut Vec<String>,
    function_name: String,
    function_actions: &Calls,
    animations: &HashMap<String, ColorAnimation>,
    profile_names: &[&str]
) -> Result<Actions, ()> {
    let functions = unsafe{
        functions_ptr.as_mut().unwrap()
    };
    if call_stack.contains(&function_name) {
        return Err(())
    }
    call_stack.push(function_name.clone());

    let Calls {
        loop_: l,
        one_time: ot
    } = function_actions;

    let mut actions = Vec::new();
    let mut loop_ = Vec::new();

    for (in_loop, ((action_name, _), args)) in l.iter().map(|a| (true, a)).chain(ot.iter().map(|a| (false, a))) {
        match get_action(action_name, &args, animations) {
            Ok(Some(action)) => {
                if in_loop {
                    loop_.push(action)
                } else {
                    actions.push(action)
                }
            },
            Ok(None) => {
                let action_name = match action_name {
                    FuncName::Other(name) => name,
                    _ => panic!("Not possible?")
                };
                let function = functions.get_mut(action_name);
                if let Some(function) = function {
                    if let MaybeExpanded::NotExpanded(not_expanded_function) = function {
                        if let Ok(acts) = expand_function(functions_ptr, call_stack, action_name.clone(), not_expanded_function, animations, profile_names) {
                            *function = MaybeExpanded::Expanded(acts.clone());
                            let Actions {
                                loop_: l,
                                one_time: ot
                            } = acts;
                            actions.extend(ot.into_iter());
                            loop_.extend(l.into_iter());
                        } else {
                            call_stack.pop();
                            return Err(())
                        }
                    } else if let MaybeExpanded::Expanded(function) = function {
                        let Actions {
                            loop_: l,
                            one_time: ot,
                        } = function;
                        actions.extend(ot.iter().map(|a| a.to_owned()));
                        loop_.extend(l.iter().map(|a| a.to_owned()));
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
    Ok(Actions {
        loop_,
        one_time: actions,
    })
}

fn get_action(action_name: &FuncName, args: &[Spanned<Value>], animations: &HashMap<String, ColorAnimation>) -> Result<Option<Action>, ()> {
    Ok(match action_name {
        FuncName::SetHotkey => {
            Some(get_hotkey_action(args)?)
        },
        FuncName::PressKey => return Err(()),
        FuncName::ReleaseKey => return Err(()),
        FuncName::SwitchProfile => return Err(()),
        FuncName::WaveEffect => Some(get_wave_effect_action(args, animations)?),
        FuncName::RippleEffect => Some(get_ripple_effect_action(args, animations)?),
        FuncName::StaticColor => Some(get_static_color_action(args)?),
        FuncName::Other(_) => None,
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
            let key: VirtualKey = match variant.as_str().try_into() {
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
            let key: VirtualKey = match variant.as_str().try_into() {
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
    let slot: VirtualKey = if let (Value::EnumVariant { enum_name, variant }, _) = args.get(0).unwrap() {
        if enum_name == "Key" {
            match variant.as_str().try_into() {
                Ok(key) => key,
                Err(_) => return Err(()),
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
        slot: VirtualKey,
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
    Expanded(Actions),
    NotExpanded(Calls)
}
