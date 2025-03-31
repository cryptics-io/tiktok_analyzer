use fantoccini::{Client, ClientBuilder};
use anyhow::Result;
use tokio::runtime::Runtime;

pub struct TiktokScraper {
    client: Client,
}

impl TiktokScraper {
    pub async fn new() -> Result<Self> {
        let client = ClientBuilder::native()
            .connect("http://localhost:52915")
            .await?;
        Ok(Self { client })
    }

    pub async fn close(self) -> Result<()> {
        self.client.close().await?;
        Ok(())
    }

    pub async fn run_async() -> Result<()> {
        let mut scraper = Self::new().await?;
        println!("Scraper initialized.");

        // We'll call scraper.scrape_profile() here later

        scraper.close().await?;
        Ok(())
    }

    pub fn run_scraper() -> Result<()>{
        let rt = Runtime::new()?; // To run async synchronously
        rt.block_on(Self::run_async()) // run the async method
    }
}
