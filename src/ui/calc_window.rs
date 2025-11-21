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

use floem::IntoView;
use floem::prelude::{Color, Decorators};
use floem::style::Background;
use floem::views::editor::text::default_light_theme;
use floem::window::Theme;
use log::warn;
use palette::{convert::FromColor, Hsl};
use palette::rgb::Rgb;

use crate::conversions::{try_convert, Unit};
use crate::evaluator::AngleMode;
use crate::ui;
use crate::ui::calculator::Calc;
use crate::ui::menu::build_menu_bar;
// use crate::ui::menu::build_menu_bar;

#[derive(Debug)]
pub struct CalcWindow {
    theme: Theme,
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
        let theme = Theme::Light;

        Self {
            theme: theme,
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

    pub(crate) fn view(&self) -> impl IntoView {
        // We first have a single column with the menu bar and the calculator view
        (
            self.build_menu_bar(),
            self.build_display(),
            self.build_calculator_buttons()
        )
            .style(|s| s.flex_col().gap(2).border(2.0).border_color(Color::BLACK).padding(4.0))

    }

    fn build_menu_bar(&self) -> impl IntoView {
        build_menu_bar()
    }

    fn build_display(&self) -> impl IntoView {
        "display placeholder"
    }
    fn build_calculator_buttons(&self) -> impl IntoView {
        "buttons placeholder"
    }

    fn format_result(v: &f64) -> String {
        if v.abs() < 0.001 || v.abs() > 10000000.0 {
            format!("= {:+e}", v)
        } else {
            let formatted = format!("= {0:.1$}", v, 10);
            formatted.trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }

    //     pub fn subscription(&self) -> Subscription<Message> {
    //         event::listen_with(|event, _status, _id| {
    //             match event {
    //                 Event::Window(window::Event::Resized(size)) => {
    //                     Some(Message::WindowResized(size.width, size.height))
    //                 }
    //                 Event::Window(window::Event::Moved ( p)) => {
    //                     Some(Message::WindowMoved(p.x, p.y))
    //                 }
    //                 Event::Window(window::Event::Closed {}) => {
    //                     Some(Message::WindowClosed())
    //                 }
    //                 // Capture key releases and forward character keys as Message::Char
    //                 Event::Keyboard(k_event) => {
    //                     // iced 0.13 represents keyboard events with a Key enum.
    //                     // We forward only character keys on release.
    //                     #[allow(unused_imports)]
    //                     use iced::keyboard;
    //                     match k_event {
    //                         keyboard::Event::KeyPressed { text, .. } => {
    //                             match text {
    //                                 Some(t) => {
    //                                     println!("KeyPressed text={}", t);
    //                                     Some(Message::Char(t.as_str().to_string()))
    //                                 }
    //                                 _ => None,
    //                             }
    //                         }
    //                         _ => None
    //                     }
    //                 }
    //                 _ => None
    //             }
    //         })
    //     }
    //
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

// fn wrap_with_copy(text: Text, value: f64) -> Element<Message> {
    //     let b= Button::new(text)
    //         .style(|theme: &Theme, _status| {
    //             button::Style {
    //                 background: Some(Background::Color(Color::TRANSPARENT)),
    //                 text_color: theme.extended_palette().background.base.text,
    //                 .. button::Style::default()
    //             }
    //         })
    //         .padding(Padding::from(0))
    //         .on_press(Message::Copy(value))
    //         .height(Length::Shrink);
    //
    //     tooltip(b, "Click to copy", Position::Left)
    //         .style(|theme| -> container::Style {
    //             container::Style{
    //                 text_color: Some(theme.extended_palette().primary.weak.text),
    //                 background: Some(Background::from(theme.extended_palette().primary.weak.color)),
    //                 border: Default::default(),
    //                 shadow: Default::default(),
    //             }
    //         })
    //         .into()
    // }

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
        name: &'a str,
        msg: Option<String>,
        danger: bool,
        span: u16,
    }
    impl<'a> ButtonBuilder<'a> {
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
            Self { name, msg: None, danger: false, span: 1 }
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
        // fn for_func(name: &'a str) -> Self {
        //     Self {name, msg: Some(Message::Func(name.to_string())), danger: false, span: 1}
        // }

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
        // fn msg(mut self, msg : Message) -> Self {
        //     self.msg = Some(msg);
        //     self
        // }

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

        // Make the button
        //     fn make(self) -> Element<'a, Message> {
        //         let container: Container<'_, Message, Theme, Renderer> = Container::new(self.name)
        //             .align_x(Horizontal::Center)
        //             .align_y(Vertical::Center)
        //             .height(Length::Fill)
        //             .width(Length::Fill);
        //
        //         Button::new(container)
        //             .width(Length::FillPortion(self.span))
        //             .height(Length::Fill)
        //             .style(move |theme, status| {
        //
        //                 let color_active = if self.danger {
        //                     theme.extended_palette().danger.strong
        //                 } else {
        //                     theme.extended_palette().secondary.strong
        //                 };
        //                 let color_hover = theme.extended_palette().secondary.base;
        //                 let color_pressed = theme.extended_palette().secondary.weak;
        //                 let style = get_style(status, color_active, color_hover, color_pressed);
        //                 style
        //             })
        //             .on_press(self.msg.unwrap_or(Message::Char(self.name.to_string())))
        //             .into()
        //     }
    }
    //
    // fn get_style(status: Status, active: Pair, hover: Pair, pressed: Pair) -> button::Style {
    //     // make a gradient from the palette
    //     let c1 = lighten(active.color, 0.20);
    //     let c2 = darken(active.color, 0.05);
    //
    //     let c3 = lighten(hover.color, 0.05);
    //     let c4 = darken(hover.color, 0.05);
    //
    //     let c5 = lighten(pressed.color, 0.05);
    //     let c6 = darken(pressed.color, 0.05);
    //
    //     match status {
    //         Status::Active => {
    //             let g = gradient::Linear::new(Radians::from(Degrees(150.0)))
    //                 .add_stop(0.0, c1)
    //                 .add_stop(1.0, c2);
    //
    //             button::Style {
    //                 background: Some(Background::from(g)),
    //                 text_color: active.text,
    //                 border: Border::default().width(Pixels::from(2)).color(Color::from_rgb8(0x20, 0x20, 0x20)),
    //                 shadow: Shadow { color: Color::WHITE, offset: Vector::new(-2.0, -2.0), blur_radius: 2.0 },
    //             }
    //         }
    //         Status::Hovered => {
    //             let g = gradient::Linear::new(Radians::from(Degrees(150.0)))
    //                 .add_stop(0.0, c3)
    //                 .add_stop(1.0, c4);
    //
    //             button::Style {
    //                 background: Some(Background::from(g)),
    //                 text_color: hover.text,
    //                 border: Border::default().width(Pixels::from(2)).color(Color::BLACK),
    //                 shadow: Default::default(),
    //             }
    //         }
    //         Status::Pressed => {
    //             let g = gradient::Linear::new(Radians::from(Degrees(150.0)))
    //                 .add_stop(0.0, c5)
    //                 .add_stop(1.0, c6);
    //
    //             button::Style {
    //                 background: Some(Background::from(g)),
    //                 text_color: pressed.text,
    //                 border: Border::default().width(Pixels::from(2)).color(Color::BLACK),
    //                 shadow: Default::default(),
    //             }
    //         }
    //         Status::Disabled => {
    //             button::Style {
    //                 background: None,
    //                 text_color: Color::BLACK,
    //                 border: Border::default().width(Pixels::from(2)).color(Color::BLACK),
    //                 shadow: Default::default(),
    //             }
    //         }
    //     }
    // }

fn darken(color: Color, amount: f32) -> Color {

    let srgb: Rgb<f32> = Rgb::new(color.r, color.g, color.b).into_format();
    let mut hsl = Hsl::from_color(srgb);

    hsl.lightness = if hsl.lightness - amount < 0.0 {
        0.0
    } else {
        hsl.lightness - amount
    };

    let rgb = Rgb::from_color(hsl);
    Color::rgb(rgb.red as f64, rgb.green as f64, rgb.blue as f64)
}

fn lighten(color: Color, amount: f32) -> Color {
    let srgb: Rgb<f32> = Rgb::new(color.r, color.g, color.b).into_format();
    let mut hsl = Hsl::from_color(srgb);

    hsl.lightness = if hsl.lightness + amount > 1.0 {
        1.0
    } else {
        hsl.lightness + amount
    };

    let rgb = Rgb::from_color(hsl);
    Color::rgb(rgb.red as f64, rgb.green as f64, rgb.blue as f64)
}

pub fn save_window_size(width: f32, height: f32) -> Result<(), String> {
    // Set the window state in `settings`
    let pref = crate::ui::preferences::manager();
    pref.put("window-width", width);
    pref.put("window-height", height);

    Ok(())
}

// fn theme_by_name(name: Option<String>) -> &'static Theme {
//     if let Some(name) = name {
//         for t in Theme::ALL.iter() {
//             if format!("{}", t) == name {
//                 return &t
//             }
//         }
//     }
//     &ui::lcd_theme()
// }
