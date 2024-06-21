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

// This module contains the logic for the extras window.
// It is broken out for the sake of maintainability and follows the same conventions as
// the main view / update logic of the main Application for ease of understanding

use iced::{Background, Border, Color, Command, Degrees, Element, gradient, Length, Padding, Pixels, Radians, Renderer, Shadow, Theme, Vector, window};
use iced::alignment::{Horizontal, Vertical};
use iced::theme::palette::Pair;
use iced::widget::{Button, Column, container, Container, Row, text, text_editor};
use iced::widget::button::{Appearance, Status};
use iced::widget::text_editor::{Action, Content, Edit, Motion};
use iced::window::Id;
use palette::{convert::FromColor, Hsl};
use palette::rgb::Rgb;

use crate::evaluator::AngleMode;
use crate::ui::calculator::Calc;
use crate::ui::messages::Message;

#[derive(Debug)]
pub(super) struct CalcWindow {
    content: Content,
    result: Option<Result<f64, String>>,
    calc: Calc,
    window_width: u32,
    window_height: u32,
    window_x: i32,
    window_y: i32,
}

impl Default for CalcWindow {
    fn default() -> Self {
        let mut calc = Calc::default();
        // Load the angle mode from preferences
        let pref = crate::ui::preferences::manager();
        if let Some(am) = pref.get::<String>(crate::ui::preferences::ANGLE_MODE) {
            calc.set_angle_mode(AngleMode::get_from_name(am.as_str()));
        }

        Self {
            content: Default::default(),
            result: None,
            calc: calc,
            window_width: 0,
            window_height: 0,
            window_x: 0,
            window_y: 0,
        }
    }
}

impl CalcWindow {

