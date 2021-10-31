# Eframe
Eframe allows you to target WebAssembly for a web application, and native applications with the same code.

## Simple Application
```rust
use eframe::{egui, epi};

pub struct RustnoteApp {
    label: String,
    value: f32,
}

impl Default for RustnoteApp {
    fn default() -> Self {
        Self {
            // This state can be mutated
            label: "Hello World!".to_owned(),
            value: 2.4,
        }
    }
}

impl epi::App for RustnoteApp {
    fn name(&self) -> &str {
        // Native menu bar title
        "eframe template"
    }

    // This updates every on every frame
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let Self { label, value } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            // Edit the state on the struct
            ui.text_edit_singleline(label);
            ui.add(egui::Slider::new(value, 0.0..=100.0));
            // ui_counter(ui, counter);
        });
    }
}

// Only compiles when targeting native
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = rustnote::RustnoteApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
```

Where lib.rs has this for the web assembly compilation:
```rust
mod app;
pub use app::RustnoteApp;

// Only compiles when targeting web assembly, matches target arch
// e.g. the below with run on `cargo build --target wasm32-unknown-unknown`
#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};
/// This is the entry-point for all the web-assembly.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let app = RustnoteApp::default();
    eframe::start_web(canvas_id, Box::new(app))
}
```

## Persist State
