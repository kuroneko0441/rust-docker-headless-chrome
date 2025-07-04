use headless_chrome::{Browser, LaunchOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let browser = Browser::new(LaunchOptions {
        headless: true,
        sandbox: false,
        ..Default::default()
    })?;

    let tab = browser.new_tab()?;
    tab.navigate_to("https://example.com")?;
    tab.wait_for_element("h1")?;

    let h1 = tab.find_element("h1")?;
    let content = h1.get_inner_text()?;

    println!("{}", content);
    Ok(())
}
