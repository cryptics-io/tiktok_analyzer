use fantoccini::{Client, ClientBuilder};
use anyhow::Result;
use tokio::runtime::Runtime;
use std::io::{self, Write};

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

        // Placeholder: scraper.scrape_profile(&profile_url).await?;

        scraper.close().await?;
        Ok(())
    }

    fn ask_userinput_profile() -> Result<String> {
        let mut input = String::new();
        print!("Enter TikTok profile URL: ");
        io::stdout().flush()?; // ensures the prompt is printed before user input
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_string();
        println!("Profile URL: {}", input);
        Ok(input)
    }

    pub fn run_scraper() -> Result<()> {
        let profile_url = Self::ask_userinput_profile()?; // static method call
        println!("We’ll scrape: {}", profile_url);

        let rt = Runtime::new()?;
        rt.block_on(Self::run_async())?;

        Ok(())
    }
}
