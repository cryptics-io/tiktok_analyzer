use tiktok_analyzer::tiktok_scraper::TiktokScraper;

#[tokio::test]
async fn it_initializes_the_scraper() {
    let scraper = TiktokScraper::new().await;
    assert!(scraper.is_ok());
}
