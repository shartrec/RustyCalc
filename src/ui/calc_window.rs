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

// This is the main ICED UI Application.

use iced::{Background, Border, Color, Degrees, Element, event, Event, gradient, Length, Padding, Pixels, Radians, Renderer, Shadow, Subscription, Task, Theme, Vector, window};
use iced::clipboard;
use iced::alignment::{Horizontal, Vertical};
use iced::theme::palette::Pair;
use iced::widget::{Button, button, Column, container, Container, horizontal_rule, Row, rule, text, Text, text_editor, tooltip};
use iced::widget::button::Status;
use iced::widget::text_editor::{Action, Content, Edit, Motion};
use iced::widget::tooltip::Position;
use log::warn;
use palette::{convert::FromColor, Hsl};
use palette::rgb::Rgb;

use crate::conversions::{try_convert, Unit};
use crate::evaluator::AngleMode;
use crate::ui;
use crate::ui::calculator::Calc;
use crate::ui::menu::build_menu_bar;
use crate::ui::messages::Message;

#[derive(Debug)]
pub(crate) struct CalcWindow {
    theme: Theme,
    content: Content,
    result: Option<Result<f64, String>>,
    calc: Calc,
    is_converting: bool,
    convert_from: Option<&'static Unit>,
    convert_to: Option<&'static Unit>,
    window_width: f32,
    window_height: f32,
    window_x: f32,
    window_y: f32,
}

impl Default for CalcWindow {
    fn default() -> Self {
        let mut calc = Calc::default();
        // Load the angle mode from preferences
        let pref = crate::ui::preferences::manager();
        if let Some(am) = pref.get::<String>(crate::ui::preferences::ANGLE_MODE) {
            calc.set_angle_mode(AngleMode::get_from_name(am.as_str()));
        }
        let theme = theme_by_name(pref.get::<String>(ui::preferences::THEME)).clone();

        Self {
            theme: theme,
            content: Default::default(),
            result: None,
            calc: calc,
            is_converting: false,
            convert_from: None,
            convert_to: None,
            window_width: 0.0,
            window_height: 0.0,
            window_x: 0.0,
            window_y: 0.0,
        }
    }
}

impl CalcWindow {

