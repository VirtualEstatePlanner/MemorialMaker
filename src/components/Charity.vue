<template>
  <div>
    <CharityButton :charity="charity" />
    {{ charity.name !== 'undefined' ? charity.name : 'NAME THIS CHARITY' }} works to {{ charity.issue !== 'undefined' ? charity.issue : `ADD THIS CHARITY'S ISSUE OF CHOICE` }}.
  </div>
</template>

<script lang="ts">

export default {
  props: {
    charity: {
      type: Object, // as Vue.PropType<{ charity: ICharity }>,
      required: true,
    },
  },
}
</script>

<style>
</style>
