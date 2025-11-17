use crate::Location;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub title: Option<String>,
    pub message: Option<String>,
    pub location: Option<Location>,
    pub result: bool,
}
