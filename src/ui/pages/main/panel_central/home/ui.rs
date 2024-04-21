use egui::Ui;

pub fn render_home(ui: &mut Ui) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add_sized(
            [160., 160.],
            egui::Image::new(egui::include_image!("../../../../media/rust.svg")),
        );
    });

    ui.add_space(20.0);

    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
        ui.hyperlink("https://www.rust-lang.org/");
        ui.hyperlink("https://github.com/emilk/egui");
        ui.hyperlink("https://github.com/tokio-rs/axum");

        ui.add_space(20.0);

        ui.label(egui::RichText::new("Copyright").underline());
        ui.label("Université Clermont Auvergne");
    });
}
