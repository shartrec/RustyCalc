use crate::evaluator::{AngleMode, Evaluator};

#[derive(Debug, Default)]
pub(crate) struct Calc {
    angle_mode: AngleMode,
}

impl Calc {

    pub(crate) fn evaluate(&mut self, expr: &str) -> Result<f64, String> {
        Evaluator::with_mode(&self.angle_mode).evaluate(&expr)
    }

    pub fn angle_mode(&self) -> &AngleMode {
        &self.angle_mode
    }
    pub fn set_angle_mode(&mut self, angle_mode: AngleMode) {
        self.angle_mode = angle_mode;
    }
}
