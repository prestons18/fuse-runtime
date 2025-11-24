use fuse_runtime_window::Window;
use tokio::runtime::Runtime as TokioRuntime;
use crate::event_bus::EventBus;

pub struct Runtime {
    windows: Vec<Window>,
    event_bus: EventBus,
    tokio: TokioRuntime,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime { 
            windows: vec![], 
            event_bus: EventBus::new(), 
            tokio: TokioRuntime::new().unwrap() 
        }
    }

    pub fn run(&mut self) {
        println!("fuse-runtime core started");
        self.tokio.block_on(async {
            // async loop
        })
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }
    
    pub fn run_window(&mut self, index: usize) -> anyhow::Result<()> {
        if index < self.windows.len() {
            let window = self.windows.remove(index);
            window.run()
        } else {
            Err(anyhow::anyhow!("Window index out of bounds"))
        }
    }

    pub fn broadcast(&self, channel: &str, payload: &str) {
        let event_bus = self.event_bus.clone();
        let channel = channel.to_string();
        let payload = payload.to_string();
        
        self.tokio.spawn(async move {
            event_bus.publish(&channel, &payload).await;
        });
    }
}