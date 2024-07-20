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

/// This module contains the funcrtions to build our menu bar and menus descending from it.
use iced::{alignment, Background, Border, Element, Length, Padding, Renderer, Theme};
use iced::advanced::text::Shaping;
use iced::border::Radius;
use iced::widget::{Button, button, Container, row, text};
use iced_aw::menu::{Item, Menu, primary};
use iced_aw::{BOOTSTRAP_FONT, menu, menu_bar};
use iced_aw::Bootstrap;
use iced_aw::style::Status;
use strum::IntoEnumIterator;
use crate::{conversions, evaluator, history, ui};
use crate::conversions::{Dimension, Unit};
use crate::ui::messages::Message;

/// Builds the menus for our calculator
pub(crate) fn build_menu_bar<'a> () -> Element<'a, Message> {

    let insert_menu = menu_insert();
    let convert_menu = menu_dimension();
    let history_menu = menu_history();
    let theme_menu = menu_theme();

    let mb = menu_bar!(
            (menu_top("Convert"), convert_menu)
            (menu_top("Insert"), insert_menu)
            (menu_top("History"), history_menu)
            (menu_top("Theme"), theme_menu)
        )
        .style(|theme:&iced::Theme, status: Status | menu::Style{
            path_border: Border{
                radius: Radius::from(1.0),
                width: 2.0,
                ..Default::default()
            },
            bar_border: Border {
                radius: Radius::from(0),
                ..Default::default()
            },
            menu_border: Border {
                radius: Radius::from(0),
                ..Default::default()
            },
            bar_background_expand: Padding::from(0),
            bar_background: Background::Color(theme.extended_palette().background.strong.color),
            menu_background: Background::Color(theme.extended_palette().background.strong.color),
            menu_background_expand: Padding::from(0),
            ..primary(theme, status)
        });
    mb.into()

}

fn menu_top(label: &str) -> Container<Message> {
    let t = text(label);
    let container = Container::new(t)
        .padding(Padding::from([0,3,0,3]));
    container
}

fn menu_insert() -> Menu<'static, Message, Theme, Renderer> {

    let mut items = Vec::new();

    items.push(Item::with_menu(menu_item("Constants".to_string(), Message::Null), menu_constants()));
    items.push(Item::with_menu(menu_item("Functions".to_string(), Message::Null), menu_functions()));

    Menu::new(items).offset(0.0).spacing(2.0).max_width(150.0)
}

fn menu_constants() -> Menu<'static, Message, Theme, Renderer> {

    let mut items = Vec::new();
    for c in evaluator::constants::get_all().iter() {
        items.push(Item::new(menu_item(c.long_name().to_string(), Message::Constant(c.name().to_string()))));
    }
    Menu::new(items).offset(0.0).spacing(2.0).max_width(150.0)
}

fn menu_functions() -> Menu<'static, Message, Theme, Renderer> {

    let mut items = Vec::new();

    for f in evaluator::functions::get_all().iter() {
        items.push(Item::new(menu_item(f.name().to_string(), Message::Func(f.name().to_string()))));
    }

    Menu::new(items).offset(0.0).spacing(2.0).max_width(75.0)

}
fn menu_history() -> Menu<'static, Message, Theme, Renderer> {

    let mut items = Vec::new();
    if let Ok(queue) = history::manager().history().entries().read().as_deref() {
        for history_item in queue.iter() {
            items.push(Item::new(menu_item(history_item.0.clone(),
                                           Message::History(history_item.0.clone(), history_item.1.clone()))));
        }
    }


    Menu::new(items).offset(0.0).spacing(2.0).max_width(200.0)

}

fn menu_theme() -> Menu<'static, Message, Theme, Renderer> {

    let mut items = Vec::new();
    items.push(Item::new(menu_item(
        ui::lcd_theme().to_string(),
        Message::ThemeChanged(ui::lcd_theme().clone())
    )));

    for t in Theme::ALL {
        items.push(Item::new(menu_item(
            t.to_string(),
            Message::ThemeChanged(t.clone())
        )));
    }
    Menu::new(items).offset(0.0).spacing(2.0).max_width(250.0)

}

fn menu_dimension()  -> Menu<'static, Message, Theme, Renderer> {
    let mut items = Vec::new();
    for d in conversions::Dimension::iter() {
        items.push(Item::with_menu(menu_item(
            d.to_string(),
            Message::Null
        ), menu_unit_from(&d)));
    }
    Menu::new(items).offset(0.0).spacing(2.0).max_width(90.0)

}

fn menu_unit_from(dimension: &Dimension)  -> Menu<'static, Message, Theme, Renderer> {
    let mut items = Vec::new();
    for unit in conversions::get_units(dimension).iter() {
        items.push(Item::with_menu(
            menu_item_sub(unit.to_string(), Message::Null),
            menu_unit_to(dimension, *unit)
            ));
    }
    Menu::new(items).offset(0.0).spacing(2.0).max_width(150.0)

}

fn menu_unit_to(dimension: &Dimension, from: &'static Unit) -> Menu<'static, Message, Theme, Renderer> {
    let mut items = Vec::new();
    for unit in conversions::get_units(dimension).iter() {
        let to = unit;
        items.push(Item::new(
            menu_item(unit.to_string(), Message::ConvertPerform(&from, *to)),
        ));
    }
    Menu::new(items).offset(0.0).spacing(2.0).max_width(140.0)
}

fn menu_item(label: String, msg: Message) -> Element<'static, Message> {
    let content = text(label)
        .width(Length::Fill)
        .shaping(Shaping::Advanced);
    menu_item_core(msg, content.into())
}

fn menu_item_sub(label: String, msg: Message) -> Element<'static, Message> {
    let content = row![
                text(label)
                    .width(Length::Fill)
                    .shaping(Shaping::Advanced)
                    .vertical_alignment(alignment::Vertical::Center),
                text(iced_aw::bootstrap::icon_to_string(
                    Bootstrap::CaretRightFill
                ))
                .font(BOOTSTRAP_FONT)
                .width(Length::Shrink)
                .vertical_alignment(alignment::Vertical::Center),
            ]
        .align_items(iced::Alignment::Center);
    menu_item_core(msg, content.into())
}

fn menu_item_core(msg: Message, content: Element<'static, Message>) -> Element<'static, Message> {
    Button::new(content)
        .style(|theme: &Theme, status| {
            match status {
                iced::widget::button::Status::Hovered => {
                    button::Style {
                        background: Some(Background::from(theme.extended_palette().background.base.color)),
                        text_color: theme.extended_palette().background.base.text,
                        ..button::Style::default()
                    }
                }
                _ => {
                    button::Style {
                        background: Some(Background::from(theme.extended_palette().background.strong.color)),
                        text_color: theme.extended_palette().background.base.text,
                        ..button::Style::default()
                    }
                }
            }
        })
        .padding(Padding::from([0, 3, 0, 3]))
        .on_press(msg)
        .height(Length::Shrink)
        .into()
}