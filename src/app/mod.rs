#![allow(dead_code)]
mod counter_window;
mod verify_window;

// 使用するモジュールの切り替え
// pub use counter_window::CounterApp as App;
pub use verify_window::VerifyApp as App;
