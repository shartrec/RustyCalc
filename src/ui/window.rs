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

use iced::{alignment, Background, Border, Color, Command, Degrees, Element, event, Event, gradient,
           Length, Pixels, Radians, Renderer, Shadow, Subscription, Theme, Vector, window};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Button, Column, Container, container, Row, text, text_editor};
use iced::widget::button::{Appearance, Status};
use iced::widget::text_editor::{Action, Content, Edit, Motion};

use crate::evaluator::AngleMode;
use crate::ui::calculator::Calc;
use crate::ui::messages::Message;

#[derive(Debug, Default)]
pub(crate) struct CalculatorApp {
    content: Content,
    result: Option<Result<f64, String>>,
    calc: Calc,
    window_width: u32,
    window_height: u32,
}

impl CalculatorApp {
    pub(crate) fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status| {
            match event {
                Event::Window(_id, window::Event::Resized { width, height }) => {
                    Some(Message::WindowResized(width, height))
                }
                _ => None
            }
        })
    }

    pub(crate) fn view(&self) -> Container<Message, Theme, Renderer> {

        // Get the sizes for the major blocks
        let (lcd_height, button_height, button_width) = get_container_sizes(self.window_width, self.window_height);


        let lcd = text_editor(&self.content)
            .height(Length::Fill)
            // .height(Length::Fill)
            .style(|_theme, _status| {
                text_editor::Appearance {
                    background: Background::Color(Color::from_rgba8(0, 0, 0, 0.0)),
                    border: Border::default().with_width(Pixels::from(1)).with_color(Color::from_rgb8(0x7f, 0x7f, 0x7f)),
                    icon: Default::default(),
                    placeholder: Default::default(),
                    value: Color::BLACK,
                    selection: Default::default(),
                }
            })
            .on_action(|action| {
                Message::EditorAction(action)
            })
            .into();

        let result = text(match &self.result {
            Some(r) => {
                match r {
                    Ok(v) => {
                        let formatted = format!("= {0:.1$}", v, 10);
                        formatted.trim_end_matches('0').trim_end_matches('.').to_string()
                    }
                    Err(e) => e.clone()
                }
            }
            None => String::from(""),
        })
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Right)
            .into();

        let mode = text(self.calc.angle_mode())
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Right)
            .into();

        let top = Column::with_children([mode, lcd, result]).spacing(5);
        let lcd_container = container(top)
            .width(Length::Fill)
            .height(lcd_height)
            .style(move |_theme, _status| {
                container::Appearance {
                    background: Some(Background::Color(Color::from_rgb8(0xd4, 0xed, 0xd4))),
                    ..Default::default()
                }
            })
            .padding(2);

        let w = button_width;
        let h = button_height;

        // The standard numer buttons
        let b_one = make_button(w, h, "1");
        let b_two = make_button(w, h, "2");
        let b_three = make_button(w, h, "3");
        let b_four = make_button(w, h, "4");
        let b_five = make_button(w, h, "5");
        let b_six = make_button(w, h, "6");
        let b_seven = make_button(w, h, "7");
        let b_eight = make_button(w, h, "8");
        let b_nine = make_button(w, h, "9");
        let b_zero = make_button(w, h, "0");
        let b_dec = make_button(w, h, ".");
        // Basic operations
        let b_plus = make_button(w, h, "+");
        let b_minus = make_button(w, h, "-");
        let b_mult = button_with_msg(w, h, "x", Message::Char("*".to_string()));
        let b_div = make_button(w, h, "/");
        let b_pow = make_button(w, h, "^");
        let b_lparen = button_with_msg(w, h, "(", Message::Func("()".to_string()));
        let b_rparen = make_button(w, h, ")");
        // Functions
        let b_sin = button_with_msg(w, h, "sin", Message::Func("sin()".to_string()));
        let b_cos = button_with_msg(w, h, "cos", Message::Func("cos()".to_string()));
        let b_tan = button_with_msg(w, h, "tan", Message::Func("tan()".to_string()));
        let b_asin = button_with_msg(w, h, "asin", Message::Func("asin()".to_string()));
        let b_acos = button_with_msg(w, h, "acos", Message::Func("acos()".to_string()));
        let b_atan = button_with_msg(w, h, "atan", Message::Func("atan()".to_string()));
        let b_exp = button_with_msg(w, h, "exp", Message::Func("exp()".to_string()));
        let b_ln = button_with_msg(w, h, "ln", Message::Func("ln()".to_string()));
        let b_log = button_with_msg(w, h, "log", Message::Func("log()".to_string()));
        let b_log2 = button_with_msg(w, h, "log2", Message::Func("log2()".to_string()));
        let b_sqrt = button_with_msg(w, h, "√", Message::Func("sqrt()".to_string()));
        let b_abs = button_with_msg(w, h, "abs", Message::Func("abs()".to_string()));
        let b_ceil = button_with_msg(w, h, "ceil", Message::Func("ceil()".to_string()));
        let b_floor = button_with_msg(w, h, "floor", Message::Func("floor()".to_string()));
        let b_fact = button_with_msg(w, h, "!", Message::Func("factorial()".to_string()));
        // Command buttons
        let b_equals = button_with_msg(w, h, "=", Message::Evaluate);
        let b_clear = button_with_msg(w, h, "AC", Message::Clear);
        let b_left = button_with_msg(w, h, "<-", Message::Move(-1));
        let b_right = button_with_msg(w, h, "->", Message::Move(1));
        let b_back = button_with_msg(w, h, "<del", Message::BackSpace);
        let b_drg = button_with_msg(w, h, "DRG", Message::ToggleMode);

        let col_all = Column::with_children([
            lcd_container.into(),
            Row::with_children([
                Column::with_children([
                    Row::with_children([b_back, b_left, b_right, b_clear, b_drg]).spacing(2).into(),
                    Row::with_children([b_sin, b_cos, b_tan, b_sqrt, b_abs]).spacing(2).into(),
                    Row::with_children([b_asin, b_acos, b_atan, b_ceil, b_floor]).spacing(2).into(),
                    Row::with_children([b_exp, b_ln, b_log, b_log2, b_fact]).spacing(2).into(),
                ]).spacing(2).into(),
            ]).spacing(2).into(),
            Row::with_children([
                Column::with_children([
                    Row::with_children([b_seven, b_eight, b_nine, b_lparen, b_rparen]).spacing(2).into(),
                    Row::with_children([b_four, b_five, b_six, b_mult, b_div]).spacing(2).into(),
                    Row::with_children([b_one, b_two, b_three, b_plus, b_minus]).spacing(2).into(),
                    Row::with_children([b_zero, b_dec, b_equals, b_pow]).spacing(2).into(),
                ]).spacing(2).into(),
            ]).spacing(2).into(),
        ]).spacing(2);

        container(col_all)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme, _status| {
                let gradient = gradient::Linear::new(Radians(135.0))
                    .add_stop(0.0, Color::from_rgb8(0x30, 0x30, 0x30))
                    .add_stop(0.25, Color::from_rgb8(0x35, 0x35, 0x35))
                    .add_stop(0.50, Color::from_rgb8(0x40, 0x40, 0x40))
                    .add_stop(0.75, Color::from_rgb8(0x45, 0x45, 0x45))
                    .add_stop(1.0, Color::from_rgb8(0x50, 0x50, 0x50))
                    .into();

                container::Appearance {
                    background: Some(Background::Gradient(gradient)),
                    ..Default::default()
                }
            })
            .padding(10)
    }

    pub(crate) fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Char(s) => {
                for c in s.chars() {
                    self.content.perform(Action::Edit(Edit::Insert(c)));
                }
                Command::none()
            }
            Message::Func(s) => {
                for c in s.chars() {
                    self.content.perform(Action::Edit(Edit::Insert(c)));
                }
                // self.content.perform(Action::Move(Motion::End));
                self.content.perform(Action::Move(Motion::Left));
                Command::none()
            }
            Message::EditorAction(action) => {
                match action {
                    Action::Edit(Edit::Enter) => {
                        self.result = Some(self.calc.evaluate(&self.content.text()))
                    }
                    _ => self.content.perform(action)
                }
                Command::none()
            }
            Message::Evaluate => {
                self.result = Some(self.calc.evaluate(&self.content.text()));
                Command::none()
            }
            Message::Clear => {
                self.content.perform(Action::Move(Motion::DocumentStart));
                self.content.perform(Action::Select(Motion::DocumentEnd));
                self.content.perform(Action::Edit(Edit::Delete));
                Command::none()
            }
            Message::Move(i) => {
                if i == -1 {
                    self.content.perform(Action::Move(Motion::Left));
                } else if i == 1 {
                    self.content.perform(Action::Move(Motion::Right));
                }
                Command::none()
            }
            Message::BackSpace => {
                self.content.perform(Action::Edit(Edit::Backspace));
                Command::none()
            }
            Message::WindowResized(w, h) => {
                self.window_width = w;
                self.window_height = h;
                Command::none()
            }
            Message::ToggleMode => {
                self.calc.set_angle_mode(match self.calc.angle_mode() {
                    AngleMode::Degrees => AngleMode::Gradians,
                    AngleMode::Radians => AngleMode::Degrees,
                    AngleMode::Gradians => AngleMode::Radians
                });
                Command::none()
            }
        }
    }
}

