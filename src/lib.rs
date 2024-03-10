use wasm_bindgen::prelude::*;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    console_error_panic_hook::set_once();

    Ok(())
}

use web_audio_api::context::{
    AudioContext, AudioContextLatencyCategory, AudioContextOptions, BaseAudioContext,
};
use web_audio_api::node::{AudioNode, AudioScheduledSourceNode};

#[wasm_bindgen]
pub struct Handle(AudioContext);

impl Drop for Handle {
    fn drop(&mut self) {
        // this closes the context but does not return somehow
        self.0.close_sync();
    }
}

#[wasm_bindgen]
pub fn beep() -> Handle {
    let options = AudioContextOptions {
        latency_hint: AudioContextLatencyCategory::Playback,
        ..AudioContextOptions::default()
    };
    let ctx = AudioContext::new(options);
    let mut osc = ctx.create_oscillator();
    osc.frequency().set_value(200.);
    osc.connect(&ctx.destination());
    osc.start();

    Handle(ctx)
}
