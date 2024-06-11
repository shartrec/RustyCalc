/*
 * Copyright (c) 2024.
 *
 * Copyright 2024 Trevor Campbell
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
 * associated documentation files (the “Software”), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge, publish, distribute,
 * sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies or
 * substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
 * NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT
 * OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 */

use iced::multi_window::Application;
use iced::Settings;
use iced::Size;

use crate::ui::calculator_app::CalculatorApp;

mod evaluator;

#[cfg(test)]
mod test;
mod ui;

/// Calculate.
fn main() -> iced::Result {

    // let event_loop = EventLoop::new();
    // let window_builder = WindowBuilder::new()
    //     .with_title("KelpieCalc")
    //     .with_window_class("KelpieCalcClass", "KelpieCalcIcon");
    //
    let window_settings = iced::window::Settings {
        size: Size::new(330.0, 420.0),
        min_size: Some(Size::new(300.0, 420.0)),
        ..iced::window::Settings::default()
    };

    let settings = Settings {
        id: Some(String::from("RustyCalc")),
        window: window_settings,
        .. Settings::default()
    };

    CalculatorApp::run(settings)
}

