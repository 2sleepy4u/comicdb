#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::{egui::{self, IconData}, icon_data, epaint::Vec2};

mod types;
mod ui;
mod crud;


use types::*;
use crud::db_search;

const ICON: &[u8; 19728] = include_bytes!("./../assets/box.png");

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size(Vec2::new(1000., 500.))
            .with_icon(icon_data::from_png_bytes(ICON).unwrap()),
        ..Default::default()
    };
    eframe::run_native(
        "Magazzino",
        options,
        Box::new(|cc| {
            let comics = db_search(&Comic::default());
                
            Box::new(MyApp { comics, ..Default::default()})
        }),
    )
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Magazzino");
            self.comics_filter(ui, ctx); 
            self.comics_list(ui, self.comics.clone(), false); 
            self.comic_detail(ui, ctx);
            self.comic_online_list(ui, ctx);
        });
    }

}


