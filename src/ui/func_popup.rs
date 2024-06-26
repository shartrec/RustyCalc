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

use std::cell::RefCell;
use std::ops::Deref;
use iced::{Background, Command, Element, Length, Renderer, Theme, window};

use iced::widget::{Column, container, horizontal_rule, pick_list};
use iced::widget::pick_list::{Appearance, DefaultStyle, Status, Style};
use iced::widget::text::Shaping;
use iced::window::Id;
use log::error;
use strum::IntoEnumIterator;

use crate::conversions::{ConversionDirection, Dimension, Unit};
use crate::evaluator::constants::{Pi, Euler, Phi, C, Planck, G};
use crate::{conversions, history, ui};

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

#[derive(Clone)]
pub struct HistoryDef {
    expr: String,
    value: f64,
}

impl ToString for HistoryDef {
    fn to_string(&self) -> String {
        format!("{} = {}", self.expr, self.value)
    }
}

impl PartialEq for HistoryDef {
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr
    }
}

#[derive(Default)]
pub(super) struct FuncPopup {
    show_convert: RefCell<bool>,
    conversion_dimension: RefCell<Option<Dimension>>,
    conversion_direction: RefCell<Option<ConversionDirection>>,
    conversion_from: RefCell<Option<Unit>>,
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
            Message::ConvertDimension(dimension) => {
                self.conversion_dimension.replace(Some(dimension));
                self.conversion_direction.replace(Some(ConversionDirection::From));
                self.show_convert.replace(true);
                Command::none()
            }
            Message::Convert(unit, direction) => {
                match direction {
                    ConversionDirection::From => {
                        self.conversion_from.replace(Some(unit));
                        self.conversion_direction.replace(Some(ConversionDirection::To));
                        self.show_convert.replace(true);
                        Command::none()
                    }
                    ConversionDirection::To => {
                        let unit_from = self.conversion_from.take().unwrap().clone();
                        let unit_to = unit.clone();
                        self.show_convert.replace(false);

                        Command::batch([
                            window::close::<Message>(id.clone()),
                            Command::perform(async {}, move |_| {
                                Message::ConvertPerform(unit_from, unit_to)
                            })
                        ])
                    }
                }


            }
            _ => Command::none(),
        }
    }
    pub(super) fn view(&self, id: &Id) -> Element<Message> {

        let id = id.clone();
        let col: Element<Message> = Column::with_children([
                Self::themes(id),
                horizontal_rule(2).into(),
                Self::functions(id),
                Self::constants(id),
                if *self.show_convert.borrow() {
                    Self::conversion_unit(self.conversion_dimension.borrow().deref(), self.conversion_direction.borrow().deref())
                } else {
                    Self::conversion_dimension()
                },
                Self::history(id),
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

    fn themes(id: Id) -> Element<'static, Message> {

        pick_list(Self::get_all_themes(), None::<Theme>, move |selected| {
            Message::CloseAndSend(id, Box::new(Message::ThemeChanged(selected)))
        })
            .placeholder("Choose your theme")
            .style(Style {
                field: Box::new(move |theme, status| { Self::get_appearance(theme, status) }),
                ..Theme::default_style()
            })
            .width(Length::Fill).into()
    }

    fn functions(id: Id) -> Element<'static, Message, Theme, Renderer> {

        let functions = vec![
            "asin".to_string(), "acos".to_string(), "atan".to_string(),
            "cosec".to_string(), "sec".to_string(), "cot".to_string(),
            "acosec".to_string(), "asec".to_string(), "acot".to_string(),
            "sinh".to_string(), "cosh".to_string(), "tanh".to_string(),
            "asinh".to_string(), "acosh".to_string(), "atanh".to_string(),
        ];

        pick_list(functions, None::<String>, move |selected| {
            Message::CloseAndSend(id, Box::new(Message::Func(selected)))
        })
            .placeholder("functions -- select")
            .width(Length::Fill)
            .text_shaping(Shaping::Advanced)
            .style(Style {
                field: Box::new(move |theme, status| { Self::get_appearance(theme, status) }),
                ..Theme::default_style()
            })
            .into()
    }

    fn constants(id: Id) -> Element<'static, Message> {

        let constants = vec![
            ConstantDef{name: Pi.name.to_string(), description: "Pi".to_string()},
            ConstantDef{name: Euler.name.to_string(), description: "Euler's number".to_string()},
            ConstantDef{name: Phi.name.to_string(), description: "Golden Ratio".to_string()},
            ConstantDef{name: C.name.to_string(), description: "Speed of light".to_string()},
            ConstantDef{name: Planck.name.to_string(), description: "Plank's constant".to_string()},
            ConstantDef{name: G.name.to_string(), description: "Gravitational Const".to_string()},
        ];

        pick_list(constants, None::<ConstantDef>, move |selected| {
            Message::CloseAndSend(id, Box::new(Message::Constant(selected.name)))
        })
            .placeholder("constants -- select")
            .width(Length::Fill)
            .text_shaping(Shaping::Advanced)
            .style(Style {
                field: Box::new(move |theme, status| { Self::get_appearance(theme, status) }),
                ..Theme::default_style()
            })
            .into()
    }

    fn conversion_dimension() -> Element<'static, Message> {
        let conversions: Vec<_> = conversions::Dimension::iter().collect();

        pick_list(conversions, None::<Dimension>, move |selected| {
            Message::ConvertDimension(selected)
        })
            .placeholder("convert -- select")
            .width(Length::Fill)
            .text_shaping(Shaping::Advanced)
            .style(Style {
                field: Box::new(move |theme, status| { Self::get_appearance(theme, status) }),
                ..Theme::default_style()
            })
            .into()
    }

    fn conversion_unit(dimension: &Option<Dimension>, direction:  &Option<ConversionDirection>) -> Element<'static, Message> {

        let conversions: Vec<_> = conversions::get_units(&dimension);
        let d = direction.as_ref().unwrap().clone();
        let place_holder = match d {
            ConversionDirection::From => "convert from",
            ConversionDirection::To => "convert to",
        };

        pick_list(conversions, None::<&Unit>, move |selected| {
            Message::Convert(selected.clone(), d)
        })
            .placeholder(place_holder)
            .width(Length::Fill)
            .text_shaping(Shaping::Advanced)
            .style(Style {
                field: Box::new(move |theme, status| { Self::get_appearance(theme, status) }),
                ..Theme::default_style()
            })
            .into()
    }


    fn history(id: Id) -> Element<'static, Message> {
        // Get a readlock on the history.  This should always work
        let history: Vec<HistoryDef> = match history::manager().history().entries().read().as_deref() {
            Ok(queue) => {
                queue.iter().map( | e | {
                    HistoryDef{expr: e.0.clone(), value: e.1}
                }).collect()
            }
            Err(e) => {
                error!("Unabel to access history - {}", e);
                Vec::<HistoryDef>::new()
            }
        };

        pick_list(history, None::<HistoryDef>, move |selected| {
            Message::CloseAndSend(id, Box::new(Message::History(selected.expr, selected.value)))
        })
            .placeholder("history")
            .width(Length::Fill)
            .text_shaping(Shaping::Advanced)
            .style(Style {
                field: Box::new(move |theme, status| { Self::get_appearance(theme, status) }),
                ..Theme::default_style()
            })
            .into()
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
                placeholder_color: theme.extended_palette().secondary.base.text,
                .. pick_list::default(theme, status)
            }
    }
}