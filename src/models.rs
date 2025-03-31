use chrono::DateTime;

pub struct VideoComments{
    comment_date: chrono::DateTime<chrono::Utc>,
    comment_text: String,
    comment_user: String,
    comment_likes: u64,
    comment_id: u64,
    comment_parent_id: u64,
}

pub struct VideoStats{
    video_url: String,
    n_likes: u64,
    n_shares: u64,
    post_date: chrono::DateTime<chrono::Utc>, // give the right format here (unix + timezone?)
    comments: HashSet<VideoComments>
}

pub struct ScrapedProfile {
    username: String,
    bio: String,
    n_following: u64,
    n_followers: u64,
    n_likes: i64,
    public_videos: HashMap<u64, VideoStats>,
}