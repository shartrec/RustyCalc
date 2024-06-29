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

use iced::{Element, event, Event, Point, Size, Subscription, Task, Theme, window};
use iced::widget::text;
use iced::window::{Id, Level, Position};

use crate::ui;
use crate::ui::calc_window::CalcWindow;
use crate::ui::func_popup::FuncPopup;
use crate::ui::messages::Message;

// #[derive(Debug)]
pub(crate) struct CalculatorApp {
    main_window: CalcWindow,
    main_window_id: Option<Id>,
    popup: Option<(Id, FuncPopup)>,
    theme: Theme
}

impl Default for CalculatorApp {
    fn default() -> Self {
        let pref = ui::preferences::manager();
        Self {
            main_window: CalcWindow::default(),
            main_window_id: None,
            popup: None,
            theme: theme_by_name(pref.get::<String>(ui::preferences::THEME)).clone(),
        }
    }
}

impl CalculatorApp {
      pub(crate) fn title(&self, id: Id) -> String {
          if let Some(main_id) = &self.main_window_id {
              if id == *main_id {
                  return self.main_window.title()
              }
          }
          if let Some((popup_id, popup)) = &self.popup {
              if id == *popup_id {
                  return popup.title()
              }
          }
          "Unknown".to_string()
      }

    pub(crate) fn update(&mut self, message: Message) -> Task<Message> {

        let mut task: Vec<Task<Message>> = vec![];

        if let Some((id, window)) = &self.popup {
            task.push(window.update(id, message.clone()));
        }
        if let Some(main_id) = &self.main_window_id {
            task.push(self.main_window.update(main_id, message.clone()));
        }

        task.push(match message {

            Message::MainWindowOpened(id) => {
                self.main_window_id = Some(id);
                Task::none()
            }
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
                let task = window::open(window::Settings {
                    level: Level::AlwaysOnTop,
                    position: new_pos,
                    exit_on_close_request: true,
                    size: Size::new(250.0, 450.0),
                    decorations: true,
                    ..Default::default()
                });
                task.map(|id| Message::PopupWindowOpened(id))
            }
            Message::PopupWindowOpened(id) => {
                self.popup = Some((id, FuncPopup::default()));
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
        if let Some((popup_id, popup)) = &self.popup {
            if id == *popup_id {
                return popup.view(&id)
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
