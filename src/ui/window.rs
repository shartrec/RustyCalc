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

use iced::{Background, Border, Color, Command, Degrees, Element, event, Event, Font, gradient, Length, Pixels, Radians, Renderer, Shadow, Subscription, Theme, Vector, window};
use iced::alignment::{Horizontal, Vertical};
use iced::font::{Family, Stretch, Style, Weight};
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

        let font1 = Font {
            family: Family::Monospace,
            weight: Weight::Bold,
            stretch: Stretch::Normal,
            style: Style::Normal,
        };

        let lcd = text_editor(&self.content)
            .height(Length::Fill)
            // .height(Length::Fill)
            .style(|_theme, _status| {
                text_editor::Appearance {
                    background: Background::Color(Color::TRANSPARENT),
                    border: Border::default().with_width(Pixels::from(1)).with_color(Color::from_rgb8(0x7f, 0x7f, 0x7f)),
                    icon: Default::default(),
                    placeholder: Default::default(),
                    value: Color::BLACK,
                    selection: Color::from_rgb8(0x7f, 0x9f, 0x9f),
                }
            })
            .on_action(|action| {
                Message::EditorAction(action)
            })
            .font(font1)
            .into();

        let font2 = Font {
            family: Family::SansSerif,
            weight: Weight::Bold,
            stretch: Stretch::Normal,
            style: Style::Normal,
        };
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
            .horizontal_alignment(Horizontal::Right)
            .font(font2)
            .into();


        let mode: Element<Message> = Button::new(text(self.calc.angle_mode()))
            .style(|_theme, _status| {
                Appearance {
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    text_color: Color::BLACK,
                    border: Border::default(),
                    shadow: Default::default(),
                }
            })
            .on_press(Message::ToggleMode)
            .into();

        let con_mode = Container::new(mode)
            .width(Length::Fill)
            .align_x(Horizontal::Right)
            .clip(false)
            .into();

        let top = Column::with_children([con_mode, lcd, result]).spacing(5);
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
        let b_one = ButtonBuilder::new("1", w, h).make();
        // let b_one = Builder::new("1", w, h).make();
        let b_two = ButtonBuilder::new("2", w, h).make();
        let b_three = ButtonBuilder::new("3", w, h).make();
        let b_four = ButtonBuilder::new("4", w, h).make();
        let b_five = ButtonBuilder::new("5", w, h).make();
        let b_six = ButtonBuilder::new("6", w, h).make();
        let b_seven = ButtonBuilder::new("7", w, h).make();
        let b_eight = ButtonBuilder::new("8", w, h).make();
        let b_nine = ButtonBuilder::new("9", w, h).make();
        let b_zero = ButtonBuilder::new("0", w, h).make();
        let b_dec = ButtonBuilder::new(".", w, h).make();
        // Basic operations
        let b_plus = ButtonBuilder::new("+", w, h).make();
        let b_minus = ButtonBuilder::new("-", w, h).make();
        let b_mult = ButtonBuilder::new("x", w, h).msg(Message::Char("*".to_string())).make();
        let b_div = ButtonBuilder::new("/", w, h).make();
        let b_pow = ButtonBuilder::new("^", w, h).make();
        let b_lparen = ButtonBuilder::new("(", w, h).msg(Message::Func("".to_string())).make();
        let b_rparen = ButtonBuilder::new(")", w, h).make();
        // Functions
        let b_sin = ButtonBuilder::for_func("sin", w, h).make();
        let b_cos = ButtonBuilder::for_func("cos", w, h).make();
        let b_tan = ButtonBuilder::for_func("tan", w, h).make();
        let b_asin = ButtonBuilder::for_func("asin", w, h).make();
        let b_acos = ButtonBuilder::for_func("acos", w, h).make();
        let b_atan = ButtonBuilder::for_func("atan", w, h).make();
        let b_exp = ButtonBuilder::for_func("exp", w, h).make();
        let b_ln = ButtonBuilder::for_func("ln", w, h).make();
        let b_log = ButtonBuilder::for_func("log", w, h).make();
        let b_log2 = ButtonBuilder::for_func("log2", w, h).make();
        let b_sqrt = ButtonBuilder::new("√", w, h).msg(Message::Func("sqrt".to_string())).make();
        let b_abs = ButtonBuilder::for_func("abs", w, h).make();
        let b_ceil = ButtonBuilder::for_func("ceil", w, h).make();
        let b_floor = ButtonBuilder::for_func("floor", w, h).make();
        let b_fact = ButtonBuilder::for_func("!", w, h).msg(Message::Func("factorial".to_string())).make();
        // Command buttons
        let b_equals = ButtonBuilder::new("=", w, h).msg(Message::Evaluate).make();
        let b_clear = ButtonBuilder::new("AC", w, h).msg(Message::Clear)
            .colors((Color::from_rgb8(0xf0, 0x24, 0x24), Color::from_rgb8(0xD0, 0x24, 0x24))).make();
        let b_left = ButtonBuilder::new("<-", w, h).msg(Message::MoveLeft).make();
        let b_right = ButtonBuilder::new("->", w, h).msg(Message::MoveRight).make();
        let b_back = ButtonBuilder::new("<-del", w, h).msg(Message::BackSpace).make();
        let b_more = ButtonBuilder::new("more..", w, h).msg(Message::Menu).make();

        let col_all = Column::with_children([
            lcd_container.into(),
            Row::with_children([
                Column::with_children([
                    Row::with_children([b_back, b_left, b_right, b_more, b_clear]).spacing(2).into(),
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
                // If we have a selection, we want to surround it with the function
                if let Some(sel) = self.content.selection() {
                    for c in s.chars() {
                        self.content.perform(Action::Edit(Edit::Insert(c)));
                    }
                    self.content.perform(Action::Edit(Edit::Insert('(')));
                    for c in sel.chars() {
                        self.content.perform(Action::Edit(Edit::Insert(c)));
                    }
                    self.content.perform(Action::Edit(Edit::Insert(')')));
                    Command::none()
                } else {
                    // determine if we are at the end of the text. If so surround all text in function call
                    let cursor = self.content.cursor_position();
                    let line_count = self.content.line_count();

                    if cursor.0 == line_count - 1 && cursor.1 == self.content.line(cursor.0).unwrap().len()
                       && cursor != (0,0) {
                        self.content.perform(Action::Move(Motion::DocumentStart));
                        for c in s.chars() {
                            self.content.perform(Action::Edit(Edit::Insert(c)));
                        }
                        self.content.perform(Action::Edit(Edit::Insert('(')));
                        Command::batch(vec![
                            // Send the Message::MoveLeft message
                            Command::perform(async {}, |_| Message::MoveEnd),
                            Command::perform(async {}, |_| Message::Char(")".to_string()))
                        ])
                    } else {  //otherwise insert the function and move cursor between the parentheses
                for c in s.chars() {
                    self.content.perform(Action::Edit(Edit::Insert(c)));
                }
                self.content.perform(Action::Edit(Edit::Insert('(')));
                self.content.perform(Action::Edit(Edit::Insert(')')));
                Command::perform(async {}, |_| Message::MoveLeft)
            }
                }
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
                self.result = None;
                Command::none()
            }
            Message::MoveLeft => {
                self.content.perform(Action::Move(Motion::Left));
                Command::none()
            }
            Message::MoveRight => {
                self.content.perform(Action::Move(Motion::Right));
                Command::none()
            }
            Message::MoveEnd => {
                self.content.perform(Action::Move(Motion::DocumentEnd));
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
                    AngleMode::Degrees => AngleMode::Radians,
                    AngleMode::Radians => AngleMode::Gradians,
                    AngleMode::Gradians => AngleMode::Degrees,
                });
                Command::none()
            }
            Message::Menu => {
                // todo Show menu
                Command::none()
            }
        }
    }
}

