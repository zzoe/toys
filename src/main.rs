mod app;

fn main() {
    eframe::run_native(
        "Toys",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(app::App::new(cc))),
    );
}
