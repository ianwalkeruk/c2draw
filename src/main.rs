mod app;
mod export;
mod model;
mod ui;

use app::C2DrawApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "C2Draw - C4 Diagram Editor",
        options,
        Box::new(|cc| Ok(Box::new(C2DrawApp::new(cc)))),
    )
}