/// Make a button in a container that centers it using the default Message generated from the button text
fn make_button(width: Length, height: Length, name: &str) -> Element<Message> {
    crate::ui::window::button_with_msg(width, height, name, Message::Char(name.to_string()))
}

/// Make a button in a container that centers it
fn button_with_msg(width: Length, height: Length, name: &str, msg: Message) -> Element<Message> {
    let container = Container::new(name)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .clip(false);

    Button::new(container)
        .width(width)
        .height(height)
        .style(|_theme, status| {
            crate::ui::window::get_style(status)
        })
        .on_press(msg)
        .into()
}

fn get_style(status: Status) -> Appearance {
    match status {
        Status::Active => {
            let g = gradient::Linear::new(Radians::from(Degrees(150.0)))
                .add_stop(0.0, Color::from_rgb8(0x24, 0x24, 0x24))
                .add_stop(1.0, Color::from_rgb8(0x55, 0x55, 0x55));

            Appearance {
                background: Some(Background::from(g)),
                text_color: Color::WHITE,
                border: Border::default().with_width(Pixels::from(2)).with_color(Color::from_rgb8(0x20, 0x20, 0x20)),
                shadow: Shadow { color: Color::WHITE, offset: Vector::new(-2.0, -2.0), blur_radius: 2.0 },
            }
        }
        Status::Hovered => {
            let g = gradient::Linear::new(Radians::from(Degrees(150.0)))
                .add_stop(0.0, Color::from_rgb8(0x54, 0x54, 0x54))
                .add_stop(1.0, Color::from_rgb8(0x85, 0x85, 0x85));

            Appearance {
                background: Some(Background::from(g)),
                text_color: Color::WHITE,
                border: Border::default().with_width(Pixels::from(2)).with_color(Color::BLACK),
                shadow: Default::default(),
            }
        }
        Status::Pressed => {
            Appearance {
                background: None,
                text_color: Color::BLACK,
                border: Border::default().with_width(Pixels::from(2)).with_color(Color::BLACK),
                shadow: Default::default(),
            }
        }
        Status::Disabled => {
            Appearance {
                background: None,
                text_color: Color::BLACK,
                border: Border::default().with_width(Pixels::from(2)).with_color(Color::BLACK),
                shadow: Default::default(),
            }
        }
    }
}

