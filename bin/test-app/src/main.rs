use fuse_runtime_core::runtime::Runtime;
use fuse_runtime_window::{Window, WindowOptions};
use anyhow::Result;

fn main() -> Result<()> {
    println!("Starting Fuse Runtime Test Application");
    
    // Create a new runtime
    let mut runtime = Runtime::new();
    println!("Runtime created");
    
    let window_options = WindowOptions {
        title: "Test Window".to_string(),
        html: "<html><body><h1>Hello from Fuse Runtime Test!</h1><p>This is a test window.</p></body></html>".to_string(),
        width: 800,
        height: 600,
        resizable: true,
        debug: true,
    };
    
    // Create the window and add it to the runtime
    let window = Window::new(window_options)?;
    runtime.add_window(window);
    println!("Window added to runtime");
    
    // Test event bus by broadcasting a message
    runtime.broadcast("test-channel", "Hello from the event bus!");
    println!("Message broadcast on 'test-channel'");
    
    println!("Running window...");
    runtime.run_window(0)?;
    
    // This won't be reached until the window is closed
    println!("Window closed");
    
    Ok(())
}
