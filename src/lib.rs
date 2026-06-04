use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct VadEngine {
    // We keep track of energy levels to smooth out the signal
    energy_history: Vec<f32>,
    threshold: f32,
}

#[wasm_bindgen]
impl VadEngine {
    // Constructor: Called from JS to start the engine
    #[wasm_bindgen(constructor)]
    pub fn new(threshold: f32) -> VadEngine {
        VadEngine {
            energy_history: Vec::new(),
            threshold,
        }
    }

    // Process a chunk of real audio from the browser
    pub fn process(&self, audio_chunk: &[f32]) -> bool {
        // 1. Calculate RMS (Volume) for this chunk
        let mut sum = 0.0;
        for &sample in audio_chunk {
            sum += sample * sample;
        }
        let rms = (sum / audio_chunk.len() as f32).sqrt();

        // 2. Simple smoothing (optional, but good for stability)
        // For now, we just return true if strictly above threshold
        rms > self.threshold
    }
}
