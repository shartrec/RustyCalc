use iced::{alignment, Background, Border, Color, Command, Degrees, event, Event, gradient, Length, Pixels, Radians, Renderer, Shadow, Subscription, Theme, Vector, window};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Button, Column, Container, container, Row, text, text_editor};
use iced::widget::button::{Appearance, Status};
use iced::widget::text_editor::{Action, Content, Edit, Motion};

use crate::evaluator::AngleMode;
use crate::ui::calculator::Calc;
use crate::ui::messages::Message;

macro_rules! calculator_button {
    ($var:ident, $b_width:ident, $b_height:ident, $name:literal, $msg:expr) => {
    let container = Container::new($name)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center);

    let $var = Button::new(container)
            .width($b_width)
            .height($b_height)
            .style(|_theme, status| {
                get_style(status)
            })
            .on_press($msg)
            .into();
    };
    ($var:ident, $b_width:ident, $b_height:ident, $name:literal) => {
        calculator_button!($var, $b_width, $b_height, $name, Message::Char(String::from($name)));
    };
}

fn get_style(status: Status) -> Appearance {
    match status {
        Status::Active => {
            let g = gradient::Linear::new(Radians::from(Degrees(150.0)))
                .add_stop(0.0, Color::from_rgb8(0x24, 0x24, 0x24))
                .add_stop(1.0, Color::from_rgb8(0x55, 0x55, 0x55));

            Appearance {
                background: Some(Background::from(g)),
                text_color: Color::WHITE,
                border: Border::default().with_width(Pixels::from(2)).with_color(Color::from_rgb8(0x20, 0x20, 0x20)),
                shadow: Shadow { color: Color::WHITE, offset: Vector::new(-2.0, -2.0), blur_radius: 2.0 },
            }
        }
        Status::Hovered => {
            let g = gradient::Linear::new(Radians::from(Degrees(150.0)))
                .add_stop(0.0, Color::from_rgb8(0x54, 0x54, 0x54))
                .add_stop(1.0, Color::from_rgb8(0x85, 0x85, 0x85));

            Appearance {
                background: Some(Background::from(g)),
                text_color: Color::WHITE,
                border: Border::default().with_width(Pixels::from(2)).with_color(Color::BLACK),
                shadow: Default::default(),
            }
        }
        Status::Pressed => {
            Appearance {
                background: None,
                text_color: Color::BLACK,
                border: Border::default().with_width(Pixels::from(2)).with_color(Color::BLACK),
                shadow: Default::default(),
            }
        }
        Status::Disabled => {
            Appearance {
                background: None,
                text_color: Color::BLACK,
                border: Border::default().with_width(Pixels::from(2)).with_color(Color::BLACK),
                shadow: Default::default(),
            }
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct CalculatorApp {
    content: Content,
    result: Option<Result<f64, String>>,
    calc: Calc,
    window_width: u32,
    window_height: u32,
}

impl CalculatorApp {
    pub(crate) fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status| {
            match event {
                Event::Window(_id, window::Event::Resized { width, height }) => {
                    Some(Message::WindowResized(width, height))
                }
                _ => None
            }
        })
    }

    pub(crate) fn view(&self) -> Container<Message, Theme, Renderer> {
        let lcd = text_editor(&self.content)
            .height(Length::Fill)
            // .height(Length::Fill)
            .style(|_theme, _status| {
                text_editor::Appearance {
                    background: Background::Color(Color::from_rgba8(0, 0, 0, 0.0)),
                    border: Border::default().with_width(Pixels::from(1)).with_color(Color::from_rgb8(0x7f, 0x7f, 0x7f)),
                    icon: Default::default(),
                    placeholder: Default::default(),
                    value: Color::BLACK,
                    selection: Default::default(),
                }
            })
            .on_action(|action| {
                Message::EditorAction(action)
            })
            .into();

        let result = text(match &self.result {
            Some(r) => {
                match r {
                    Ok(v) => {
                        let formatted = format!("= {0:.1$}", v, 10);
                        formatted.trim_end_matches('0').trim_end_matches('.').to_string()
                    }
                    Err(e) => e.clone()
                }
            }
            None => String::from(""),
        })
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Right)
            .into();

        let mode = text(self.calc.angle_mode())
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Right)
            .into();

        let top = Column::with_children([mode, lcd, result]).spacing(5);
        let lcd_container = container(top)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme, _status| {
                container::Appearance {
                    background: Some(Background::Color(Color::from_rgb8(0xd4, 0xed, 0xd4))),
                    ..Default::default()
                }
            })
            .padding(2);

        let w = ((self.window_width as f32 - 4.0) / 5.0 - 4.0).max(55.0);
        let h = ((self.window_height as f32 - 100.0) / 8.0 - 4.0).max(32.0).min(55.0);

        calculator_button!(b_one, w, h, "1");
        calculator_button!(b_two, w, h, "2");
        calculator_button!(b_three, w, h, "3");
        calculator_button!(b_four, w, h, "4");
        calculator_button!(b_five, w, h, "5");
        calculator_button!(b_six, w, h, "6");
        calculator_button!(b_seven, w, h, "7");
        calculator_button!(b_eight, w, h, "8");
        calculator_button!(b_nine, w, h, "9");
        calculator_button!(b_zero, w, h, "0");
        calculator_button!(b_dec, w, h, ".");

        calculator_button!(b_plus, w, h, "+");
        calculator_button!(b_minus, w, h, "-");
        calculator_button!(b_mult, w, h, "x", Message::Char("*".to_string()));
        calculator_button!(b_div, w, h, "/");
        calculator_button!(b_pow, w, h, "^");
        calculator_button!(b_lparen, w, h, "(", Message::Func("()".to_string()));
        calculator_button!(b_rparen, w, h, ")");

        calculator_button!(b_sin, w, h, "sin", Message::Func("sin()".to_string()));
        calculator_button!(b_cos, w, h, "cos", Message::Func("cos()".to_string()));
        calculator_button!(b_tan, w, h, "tan", Message::Func("tan()".to_string()));
        calculator_button!(b_asin, w, h, "asin", Message::Func("asin()".to_string()));
        calculator_button!(b_acos, w, h, "acos", Message::Func("acos()".to_string()));
        calculator_button!(b_atan, w, h, "atan", Message::Func("atan()".to_string()));
        calculator_button!(b_exp, w, h, "exp", Message::Func("exp()".to_string()));
        calculator_button!(b_ln, w, h, "ln", Message::Func("ln()".to_string()));
        calculator_button!(b_log, w, h, "log", Message::Func("log()".to_string()));
        calculator_button!(b_log2, w, h, "log2", Message::Func("log2()".to_string()));
        calculator_button!(b_sqrt, w, h, "√", Message::Func("sqrt()".to_string()));
        calculator_button!(b_abs, w, h, "abs", Message::Func("abs()".to_string()));
        calculator_button!(b_ceil, w, h, "ceil", Message::Func("ceil()".to_string()));
        calculator_button!(b_floor, w, h, "floor", Message::Func("floor()".to_string()));
        calculator_button!(b_fact, w, h, "!", Message::Func("factorial()".to_string()));


        calculator_button!(b_equals, w, h, "=", Message::Evaluate);
        calculator_button!(b_clear, w, h, "AC", Message::Clear);
        calculator_button!(b_left, w, h, "<-", Message::Move(-1));
        calculator_button!(b_right, w, h, "->", Message::Move(1));
        calculator_button!(b_back, w, h, "<del", Message::BackSpace);
        calculator_button!(b_drg, w, h, "DRG", Message::ToggleMode);

        let col_all = Column::with_children([
            lcd_container.into(),
            Row::with_children([
                Column::with_children([
                    Row::with_children([b_back, b_left, b_right, b_clear, b_drg]).spacing(2).into(),
                    Row::with_children([b_sin, b_cos, b_tan, b_sqrt, b_abs]).spacing(2).into(),
                    Row::with_children([b_asin, b_acos, b_atan, b_ceil, b_floor]).spacing(2).into(),
                    Row::with_children([b_exp, b_ln, b_log, b_log2, b_fact]).spacing(2).into(),
                ]).spacing(2).into(),
            ]).spacing(2).into(),
            Row::with_children([
                Column::with_children([
                    Row::with_children([b_seven, b_eight, b_nine, b_lparen, b_rparen]).spacing(2).into(),
                    Row::with_children([b_four, b_five, b_six, b_mult, b_div]).spacing(2).into(),
                    Row::with_children([b_one, b_two, b_three, b_plus, b_minus]).spacing(2).into(),
                    Row::with_children([b_zero, b_dec, b_equals, b_pow]).spacing(2).into(),
                ]).spacing(2).into(),
            ]).spacing(2).into(),
        ]).spacing(2);

        container(col_all)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme, _status| {
                let gradient = gradient::Linear::new(Radians(135.0))
                    .add_stop(0.0, Color::from_rgb8(0x30, 0x30, 0x30))
                    .add_stop(0.25, Color::from_rgb8(0x35, 0x35, 0x35))
                    .add_stop(0.50, Color::from_rgb8(0x40, 0x40, 0x40))
                    .add_stop(0.75, Color::from_rgb8(0x45, 0x45, 0x45))
                    .add_stop(1.0, Color::from_rgb8(0x50, 0x50, 0x50))
                    .into();

                container::Appearance {
                    background: Some(Background::Gradient(gradient)),
                    ..Default::default()
                }
            })
            .padding(10)
    }

    pub(crate) fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Char(s) => {
                for c in s.chars() {
                    self.content.perform(Action::Edit(Edit::Insert(c)));
                }
                Command::none()
            }
            Message::Func(s) => {
                for c in s.chars() {
                    self.content.perform(Action::Edit(Edit::Insert(c)));
                }
                // self.content.perform(Action::Move(Motion::End));
                self.content.perform(Action::Move(Motion::Left));
                Command::none()
            }
            Message::EditorAction(action) => {
                match action {
                    Action::Edit(Edit::Enter) => {
                        self.result = Some(self.calc.evaluate(&self.content.text()))
                    }
                    _ => self.content.perform(action)
                }
                Command::none()
            }
            Message::Evaluate => {
                self.result = Some(self.calc.evaluate(&self.content.text()));
                Command::none()
            }
            Message::Clear => {
                self.content.perform(Action::Move(Motion::DocumentStart));
                self.content.perform(Action::Select(Motion::DocumentEnd));
                self.content.perform(Action::Edit(Edit::Delete));
                Command::none()
            }
            Message::Move(i) => {
                if i == -1 {
                    self.content.perform(Action::Move(Motion::Left));
                } else if i == 1 {
                    self.content.perform(Action::Move(Motion::Right));
                }
                Command::none()
            }
            Message::BackSpace => {
                self.content.perform(Action::Edit(Edit::Backspace));
                Command::none()
            }
            Message::WindowResized(w, h) => {
                self.window_width = w;
                self.window_height = h;
                Command::none()
            }
            Message::ToggleMode => {
                self.calc.set_angle_mode(match self.calc.angle_mode() {
                    AngleMode::Degrees => AngleMode::Gradians,
                    AngleMode::Radians => AngleMode::Degrees,
                    AngleMode::Gradians => AngleMode::Radians
                });
                Command::none()
            }
        }
    }
}
