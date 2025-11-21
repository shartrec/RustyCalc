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

use crate::conversions::{Dimension, Unit};
use crate::{conversions, evaluator};
use floem::menu::{Menu, MenuItem};
/// This module contains the functions to build our menu bar and menus descending from it.

use floem::IntoView;
use floem::peniko::Color;
use floem::prelude::Decorators;
use floem::views::Button;
use strum::IntoEnumIterator;

/// Builds the menus for our calculator
pub fn build_menu_bar () -> impl IntoView {

    (
        Button::new("Convert").popout_menu(menu_dimension()),
        Button::new("Insert").popout_menu(menu_insert()),
        Button::new("History").popout_menu(menu_history()),
        Button::new("Theme").popout_menu(menu_theme()),

    )
        .style(|s| s.flex_row().gap(1).border(1.0).border_color(Color::BLACK).padding(1.0))
    // mb.style(|theme:&Theme, status: Status | menu::Style{
    //         path_border: Border{
    //             radius: Radius::from(1.0),
    //             width: 2.0,
    //             ..Default::default()
    //         },
    //         bar_border: Border {
    //             radius: Radius::from(0),
    //             ..Default::default()
    //         },
    //         menu_border: Border {
    //             radius: Radius::from(0),
    //             ..Default::default()
    //         },
    //         bar_background_expand: Padding::from(0),
    //         bar_background: Background::Color(theme.extended_palette().background.strong.color),
    //         menu_background: Background::Color(theme.extended_palette().background.strong.color),
    //         menu_background_expand: Padding::from(0),
    //         ..primary(theme, status)
    //     })
    // .into()
}

// fn menu_top(label: &str) -> Container<'_, Message> {
//     let t = text(label);
//     let container = Container::new(t)
//         .padding(Padding::from([0,3]));
//     container
// }

fn menu_insert() -> fn() -> Menu {
        let menu = || { Menu::new("Insert")
            .entry(menu_constants())
            .separator()
            .entry(menu_functions())
        };
        menu
}

fn menu_constants() -> Menu {
    let mut menu = Menu::new("Constants");
    for c in evaluator::constants::get_all().iter() {
        let m = MenuItem::new(c.name().to_string())
            .action(|| {});
        menu = menu.entry(m).style(|s| s.color(Color::BLACK).background(Color::WHITE).border(1.0).border_color(Color::BLACK).padding(4.0));
    }
    menu
}

fn menu_functions() -> Menu {

    let mut menu = Menu::new("Functions");
    for f in evaluator::functions::get_all().iter() {
        let m = MenuItem::new(f.name().to_string())
            .action(|| {});
        menu = menu.entry(m);
    }
    menu

}
fn menu_history() ->  fn() -> Menu {
    let menu = || { Menu::new("History")};

    menu
}

fn menu_theme() -> fn() -> Menu {
    let menu = || { Menu::new("Theme")};
    menu
}

fn menu_dimension()  ->  fn() -> Menu  {
    let mut menu = || {
        let mut menu = Menu::new("Constants");
        for d in Dimension::iter() {
            let mut m = MenuItem::new(d.to_string())
                .action(|| {});
            menu = menu.entry(m);
        }
        menu
    };
    menu
}

fn menu_unit_from(dimension: &Dimension)  -> Menu {
    let mut menu = Menu::new("Constants");
    for unit in conversions::get_units(dimension).iter() {
        let mut m = MenuItem::new(unit.to_string())
        .action(|| {
            // todo!("Implement unit conversion action");
        });
        menu = menu.entry(m);
    }
    menu
}

fn menu_unit_to(dimension: &Dimension, from: &'static Unit) -> Menu {
    let mut menu = Menu::new("Constants");
    for unit in conversions::get_units(dimension).iter() {
        let mut m = MenuItem::new(unit.to_string())
            .action(|| {
                // todo!("Implement unit conversion action");
            });
        menu = menu.entry(m);
    }
    menu
}
