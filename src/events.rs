use std::collections::HashMap;

pub struct EventEmitter {
    events: HashMap<String, Vec<Box<dyn Fn(Option<&Vec<u8>>) + Send + Sync>>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        EventEmitter {
            events: HashMap::new(),
        }
    }

    pub fn emit(&self, event: &str, data: Option<&Vec<u8>>) {
        if let Some(callbacks) = self.events.get(event) {
            for callback in callbacks {
                callback(data);
            }
        }
    }

    pub fn on<F>(&mut self, event: &str, callback: F)
    where
        F: Fn(Option<&Vec<u8>>) + Send + Sync + 'static,
    {
        self.events
            .entry(event.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
    }
}