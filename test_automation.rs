//! Simple test to verify UIAutomation works

#[tokio::main]
async fn main() {
    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();
    
    println!("Root element created successfully");
    
    let control_type = root.get_control_type();
    println!("Control type: {:?}", control_type);
    
    let enabled = root.is_enabled();
    println!("Is enabled: {:?}", enabled);
    
    let offscreen = root.is_offscreen();
    println!("Is offscreen: {:?}", offscreen);
}
