<template>
  <div v-if="$props.state.areLivestreamingFuneral">
    <h2>
      <a
        class="button"
        target="_blank"
        rel="noopener"
        :href="$props.state.funeralLivestreamURL"
      >
        Funeral Livestream - {{ $props.state.funeralLivestreamURL ? '' : 'VUE_APP_FUNERAL_LIVESTREAM_URL' }} {{ $props.state.funeralTime ? $props.state.funeralTime : 'VUE_APP_FUNERAL_TIME' }}</a>
    </h2>
    <LivestreamCredentials :state="$props.state"></LivestreamCredentials>
    <p>The funeral service is being held at {{ $props.state.funeralStreetAddress && $props.state.funeralCityAddress && $props.state.funeralStateAddress && $props.state.funeralZipAddress ? $props.state.funeralAddress : 'VUE_APP_FUNERAL_every-field_ADDRESS' }} on {{ !$props.state.funeralDate.substr('20') ? $props.state.funeralDate : 'VUE_APP_FUNERAL_DATE' }} at {{ $props.state.funeralTime.substr(':') ? $props.state.funeralTime : 'VUE_APP_FUNERAL_TIME' }}.</p>
  </div>
</template>

<script lang="ts">
import LivestreamCredentials from './LivestreamCredentials.vue'
export default {
  components: {
    LivestreamCredentials,
  },
  props: {
    state: { type: Object, required: true },
  },
}
</script>

<style>
</style>
