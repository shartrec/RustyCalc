use iced::{program, Size};

use crate::ui::window::CalculatorApp;

mod evaluator;

#[cfg(test)]
mod test;
mod ui;

/// Calculate.
fn main() -> iced::Result {
    program("My Calculator", CalculatorApp::update, CalculatorApp::view)
        .window_size(Size::new(330.0, 450.0))
        .subscription(CalculatorApp::subscription)
        .run()
}

