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

use iced::{Command, Element, event, Event, executor, multi_window, Point, Size, Subscription, Theme, window};
use iced::widget::text;
use iced::window::{Id, Level, Position};

use crate::ui;
use crate::ui::calc_window::CalcWindow;
use crate::ui::func_popup::FuncPopup;
use crate::ui::messages::Message;

#[derive(Debug)]
pub(crate) struct CalculatorApp {
    main_window: CalcWindow,
    pick_window: Option<(Id, FuncPopup)>,
    theme: Theme
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            main_window: CalcWindow::default(),
            pick_window: None,
            theme: ui::lcd_theme().clone(),
        }
    }
}


impl multi_window::Application for CalculatorApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            CalculatorApp::default(),
            Command::none(),
        )
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status| {
            match event {
                Event::Window(id, window::Event::Resized { width, height}) => {
                    Some(Message::WindowResized(id, width, height))
                }
                Event::Window(id, window::Event::Moved { x, y}) => {
                    Some(Message::WindowMoved(id, x, y))
                }
                _ => None
            }
        })
    }

    fn view(&self, id: Id) -> Element<Message> {

        match id {
            Id::MAIN => self.main_window.view(id),
            _ => match &self.pick_window {
                Some((id_settings, settings)) if id == *id_settings => settings.view(id),
                _ => text("WE HAVE A PROBLEM").into(),
            }
        }

    }

    fn update(&mut self, message: Message) -> Command<Message> {

        let mut commands: Vec<Command<Message>> = vec![];

        if let Some((id, window)) = &self.pick_window {
            commands.push(window.update(id, message.clone()));
        }
        commands.push(self.main_window.update(message.clone()));


        commands.push(match message {

            Message::FuncPopup => {
                // Get the position of the main window
                let (x, y) = self.main_window.position();
                // window moved events only work on some platforms, so if "(0, 0)" use default
                let (w, _h) = self.main_window.size();
                let new_pos =  if (x, y) == (0, 0) {
                    Position::Default
                } else {
                    Position::Specific(Point::new((x + w as i32 - 50) as f32, (y + 100) as f32))
                };

                // Open a settings window and store a reference to it
                let (id, spawn_window) = window::spawn(window::Settings {
                    level: Level::AlwaysOnTop,
                    position: new_pos,
                    exit_on_close_request: true,
                    size: Size::new(250.0, 450.0),
                    decorations: true,
                    ..Default::default()
                });

                self.pick_window = Some((id, FuncPopup::default()));
                spawn_window
            }
            Message::ThemeChanged(t) => {
                self.theme = t;
                Command::none()
            }
            _ => {
                Command::none()
            }
        });

        Command::batch(commands)
    }

    fn title(&self, id: Id) -> String {
        match id {
            Id::MAIN => self.main_window.title(),
            _ => match &self.pick_window {
                Some((id_settings, settings)) if id == *id_settings => settings.title(),
                _ => "Unknown".to_string(),
            }
        }
    }

    fn theme(&self, _window: Id) -> Self::Theme {
        self.theme.clone()
    }
}

