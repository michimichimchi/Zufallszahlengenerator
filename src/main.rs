#![windows_subsystem = "windows"]
use eframe::egui;
mod rng;

const START: i16 = 1;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Zufallszahlengenerator",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}

struct MyApp {
    end: i16,
    random_number: Option<i16>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            end: 2,
            random_number: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Zahlenbereich auswählen: ");

            ui.separator();

            ui.scope(|ui| {
                ui.style_mut().override_font_id = Some(egui::FontId::proportional(20.0));
                ui.add_sized(
                    [ui.available_width(), 36.0],
                    egui::Slider::new(&mut self.end, 2..=4000).text("Endzahl"),
                );
            });

            let end = self.end.max(START);
            if ui.add(
                egui::Button::new(egui::RichText::new("Generieren").size(22.0))
                    .min_size(egui::vec2(160.0, 44.0)),
            ).clicked() {
                self.random_number = Some(rng::random_between(START, end));
            }

            if let Some(random_number) = self.random_number {
                ui.label(
                    egui::RichText::new(format!(
                        "Die Zufallszahl ist: {}",
                        random_number
                    ))
                    .size(24.0),
                );
            }

            ui.separator();

            ui.add(
                egui::Image::new(egui::include_image!("../29300_de_387px.jpg"))
                    .max_size(egui::vec2(220.0, 220.0))
            );

            ui.separator();

            ui.label(
                egui::RichText::new("\n Zufallszahlengenerator für Gewinnspiele der Sto group \n
                Geschrieben von Michael Zerner (DH Student Wirtschaftsinformatik) \n
                Ausbilder: Frank Hotz \n
                Nutzung durch Abteilung TF (Business Field Facade & Interior, Francisco Javier Ramos Fernandez) \n
                Quellcode öffentlich einsehbar unter https://github.com/michimichimchi/Zufallszahlengenerator").size(10.0)
            );
        });
    }
}