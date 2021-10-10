mod IAudio;
mod IPhoto;
mod ISong;
mod IVideo;

use date::DateTime;
use date::Utc;
use image::ImageBuffer;
use uuid::Uuid;

pub struct IContentItem {
  key: &str,
  date: date::DateTime<date::Utc>,
  description: &str,
  content: Vec<UContentItem>,
  thumbnail: ImageBuffer,
}

pub fn new() -> IContentItem {
  IContentItem {
    key: Uuid::new_v5().to_string(),
    date: Utc::now(),
    description: "default-content-item-description",
    content: ["default-content"]
      .iter()
      .map(|s| UContentItem::new(s))
      .collect(),
    thumbnail: ImageBuffer::new(0, 0),
  }
}