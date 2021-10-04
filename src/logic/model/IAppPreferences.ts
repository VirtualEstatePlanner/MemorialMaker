import IServicePreferences from './IServicePreferences'

interface IAppPreferences {
  appWide: {
    displayOnlyMiddleInitial: Boolean
  }
  servicePreferences: {
    burial: IServicePreferences
    cremation: IServicePreferences
    funeral: IServicePreferences
    scattering: IServicePreferences
  }
}

export default IAppPreferences
