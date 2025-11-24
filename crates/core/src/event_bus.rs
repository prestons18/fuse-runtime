use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Event {
    pub channel: String,
    pub payload: String,
}

type Callback = Box<dyn Fn(Event) + Send + Sync>;

#[derive(Clone)]
pub struct EventBus {
    subscribers: Arc<Mutex<HashMap<String, Vec<Callback>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { subscribers: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub fn subscribe<F>(&self, channel: &str, callback: F)
    where F: Fn(Event) + Send + Sync + 'static {
        self.subscribers.lock().unwrap()
            .entry(channel.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
    }

    pub fn publish(&self, channel: &str, payload: &str) {
        if let Some(callbacks) = self.subscribers.lock().unwrap().get(channel) {
            let event = Event { channel: channel.to_string(), payload: payload.to_string() };
            for cb in callbacks { cb(event.clone()); }
        }
    }

    pub fn publish_event(&self, event: Event) {
        self.publish(&event.channel, &event.payload);
    }
}