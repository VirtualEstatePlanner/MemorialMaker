import { acceptHMRUpdate, defineStore } from 'pinia'
import ILovingMemoryApplicationState from '~/logic/model/ILovingMemoryApplicationState'
import defaultStateObject from '~/logic/constants/defaultStateObject'

const useAppState = defineStore('appState', () => {
  const currentState: ILovingMemoryApplicationState = reactive(defaultStateObject)
  const previousAppStates = reactive(new Set<ILovingMemoryApplicationState>())

  const usedAppStates = computed(() => Array.from(previousAppStates))
  const otherAppStates = computed(() => usedAppStates.value.filter((state: ILovingMemoryApplicationState) => state !== currentState))

  function setNewAppState(state: ILovingMemoryApplicationState) {
    if (currentState) previousAppStates.add(currentState)

    currentState.preferences.appWide.displayOnlyMiddleInitial = state.preferences.appWide.displayOnlyMiddleInitial
  }
  return {
    setNewAppState,
    otherAppStates,
    currentState,
  }
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(useAppState, import.meta.hot))

export default useAppState
