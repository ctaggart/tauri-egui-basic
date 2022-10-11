//! A basic Windows app using tauri-egui

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use tauri::{RunEvent, State};
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_native_window])
        .setup(|app| {
            app.wry_plugin(tauri_egui::EguiPluginBuilder::new(app.handle()));
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app, event| {
            if let RunEvent::WindowEvent { label, event, .. } = event {
                println!("{} {:?}", label, event);
            }
        });
}

#[tauri::command]
async fn open_native_window(
    egui_handle: State<'_, tauri_egui::EguiPluginHandle>,
) -> Result<(), ()> {
    let native_options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some([1280.0, 1024.0].into()),
        ..Default::default()
    };

    let _window = egui_handle
        .create_window(
            "native-window".to_string(),
            Box::new(|cc| Box::new(BasicApp::default())),
            "Login".into(),
            native_options,
        )
        .unwrap();

    Ok(())
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
