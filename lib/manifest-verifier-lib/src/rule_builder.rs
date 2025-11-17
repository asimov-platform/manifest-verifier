use crate::{Checker, Location, Rule};
use marked_yaml::Spanned;

pub struct RuleBuilder<'a> {
    checker: &'a mut Checker,
    title: Option<String>,
    message: Option<String>,
    location: Option<Location>,
    result: Option<bool>,
}

impl<'a> RuleBuilder<'a> {
    pub(crate) fn new(checker: &'a mut Checker) -> Self {
        Self {
            checker,
            title: None,
            message: None,
            location: None,
            result: None,
        }
    }

    pub fn with_title<S>(mut self, title: S) -> Self
    where
        S: Into<String>,
    {
        self.title = Some(title.into());
        self
    }

    pub fn with_message<S>(mut self, message: S) -> Self
    where
        S: Into<String>,
    {
        self.message = Some(message.into());
        self
    }

    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_property<T>(mut self, property: &Spanned<T>) -> Self {
        let span = property.span();
        let start = span.start().expect("Property must have a start!");
        let end_line = span.end().map(|e| e.line());
        let end_column = span.end().map(|e| e.column());
        self.location = Some(Location::new(
            start.line(),
            end_line,
            start.column(),
            end_column,
        ));
        self
    }

    pub fn with_condition(mut self, condition: bool) -> Self {
        self.result = Some(condition);
        self
    }

    pub fn build(self) {
        let result = self.result.expect("Condition result must be set");

        if self.title.is_none() && self.message.is_none() {
            panic!("Rule must have a title or a message");
        }

        let rule = Rule {
            title: self.title,
            message: self.message,
            location: self.location,
            result,
        };

        self.checker.append_rule(rule);
    }
}
