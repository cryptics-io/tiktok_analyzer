mod tiktok_scraper;

use crate::tiktok_scraper::TiktokScraper;
use anyhow::Result;


fn main() -> Result<()> {
    println!("Launching scraper...");

    TiktokScraper::run_scraper()
}