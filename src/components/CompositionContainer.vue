<template>
  <div>
    <router-view>
    </router-view>
  </div>
</template>

<script lang="ts">
import useAppState from '~/stores/appState'

export default {
  setup() {
    const appState = useAppState()
    const t = useI18n()
    // eslint-disable-next-line no-console
    console.log(appState)
    // eslint-disable-next-line no-console
    console.log(t)
  },

}
</script>

<style>
</style>
