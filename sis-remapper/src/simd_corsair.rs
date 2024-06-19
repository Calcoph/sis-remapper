use std::{os::raw::c_void, sync::mpsc::{self, Receiver, Sender}, time::{Duration, Instant}};

use icue_bindings::{types::{CorsairDeviceId, CorsairDeviceType, CorsairLedColor, CorsairLedLuid, CorsairSessionState}, CorsairConnect, CorsairGetDevices, CorsairGetLedPositions, CorsairSetLedColors};
use simd_effects::SimdLeds;

use crate::corsair::{corsair_connect, effects::{CorsairLedColorf32, Effect}, CorsairMsg};

use self::simd_effects::{ripple_effect, static_effect, wave_effect, LedInfof32};

static mut STATE: CorsairSessionState = CorsairSessionState::Invalid;

pub(crate) mod simd_effects;

#[cfg(feature = "testable_privates")]
pub mod test_exposer;

pub(crate) fn init_corsair() -> Sender<CorsairMsg> {
    let (tx, rx) = corsair_connect();
    std::thread::spawn(||listener(rx));

    tx
}

fn wait_connection(corsair_state: &mut CorsairState, rx: &Receiver<CorsairMsg>, connected: &mut bool) {
    while let Ok(msg) = rx.recv() {
        corsair_state.handle_msg(connected, msg);
        if *connected {
            break;
        }
    }
}

fn listener(rx: Receiver<CorsairMsg>) {
    let mut connected = false;
    let mut corsair_state = CorsairState::new();
    loop {
        if !connected {
            wait_connection(&mut corsair_state, &rx, &mut connected);
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
    keyboard_id: Option<CorsairDeviceId>,
    leds: SimdLeds,
    effects: Vec<Effect>,
    key_effects: Vec<(CorsairLedLuid, Effect)>,
}

impl CorsairState {
    fn new() -> CorsairState {
        CorsairState {
            start_time: Instant::now(),
            keyboard_id: None,
            leds: SimdLeds::new(),
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
                    self.keyboard_id = Some(device.id)
                }
            }

            // TODO: Do Option<String> instead of String for this reason
            if let Some(id) = &self.keyboard_id {
                self.leds.load(CorsairGetLedPositions(id).unwrap());
            }
        }
    }

    fn get_led_colors(&mut self) -> Vec<CorsairLedColor> {
        // TODO: Improve performance. Too many clones
        let dt = self.start_time.elapsed().as_millis() as u64;
        //let nanos = self.start_time.elapsed().as_nanos() as u64;
        {
            let leds = self.leds.get_leds();

            for effect in self.effects.iter() {
                match effect {
                    Effect::Static(color) => static_effect(leds, color),
                    Effect::Wave(wave) => wave_effect(leds, dt, &wave),
                    Effect::Ripple(ripple) => ripple_effect(leds, dt, &ripple),
                    Effect::ColorChange => (),
                }
            }
        }

        /* for (key, effect) in self.key_effects.iter() { // TODO: rewrite this part so it works. Cannot be easily done when mutating leds
            let effect: Box<dyn Fn(LedInfof32) -> LedInfof32> = match effect {
                Effect::Static(color) => Box::new(move |key| static_key(key, color.clone())),
                Effect::Wave(wave) => Box::new(move |key| wave_key(key, dt, wave)),
                Effect::Ripple(ripple) => Box::new(move |key| ripple_key(key, dt, ripple)),
                Effect::ColorChange => Box::new(move |key| key),
            };

            leds = Box::new(leds.map(move |led| {
                if led.1.id == *key {
                    effect(led)
                } else {
                    led
                }
            }))
        } */

        self.leds.get_led_colors()
    }

    fn tick(&mut self) {
        if self.keyboard_id.is_some() {
            let leds = self.get_led_colors();
            unsafe {
                CorsairSetLedColors(self.keyboard_id.as_ref().unwrap(), leds).unwrap();
            }
            // TODO: Allow change the thread::sleep time from config file
            std::thread::sleep(Duration::from_millis(100)) // Refresh color once per second (+ the time it takes to update)
        }
    }

    fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect)
    }

    fn add_effect_led(&mut self, led: CorsairLedLuid, effect: Effect) {
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
