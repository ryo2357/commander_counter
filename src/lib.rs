// slint::slint! {
//     export component AppWindow inherits Window {
//         Text {
//             text: "Slint & Android";
//         }
//     }
// }
mod ui;

slint::slint! {
    import { Button, VerticalBox } from "std-widgets.slint";

    export component AppWindow inherits Window {
        in property <int> counter: 1; // 1
        callback clicked <=> btn.clicked; // 2, 4
        VerticalBox {
            Text { text: "Hello World" + counter; }
            btn := Button { text: "yay"; } // 3
        }
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    // let app_window = AppWindow::new().unwrap();
    // initialize_app(app_window)

    ui::app_window::start_app();
}

#[cfg(not(target_os = "android"))]
pub fn pc_main() {
    // let app = AppWindow::new().unwrap();
    // initialize_app(app)

    ui::app_window::start_app();
}

fn initialize_app(app: AppWindow) {
    // 処理を実装していく
    let weak = app.as_weak();
    app.on_clicked(move || {
        let app = weak.upgrade().unwrap();
        app.set_counter(app.get_counter() + 2);
    });
    app.run().unwrap();
}
