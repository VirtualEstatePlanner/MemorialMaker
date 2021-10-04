<template>
  <div>
    <h2>
      <a
        :key="charity.name"
        :charity="charity"
        class="button"
        target="_blank"
        :href="charity.url"
      >
        Donate to {{ charity.name !== 'undefined' ? charity.name : 'NAME THIS CHARITY' }}
      </a>
    </h2>
  </div>
</template>

<script lang="ts">
export default {
  props: {
    charity: {
      type: Object,
      required: true,
    },
  },
}
</script>

<style>
</style>
