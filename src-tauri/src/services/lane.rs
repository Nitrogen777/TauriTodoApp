use scraper::{Html, Selector};

const FASTLANE_URL: &str = "https://www.fastlane.co.il/";

#[tauri::command]
pub async fn get_price() -> Result<String, String> {
    let html = fetch_html(FASTLANE_URL).await.map_err(|err| err.to_string())?;
    let document = Html::parse_document(&html);
    let find_price = Selector::parse("span#lblPrice").unwrap();
    
    if let Some(element) = document.select(&find_price).next() {
        return Ok(element.text().collect::<String>());
    } 

    Err("Could not find price".to_string())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    reqwest::get(url).await?.error_for_status()?.text().await
}