    pub fn title(&self) -> String {
        "Rusty Calculator".to_string()
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Char(s) => {
                for c in s.chars() {
                    self.content.perform(Action::Edit(Edit::Insert(c)));
                }
                Task::none()
            }
            Message::Constant(s) => {
                for c in s.chars() {
                    self.content.perform(Action::Edit(Edit::Insert(c)));
                }
                Task::none()
            }
            Message::Copy(v) => {
                clipboard::write(v.to_string())
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
                    Task::none()
                } else {  //otherwise insert the function and move cursor between the parentheses
                    for c in s.chars() {
                        self.content.perform(Action::Edit(Edit::Insert(c)));
                    }
                    self.content.perform(Action::Edit(Edit::Insert('(')));
                    self.content.perform(Action::Edit(Edit::Insert(')')));
                    Task::perform(async {}, |_| Message::MoveLeft)
                }
            }
            Message::EditorAction(action) => {
                match action {
                    Action::Edit(Edit::Enter) => {
                        self.result = Some(self.calc.evaluate(&self.content.text().trim()));
                        Task::perform(async {}, |_| Message::MoveEnd)
                    }
                    _ => {
                        self.content.perform(action);
                        Task::none()
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
                Task::none()
            }

            Message::Evaluate => {
                self.result = Some(self.calc.evaluate(&self.content.text().trim()));
                Task::none()
            }
            Message::Clear => {
                self.content.perform(Action::Move(Motion::DocumentStart));
                self.content.perform(Action::Select(Motion::DocumentEnd));
                self.content.perform(Action::Edit(Edit::Delete));
                self.is_converting = false;
                self.convert_from = None;
                self.convert_to = None;
                self.result = None;
                Task::none()
            }
            Message::MoveLeft => {
                self.content.perform(Action::Move(Motion::Left));
                Task::none()
            }
            Message::MoveRight => {
                self.content.perform(Action::Move(Motion::Right));
                Task::none()
            }
            Message::MoveEnd => {
                self.content.perform(Action::Move(Motion::DocumentEnd));
                Task::none()
            }
            Message::BackSpace => {
                self.content.perform(Action::Edit(Edit::Backspace));
                Task::none()
            }
            Message::ConvertPerform(from_unit, to_unit) => {
                self.is_converting = true;
                self.convert_from = Some(from_unit);
                self.convert_to = Some(to_unit);
                if self.content.text().trim().len() > 0 {
                    Task::perform(async {}, |_| Message::Evaluate)
                } else {
                    Task::none()
                }
            }
            Message::ThemeChanged(t) => {
                self.theme = t;
                let pref = ui::preferences::manager();
                pref.put(ui::preferences::THEME, format!("{}", &self.theme));
                Task::none()
            }
            Message::WindowResized(w, h) => {
                self.window_width = w.clone();
                self.window_height = h.clone();
                Task::none()
            }
            Message::WindowMoved(x, y) => {
                self.window_x = x.clone();
                self.window_y = y.clone();
                Task::none()
            }
            Message::WindowClosed() => {
                let _ = save_window_size(self.window_width, self.window_height);
                Task::none()
            }
            Message::ToggleMode => {
                self.calc.set_angle_mode(match self.calc.angle_mode() {
                    AngleMode::Degrees => AngleMode::Radians,
                    AngleMode::Radians => AngleMode::Gradians,
                    AngleMode::Gradians => AngleMode::Degrees,
                });
                let pref = crate::ui::preferences::manager();
                pref.put(crate::ui::preferences::ANGLE_MODE, self.calc.angle_mode());
                Task::none()
            }
            Message::Null => Task::none()
        }
    }
    pub(crate) fn view<'a>(&'a self) -> Element<Message> {
        let lcd = text_editor(&self.content)
            .height(Length::Fill)
            .style(|theme: &Theme, status| {
                text_editor::Style {
                    background: Background::Color(Color::TRANSPARENT),
                    border: Border::default().width(Pixels::from(1))
                        .color(theme.extended_palette().background.base.text),
                    .. text_editor::default(theme, status)
                }
            })
            .on_action(|action| {
                Message::EditorAction(action)
            })
            .into();

        let r64 = &self.result;
        let result: Element<Message> = match r64 {
                Some(r) => {
                    match r {
                        Ok(v) => {
                            wrap_with_copy(text(Self::format_result(v)), v.clone())
                        }
                        Err(e) => text(e.clone()).into()
                    }
                }
                None => text("".to_string()).into(),
            };


        let mode: Element<Message> = text(self.calc.angle_mode().to_string())
            .style(|theme: &Theme| {
                text::Style {
                    color: Some(theme.extended_palette().background.base.text),
                }
            })
            .height(Length::Shrink)
            .into();

        let con_mode = Container::new(mode)
            .width(Length::Fill)
            .align_x(Horizontal::Right)
            .clip(false)
            .into();

        let con_result = Container::new(result)
            .width(Length::Fill)
            .align_x(Horizontal::Right)
            .clip(false)
            .into();

        let mb = build_menu_bar().into();

        let menu_row = Row::with_children([mb, con_mode]).into();

        let sp = 2;
        let top =
            if !self.is_converting {
                Column::with_children([menu_row, lcd, con_result]).spacing(sp)
            } else {

                let conv_from = if let Some(unit_from) = &self.convert_from {
                    text(unit_from.name)

                } else {
                    warn!("Converting units, but no 'from' unit set");
                    text("")
                }.align_x(Horizontal::Left).into();

                let conv_to = if let Some(unit_to) = &self.convert_to {
                    text(unit_to.name)

                } else {
                    warn!("Converting units, but no 'to' unit set");
                    text("")
                }.align_x(Horizontal::Left)
                    .into();
                let converted_result = match &self.result {
                    Some(r) => {
                        match r {
                            Ok(v) => {
                                let cv = try_convert(v, &self.convert_from, &self.convert_to);
                                wrap_with_copy(text(Self::format_result(&cv)), cv)
                            }
                            Err(e) => text(e.clone()).into()
                        }
                    }
                    None => text(String::from("")).into()
                };
                let con_conv_result = Container::new(converted_result)
                    .width(Length::Fill)
                    .align_x(Horizontal::Right)
                    .clip(false)
                    .into();

                let r1 = Row::with_children([conv_from, con_result]).into();
                let r2 = Row::with_children([conv_to, con_conv_result]).into();

                let rule1:Element<Message> = horizontal_rule(1)
                    .style(|theme| {
                        iced::widget::rule::Style {
                                color: Color::from_rgb8(0x35, 0x3f, 0x3f),
                                .. rule::default(theme)
                            }
                        })
                    .into();
                Column::with_children([menu_row, lcd, r1, rule1, r2]).spacing(sp)
            };
        let lcd_container = container(top)
            .width(Length::Fill)
            .style(move |theme| {
                container::Style {
                    background: Some(Background::Color(theme.extended_palette().background.strong.color)),
                    border: Border::default().width(Pixels::from(1)).color(Color::from_rgb8(0x7f, 0x7f, 0x7f)),
                    ..Default::default()
                }
            })
            .padding(2);

        // The standard number buttons
        let b_one = ButtonBuilder::new("1").make();
        let b_two = ButtonBuilder::new("2").make();
        let b_three = ButtonBuilder::new("3").make();
        let b_four = ButtonBuilder::new("4").make();
        let b_five = ButtonBuilder::new("5").make();
        let b_six = ButtonBuilder::new("6").make();
        let b_seven = ButtonBuilder::new("7").make();
        let b_eight = ButtonBuilder::new("8").make();
        let b_nine = ButtonBuilder::new("9").make();
        let b_zero = ButtonBuilder::new("0").make();
        let b_dec = ButtonBuilder::new(".").make();
        // Basic operations
        let b_plus = ButtonBuilder::new("+").make();
        let b_minus = ButtonBuilder::new("-").make();
        let b_mult = ButtonBuilder::new("x").msg(Message::Char("*".to_string())).make();
        let b_div = ButtonBuilder::new("/").make();
        let b_pow = ButtonBuilder::new("^").make();
        let b_lparen = ButtonBuilder::new("(").msg(Message::Func("".to_string())).make();
        let b_rparen = ButtonBuilder::new(")").make();
        // Functions
        let b_sin = ButtonBuilder::for_func("sin").make();
        let b_cos = ButtonBuilder::for_func("cos").make();
        let b_tan = ButtonBuilder::for_func("tan").make();
        let b_asin = ButtonBuilder::for_func("asin").make();
        let b_acos = ButtonBuilder::for_func("acos").make();
        let b_atan = ButtonBuilder::for_func("atan").make();
        let b_exp = ButtonBuilder::for_func("exp").make();
        let b_ln = ButtonBuilder::for_func("ln").make();
        let b_log = ButtonBuilder::for_func("log").make();
        let b_log2 = ButtonBuilder::for_func("log2").make();
        let b_sqrt = ButtonBuilder::new("√").msg(Message::Func("sqrt".to_string())).make();
        let b_abs = ButtonBuilder::for_func("abs").make();
        let b_ceil = ButtonBuilder::for_func("ceil").make();
        let b_floor = ButtonBuilder::for_func("floor").make();
        let b_fact = ButtonBuilder::for_func("!").msg(Message::Func("factorial".to_string())).make();
        // Command buttons
        let b_equals = ButtonBuilder::new("=").msg(Message::Evaluate).span(2).make();
        let b_clear = ButtonBuilder::new("AC")
            .msg(Message::Clear)
            .danger(true)
            .make();
        let b_left = ButtonBuilder::new("<-").msg(Message::MoveLeft).make();
        let b_right = ButtonBuilder::new("->").msg(Message::MoveRight).make();
        let b_back = ButtonBuilder::new("<del").msg(Message::BackSpace).make();
        let b_more = ButtonBuilder::new("DRG").msg(Message::ToggleMode).make();

        let row_height = Length::FillPortion(1);
        let col_all = Column::with_children([
            lcd_container.height(Length::FillPortion(3)).into(),
            Row::with_children([
                Column::with_children([
                    Row::with_children([b_back, b_left, b_right, b_more, b_clear]).spacing(sp).height(row_height).into(),
                    Row::with_children([b_sin, b_cos, b_tan, b_sqrt, b_abs]).spacing(sp).height(row_height).into(),
                    Row::with_children([b_asin, b_acos, b_atan, b_ceil, b_floor]).spacing(sp).height(row_height).into(),
                    Row::with_children([b_exp, b_ln, b_log, b_log2, b_fact]).spacing(sp).height(row_height).into(),
                    Row::with_children([b_seven, b_eight, b_nine, b_lparen, b_rparen]).spacing(sp).height(row_height).into(),
                    Row::with_children([b_four, b_five, b_six, b_mult, b_div]).spacing(sp).height(row_height).into(),
                    Row::with_children([b_one, b_two, b_three, b_plus, b_minus]).spacing(sp).height(row_height).into(),
                    Row::with_children([b_zero, b_dec, b_equals, b_pow]).spacing(sp).height(row_height).into(),
                ]).spacing(sp).into(),
            ]).spacing(sp).height(Length::FillPortion(6)).into(),
        ]).spacing(sp);

        container(col_all)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme| {
                container::Style {
                    background: Some(Background::Color(_theme.extended_palette().background.weak.color)),
                    ..Default::default()
                }
            })
            .padding(5).into()
    }

    fn format_result(v: &f64) -> String {
        if v.abs() < 0.001 || v.abs() > 10000000.0 {
            format!("= {:+e}", v)
        } else {
            let formatted = format!("= {0:.1$}", v, 10);
            formatted.trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }

    pub(crate) fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _id| {
            match event {
                Event::Window(window::Event::Resized(size)) => {
                    Some(Message::WindowResized(size.width, size.height))
                }
                Event::Window(window::Event::Moved ( p)) => {
                    Some(Message::WindowMoved(p.x, p.y))
                }
                Event::Window(window::Event::Closed {}) => {
                    Some(Message::WindowClosed())
                }
                _ => None
            }
        })
    }

    pub(crate) fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn wrap_with_copy(text: Text, value: f64) -> Element<Message> {
    let b= Button::new(text)
        .style(|theme: &Theme, _status| {
            button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: theme.extended_palette().background.base.text,
                .. button::Style::default()
            }
        })
        .padding(Padding::from(0))
        .on_press(Message::Copy(value))
        .height(Length::Shrink);

    tooltip(b, "Click to copy", Position::Left)
        .style(|theme| -> container::Style {
            container::Style{
                text_color: Some(theme.extended_palette().primary.weak.text),
                background: Some(Background::from(theme.extended_palette().primary.weak.color)),
                border: Default::default(),
                shadow: Default::default(),
            }
        })
        .into()
}

