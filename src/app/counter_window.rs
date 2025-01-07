use slint::ComponentHandle;

use crate::ui::CounterWindow;
pub struct CounterApp {
    app: CounterWindow
}

impl CounterApp {
    pub fn new() -> Self {
        let app = CounterWindow::new().unwrap();
        Self { app }
    }

    pub fn start_app(&self) {
        let weak = self.app.as_weak();
        self.app.on_clicked(move || {
            let app = weak.upgrade().unwrap();
            app.set_counter(app.get_counter() + 2);
        });
        self.app.run().unwrap();
    }
}