struct ButtonBuilder<'a> {
    name : &'a str,
    w : Length,
    h : Length,
    msg : Option<Message>,
    colors : Option<(Color, Color)>,
}
impl <'a> ButtonBuilder<'static> {

    fn new(name: &'static str, w: Length, h: Length) -> Self {
        Self {name, w, h , msg: None, colors: None}
    }

    fn for_func(name: &'static str, w: Length, h: Length) -> Self {
        Self {name, w, h, msg: Some(Message::Func(name.to_string())), colors: None}
    }
    fn msg(self, msg : Message) -> Self {
        Self {
            msg : Some(msg),
            ..self
    }
    }

    fn colors(self, colors : (Color, Color)) -> Self {
        Self {
            colors : Some(colors),
            ..self
        }
    }

    fn make(self) -> Element<'static, Message> {
        make_button(self.w, self.h, self.name,
                    self.msg.unwrap_or_else(|| {Message::Char(self.name.to_string())}),
                    self.colors.unwrap_or_else(|| {(Color::from_rgb8(0x24, 0x24, 0x24), Color::from_rgb8(0x55, 0x55, 0x55))}))
    }
}


/// Make a button in a container that centers it
fn make_button(width: Length, height: Length, name: &str, msg: Message, colors: (Color, Color)) -> Element<Message> {
    let container = Container::new(name)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .clip(false);

    Button::new(container)
        .width(width)
        .height(height)
        .style(move |_theme, status| {
            get_style(status, colors)
        })
        .on_press(msg)
        .into()
}

fn get_style(status: Status, colors: (Color, Color)) -> Appearance {
    match status {
        Status::Active => {
            let g = gradient::Linear::new(Radians::from(Degrees(150.0)))
                .add_stop(0.0, colors.0)
                .add_stop(1.0, colors.1);

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
                background: Some(Background::from(Color::from_rgb8(0xd0, 0xd0, 0xd0))),
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
    const MIN_LCD_PANEL_HEIGHT: f32 = 110.0;
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
