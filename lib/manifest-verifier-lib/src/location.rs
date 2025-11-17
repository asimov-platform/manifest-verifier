#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub end_line: Option<usize>,
    pub column: usize,
    pub end_column: Option<usize>,
}

impl Location {
    pub fn new(
        line: usize,
        end_line: Option<usize>,
        column: usize,
        end_column: Option<usize>,
    ) -> Self {
        Self {
            line,
            end_line,
            column,
            end_column,
        }
    }

    pub fn from_line_and_col(line: usize, col: usize) -> Self {
        Self::new(line, None, col, None)
    }
}
