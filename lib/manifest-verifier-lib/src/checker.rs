use crate::{Rule, RuleBuilder};

pub struct Checker {
    rules: Vec<Rule>,
}

impl Checker {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self) -> RuleBuilder<'_> {
        RuleBuilder::new(self)
    }

    pub(crate) fn append_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn run<F>(&self, error_fn: F) -> bool
    where
        F: Fn(&Rule),
    {
        let mut has_error = false;

        for rule in &self.rules {
            if !rule.result {
                error_fn(rule);
                has_error = true;
            }
        }

        has_error
    }
}