/// A builder for making the button widgets.
/// Note that the functions all take ownership of self and then return self; this allows
/// us to avoid returning mutable references and so avoid ugly 'static life times.
/// # Examples
/// Build a button using the default message ```Message::Char(self.name.to_string()```
/// ```
/// let b_one = ButtonBuilder::new("1").make();
/// ```
/// Build a message specifying message and colors
/// ```
/// let b_clear = ButtonBuilder::new("AC")
///             .msg(Message::Clear)
///             .colors((Color::from_rgb8(0xf0, 0x24, 0x24), Color::from_rgb8(0xD0, 0x24, 0x24)))
///             .make();
/// ```
struct ButtonBuilder<'a> {
    name : &'a str,
    msg : Option<Message>,
    danger : bool,
    span : u16,
}
impl <'a> ButtonBuilder<'a> {

    /// Get a new builder for a button with name, width and height specified
    ///
    /// # Arguments
    ///
    /// * `name`: The string to show on the button. This will also be used as the default
    ///           Message value if no Message is added to the builder
    ///
    /// returns: ButtonBuilder
    ///
    /// # Examples
    ///
    /// ```
    /// ButtonBuilder::new("1")
    /// ```
    fn new(name: &'a str) -> Self {
        Self {name, msg: None, danger: false, span: 1}
    }

