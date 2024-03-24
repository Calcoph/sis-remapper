use std::{ffi::CString, os::raw::c_void, sync::mpsc::{self, Receiver, Sender}, time::{Duration, Instant}};

use icue_bindings::{sys::{CorsairLedColor, CorsairLedPosition}, CorsairConnect, CorsairDeviceType, CorsairGetDevices, CorsairGetLedPositions, CorsairSessionState, CorsairSetLedColors};

use self::effects::{clorled_to_floatled, floatled_to_colorled, ripple_effect, ripple_key, static_effect, static_key, wave_effect, wave_key, Effect, LedInfof32};

static mut STATE: CorsairSessionState = CorsairSessionState::Invalid;

pub(crate) mod effects;

pub(crate) enum CorsairMsg {
    Connected,
    NotConnected,
    AddEffect(Box<Effect>),
    AddEffectLed(LedId, Box<Effect>),
    RemoveAllEffects
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
enum LedId {
    Esc = 1,
    F1 = 2,
    F2 = 3,
    F3 = 4,
    F4 = 5,
    F5 = 6,
    F6 = 7,
    F7 = 8,
    F8 = 9,
    F9 = 10,
    F10 = 11,
    F11 = 12,
    F12 = 13,
    BloqDespl = 85,
    CorsairLogo = 196609,
}

fn corsair_handler(
    state: CorsairSessionState,
    details: icue_bindings::sys::CorsairSessionDetails,
    tx: &Sender<CorsairMsg>
) {
    unsafe {
        STATE = state
    }
    if state == CorsairSessionState::Connected {
        tx.send(CorsairMsg::Connected).unwrap();
    } else {
        tx.send(CorsairMsg::NotConnected).unwrap();
    }
}

pub(crate) fn init_corsair() -> Sender<CorsairMsg> {
    unsafe {
        let context: *mut c_void = ((&mut [0;2]) as *mut i32) as *mut c_void; // Allocate sizeof::<i32>() * 2 = 4*2 = 8 bytes
        let (tx, rx) = mpsc::channel();
        let corsair_tx = tx.clone();
        let _ = CorsairConnect(
            Some(Box::new(move |state, details| {
                corsair_handler(state, details, &tx)
            })),
            context
        );
        std::thread::spawn(||listener(rx));

        corsair_tx
    }
}

fn listener(rx: Receiver<CorsairMsg>) {
    let mut connected = false;
    let mut corsair_state = CorsairState::new();
    loop {
        if !connected {
            while let Ok(msg) = rx.recv() {
                corsair_state.handle_msg(&mut connected, msg);
                if connected {
                    break;
                }
            }
            connected = true;
            corsair_state.setup()
        } else {
            match rx.try_recv() {
                Ok(msg) => corsair_state.handle_msg(&mut connected, msg),
                Err(err) => match err {
                    mpsc::TryRecvError::Empty => corsair_state.tick(),
                    mpsc::TryRecvError::Disconnected => panic!("Channel closed"),
                },
            }
        }
    }
}

struct CorsairState {
    start_time: Instant,
    keyboard_id: CString,
    leds: Vec<CorsairLedPosition>,
    effects: Vec<Effect>,
    key_effects: Vec<(LedId, Effect)>,
}

impl CorsairState {
    fn new() -> CorsairState {
        CorsairState {
            start_time: Instant::now(),
            keyboard_id: CString::new("").unwrap(),
            leds: Vec::new(),
            effects: Vec::new(),
            key_effects: Vec::new(),
        }
    }

    fn setup(&mut self) {
        self.start_time = Instant::now();
        unsafe {
            let devices = CorsairGetDevices().unwrap();
            for device in devices {
                println!("Device found:");
                dbg!(&device);
                if device.type_ == CorsairDeviceType::Keyboard {
                    self.keyboard_id = device.id
                }
            }

            // TODO: Do Option<String> instead of String for this reason
            if !self.keyboard_id.is_empty() {
                self.leds = CorsairGetLedPositions(self.keyboard_id.clone().into_raw()).unwrap();
            }
        }
    }

    fn tick(&mut self) {
        // TODO: Improve performance. Too many clones
        let dt = self.start_time.elapsed().as_millis() as u64;
        //let nanos = self.start_time.elapsed().as_nanos() as u64;
        let leds = self.leds.iter()
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|led| {
                ((led.cx, led.cy), CorsairLedColor {
                    id: led.id,
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                })
        });
        let mut leds = clorled_to_floatled(Box::new(leds));
        for effect in self.effects.iter() {
            match effect {
                Effect::Static(color) => leds = static_effect(leds, color.clone()),
                Effect::Wave(wave) => leds = wave_effect(leds, dt, &wave),
                Effect::Ripple(ripple) => leds = ripple_effect(leds, dt, &ripple),
                Effect::ColorChange => (),
            }
        }

        for (key, effect) in self.key_effects.iter() {
            let effect: Box<dyn Fn(LedInfof32) -> LedInfof32> = match effect {
                Effect::Static(color) => Box::new(move |key| static_key(key, color.clone())),
                Effect::Wave(wave) => Box::new(move |key| wave_key(key, dt, &wave)),
                Effect::Ripple(ripple) => Box::new(move |key| ripple_key(key, dt, &ripple)),
                Effect::ColorChange => Box::new(move |key| key),
            };

            leds = Box::new(leds.map(move |led| {
                if led.1.id == *key as u32 {
                    effect(led)
                } else {
                    led
                }
            }))
        }

        let leds: Vec<_> = floatled_to_colorled(leds).map(|(_, led)| led).collect();
        unsafe {
            CorsairSetLedColors(self.keyboard_id.clone().into_raw(), leds).unwrap();
        }
        // TODO: Allow change the thread::sleep time from config file
        std::thread::sleep(Duration::from_millis(100)) // Refresh color once per second (+ the time it takes to update)
    }

    fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect)
    }

    fn add_effect_led(&mut self, led: LedId, effect: Effect) {
        self.key_effects.push((led, effect))
    }

    fn handle_msg(&mut self, connected: &mut bool, msg: CorsairMsg) {
        match msg {
            CorsairMsg::Connected => *connected = true,
            CorsairMsg::NotConnected => *connected = false,
            CorsairMsg::AddEffect(effect) => self.add_effect(*effect),
            CorsairMsg::AddEffectLed(led, effect) => self.add_effect_led(led, *effect),
            CorsairMsg::RemoveAllEffects => self.remove_all_effects(),
        }
    }

    fn remove_all_effects(&mut self) {
        self.effects = Vec::new();
        self.key_effects = Vec::new();
    }
}
