//! WordCraft on Android.
//!
//! This is the activity entry only. It starts the same `WordApp` the desktop and web shells
//! start, with no desktop services (file dialogs, clipboard, print).
//! Packaging is [cargo-apk](https://github.com/rust-mobile/cargo-apk) using the
//! `android-native-activity` backend, matching eframe 0.36.2's `hello_android` example.
//! `cargo-apk` generates the manifest for `NativeActivity`; this crate does not ship Kotlin.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let options = eframe::NativeOptions { android_app: Some(app), ..Default::default() };
    let launch = eframe::run_native("WordCraft", options, Box::new(|_cc| Ok(Box::new(Shell::new()))));
    if let Err(err) = launch {
        log::error!("WordCraft failed to start: {err}");
    }
}

/// The egui shell. On Android, `eframe` calls [`eframe::App::logic`] and [`eframe::App::ui`].
#[cfg(target_os = "android")]
struct Shell(wordcraft_ui_egui::WordApp);

#[cfg(target_os = "android")]
impl Shell {
    fn new() -> Self {
        let doc = wordcraft_doc::Document::new();
        let session = wordcraft_engine::Session::new(doc);
        Shell(wordcraft_ui_egui::WordApp::new(session, wordcraft_ui_egui::Services::default()))
    }
}

#[cfg(target_os = "android")]
impl eframe::App for Shell {
    fn logic(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.0.logic(ctx);
    }

    fn raw_input_hook(&mut self, _ctx: &eframe::egui::Context, raw: &mut eframe::egui::RawInput) {
        self.0.raw_input_hook(raw);
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        self.0.ui(ui);
    }
}
