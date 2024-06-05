use crate::evaluator::Evaluator;

#[derive(Debug, Default)]
pub(crate) struct Calc {
    expression: String,
    result: Option<f64>
}

impl Calc {

    pub(crate) fn evaluate(&mut self) {
        self.result = Evaluator::default().evaluate(&self.expression).ok();
    }

    pub(crate) fn add_chunk(&mut self, chunk: String) {
        self.expression.push_str(&chunk);
    }

    pub(crate) fn insert(&mut self, chunk: String, pos: usize) {
        self.expression.insert_str(pos, &chunk);
    }

    pub(crate) fn remove(&mut self, pos: usize) {
        self.expression.remove(pos);
    }

    pub(crate) fn clear(&mut self) {
        self.expression.clear();
    }

    pub(crate) fn remove_last(&mut self) {
        let _ = self.expression.pop();
    }

    pub(crate) fn get_expression(&self) -> &str {
        &self.expression
    }


    pub fn result(&self) -> Option<f64> {
        self.result
    }
}
