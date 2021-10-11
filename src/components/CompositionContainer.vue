<template>
  <div>
    <router-view>
    </router-view>
  </div>
</template>

<script lang="ts">

export default {
  setup() {
    const t = useI18n()
    // eslint-disable-next-line no-console
    console.log(t)
  },

}
</script>

<style>
</style>
