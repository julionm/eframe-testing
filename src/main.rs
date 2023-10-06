use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Hello World",
        options,
        Box::new(|cc| {
            Box::<MyApp>::default()
        })
    )
}

#[derive(Default)]
struct MyApp {

}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hellow World App!");
            if ui.button("Primeiro Botão").clicked() {
                println!("clicou no botão?")
            }
        });
    }
}