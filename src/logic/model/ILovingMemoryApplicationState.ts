import IAppPreferences from './IAppPreferences'
import IAudio from './IAudio'
import ICharity from './ICharity'
import IMemorializedPerson from './IMemorializedPerson'
import IPhoto from './IPhoto'
import IService from './IService'
import ISong from './ISong'
import IVideo from './IVideo'

interface ILovingMemoryApplicationState {
  preferences: IAppPreferences
  memorialized: IMemorializedPerson
  services: {
    burial: IService
    cremation: IService
    funeral: IService
    scattering: IService
  }
  charities: ICharity[]
  contentItems: {
    photos: IPhoto[]
    videos: IVideo[]
    songs: ISong[]
    audioClips: IAudio[]
  }
}

export default ILovingMemoryApplicationState
