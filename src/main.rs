//! A basic Windows app using tauri-egui
//! $env:RUST_LOG="debug"
//! cargo run

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use tauri::{RunEvent, Manager, WindowEvent};
use tracing::{debug, info};

fn main() {
    tracing_subscriber::fmt::init();
    tauri::Builder::default()
        .setup(|app| {
            app.wry_plugin(tauri_egui::EguiPluginBuilder::new(app.handle()));

            let native_options = eframe::NativeOptions {
                drag_and_drop_support: true,
                initial_window_size: Some([1280.0, 1024.0].into()),
                ..Default::default()
            };

            app.state::<tauri_egui::EguiPluginHandle>()
            .create_window(
                "main".to_string(),
                Box::new(|_cc| Box::new(BasicApp::default())),
                "Basic App".into(),
                native_options,
            )
            .unwrap();

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            if let RunEvent::WindowEvent { label, event, .. } = event {
                debug!("{} {:?}", label, event);
                if let WindowEvent::CloseRequested { .. } = event {
                    app.exit(0);
                }
            }
        });
}

#[derive(Default)]
struct BasicApp {}

impl tauri_egui::eframe::App for BasicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Basic App");
            let sign_in = ui.button("Sign in");
            if sign_in.clicked() {
                info!("Sign in clicked");
            }
        });
    }
}
