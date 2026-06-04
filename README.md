# 🎙️ VAD Processor (Client-Side WebAssembly)

A real-time, client-side Voice Activity Detection (VAD) module compiled to WebAssembly (WASM).

This module acts as the audio processing frontline for the Emotion-Aware Voice Assistant. By running an ONNX VAD model directly in the browser via Rust and WebAssembly, it significantly reduces latency and server overhead. It efficiently chunks valid speech _before_ routing it to the Llama 3.3 backend inference engine, ensuring the backend only processes actual human speech rather than dead air or background noise.

---

## 🚀 Architecture

- **Core Logic:** Rust (`src/lib.rs`) handles the raw audio buffer processing, silence thresholding (padding), and the ONNX model execution.
- **Web Integration:** `wasm-bindgen` bridges the highly-performant Rust backend with the browser's native `MediaDevices` API for microphone capture.
- **Test Interface:** A lightweight `index.html` serves as the testbed for microphone capture and real-time VAD visualization.

---

## 🗂️ Project Structure

```text
vad_processor/
├── src/
│   └── lib.rs             # Rust source code for audio processing & VAD logic
├── pkg/                   # Generated WASM binaries and JavaScript bindings (Ignored in Git)
├── index.html             # Browser test interface for microphone capture
├── Cargo.toml             # Rust dependencies and package configuration
├── Cargo.lock             # Dependency tree lockfile
└── README.md              # Project documentation
```

---

## 🛠️ Prerequisites & Build Instructions

To compile this project, you will need the Rust toolchain and `wasm-pack` installed on your system.

### 1. Install Dependencies

If you don't have `wasm-pack` installed globally:

```bash
cargo install wasm-pack
```

### 2. Build the WebAssembly Package

Compile the Rust code into a web-compatible WASM module:

```bash
wasm-pack build --target web
```

_Note: This command generates the `.wasm` binary and `.js` wrappers in the `pkg/` directory. This directory is excluded from version control to keep the repository lean._

---

## 💻 Local Testing

Because modern browsers strictly restrict microphone access to secure contexts (HTTPS or `localhost`), you must run a local web server to test the `index.html` interface. Opening the file directly via `file://` will result in blocked audio capture.

### 1. Serve the Directory

Use Python's built-in HTTP server from the root of the `vad_processor` directory:

```bash
python -m http.server 8000
```

### 2. Access the Interface

Open your browser and navigate to:

```text
http://localhost:8000
```

Grant microphone permissions when prompted to begin testing the real-time voice detection.

---

## ⚙️ Integration Roadmap

- [ ] **Threshold Tuning:** Fine-tune the silence thresholding and padding logic to prevent cutting off trailing phonemes during natural pauses in speech.
- [ ] **Backend Handoff:** Finalize the WebSocket/WebRTC implementation to securely and rapidly stream validated speech chunks to the Llama 3.3 backend.
- [ ] **Emotion Layer Prep:** Expose audio feature extraction hooks in the WASM layer to feed acoustic data (pitch, tone, cadence) in parallel to the transcription engine.
