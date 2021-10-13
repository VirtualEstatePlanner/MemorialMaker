//use image::ImageBuffer;

pub struct IPhoto {
  caption: String,
  //original: ImageBuffer
  // generateThumbnail: (photo: ImageBitmap) => ImageBitmap // move to utility-functions
}

pub fn new() -> IPhoto {
  IPhoto {
    caption: "photo-caption".to_string(),
    //original: ImageBuffer::new(0, 0),
  }
}
