use std::rc::Rc;

/// A string that cannot be changed.
#[derive(Clone, Debug, PartialEq)]
pub struct ImmutableString {
    inner: Rc<String>,
}

impl ImmutableString {
    pub fn as_ref(&self) -> &str {
        &self.inner
    }

    pub(super) fn new(text: String) -> ImmutableString {
        ImmutableString {
            inner: Rc::new(text),
        }
    }

    #[cfg(test)]
    pub(super) fn from(text: &str) -> ImmutableString {
        ImmutableString {
            inner: Rc::new(String::from(text)),
        }
    }
}

/// Positional information about a start and end point in the text.
#[derive(Debug, PartialEq, Clone)]
pub struct Range {
    /// Start position of the node in the text.
    pub start: usize,
    /// End position of the node in the text.
    pub end: usize,
    /// Line of the start position of the node in the text.
    pub start_line: usize,
    /// Line of the end position of the node in the text.
    pub end_line: usize,
}
