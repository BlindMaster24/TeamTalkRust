//! Declarative linear dialog flow used by
//! [`super::machine::DialogMachine::restart_flow`] and
//! [`super::machine::DialogMachine::advance_flow`].

/// A named, ordered sequence of dialog steps.
///
/// A `DialogFlow` is a lightweight description of a linear conversation:
/// a `start_step`, then zero or more subsequent `steps` visited in
/// order. It does not own any state itself - step navigation is always
/// performed against an external [`super::state::DialogState`] via
/// [`super::machine::DialogMachine`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogFlow {
    name: String,
    start_step: String,
    steps: Vec<String>,
}

impl DialogFlow {
    /// Creates a new flow with the given dialog name and start step.
    pub fn new(name: impl Into<String>, start_step: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start_step: start_step.into(),
            steps: Vec::new(),
        }
    }

    /// Appends `step` to the end of the flow.
    #[must_use]
    pub fn step(mut self, step: impl Into<String>) -> Self {
        self.steps.push(step.into());
        self
    }

    /// Returns the dialog name associated with this flow.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the start step label.
    #[must_use]
    pub fn start_step(&self) -> &str {
        &self.start_step
    }

    /// Returns the ordered list of non-start steps.
    #[must_use]
    pub fn steps(&self) -> &[String] {
        &self.steps
    }

    /// Returns `true` if `step` is the start step or appears in the
    /// step list.
    #[must_use]
    pub fn contains_step(&self, step: &str) -> bool {
        step == self.start_step || self.steps.iter().any(|item| item == step)
    }

    /// Returns the step that follows `step`, or `None` if `step` is
    /// unknown or terminal.
    pub fn next_step(&self, step: &str) -> Option<&str> {
        if step == self.start_step {
            return self.steps.first().map(String::as_str);
        }

        self.steps
            .windows(2)
            .find_map(|window| (window[0] == step).then_some(window[1].as_str()))
    }

    /// Returns the step that precedes `step`, or `None` if `step` is
    /// the start step or unknown.
    #[allow(clippy::must_use_candidate)]
    pub fn previous_step(&self, step: &str) -> Option<&str> {
        if let Some(first) = self.steps.first()
            && first == step
        {
            return Some(&self.start_step);
        }

        self.steps
            .windows(2)
            .find_map(|window| (window[1] == step).then_some(window[0].as_str()))
    }

    /// Returns `true` when `step` is the start step.
    #[must_use]
    pub fn is_start_step(&self, step: &str) -> bool {
        step == self.start_step
    }

    /// Returns `true` when `step` is the last step in the flow.
    #[must_use]
    pub fn is_terminal_step(&self, step: &str) -> bool {
        self.steps.last().is_some_and(|last| last == step)
    }
}
