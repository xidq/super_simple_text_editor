use std::fs::File;
use egui::{global_theme_preference_buttons, global_theme_preference_switch, CentralPanel, FontId, Sense, TextStyle, ThemePreference};
use rfd::FileDialog;
use std::io::{self, Read, Write};
use eframe::epaint::FontFamily::Proportional;
use egui::FontFamily::Monospace;
use egui::introspection::font_id_ui;
use egui::TextStyle::{Body, Button, Heading, Small};
use egui::UiKind::Window;


pub struct Notatnik{
    pub menu_file: &'static str,
    pub treść: String,
    pub window:bool,
    pub na_pewno:u8,
}


impl Notatnik {

    pub fn name() -> &'static str {
        concat!("Notatnik v", env!("CARGO_PKG_VERSION"))
    }

}

impl Default for Notatnik {
    fn default() -> Self {
        Notatnik {
            menu_file: "Menu",
            treść:"".to_string(),
            window: false,
            na_pewno: 0,
        }
    }
}


impl eframe::App for Notatnik{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if ctx.input(|i|i.key_pressed(egui::Key::S) && i.modifiers.ctrl ){
            zapisz_do_pliku(&self.treść);
        }
        if ctx.input(|i|i.key_pressed(egui::Key::O) && i.modifiers.ctrl ){
            if self.treść.is_empty(){
                self.treść = otwórz_plik();
            }else{
                self.window = true;
            }
        }
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (TextStyle::Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
            .into();
        ctx.set_style(style);

        ctx.request_repaint();

        CentralPanel::default()
            .show(ctx, |ui| {

                // Adding things in columns
                ui.columns(2,|column|{
                    // adding theme switch buttons
                    column[0].vertical_centered_justified(|ui|{
                        ui.horizontal(|ui|{
                            ui.label("kolor ui:");
                            global_theme_preference_buttons(ui);
                        });
                    });

                    // some hints etc
                    column[1].vertical_centered_justified(|ui| {
                        ui.label("Ctrl + S -- save   ||    Ctrl + O -- open");
                    });
                });

                ui.separator();

                // Adding scroll area for text field
                egui::scroll_area::ScrollArea::vertical().show(ui, |ui| {

                    // add_sized works good for filling avaliable space tho
                    // managing all data in memory b4 save or something
                    ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.treść).frame(false));
                    
                });

            });

        // confirmation window
        if self.window {
            egui::Window::new("Otwarcie Pliku")/*.open(&mut self.window)*/
                .resizable(false) //don't wanna resizable window
                .show(ctx, |ui| {
                    //monit text
                ui.label("W aktualnym pliku znajduje się tekst,\nczy na pewno chcesz kontynuować?");

                    // yes/no choice
                ui.horizontal(|ui| {
                    // is like that clear enough or better to use let statement and
                    // .clicked() to that let?
                    if ui.add(egui::Button::new("tak").sense(Sense::click())).clicked() {
                        println!("lol");
                        self.treść = otwórz_plik();
                        self.window = false;
                    }
                    if ui.add(egui::Button::new("nie").sense(Sense::click())).clicked() {
                        self.window = false;
                    }
                });
            });
        }
    }
}

// saving to file
fn zapisz_do_pliku(dane:&String){

        if let Some(ścieżka) = FileDialog::new().add_filter("Pliki tekstowe",&["txt","lua","cfg", "json"]).set_file_name("PlikTekstowy.txt").save_file() {
            let mut plik = File::create(ścieżka).unwrap();


            plik.write_all(dane.as_bytes()).unwrap();
        };

}

// opening existing file
fn otwórz_plik()->String{

    let mut buffer = String::new();

        if let Some(ścieżka) = FileDialog::new().add_filter("Pliki tekstowe",&["txt", "cfg", "lua","json"]).pick_file() {
        let mut plik = File::open(ścieżka).unwrap();

        plik.read_to_string(&mut buffer).unwrap();

        buffer

        } else {
            String::new()
        }

}
