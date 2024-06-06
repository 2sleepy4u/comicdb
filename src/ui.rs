use eframe::egui;
use egui_extras::{TableBuilder, Column};

use crate::types::*;
use crate::crud::{db_search, google_search, insert_comic, update_comic, carica_comic, scarica_comic};



impl MyApp {
    pub fn comics_filter(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            if ui.button("Nuovo").clicked() {
                self.detail_opened = Some(DetailComic { detail_type: DetailType::New, ..Default::default() });
            }

            if !self.zoom && ui.button("Attiva zoom").clicked() {
                ctx.set_pixels_per_point(1.5);
                self.zoom = true;
            } else if self.zoom && ui.button("Disattiva zoom").clicked() {
                ctx.set_pixels_per_point(1.);
                self.zoom = false;
            }

        });
        ui.vertical(|ui| {
            let isbn_input = 
                ui.horizontal(|ui| {
                    let isbn_label = ui.label("ISBN: ");
                    let isbn_input = ui.text_edit_singleline(&mut self.search.isbn)
                        .labelled_by(isbn_label.id);
                    isbn_input
                }).inner;

            let title_input = 
                ui.horizontal(|ui| {
                    let title_label = ui.label("Titolo: ");
                    let title_input = ui.text_edit_singleline(&mut self.search.title)
                        .labelled_by(title_label.id);

                    title_input
                }).inner;

            let author_input = 
                ui.horizontal(|ui| {
                    let author_label = ui.label("Autore: ");
                    let author_input = ui.text_edit_singleline(&mut self.search.author)
                        .labelled_by(author_label.id);

                    author_input
                }).inner;


            let genre_input = 
                ui.horizontal(|ui| {
                    let genre_label = ui.label("Genere: ");
                    let genre_input = ui.text_edit_singleline(&mut self.search.genre)
                        .labelled_by(genre_label.id);

                    genre_input
                }).inner;

            ui.horizontal(|ui| {

                ui.checkbox(&mut self.online_search, "Online");

                if (
                    isbn_input.lost_focus() ||
                    title_input.lost_focus() ||
                    author_input.lost_focus() ||
                    genre_input.lost_focus() 
                   ) && ui.input(|x| x.key_pressed(egui::Key::Enter)) || ui.button("Cerca").clicked()
                {
                    if self.online_search {
                        let internal_result = db_search(&self.search);
                        if internal_result.iter().find(|x| x.isbn == self.search.isbn).is_none() {
                            self.online_search_results = google_search(&self.search);
                        } else {
                            self.toasts.warning("Il codice ISBN cercato è già presente in magazzino");
                        }
                    } else {
                        let comic_result = db_search(&self.search);
                        self.comics = comic_result;
                    }
                }

                if ui.button("Pulisci").clicked() {
                    self.search = Comic::default();
                }
            });
        });
    }
    pub fn comic_online_list(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(mut comic) = self.online_search_results.clone() {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("search_result"),
                egui::ViewportBuilder::default()
                .with_title("Risultati ricerca")
                .with_inner_size([500.0, 200.0]),
                |ctx, class| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        egui_extras::install_image_loaders(ctx);
                        let list = self.online_search_results.clone().unwrap();
                        self.comics_list(ui, list, true);
                    });
                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.online_search_results = None;
                    }
                });
        }
    }
    pub fn comics_list(&mut self, ui: &mut egui::Ui, list: Vec<Comic>, is_online: bool) {
        TableBuilder::new(ui)
            .column(Column::remainder().at_least(20.0))
            .column(Column::remainder().at_least(100.0))
            .column(Column::remainder().at_least(500.0))
            .column(Column::remainder().at_least(70.0))
            .column(Column::remainder().at_least(80.0))
            .column(Column::remainder().at_least(70.0))
            .striped(true)
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("");
                });
                header.col(|ui| {
                    ui.heading("ISBN");
                });
                header.col(|ui| {
                    ui.heading("Titolo");
                });

                header.col(|ui| {
                    ui.heading("Genere");
                });

                header.col(|ui| {
                    ui.heading("Quantità");
                });

                header.col(|ui| {
                    ui.heading("Prezzo");
                });
            })
        .body(|mut body| {
            let row_height = 18.0;
            let num_rows = list.len();
            body.rows(row_height, num_rows, |mut row| {
                let row_index = row.index();
                let comic = list.get(row_index).unwrap();

                row.col(|ui| {
                    let response = ui.add(egui::Button::new("+").sense(egui::Sense::click()));
                    response.context_menu(|ui| {
                        if ui.button("Dettaglio").clicked() {
                            self.detail_opened = Some(DetailComic { 
                                comic: comic.clone(), 
                                str_price: comic.price.to_string(),
                                str_quantity: comic.quantity.to_string(),
                                detail_type: if is_online { DetailType::New } else { DetailType::Detail },
                                ..Default::default() 
                            });
                            ui.close_menu();
                        }

                        if !is_online && ui.button("Modifica").clicked() {
                            self.detail_opened = Some(DetailComic { 
                                comic: comic.clone(), 
                                str_price: comic.price.to_string(),
                                str_quantity: comic.quantity.to_string(),
                                detail_type: DetailType::Modify,
                                ..Default::default() 
                            });
                            ui.close_menu();
                        }

                        if !is_online && ui.button("Carico").clicked() {
                            self.detail_opened = Some(DetailComic { 
                                comic: comic.clone(), 
                                str_price: comic.price.to_string(),
                                str_quantity: comic.quantity.to_string(),
                                detail_type: DetailType::Carico,
                                ..Default::default() 
                            });
                            ui.close_menu();

                        }

                        if !is_online && ui.button("Scarico").clicked() {
                            self.detail_opened = Some(DetailComic { 
                                comic: comic.clone(),
                                str_price: comic.price.to_string(),
                                str_quantity: comic.quantity.to_string(),
                                detail_type: DetailType::Scarico,
                                ..Default::default() 
                            });
                            ui.close_menu();

                        }

                    });
                });
                row.col(|ui| {
                    ui.label(format!("{}", comic.isbn));
                });
                row.col(|ui| {
                    ui.label(format!("{}", comic.title));
                });
                row.col(|ui| {
                    ui.label(format!("{}", comic.genre));
                });
                row.col(|ui| {
                    ui.label(format!("{}", comic.quantity));
                });

                row.col(|ui| {
                    ui.label(format!("{}", comic.price));
                });
            });
        });
    }

    pub fn comic_detail(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(mut detail_comic) = self.detail_opened.clone() {
            let title = 
                match detail_comic.detail_type {
                    DetailType::New => "Nuovo articolo",
                    DetailType::Detail => "Dettaglio articolo",
                    DetailType::Modify => "Modifica articolo",
                    DetailType::Carico => "Carico magazzino",
                    DetailType::Scarico => "Scarico magazzino"
                };
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of(title),
                egui::ViewportBuilder::default()
                .with_title(title)
                .with_inner_size([530.0, 350.0]),
                |ctx, class| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        egui_extras::install_image_loaders(ctx);
                        let can_modify = 
                            match detail_comic.detail_type {
                                DetailType::Modify => true,
                                DetailType::New => true,
                                _ => false
                            };

                        let can_move = 
                            match detail_comic.detail_type {
                                DetailType::Carico => true,
                                DetailType::Scarico => true,
                                _ => false
                            };
                        let is_new = 
                            if let DetailType::New = detail_comic.detail_type { 
                                true 
                            } else {
                                false
                            };
                        ui.horizontal(|ui| {
                            ui.group(|ui|{
                                ui.vertical(|ui| {
                                    ui.label("Immagine:");
                                    ui.group(|ui| {
                                        ui.set_height(200.);
                                        ui.add(
                                            egui::Image::new(&detail_comic.comic.image).max_width(150.)
                                            );
                                    })
                                });

                                ui.vertical(|ui| {
                                    ui.group(|ui| {
                                        ui.label("Dati articolo:");
                                        ui.horizontal(|ui| {
                                            ui.set_enabled(is_new);
                                            let title_label = ui.label("ISBN: ");
                                            ui.text_edit_singleline(&mut detail_comic.comic.isbn)
                                                .labelled_by(title_label.id);
                                        });
                                        ui.horizontal(|ui| {
                                            ui.set_enabled(can_modify);
                                            let title_label = ui.label("Titolo: ");
                                            ui.text_edit_singleline(&mut detail_comic.comic.title)
                                                .labelled_by(title_label.id);
                                        });
                                        ui.horizontal(|ui| {
                                            ui.set_enabled(can_modify);
                                            let title_label = ui.label("Autore: ");
                                            ui.text_edit_singleline(&mut detail_comic.comic.author)
                                                .labelled_by(title_label.id);
                                        });
                                        ui.horizontal(|ui| {
                                            ui.set_enabled(can_modify);
                                            let genre_label = ui.label("Genere: ");
                                            ui.text_edit_singleline(&mut detail_comic.comic.genre)
                                                .labelled_by(genre_label.id);
                                        });
                                        ui.horizontal(|ui| {
                                            ui.set_enabled(can_modify);
                                            let label = ui.label("Prezzo: ");
                                            ui.text_edit_singleline(&mut detail_comic.str_price)
                                                .labelled_by(label.id);
                                        });
                                        ui.horizontal(|ui| {
                                            ui.set_enabled(is_new);
                                            let label = ui.label("Quantità: ");
                                            ui.text_edit_singleline(&mut detail_comic.str_quantity)
                                                .labelled_by(label.id);
                                        });
                                    });


                                    if is_new {
                                        /*
                                           let img_bytes = reqwest::blocking::get(&detail_comic.comic.image)
                                           .unwrap()
                                           .bytes()
                                           .unwrap();

                                           let image = image::load_from_memory(&img_bytes).unwrap();
                                           */

                                        if ui.button("Aggiungi").clicked() {
                                            if let Ok(val) = detail_comic.str_price.parse() {
                                                detail_comic.comic.price = val;
                                                insert_comic(&detail_comic.comic);
                                                self.detail_opened = None;
                                                self.comics = db_search(&Comic::default());
                                            }
                                        } else {
                                            self.detail_opened = Some(detail_comic);
                                        }
                                    } else {
                                        if let DetailType::Carico = detail_comic.detail_type {
                                            ui.group(|ui| {
                                                ui.label("Carico articolo:");
                                                let label = ui.label("Quantità carico");
                                                ui.text_edit_singleline(&mut detail_comic.str_sc_quantity)
                                                    .labelled_by(label.id);


                                                let note_label = ui.label("Note aggiuntive");
                                                ui.text_edit_multiline(&mut detail_comic.note)
                                                    .labelled_by(note_label.id);

                                                if ui.button("Carica").clicked() {
                                                    if let Ok(val) = detail_comic.str_sc_quantity.parse() {
                                                        detail_comic.comic.quantity = val;
                                                        carica_comic(&detail_comic.comic, detail_comic.comic.quantity, None);
                                                        self.detail_opened = None;
                                                        self.comics = db_search(&Comic::default());
                                                    } else {
                                                        self.toasts.error("Quantità inserita non valida");
                                                    }
                                                } else {
                                                    self.detail_opened = Some(detail_comic);
                                                }

                                            });

                                        } else if let DetailType::Scarico = detail_comic.detail_type {
                                            ui.group(|ui| {
                                                ui.label("Scarico articolo:");
                                                let label = ui.label("Quantità scarico");
                                                ui.text_edit_singleline(&mut detail_comic.str_sc_quantity)
                                                    .labelled_by(label.id);

                                                let note_label = ui.label("Note aggiuntive");
                                                ui.text_edit_multiline(&mut detail_comic.note)
                                                    .labelled_by(note_label.id);


                                                if ui.button("Sarica").clicked() {
                                                    if let Ok(val) = detail_comic.str_sc_quantity.parse() {
                                                        detail_comic.comic.quantity = val;
                                                        scarica_comic(&detail_comic.comic, detail_comic.comic.quantity, None);
                                                        self.detail_opened = None;
                                                        self.comics = db_search(&Comic::default());
                                                    } else {
                                                        self.toasts.error("Quantità inserita non valida");
                                                    }
                                                } else {
                                                    self.detail_opened = Some(detail_comic);
                                                }

                                            });

                                        } else if let DetailType::Modify = detail_comic.detail_type {
                                            if ui.button("Salva").clicked() {
                                                if let Ok(val) = detail_comic.str_price.parse() {
                                                    detail_comic.comic.price = val;
                                                    update_comic(&detail_comic.comic);
                                                    self.detail_opened = None;
                                                    self.comics = db_search(&Comic::default());
                                                } else {
                                                    self.toasts.error("Prezzo inserito non valido");
                                                }
                                            } else {
                                                self.detail_opened = Some(detail_comic);
                                            }
                                        }

                                    }
                                });
                            });
                        });
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.detail_opened = None;
                    }
                },
                );
        }
    }
}
