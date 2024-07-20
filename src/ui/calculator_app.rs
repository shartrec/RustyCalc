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

use iced::{Element, event, Event, Subscription, Task, Theme, window};
use iced::widget::text;
use iced::window::Id;

use crate::ui;
use crate::ui::calc_window::CalcWindow;
use crate::ui::messages::Message;

// #[derive(Debug)]
pub(crate) struct CalculatorApp {
    main_window: CalcWindow,
    main_window_id: Option<Id>,
    theme: Theme
}

impl Default for CalculatorApp {
    fn default() -> Self {
        let pref = ui::preferences::manager();
        Self {
            main_window: CalcWindow::default(),
            main_window_id: None,
            theme: theme_by_name(pref.get::<String>(ui::preferences::THEME)).clone(),
        }
    }
}

impl CalculatorApp {
      pub(crate) fn title(&self, _id: Id) -> String {
          self.main_window.title()
      }

    pub(crate) fn update(&mut self, message: Message) -> Task<Message> {

        let mut task: Vec<Task<Message>> = vec![];

        if let Some(main_id) = &self.main_window_id {
            task.push(self.main_window.update(main_id, message.clone()));
        }

        task.push(match message {

            Message::MainWindowOpened(id) => {
                self.main_window_id = Some(id);
                Task::none()
            }
            Message::ThemeChanged(t) => {
                self.theme = t;
                let pref = ui::preferences::manager();
                pref.put(ui::preferences::THEME, format!("{}", &self.theme));
                Task::none()
            }
            _ => {
                Task::none()
            }
        });

        Task::batch(task)
    }

    pub(crate) fn view(&self, id: Id) -> Element<Message> {

        if let Some(main_id) = &self.main_window_id {
            if id == *main_id {
                return self.main_window.view(&id)
            }
        }
        text("WE HAVE A PROBLEM").into()
    }

    pub(crate) fn theme(&self, _window: Id) -> Theme {
        self.theme.clone()
    }

    pub(crate) fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, id| {
            match event {
                Event::Window(window::Event::Resized { width, height}) => {
                    Some(Message::WindowResized(id, width, height))
                }
                Event::Window(window::Event::Moved { x, y}) => {
                    Some(Message::WindowMoved(id, x, y))
                }
                Event::Window(window::Event::Closed {}) => {
                    Some(Message::WindowClosed(id))
                }
                _ => None
            }
        })
    }
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
