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

use std::sync::Arc;

use iced::{Color, Theme};
use iced::theme::Custom;
use iced::theme::Palette;
use iced::theme::palette::{Background, Danger, Extended, Pair, Primary, Secondary, Success};
use lazy_static::lazy_static;

pub(crate) mod calculator;
pub(crate) mod messages;
pub(crate) mod calculator_app;
mod func_popup;
mod calc_window;
pub(super) mod preferences;

pub static PALETT_LCD: Palette = Palette {
    background: Color::from_rgb(  // LCD Green background
        0xd4 as f32 / 255.0,
        0xed as f32 / 255.0,
        0xd4 as f32 / 255.0,
    ),
    text: Color::BLACK,
    primary: Color::from_rgb(
        0x40 as f32 / 255.0,
        0x40 as f32 / 255.0,
        0x40 as f32 / 255.0,
    ),
    success: Color::from_rgb(   // Dark teal
        0x12 as f32 / 255.0,
        0x66 as f32 / 255.0,
        0x4F as f32 / 255.0,
    ),
    danger: Color::from_rgb(
        0xC3 as f32 / 255.0,
        0x00 as f32 / 255.0,
        0x00 as f32 / 255.0,
    ),
};

lazy_static! {
    static ref LCD_THEME: Theme =
        Theme::Custom(Arc::new(Custom::with_fn(String::from("Lcd Calculator"), PALETT_LCD, |palette| -> Extended {
            Extended {
                background: Background{
                    weak: Pair::new(Color::from_rgb8(0x00,0x50,0x60), palette.text), // Grey Blue
                    .. Background::new(palette.background, palette.text)
                },
                primary: Primary::generate (
                    palette.primary,
                    Color::BLACK,
                    Color::WHITE,
                ),
                secondary: Secondary{
                    strong: Pair::new(Color::from_rgb8(0x04,0x04,0x04,), Color::WHITE),
                    .. Secondary::generate(Color::BLACK, Color::WHITE)
                },
                success: Success::generate(
                    palette.primary,
                    Color::BLACK,
                    Color::WHITE,
                ),
                danger: Danger::generate(
                    palette.danger,
                    palette.background,
                    palette.text,
                ),
                is_dark: false,
            }
        })));
}

fn lcd_theme() -> &'static Theme {
    &LCD_THEME
}