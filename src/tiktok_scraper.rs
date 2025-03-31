use fantoccini::{Client, ClientBuilder};
use anyhow::Result;
use tokio::{runtime::Runtime, sync::TryAcquireError};
use std::{f32::MANTISSA_DIGITS, fmt, io::{self, Error, Write}};

#[derive(Debug)]
enum ScraperError {
    InvalidInput(String),
    IoError(std::io::Error),
}

impl From<std::io::Error> for ScraperError {
    fn from(e: std::io::Error) -> Self {
        ScraperError::IoError(e)
    }
}

impl std::fmt::Display for ScraperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScraperError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ScraperError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for ScraperError {}


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

    fn ask_userinput_profile() -> Result<String, ScraperError> {
        let mut input = String::new();
        print!("Enter TikTok profile URL: ");
        io::stdout().flush()?; // ensures the prompt is printed before user input
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();

        match trimmed.contains("tiktok.com"){
            true => Ok(trimmed.to_string()),
            false => Err(ScraperError::InvalidInput(trimmed.to_string()))
        }
    }

    fn retry_till_valid_userinput() -> Result<String>{
        loop{
            match TiktokScraper::ask_userinput_profile(){
                Ok(userinput) => return Ok(userinput),
                Err(ScraperError::InvalidInput(msg)) => {
                    println!("Invalid input: '{}'", msg)
                },
                Err(ScraperError::IoError(msg)) => {
                    println!("Io Error: '{}'", msg)
                },
            }
        }
    }

/*     Todo: 
    - Build create profile
    - Visit profile
    - Check all videos
    - Scrape all comments, number of videos, likes, views per videos
    - Display results 
    - Save results in db */



    pub fn run_scraper() -> Result<()> {
        let profile_url = TiktokScraper::retry_till_valid_userinput()?; // static method call
        println!("We’ll scrape: {}", profile_url);

        let rt = Runtime::new()?;
        rt.block_on(Self::run_async())?;

        Ok(())
    }
}
