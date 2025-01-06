slint::slint! {
    import { Button, VerticalBox } from "std-widgets.slint";

    export component AppWindow inherits Window {
        in property <int> counter: 1; // 1
        callback clicked <=> btn.clicked; // 2, 4
        VerticalBox {
            Text { text: "Hello World from module" + counter; }
            btn := Button { text: "yay"; } // 3
        }
    }
}

pub fn start_app() {
    let app = AppWindow::new().unwrap();
    let weak = app.as_weak();
    app.on_clicked(move || {
        let app = weak.upgrade().unwrap();
        app.set_counter(app.get_counter() + 2);
    });
    app.run().unwrap();
}
