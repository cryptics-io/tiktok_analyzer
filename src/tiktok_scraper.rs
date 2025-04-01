use fantoccini::{Client, ClientBuilder};
use anyhow::Result;
use tokio::{runtime::Runtime, stream};
use std::{fmt, io::{self, Error, Write}};
use regex::Regex;
use std::collections::{HashSet, HashMap};

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

    pub fn extract_video_urls(html: & str) -> HashSet<String>{
        let mut video_urls: HashSet<String> = HashSet::new();
        // Same pattern as your JS version
        let re = Regex::new(r"https://www\.tiktok\.com/@[^/]+/video/\d+").unwrap();

        for mat in re.find_iter(html) {
            video_urls.insert(mat.as_str().to_string());
        }

        video_urls

    }

    pub async fn scrape_profile(&self, profile_url: &str) -> Result<()> {

        self.client.goto(profile_url).await;
        let page_source= self.client.source().await?;
        let video_urls = TiktokScraper::extract_video_urls(&page_source);
        println!("Found {} Videos", video_urls.len());
        for url in video_urls {
            println!("Found {}", url);
        }

        Ok(())
    }

    

    pub async fn run_async(profile_url: &str) -> Result<()> {
        let scraper = Self::new().await?;
        println!("Scraper initialized.");
        scraper.scrape_profile(profile_url).await;

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
        rt.block_on(Self::run_async(&profile_url))?;

        Ok(())
    }
}
