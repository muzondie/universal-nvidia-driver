mod gui;
mod driver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    gui::create_main_window();
    Ok(())
}