    pub fn title(&self) -> String {
        "Rusty Calculator".to_string()
    }
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Char(s) => {
                for c in s.chars() {
                    self.content.perform(Action::Edit(Edit::Insert(c)));
                }
                Command::none()
            }
            Message::Constant(s) => {
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
                } else {  //otherwise insert the function and move cursor between the parentheses
                    for c in s.chars() {
                        self.content.perform(Action::Edit(Edit::Insert(c)));
                    }
                    self.content.perform(Action::Edit(Edit::Insert('(')));
                    self.content.perform(Action::Edit(Edit::Insert(')')));
                    Command::perform(async {}, |_| Message::MoveLeft)
                }
            }
            Message::EditorAction(action) => {
                match action {
                    Action::Edit(Edit::Enter) => {
                        self.result = Some(self.calc.evaluate(&self.content.text().trim()));
                        Command::perform(async {}, |_| Message::MoveEnd)
                    }
                    _ => {
                        self.content.perform(action);
                        Command::none()
                    }
                }
            }
            Message::History(expr, value) => {
                self.content.perform(Action::Move(Motion::DocumentStart));
                self.content.perform(Action::Select(Motion::DocumentEnd));
                self.content.perform(Action::Edit(Edit::Delete));
                for c in expr.chars() {
                    self.content.perform(Action::Edit(Edit::Insert(c)));
                }
                self.result = Some(Ok(value));
                Command::none()
            }

            Message::Evaluate => {
                self.result = Some(self.calc.evaluate(&self.content.text().trim()));
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
            Message::WindowResized(id, w, h) => {
                if id == Id::MAIN {
                    self.window_width = w.clone();
                    self.window_height = h.clone();
                }
                Command::none()
            }
            Message::WindowMoved(id, x, y) => {
                if id == Id::MAIN {
                    self.window_x = x.clone();
                    self.window_y = y.clone();
                }
                Command::none()
            }
           Message::WindowClose(id) => {
                if id == Id::MAIN {
                    let _ = save_window_size(self.window_width, self.window_height);
                    window::close::<Message>(id)
                } else {
                    Command::none()
                }
            }
            Message::ToggleMode => {
                self.calc.set_angle_mode(match self.calc.angle_mode() {
                    AngleMode::Degrees => AngleMode::Radians,
                    AngleMode::Radians => AngleMode::Gradians,
                    AngleMode::Gradians => AngleMode::Degrees,
                });
                let pref = crate::ui::preferences::manager();
                pref.put(crate::ui::preferences::ANGLE_MODE, self.calc.angle_mode());
                Command::none()
            }
            _ => {
                Command::none()
            }
        }
    }
    pub(super) fn view<'a>(&'a self, _id: Id) -> Element<Message> {
        let lcd = text_editor(&self.content)
            .height(Length::Fill)
            // .height(Length::Fill)
            .style(|_theme, _status| {
                text_editor::Appearance {
                    background: Background::Color(Color::TRANSPARENT),
                    border: Border::default().with_width(Pixels::from(1)).with_color(Color::from_rgb8(0x7f, 0x7f, 0x7f)),
                    .. text_editor::default(_theme, _status)
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
                        if *v < 0.001 || *v > 10000000.0 {
                            format!("= {:+e}", v)
                        } else {
                            let formatted = format!("= {0:.1$}", v, 10);
                            formatted.trim_end_matches('0').trim_end_matches('.').to_string()
                        }
                    }
                    Err(e) => e.clone()
                }
            }
            None => String::from(""),
        })
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Right)
            .into();

        let mode: Element<Message> = Button::new(text(self.calc.angle_mode()))
            .style(|theme: &Theme, _status| {
                Appearance {
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    text_color: theme.extended_palette().background.base.text,
                    .. Appearance::default()
                }
            })
            .padding(Padding::from(0))
            .on_press(Message::ToggleMode)
            .height(Length::Shrink)
            .into();

        let con_mode = Container::new(mode)
            .width(Length::Fill)
            .align_x(Horizontal::Right)
            .clip(false)
            .into();

        let top = Column::with_children([con_mode, lcd, result]).spacing(5);
        let lcd_container = container(top)
            .width(Length::Fill)
            .style(move |_theme, _status| {
                container::Appearance {
                    background: Some(Background::Color(_theme.extended_palette().background.strong.color)),
                    ..Default::default()
                }
            })
            .padding(2);

        let w = Length::FillPortion(1);
        let h = Length::FillPortion(1);

        // The standard number buttons
        let b_one = ButtonBuilder::new("1", w, h).make();
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
        let b_pow = ButtonBuilder::new("^", Length::FillPortion(1), h).make();
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
        let b_equals = ButtonBuilder::new("=", Length::FillPortion(2), h).msg(Message::Evaluate).make();
        let b_clear = ButtonBuilder::new("AC", w, h)
            .msg(Message::Clear)
            .danger(true)
            .make();
        let b_left = ButtonBuilder::new("<-", w, h).msg(Message::MoveLeft).make();
        let b_right = ButtonBuilder::new("->", w, h).msg(Message::MoveRight).make();
        let b_back = ButtonBuilder::new("<del", w, h).msg(Message::BackSpace).make();
        let b_more = ButtonBuilder::new("more..", w, h).msg(Message::FuncPopup).make();

        let col_all = Column::with_children([
            lcd_container.height(Length::FillPortion(2)).into(),
            Row::with_children([
                Column::with_children([
                    Row::with_children([b_back, b_left, b_right, b_more, b_clear]).spacing(2).into(),
                    Row::with_children([b_sin, b_cos, b_tan, b_sqrt, b_abs]).spacing(2).into(),
                    Row::with_children([b_asin, b_acos, b_atan, b_ceil, b_floor]).spacing(2).into(),
                    Row::with_children([b_exp, b_ln, b_log, b_log2, b_fact]).spacing(2).into(),
                ]).spacing(2).into(),
            ]).spacing(2).height(Length::FillPortion(3)).into(),
            Row::with_children([
                Column::with_children([
                    Row::with_children([b_seven, b_eight, b_nine, b_lparen, b_rparen]).spacing(2).into(),
                    Row::with_children([b_four, b_five, b_six, b_mult, b_div]).spacing(2).into(),
                    Row::with_children([b_one, b_two, b_three, b_plus, b_minus]).spacing(2).into(),
                    Row::with_children([b_zero, b_dec, b_equals, b_pow]).spacing(2).into(),
                ]).spacing(2).into(),
            ]).spacing(2).height(Length::FillPortion(3)).into(),
        ]).spacing(2);

        container(col_all)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme, _status| {
                container::Appearance {
                    background: Some(Background::Color(_theme.extended_palette().background.weak.color)),
                    ..Default::default()
                }
            })
            .padding(10).into()
    }

    pub(crate) fn position(&self) -> (i32, i32) {
        (self.window_x, self.window_y)
    }

    pub(crate) fn size(&self) -> (u32, u32) {
        (self.window_width, self.window_height)
    }

}

/// A builder for making the button widgets.
/// Note that the functions all take ownership of self and then return self; this allows
/// us to avoid returning mutable references and so avoid ugly 'static life times.
/// # Examples
/// Build a button using the default message ```Message::Char(self.name.to_string()```
/// ```
/// let b_one = ButtonBuilder::new("1", w, h).make();
/// ```
/// Build a message specifying message and colors
/// ```
/// let b_clear = ButtonBuilder::new("AC", w, h)
///             .msg(Message::Clear)
///             .colors((Color::from_rgb8(0xf0, 0x24, 0x24), Color::from_rgb8(0xD0, 0x24, 0x24)))
///             .make();
/// ```
struct ButtonBuilder<'a> {
    name : &'a str,
    w : Length,
    h : Length,
    msg : Option<Message>,
    danger : bool,
}
impl <'a> ButtonBuilder<'a> {

    /// Get a new builder for a button with name, width and height specified
    ///
    /// # Arguments
    ///
    /// * `name`: The string to show on the button. This will also be used as the default
    ///           Message value if no Message is added to the builder
    /// * `w`: The width of the button
    /// * `h`: The height of the button
    ///
    /// returns: ButtonBuilder
    ///
    /// # Examples
    ///
    /// ```
    /// ButtonBuilder::new("1", w, h)
    /// ```
    fn new(name: &'a str, w: Length, h: Length) -> Self {
        Self {name, w, h , msg: None, danger: false}
    }

