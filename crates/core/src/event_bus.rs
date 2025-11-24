use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Event {
    pub channel: String,
    pub payload: String,
}

type AsyncCallback = Arc<dyn Fn(Event) + Send + Sync>;

#[derive(Clone)]
pub struct EventBus {
    subscribers: Arc<RwLock<HashMap<String, Vec<AsyncCallback>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { subscribers: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub async fn subscribe<F>(&self, channel: &str, callback: F)
    where F: Fn(Event) + Send + Sync + 'static {
        self.subscribers.write().await
            .entry(channel.to_string())
            .or_insert_with(Vec::new)
            .push(Arc::new(callback));
    }

    pub async fn publish(&self, channel: &str, payload: &str) {
        let callbacks = self.subscribers.read().await.get(channel).cloned();
        
        if let Some(callbacks) = callbacks {
            let event = Event { 
                channel: channel.to_string(), 
                payload: payload.to_string() 
            };
            
            for cb in callbacks {
                let event = event.clone();
                tokio::spawn(async move { cb(event); });
            }
        }
    }

    pub async fn publish_event(&self, event: Event) {
        self.publish(&event.channel, &event.payload).await;
    }
}