    /// Get a new builder for a button that provides a Func message.
    /// This will add a message of the form ```Message::Func(name.to_string()))```
    /// and saves repeating the name.
    ///
    /// # Arguments
    ///
    /// * `name`: The string to show on the button. This will also be used as the default
    ///           Message value if no Message is added to the builder
    ///
    /// returns: ButtonBuilder
    ///
    /// # Examples
    ///
    /// ```
    /// ButtonBuilder::for_func("ln")
    /// ```
    fn for_func(name: &'a str) -> Self {
        Self {name, msg: Some(Message::Func(name.to_string())), danger: false, span: 1}
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

    /// Specify the relative width of a button
    ///
    /// # Arguments
    ///
    /// * `span`: Defaults to 1,
    ///
    /// returns: ButtonBuilder
    ///
    /// # Examples
    /// A button that spans 2 button widths
    /// ```
    /// .span(2)
    /// ```
    fn span(mut self, span: u16) -> Self {
        self.span = span;
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
            .width(Length::FillPortion(self.span))
            .height(Length::Fill)
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

fn get_style(status: Status, active: Pair, hover: Pair, pressed: Pair) -> button::Style {
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

            button::Style {
                background: Some(Background::from(g)),
                text_color: active.text,
                border: Border::default().width(Pixels::from(2)).color(Color::from_rgb8(0x20, 0x20, 0x20)),
                shadow: Shadow { color: Color::WHITE, offset: Vector::new(-2.0, -2.0), blur_radius: 2.0 },
            }
        }
        Status::Hovered => {
            let g = gradient::Linear::new(Radians::from(Degrees(150.0)))
                .add_stop(0.0, c3)
                .add_stop(1.0, c4);

            button::Style {
                background: Some(Background::from(g)),
                text_color: hover.text,
                border: Border::default().width(Pixels::from(2)).color(Color::BLACK),
                shadow: Default::default(),
            }
        }
        Status::Pressed => {
            let g = gradient::Linear::new(Radians::from(Degrees(150.0)))
                .add_stop(0.0, c5)
                .add_stop(1.0, c6);

            button::Style {
                background: Some(Background::from(g)),
                text_color: pressed.text,
                border: Border::default().width(Pixels::from(2)).color(Color::BLACK),
                shadow: Default::default(),
            }
        }
        Status::Disabled => {
            button::Style {
                background: None,
                text_color: Color::BLACK,
                border: Border::default().width(Pixels::from(2)).color(Color::BLACK),
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

pub fn save_window_size(width: f32, height: f32) -> Result<(), String> {
    // Set the window state in `settings`
    let pref = crate::ui::preferences::manager();
    pref.put("window-width", width);
    pref.put("window-height", height);

    Ok(())
}

fn theme_by_name(name: Option<String>) -> &'static Theme {
    if let Some(name) = name {
        for t in Theme::ALL.iter() {
            if format!("{}", t) == name {
                return &t
            }
        }
    }
    &ui::lcd_theme()
}
