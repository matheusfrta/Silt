use crate::primitives::Signal;
use std::collections::HashMap;

pub struct Store<T> {
    signals: HashMap<String, Signal<T>>,
}

impl<T: Clone + PartialEq + 'static> Store<T> {
    pub fn new() -> Self {
        Self { signals: HashMap::new() }
    }

    pub fn get(&self, k: &str) -> Option<T> {
        self.signals.get(k).map(|s| s.get())
    }

    pub fn set(&mut self, k: &str, v: T) {
        if let Some(s) = self.signals.get(k) {
            s.set(v);
        } else {
            self.signals.insert(k.to_string(), Signal::new(v));
        }
    }
}