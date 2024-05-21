use calc::app::MyApp;
use egui::{vec2, ViewportBuilder};

fn main() {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(vec2(600.0, 100.0))
            .with_always_on_top()
            .with_transparent(true)
            .with_decorations(false),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "Flight Visualizer",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
    .expect("Cannot start window");
}
