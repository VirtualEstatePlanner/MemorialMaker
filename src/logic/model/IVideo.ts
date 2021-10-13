interface IVideo {
  key: number // should be generated by UUID
  caption: String
  // generateThumbnail to an array of frames selected based on the number of frames in the video
  thumbnail: ImageBitmap[]
  // probably requires video reading libaray such as ffmpeg
  videoData: URL // | file location | mp4 | stream data
}

export default IVideo
