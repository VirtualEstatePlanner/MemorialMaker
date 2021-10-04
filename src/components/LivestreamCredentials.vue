<template>
  <div>
    <ul
      v-if="!$props.state.funeralLivestreamIsPublic"
      class="fa-ul"
    >
      <ListItem>Username: <span class="highlight">{{ $props.state.funeralLivestreamUsername ? $props.state.funeralLivestreamUsername : 'VUE_APP_FUNERAL_LIVESTREAM_USERNAME' }}</span> <CopyButton :text="$props.state.funeralLivestreamUsername" /></ListItem>
      <ListItem>Password: <span class="highlight">{{ $props.state.funeralLivestreamPassword ? $props.state.funeralLivestreamPassword : 'VUE_APP_FUNERAL_LIVESTREAM_PASSWORD' }}</span> <CopyButton :text="$props.state.funeralLivestreamPassword" /></ListItem>
    </ul>
  </div>
</template>

<script lang="ts">
export default {
  components: {
  },
  props: {
    state: {
      type: Object,
      required: true,
    },
  },
}
</script>

<style>
</style>
