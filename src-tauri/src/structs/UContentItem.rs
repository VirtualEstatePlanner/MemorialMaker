use crate::structs::IAudio;
use crate::structs::IPhoto;
use crate::structs::ISong;
use crate::structs::IVideo;

pub const new_audio_template: IAudio::IAudio = IAudio::new();
pub const new_song_template: ISong::ISong = ISong::new();
pub const new_photo_template: IPhoto::IPhoto = IPhoto::new();
pub const new_video_template: IVideo::IVideo = IVideo::new();

pub enum UContentItem {
  NewAudioTemplate,
  NewSongTemplate,
  NewPhotoTemplate,
  NewVideoTemplate,
}
