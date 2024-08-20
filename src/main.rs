mod enums;
mod structs;
mod services;
use std::{clone, collections::HashMap, sync::Arc};
use egui::mutex::Mutex;
use enums::http_method::HttpMethod;
use structs::http_request::HttpRequest;
use services::http_service::handle_req;
use tokio::runtime::Runtime;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("HostMan", native_options, Box::new(|cc| Ok(Box::new(HostMan::new(cc))))).unwrap();
}

struct HostMan {
    request: HttpRequest,
    runtime: Runtime,
    is_loading: bool,
    response: Arc<Mutex<String>>,
    pending_request: Option<HttpRequest>,
}

impl HostMan {
    fn new (_cc: &eframe::CreationContext<'_>) -> Self {
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
                    Ok(_) => "Request successful".to_string(),
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
                    self.send_request();
                }
            });

            if self.is_loading {
                ui.spinner();
            } else {
                let response = self.response.lock();
                ui.label(&*response);
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