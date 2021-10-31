# egui
## Simple Application With Eframe
Eframe allows you to target WebAssembly for a web application, and native applications with the same code, very cool! The bellow is the most simple implementation of an app.
```rust
use eframe::{egui, epi};

pub struct TemplateApp {
    label: String,
    value: f32,
}

impl Default for TemplateApp {
	fn default() -> Self {
		Self {
			// The values that will be mutable from the ui
            label: "Hello World!".to_owned(),
            value: 2.4,
        }
    }
}

// Epi is what hooks Egui into Eframe
impl epi::App for TemplateApp {
	// The name of the app in the native menu bar
    fn name(&self) -> &str {
        "Hello World App"
    }

	// Runs on first load
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

	// Runs on every frame like a game engine!
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let Self { label, value } = self;

		// Make a single panel with a heading and value slider
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(label);
            ui.add(egui::Slider::new(value, 0.0..=100.0));
        });
    }
}

```

## Change style example
```rust
let style = ui.style_mut();
style.spacing.item_spacing = egui::Vec2 { x: 10.0, y: 10.0 };
```
