<script setup lang="ts">
const { t } = useI18n()

</script>

<template>
  <div>
    <div class="py-4"></div>
    <div>
      sample checkbox
      <input type="checkbox" :label="t('intro.whats-your-name')" />
    </div>
    <p>
      placeholder for name from tauri store
    </p>
    <input
      id="input"
      :placeholder="t('intro.whats-your-name')"
      :aria-label="t('intro.whats-your-name')"
      type="text"
      autocomplete="false"
      p="x-4 y-2"
      w="250px"
      text="center"
      bg="transparent"
      border="~ rounded gray-200 dark:gray-700"
      outline="none active:none"
    >
    <label class="hidden" for="input">{{ t('intro.whats-your-name') }}</label>
  </div>
</template>

<route lang="yaml">
meta:
  layout: input
</route>
