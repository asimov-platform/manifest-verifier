use marked_yaml::Spanned;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Manifest {
    pub name: Option<Spanned<String>>,
    pub label: Option<Spanned<String>>,
    // pub title: Option<Spanned<String>>,
    pub summary: Option<Spanned<String>>,
}
