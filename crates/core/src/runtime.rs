use tokio::runtime::Runtime as TokioRuntime;
use crate::event_bus::EventBus;

pub struct Runtime {
    windows: Vec<String>,
    event_bus: EventBus,
    tokio: TokioRuntime,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime { windows: vec![], event_bus: EventBus::new(), tokio: TokioRuntime::new().unwrap() }
    }

    pub fn run(&mut self) {
        println!("fuse-runtime core started");
        self.tokio.block_on(async {
            // async loop
        })
    }

    pub fn add_window(&mut self, id: String) {
        self.windows.push(id);
    }

    pub fn broadcast(&self, channel: &str, payload: &str) {
        let event = crate::event_bus::Event {
            channel: channel.to_string(),
            payload: payload.to_string(),
        };
        self.event_bus.publish_event(event);
    }
}