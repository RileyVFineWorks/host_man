#![windows_subsystem = "windows"]
mod enums;
mod structs;
mod services;
use std::{collections::HashMap, sync::Arc};
mod tests;
use egui::mutex::Mutex;
use enums::http_method::HttpMethod;
use structs::http_request::HttpRequest;
use services::http_service::handle_req;
use tokio::runtime::Runtime;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("HostMan", native_options, Box::new(|_cc| Ok(Box::new(HostMan::new())))).unwrap();
}

struct HostMan {
    request: HttpRequest,
    runtime: Runtime,
    is_loading: bool,
    response: Arc<Mutex<String>>,
    pending_request: Option<HttpRequest>,
}

impl HostMan {
    fn new () -> Self {
        // customize egui here
        Self {
            request: HttpRequest::default(),
            runtime: Runtime::new().expect("Failed to create Tokio runtime"),
            is_loading: false,
            response: Arc::new(Mutex::new(String::new())),
            pending_request: None,
        }
    }

    fn send_request(&mut self) {
        if !self.is_loading {
            self.is_loading = true;
            self.pending_request = Some(self.request.clone());
        }
    }

    fn check_pending_request(&mut self, ctx: &egui::Context) {
        if let Some(request) = self.pending_request.take() {
            let response_clone = self.response.clone();
            let ctx = ctx.clone();
    
            self.runtime.spawn(async move {
                let result = handle_req(&request).await;
                let mut response = response_clone.lock();
                *response = match result {
                    Ok(x) => format!("Status {:?} \nResponse: {:?}", x.0, serde_json::to_string_pretty(&x.1).unwrap()),
                    Err(e) => format!("Error: {}", e),
                };
                ctx.request_repaint();
            });
        }
    }
}

impl eframe::App for HostMan {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_pending_request(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("HostMan - HTTP Request Tool")
                    .on_hover_text("Send and manage HTTP requests easily");
            });

            ui.add_space(10.0);

            egui::Grid::new("http_request_grid")
                .num_columns(2)
                .spacing([10.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("HTTP Method:");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:?}", self.request.method))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.request.method, HttpMethod::GET, "GET");
                            ui.selectable_value(&mut self.request.method, HttpMethod::POST, "POST");
                            ui.selectable_value(&mut self.request.method, HttpMethod::PATCH, "PATCH");
                            ui.selectable_value(&mut self.request.method, HttpMethod::DELETE, "DELETE");
                            ui.selectable_value(&mut self.request.method, HttpMethod::PUT, "PUT");
                        });
                    ui.end_row();

                    ui.label("URL:");
                    ui.add(egui::TextEdit::singleline(&mut self.request.url).hint_text("Enter URL here"));
                    ui.end_row();
                });

            ui.add_space(10.0);

            egui::CollapsingHeader::new("Headers")
                .default_open(true)
                .show(ui, |ui| {
                    let mut headers = self.request.headers.clone();
                    if self.key_value_pair_editor(ui, &mut headers, "Headers") {
                        self.request.headers = headers;
                    }
                });

            egui::CollapsingHeader::new("Query Parameters")
                .default_open(true)
                .show(ui, |ui| {
                    let mut params = self.request.query_params.clone();
                    if self.key_value_pair_editor(ui, &mut params, "Query Params") {
                        self.request.query_params = params;
                    }
                });

            egui::CollapsingHeader::new("Body")
                .default_open(true)
                .show(ui, |ui| {
                    let mut body = self.request.body.clone();
                    if self.key_value_pair_editor(ui, &mut body, "Body") {
                        self.request.body = body;
                    }
                });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                if ui.add_sized([120.0, 30.0], egui::Button::new("Send Request")).clicked() {
                    self.send_request();
                }
            });

            ui.add_space(20.0);

            if self.is_loading {
                ui.vertical_centered(|ui| {
                    ui.add(egui::Spinner::new().size(32.0));
                    ui.label("Sending request...");
                });
            } else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let response = self.response.lock();
                    ui.add(
                        egui::TextEdit::multiline(&mut response.as_str())
                            .desired_width(f32::INFINITY)
                            .font(egui::TextStyle::Monospace)
                    );
                });
            }
        });

        if self.is_loading {
            let response = self.response.lock();
            if !response.is_empty() {
                self.is_loading = false;
            }
        }
    }
}


// way to modify key value pairs
impl HostMan {
    fn key_value_pair_editor(&mut self, ui: &mut egui::Ui, map: &mut HashMap<String, String>, label: &str) -> bool {
        let mut changed = false;
        let mut to_remove = None;
        let mut to_edit = Vec::new();

        egui::Grid::new(format!("{}_grid", label))
            .num_columns(3)
            .spacing([5.0, 5.0])
            .striped(true)
            .show(ui, |ui| {
                for (key, value) in map.iter() {
                    let mut key_edit = key.clone();
                    let key_changed = ui.add(egui::TextEdit::singleline(&mut key_edit).hint_text("Key")).changed();
                    
                    let mut value_edit = value.clone();
                    let value_changed = ui.add(egui::TextEdit::singleline(&mut value_edit).hint_text("Value")).changed();
                    
                    if key_changed || value_changed {
                        to_edit.push((key.clone(), key_edit, value_edit));
                        changed = true;
                    }

                    if ui.button("🗑").clicked() {
                        to_remove = Some(key.clone());
                        changed = true;
                    }
                    ui.end_row();
                }
            });

        if let Some(key) = to_remove {
            map.remove(&key);
        }

        for (old_key, new_key, new_value) in to_edit {
            map.remove(&old_key);
            map.insert(new_key, new_value);
        }

        if ui.button(format!("➕ Add {}", label)).clicked() {
            map.insert(String::new(), String::new());
            changed = true;
        }

        changed
    }
}
