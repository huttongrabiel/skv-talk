use skv_talk::tui::tui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tui().await;

    Ok(())
}
