interface IPhoto {
  caption: String
  original: ImageBitmap
  generateThumbnail: (photo: ImageBitmap) => ImageBitmap // move to utility-functions
}

export default IPhoto
