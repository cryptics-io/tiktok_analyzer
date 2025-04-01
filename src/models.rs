use chrono::DateTime;
use std::collections::HashMap;

#[derive(Debug)]
pub struct VideoComment{
    pub comment_date: chrono::DateTime<chrono::Utc>,
    pub comment_text: String,
    pub comment_user: String,
    pub comment_likes: u64,
    pub comment_id: u64,
    pub comment_parent_id: u64,
}

#[allow(dead_code)]
impl VideoComment{
    pub fn new(
        comment_date: chrono::DateTime<chrono::Utc>,
        comment_text: String,
        comment_user: String,
        comment_likes: u64,
        comment_id: u64,
        comment_parent_id: u64,
    ) -> Self {
        Self{
            comment_date,
            comment_text,
            comment_user,
            comment_likes,
            comment_id,
            comment_parent_id,
        }
    }

    pub fn display(&self){
        println!("{}", self.comment_user);
        println!("{}", self.comment_text);
        println!("{}     Reply     ♥ {}", self.comment_date, self.comment_likes);
    }

}


// Not sure yet if we have access to the id TikTok uses
// Same comment can have different number of likes etc. in different
// points in time. 
impl PartialEq for VideoComment{
    fn eq(&self, other: &Self) -> bool{
        //self.comment_id == other.comment_id && 
        self.comment_text == other.comment_text && 
        self.comment_user == other.comment_user
    }
}

impl Eq for VideoComment{}

pub struct VideoStats{
    pub video_id: u64,
    pub video_url: String,
    pub n_likes: u64,
    pub n_shares: u64,
    pub post_date: chrono::DateTime<chrono::Utc>, // give the right format here (unix + timezone?)
    pub comments: HashMap<u64,VideoComment>
}

impl VideoStats{
    pub fn new(
        video_id: u64,
        video_url: String,
        n_likes: u64,
        n_shares: u64,
        post_date: chrono::DateTime<chrono::Utc>, // give the right format here (unix + timezone?)
        comments: HashMap<u64,VideoComment>
    ) -> Self {
        Self { 
            video_id, 
            video_url, 
            n_likes, 
            n_shares, 
            post_date, 
            comments,
        }
    }

    // Nested comments implemented using parent_id
    pub fn add_comment(&mut self, comment: VideoComment) {
        self.comments.insert(comment.comment_id, comment);
    }

    
}

pub struct ScrapedProfile {
    profile_id: u64, //=user_id
    username: String,
    bio: String,
    n_following: u64,
    n_followers: u64,
    n_likes: i64,
    public_videos: HashMap<u64, VideoStats>,
}

