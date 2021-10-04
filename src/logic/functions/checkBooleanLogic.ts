import IServicePreferences from '~/logic/model/IServicePreferences'
import ILovingMemoryApplicationState from '~/logic/model/ILovingMemoryApplicationState'

/*
// function to check that no streams exist for nonexistent events
const checkStreamLogic: Function = (BooleansForStream: Boolean[]): void => {
  if (BooleansForStream[1]) {
    if (!BooleansForStream[0]) {
      throw new Error(``)
    }
  }
}
*/

const checkPreferencesLogic = (state: ILovingMemoryApplicationState): void => {
  if (state.charities.length >= 7) state.charities.slice(0, 7)
  // check that ashes should exist to scatter if scattering
  if (state.preferences.servicePreferences.scattering.areHaving) {
    if (state.preferences.servicePreferences.cremation.areHaving)
      throw new Error('should have ashes to scatter')
  }
  const servicesArray: IServicePreferences[] = [
    state.preferences.servicePreferences.burial,
    state.preferences.servicePreferences.cremation,
    state.preferences.servicePreferences.funeral,
    state.preferences.servicePreferences.scattering,
  ]
  servicesArray.forEach((servicePrefs) => {
    if (servicePrefs.areLivestreaming) {
      if (servicePrefs.areHaving)
        // eslint-disable-next-line no-console
        console.log(`are livestreaming ${servicePrefs.type}`)
      else
        throw new Error(`you must have a ${servicePrefs.type} in order to livestream it`)
    }
  })
}

export default checkPreferencesLogic