/// Calculate the sizes for the variable components of the display.
/// These are needed to make sure the display scales sensibly and smoothly as the window is resized.
///
/// Returns the height of the *LCD* panel, followed by the height and width of the buttons.
/// The returned values may be Length::Fill
fn get_container_sizes(width: u32, height: u32) -> (Length, Length, Length) {
    const MIN_LCD_PANEL_HEIGHT: f32 = 100.0;
    const MIN_BUTTON_HEIGHT: f32 = 33.0;
    const MIN_BUTTON_WIDTH: f32 = 55.0;
    const MAX_BUTTON_HEIGHT: f32 = 45.0;

    // The buttons take up rows * (button height + button spacing) + container spacing.
    let b_h = ((height as f32 - MIN_LCD_PANEL_HEIGHT) / 8.0 - 4.0).max(MIN_BUTTON_HEIGHT).min(MAX_BUTTON_HEIGHT);
    let b_w = ((width as f32 - 4.0) / 5.0 - 4.0).max(MIN_BUTTON_WIDTH);

    let t_panel = if b_h < MAX_BUTTON_HEIGHT {
        Length::from(MIN_LCD_PANEL_HEIGHT)
    } else {
        Length::Fill
    };

    (t_panel, Length::from(b_h), Length::from(b_w))
}
