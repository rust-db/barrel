/// Functions to generate default values
#[derive(Debug, Clone, PartialEq)]
pub enum AutogenFunction {
    /// Gives the current timestamp
    CurrentTimestamp,
}

/// Generates the current timestamp
pub fn current_timestamp() -> AutogenFunction {
    AutogenFunction::CurrentTimestamp
}
