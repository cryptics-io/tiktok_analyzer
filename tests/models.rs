use tiktok_analyzer::models::*;
use chrono::Utc;

#[test]
fn initialize_video_comment() {
    let now = Utc::now();
    let vc1 = VideoComment {
        comment_date: now,
        comment_text: String::from("This is a test comment"),
        comment_user: String::from("test_user123"),
        comment_likes: 42,
        comment_id: 1,
        comment_parent_id: 0,
    };

    let vc2 = VideoComment::new(
        now,
        String::from("This is a test comment"),
        String::from("test_user123"),
        42,
        1,
        0,
    );

    assert_eq!(vc1.comment_text, vc2.comment_text);
    assert_eq!(vc1, vc2)
}