mod enums;
mod structs;
use std::{collections::HashMap, io::{self, Write}};
use enums::http_method::HttpMethod;
use reqwest::{header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, AUTHORIZATION}, Client, Error, Response, StatusCode};
use structs::http_request::HttpRequest;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("HostMan", native_options, Box::new(|cc| Ok(Box::new(HostMan::new(cc))))).unwrap();
}

#[derive(Default)]
struct HostMan {
    request: HttpRequest,
}

impl HostMan {
    fn new (cc: &eframe::CreationContext<'_>) -> Self {
        // customize egui here
        Self::default()
    }
}

impl eframe::App for HostMan {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello From Hostman");

            ui.horizontal(|ui| {
                ui.label("Select HTTP Method");
                egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", self.request.method))
                .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.request.method, HttpMethod::GET, "GET");
                ui.selectable_value(&mut self.request.method, HttpMethod::POST, "POST");
                ui.selectable_value(&mut self.request.method, HttpMethod::PATCH, "PATCH");
                ui.selectable_value(&mut self.request.method, HttpMethod::DELETE, "DELETE");
                ui.selectable_value(&mut self.request.method, HttpMethod::PUT, "PUT");
            });
        });
        ui.horizontal(|ui| {
            ui.label("Enter URL:");
            ui.text_edit_singleline(&mut self.request.url);
        });
        ui.collapsing("Headers", |ui| {
            let mut headers = self.request.headers.clone();
            if self.key_value_pair_editor(ui, &mut headers, "Headers") {
                self.request.headers = headers;
            }
        });
        ui.collapsing("Query Parameters", |ui| {
            let mut params = self.request.query_params.clone();
            if self.key_value_pair_editor(ui, &mut params, "Query Params") {
                self.request.query_params = params;
            }
        });
        ui.collapsing("Body", |ui| {
            let mut body = self.request.body.clone();
            if self.key_value_pair_editor(ui, &mut body, "Query Params") {
                self.request.body = body;
            }
        });
        ui.horizontal(|ui| {
            if ui.button("Send Request").clicked() {
                print!("I have been Clicked");
            }
        })
    }
);
    }
}


// way to modify key value pairs
impl HostMan {
    fn key_value_pair_editor(&mut self, ui: &mut egui::Ui, map: &mut HashMap<String, String>, label: &str) -> bool {
        let mut changed = false;
        let mut to_remove = None;
        let mut to_edit = Vec::new();

        for (key, value) in map.iter() {
            ui.horizontal(|ui| {
                let mut key_edit = key.clone();
                let key_changed = ui.text_edit_singleline(&mut key_edit).changed();
                
                let mut value_edit = value.clone();
                let value_changed = ui.text_edit_singleline(&mut value_edit).changed();
                
                if key_changed || value_changed {
                    to_edit.push((key.clone(), key_edit, value_edit));
                    changed = true;
                }

                if ui.button("X").clicked() {
                    to_remove = Some(key.clone());
                    changed = true;
                }
            });
        }

        if let Some(key) = to_remove {
            map.remove(&key);
        }

        for (old_key, new_key, new_value) in to_edit {
            map.remove(&old_key);
            map.insert(new_key, new_value);
        }

        if ui.button(format!("Add {}", label)).clicked() {
            map.insert(String::new(), String::new());
            changed = true;
        }

        changed
    }
}