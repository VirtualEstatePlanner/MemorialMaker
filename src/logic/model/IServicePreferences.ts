import TServiceType from './TServiceType'

interface IServicePreferences {
  type: TServiceType
  areHaving: Boolean
  areLivestreaming: Boolean
  livestreamIsPublic: Boolean
}

export default IServicePreferences
