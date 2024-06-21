use icue_bindings::types::{CorsairLedColor, CorsairLedLuid};
use sis_core::{RGBAf32, RippleAnimation, WaveAnimation};

/// Exposes the private functions as public just for the purpose of testing
use super::{corsair_connect, wait_connection, CorsairMsg, CorsairState};
use crate::corsair::effects::Effect;

pub enum PubEffect {
    Static(RGBAf32),
    Wave(WaveAnimation),
    Ripple(RippleAnimation),
    ColorChange,
}

impl From<PubEffect> for Effect {
    fn from(value: PubEffect) -> Self {
        match value {
            PubEffect::Static(e) => Effect::Static(e),
            PubEffect::Wave(e) => Effect::Wave(e),
            PubEffect::Ripple(e) => Effect::Ripple(e),
            PubEffect::ColorChange => Effect::ColorChange,
        }
    }
}

pub struct PubCorsairState(CorsairState);

impl PubCorsairState {
    pub fn new() -> PubCorsairState {
        PubCorsairState(CorsairState::new())
    }

    pub fn setup(&mut self) {
        self.0.setup()
    }

    pub fn tick(&mut self) {
        self.0.tick()
    }

    pub fn get_led_colors(&mut self) -> Vec<CorsairLedColor> {
        self.0.get_led_colors()
    }

    pub fn add_effect(&mut self, effect: PubEffect) {
        self.0.add_effect(effect.into())
    }

    pub fn add_effect_led(&mut self, led: CorsairLedLuid, effect: Effect) {
        self.0.add_effect_led(led, effect)
    }

    pub fn handle_msg(&mut self, connected: &mut bool, msg: CorsairMsg) {
        self.0.handle_msg(connected, msg)
    }

    pub fn remove_all_effects(&mut self) {
        self.0.remove_all_effects()
    }
}

pub fn corsair_singlethread_connect() -> PubCorsairState {
    let (_tx, rx) = corsair_connect();
    let mut corsair_state = CorsairState::new();
    wait_connection(&mut corsair_state, &rx, &mut false);
    corsair_state.setup();

    PubCorsairState(corsair_state)
}
