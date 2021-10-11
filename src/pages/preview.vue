<script setup lang="ts">
const { t } = useI18n()
</script>

<template>
  <div>
    <div class="py-4"></div>
    <p>
      placeholder for value from tauri state
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
    <div>
      <button
        class="m-3 text-sm btn"
        :disabled="false"
      >
        {{ t('button.go') }}
      </button>
    </div>
  </div>
</template>

<route lang="yaml">
meta:
  layout: beta
</route>
