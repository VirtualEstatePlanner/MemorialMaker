import IAudio from './IAudio'
import IPhoto from './IPhoto'
import ISong from './ISong'
import IVideo from './IVideo'

interface IContentItem {
  key: number // should be generated by UUID
  date: string | Date | null
  description: string | null
  content: IPhoto | IAudio | ISong | IVideo
  thumbnail: ImageBitmap
}

export default IContentItem
