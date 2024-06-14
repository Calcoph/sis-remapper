#[cfg(feature = "testable_privates")]
mod test {
    use std::time::Duration;
    use cgmath::Deg;

    use sis_core::{ColorAnimation, Keyframe, RippleAnimation, WaveAnimation};
    use sis_remapper::corsair::test_exposer::{PubCorsairState as CorsairState, PubEffect as Effect, corsair_singlethread_connect};

    #[test]
    fn run_test() {
        let mut corsair_state = corsair_singlethread_connect();
        let len = corsair_state.get_led_colors().len();
        panic!("{len}")
    }
}
