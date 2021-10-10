mod IAudio;
mod IPhoto;
mod ISong;
mod IVideo;

pub struct IContentItem {
  key: &str,
  date: date::DateTime<date::Utc>,
  description: &str,
  content: &str, // should be union of IAudio, IPhoto, ISong, IVideo
  thumbnail: image::ImageBuffer,
}
