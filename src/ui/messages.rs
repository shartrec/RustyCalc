use iced::widget::text_editor::Action;

#[derive(Debug, Clone)]
pub enum Message {
    WindowResized(u32, u32),
    Char(String),
    EditorAction(Action),
    Func(String),
    Move(i32),
    BackSpace,
    Clear,
    Evaluate,
    ToggleMode,
}