    /// Get a new builder for a button that provides a Func message.
    /// This will add a message of the form ```Message::Func(name.to_string()))```
    /// and saves repeating the name.
    ///
    /// # Arguments
    ///
    /// * `name`: The string to show on the button. This will also be used as the default
    ///           Message value if no Message is added to the builder
    /// * `w`: The width of the button
    /// * `h`: The height of the button
    ///
    /// returns: ButtonBuilder
    ///
    /// # Examples
    ///
    /// ```
    /// ButtonBuilder::for_func("ln", w, h)
    /// ```
    fn for_func(name: &'a str, w: Length, h: Length) -> Self {
        Self {name, w, h, msg: Some(Message::Func(name.to_string())), danger: false}
    }

    /// Add the message to be generated by the button. This will replace any default message.
    ///
    /// # Arguments
    ///
    /// * `msg`: A message variant
    ///
    /// returns: ButtonBuilder
    ///
    /// # Examples
    ///
    /// ```
    /// .msg(Message::Func("factorial".to_string()))
    /// ```
    fn msg(mut self, msg : Message) -> Self {
        self.msg = Some(msg);
        self
    }

    /// Specify the colors for a button
    ///
    /// # Arguments
    ///
    /// * `colors`: A tuple of two colors. The button  maker generates a gradient using these two colors
    ///
    /// returns: ButtonBuilder
    ///
    /// # Examples
    /// A red button
    /// ```
    /// .colors((Color::from_rgb8(0xf0, 0x24, 0x24), Color::from_rgb8(0xD0, 0x24, 0x24)))
    /// ```
    fn danger(mut self, danger: bool) -> Self {
        self.danger = danger;
        self
    }

    /// Make the button
    fn make(self) -> Element<'a, Message> {
        let container: Container<'_, Message, Theme, Renderer> = Container::new(self.name)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .height(Length::Fill)
            .width(Length::Fill);

        Button::new(container)
            .width(self.w)
            .height(self.h)
            .style(move |theme, status| {

                let color_active = if self.danger {
                    theme.extended_palette().danger.strong
                } else {
                    theme.extended_palette().secondary.strong
                };
                let color_hover = theme.extended_palette().secondary.base;
                let color_pressed = theme.extended_palette().secondary.weak;
                let style = get_style(status, color_active, color_hover, color_pressed);
                style
            })
            .on_press(self.msg.unwrap_or(Message::Char(self.name.to_string())))
            .into()
    }
}

fn get_style(status: Status, active: Pair, hover: Pair, pressed: Pair) -> Appearance {
    // make a gradient from the palette
    let c1 = lighten(active.color, 0.20);
    let c2 = darken(active.color, 0.05);

    let c3 = lighten(hover.color, 0.05);
    let c4 = darken(hover.color, 0.05);

    let c5 = lighten(pressed.color, 0.05);
    let c6 = darken(pressed.color, 0.05);

    match status {
        Status::Active => {
            let g = gradient::Linear::new(Radians::from(Degrees(150.0)))
                .add_stop(0.0, c1)
                .add_stop(1.0, c2);

            Appearance {
                background: Some(Background::from(g)),
                text_color: active.text,
                border: Border::default().with_width(Pixels::from(2)).with_color(Color::from_rgb8(0x20, 0x20, 0x20)),
                shadow: Shadow { color: Color::WHITE, offset: Vector::new(-2.0, -2.0), blur_radius: 2.0 },
            }
        }
        Status::Hovered => {
            let g = gradient::Linear::new(Radians::from(Degrees(150.0)))
                .add_stop(0.0, c3)
                .add_stop(1.0, c4);

            Appearance {
                background: Some(Background::from(g)),
                text_color: hover.text,
                border: Border::default().with_width(Pixels::from(2)).with_color(Color::BLACK),
                shadow: Default::default(),
            }
        }
        Status::Pressed => {
            let g = gradient::Linear::new(Radians::from(Degrees(150.0)))
                .add_stop(0.0, c5)
                .add_stop(1.0, c6);

            Appearance {
                background: Some(Background::from(g)),
                text_color: pressed.text,
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

fn darken(color: Color, amount: f32) -> Color {

    let srgb = Rgb::from(color);
    let mut hsl = Hsl::from_color(srgb);

    hsl.lightness = if hsl.lightness - amount < 0.0 {
        0.0
    } else {
        hsl.lightness - amount
    };

    Color::from(Rgb::from_color(hsl))
}

fn lighten(color: Color, amount: f32) -> Color {
    let srgb = Rgb::from(color);
    let mut hsl = Hsl::from_color(srgb);

    hsl.lightness = if hsl.lightness + amount > 1.0 {
        1.0
    } else {
        hsl.lightness + amount
    };

    Color::from(Rgb::from_color(hsl))
}

pub fn save_window_size(width: u32, height: u32) -> Result<(), String> {
    // Set the window state in `settings`
    let pref = crate::ui::preferences::manager();
    pref.put("window-width", width);
    pref.put("window-height", height);

    Ok(())
}