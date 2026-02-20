use super::storage::StateStore;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogState {
    pub dialog: String,
    pub step: String,
}

impl DialogState {
    pub fn encode(&self) -> String {
        format!("{}|{}", self.dialog, self.step)
    }

    pub fn decode(raw: &str) -> Option<Self> {
        let (dialog, step) = raw.split_once('|')?;
        if dialog.is_empty() || step.is_empty() {
            return None;
        }
        Some(Self {
            dialog: dialog.to_owned(),
            step: step.to_owned(),
        })
    }
}

pub struct DialogMachine<'a> {
    store: &'a mut dyn StateStore,
    prefix: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogFlow {
    name: String,
    start_step: String,
    steps: Vec<String>,
}

impl DialogFlow {
    pub fn new(name: impl Into<String>, start_step: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start_step: start_step.into(),
            steps: Vec::new(),
        }
    }

    pub fn step(mut self, step: impl Into<String>) -> Self {
        self.steps.push(step.into());
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn start_step(&self) -> &str {
        &self.start_step
    }

    pub fn steps(&self) -> &[String] {
        &self.steps
    }

    pub fn contains_step(&self, step: &str) -> bool {
        step == self.start_step || self.steps.iter().any(|item| item == step)
    }
}

impl<'a> DialogMachine<'a> {
    pub fn new(store: &'a mut dyn StateStore) -> Self {
        Self {
            store,
            prefix: "bot:dialog".to_owned(),
        }
    }

    pub fn with_prefix(store: &'a mut dyn StateStore, prefix: impl Into<String>) -> Self {
        Self {
            store,
            prefix: prefix.into(),
        }
    }

    fn key(&self, source_id: i32) -> String {
        format!("{}:{}", self.prefix, source_id)
    }

    pub fn start(&mut self, source_id: i32, dialog: impl Into<String>, step: impl Into<String>) {
        let state = DialogState {
            dialog: dialog.into(),
            step: step.into(),
        };
        self.store.set(self.key(source_id), state.encode());
    }

    pub fn current(&self, source_id: i32) -> Option<DialogState> {
        self.store
            .get(&self.key(source_id))
            .and_then(|raw| DialogState::decode(&raw))
    }

    pub fn is_in(&self, source_id: i32, dialog: &str, step: &str) -> bool {
        self.current(source_id)
            .is_some_and(|state| state.dialog == dialog && state.step == step)
    }

    pub fn advance(&mut self, source_id: i32, next_step: impl Into<String>) -> Option<DialogState> {
        let mut state = self.current(source_id)?;
        state.step = next_step.into();
        self.store.set(self.key(source_id), state.encode());
        Some(state)
    }

    pub fn stop(&mut self, source_id: i32) -> Option<DialogState> {
        self.store
            .remove(&self.key(source_id))
            .and_then(|raw| DialogState::decode(&raw))
    }
}
