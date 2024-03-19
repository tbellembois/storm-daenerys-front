use egui::Ui;

use crate::defines::{AF_HALF_LOCK_CODE, AF_LOCK_CODE};

pub fn render_home(ui: &mut Ui) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add_sized(
            [40., 40.],
            egui::Image::new(egui::include_image!(
                "../../../../media/circle-question-regular.svg"
            )),
        );
    });

    ui.add_space(20.0);

    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
        ui.label(
            egui::RichText::new("Why are some directories disabled on the left panel?").underline(),
        );

        ui.add_space(20.0);

        ui.label("The root directory names can only contain letters (lower and upper case), digits and the characters '_' and '-'.");
        ui.label("This rule is strictly enforced. You won't be able to manage directories not respecting this naming convention.");

        ui.add_space(20.0);

        ui.label(
            egui::RichText::new("Can I set ACLs on subdirectories?")
                .underline(),
        );

        ui.add_space(20.0);

        ui.label("No, for technical reasons it is not possible.");

        ui.add_space(20.0);

        ui.label(
            egui::RichText::new("I have given permission to a person on a directory but he/she can't access it").underline(),
        );

        ui.add_space(20.0);

        ui.label(format!("Check that this person is member of one of the group with a {} or a {}.", AF_LOCK_CODE, AF_HALF_LOCK_CODE));
    });

    ui.add_space(40.0);

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add_sized(
            [40., 40.],
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
