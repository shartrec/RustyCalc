#[derive(Debug, Clone)]
pub enum Message {
    WindowResized(u32, u32),
    Char(String),
    Func(String),
    Move(i32),
    BackSpace,
    Clear,
    Evaluate,
}