use super::Environment;
use crate::Expression;

#[derive(Default)]
pub struct EnvironmentBuilder(Environment);

impl EnvironmentBuilder {
    pub fn set(mut self, identifier: &str, expression: Expression) -> Self {
        self.0.data.insert(identifier.to_owned(), expression);
        self
    }

    pub fn build(self) -> Environment {
        self.0
    }
}
