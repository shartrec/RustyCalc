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

use iced::{Background, Color, Element, gradient, Length, Radians};
use iced::widget::{Column, container, pick_list};

use crate::ui::messages::Message;

#[derive(Debug, Default)]
pub(super) struct FuncPopup {


}

impl FuncPopup {

    pub fn title(&self) -> String {
        "Functions".to_string()
    }

    pub(super) fn view(&self) -> Element<Message> {

        let functions = vec!["sinh".to_string(), "cosh".to_string(), "tanh".to_string()];
        let constants = vec!["Pi".to_string(), "C (speed of light)".to_string(), "Avagadro's No".to_string()];
        let conversions = vec!["Miles -> Kilometres".to_string(), "Lbs -> Kgs".to_string(), "X -Y".to_string()];

        let col: Element<Message> = Column::with_children([
            pick_list(functions, None::<String>, |selected| Message::Func(selected))
                .placeholder("Functions")
                .width(Length::Fill)
                .into(),
            pick_list(constants, None::<String>, |selected| Message::Func(selected))
                .placeholder("Constants")
                .width(Length::Fill)
                .into(),
            pick_list(conversions, None::<String>, |selected| Message::Func(selected))
                .placeholder("Conversions")
                .width(Length::Fill)
                .into(),

            ]).spacing(10).into();

        container(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme, _status| {
                let gradient = gradient::Linear::new(Radians(135.0))
                    .add_stop(0.0, Color::from_rgb8(0x00, 0x00, 0x00))
                    .add_stop(0.25, Color::from_rgb8(0x14, 0x14, 0x14))
                    .add_stop(0.50, Color::from_rgb8(0x28, 0x28, 0x28))
                    .add_stop(0.75, Color::from_rgb8(0x3c, 0x3c, 0x3c))
                    .add_stop(1.0, Color::from_rgb8(0x50, 0x50, 0x50))
                    .into();

                container::Appearance {
                    background: Some(Background::Gradient(gradient)),
                    ..Default::default()
                }
            })
            .padding(10).into()
    }

}