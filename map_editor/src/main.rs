use eframe::egui;

fn main() -> Result<(), ()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "AFG Map Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(AFGMapEditor::new(cc)))),
    )
    .map_err(|e| println!("Error running the project: {e}"))
}

#[derive(Default)]
struct AFGMapEditor {}

impl AFGMapEditor {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for AFGMapEditor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New...").clicked() {
                        println!("Open new project dialog");
                    }

                    if ui.button("Open").clicked() {
                        println!("Open a project");
                    }

                    if ui.button("Save").clicked() {
                        println!("Save project");
                    }

                    if ui.button("Save As").clicked() {
                        println!("Open dialog to save as");
                    }

                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    ui.add_enabled(false, egui::Button::new("Soon"))
                })
            });

            ui.heading("Hello World!");
        });
    }
}
