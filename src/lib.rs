#[allow(clippy::all)]
pub mod ui {
    slint::include_modules!();
}

mod app;

#[cfg(not(target_os = "android"))]
pub fn pc_main() {
    let app = app::App::new();
    app.start_app();
}

#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    let app = app::App::new();
    app.start_app();
}
