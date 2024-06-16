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

use std::ops::Deref;

use iced::{Background, Command, Element, Length, Theme, window};
use iced::widget::{Column, container, horizontal_rule, pick_list};
use iced::widget::pick_list::{Appearance, DefaultStyle, Status, Style};
use iced::window::Id;
use crate::ui;

use crate::ui::messages::Message;

#[derive(Clone)]
pub struct ConstantDef {
    name: String,
    description: String,
}

impl ToString for ConstantDef {
    fn to_string(&self) -> String {
        format!("{} - {}", self.name, self.description)
    }
}

impl PartialEq for ConstantDef {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Default)]
pub(super) struct FuncPopup {


}

impl FuncPopup {

    pub fn title(&self) -> String {
        "Functions".to_string()
    }
    pub(super) fn update(&self, id: &Id, message: Message) -> Command<Message> {
        match message {
            Message::CloseAndSend(id_msg, message) if id_msg == *id => {
                Command::batch([
                    window::close::<Message>(id.clone()),
                    Command::perform(async {}, move |_| message.deref().clone()),
                ])
            }
            _ => Command::none(),
        }
    }
    pub(super) fn view(&self, id: Id) -> Element<Message> {

        let functions = vec![
                    "asin".to_string(), "acos".to_string(), "atan".to_string(),
                    "cosec".to_string(), "sec".to_string(), "cot".to_string(),
                    "acosec".to_string(), "asec".to_string(), "acot".to_string(),
                    "sinh".to_string(), "cosh".to_string(), "tanh".to_string(),
                    "asinh".to_string(), "acosh".to_string(), "atanh".to_string(),
                    ];

        let constants = vec![
            ConstantDef{name: "π".to_string(), description: "Pi".to_string()},
            ConstantDef{name: "e".to_string(), description: "Euler's number".to_string()},
            ConstantDef{name: "Φ".to_string(), description: "Golden Ratio".to_string()},
            ConstantDef{name: "C".to_string(), description: "Speed of light".to_string()},
            ConstantDef{name: "ℎ".to_string(), description: "Plank's constant".to_string()},
            ConstantDef{name: "G".to_string(), description: "Gravitational Const".to_string()},
        ];

        let conversions = vec!["Miles -> Kilometres".to_string(), "Lbs -> Kgs".to_string(), "X -Y".to_string()];

        let col: Element<Message> = Column::with_children([
            pick_list(functions, None::<String>, move |selected| {
                    Message::CloseAndSend(id, Box::new(Message::Func(selected)))
                })
                .placeholder("functions -- select")
                .width(Length::Fill)
                .style(Style {
                    field: Box::new(move |theme, status| { Self::get_appearance(theme, status)}),
                    ..Theme::default_style()
                })
                .into(),
            pick_list(constants, None::<ConstantDef>, move |selected| {
                    Message::CloseAndSend(id, Box::new(Message::Constant(selected.name)))
                })
                .placeholder("constants -- select")
                .width(Length::Fill)
                .style(Style {
                    field: Box::new(move |theme, status| { Self::get_appearance(theme, status)}),
                    ..Theme::default_style()
                })
                .into(),
            pick_list(conversions, None::<String>, move |selected| {
                    Message::CloseAndSend(id, Box::new(Message::Func(selected)))
                })
                .placeholder("conversions -- select")
                .width(Length::Fill)
                .style(Style {
                    field: Box::new(move |theme, status| { Self::get_appearance(theme, status)}),
                    ..Theme::default_style()
                })
                .into(),

            horizontal_rule(2).into(),

            pick_list(Self::get_all_themes(), None::<Theme>, move |selected| {
                    Message::CloseAndSend(id, Box::new(Message::ThemeChanged(selected)))
                })
                .placeholder("Choose your theme")
                .style(Style {
                    field: Box::new(move |theme, status| { Self::get_appearance(theme, status)}),
                    ..Theme::default_style()
                })
                .width(Length::Fill).into(),

            ]).spacing(4).into();

        container(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme, _status| {
                container::Appearance {
                    background: Some(Background::Color(_theme.extended_palette().background.weak.color)),
                    ..Default::default()
                }
            })
            .padding(4).into()
    }

    fn get_all_themes() -> Vec<Theme> {
        let mut themes: Vec<Theme> = Vec::new();

        themes.insert(0, ui::lcd_theme().clone());
        for t in Theme::ALL {
            themes.push(t.clone());
        }

        themes
    }
    fn get_appearance(theme: &Theme, status: Status) -> Appearance {
            Appearance {
                background: Background::from(theme.extended_palette().secondary.strong.color),
                text_color: theme.extended_palette().secondary.strong.text,
                placeholder_color: theme.extended_palette().secondary.base.text,
                .. pick_list::default(theme, status)
            }
    }
}