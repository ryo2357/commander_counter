use slint::ComponentHandle;

use crate::ui::VerifyWindow;
pub struct VerifyApp {
    app: VerifyWindow,
}

impl VerifyApp {
    pub fn new() -> Self {
        let app = VerifyWindow::new().unwrap();
        Self { app }
    }

    pub fn start_app(&self) {
        self.app.run().unwrap();
    }
}
