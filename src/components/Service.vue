<template>
  <div>
    <h2>
      {{ $props.service.event.contact.type !== null ? $props.service.event.contact.type : `SELECT A SERVICE TYPE FROM THE DROP-DOWN MENU` }}
      {{ $props.service.event.contact.name !== null || undefined || `undefined` || `` ? $props.service.event.contact.name : `NAME THIS SERVICE` }}
      {{ $props.service.event.contact.location !== null || undefined || `undefined` || `` ? `, ` + $props.service.event.contact.location : `ADD THE LOCATION FOR THIS SERVICE` }}
      {{ $props.service.event.contact.time !== null || undefined || `undefined` || `` ? `, ` + $props.service.event.contact.time : `ADD THE TIME FOR THIS SERVICE` }}
      {{ $props.service.event.contact.additional_info !== null || undefined || `undefined` || `` ? ` - ` + $props.service.event.contact.additional_info : `ADD INFORMATION ABOUT YOUR SERVICE HERE` }}
      <div id="array-rendering">
        Programme
        <ul
          v-for="item in service.programme"
          :key="item.key"
        >
          <AgendaItem
            :key="item.key"
            :description="item.description"
            :value="item.value"
          />
        </ul>
      </div>
      {{ $props.service.event.contact !== null || undefined ? `Contact: ` + $props.service.event.contact.name.first + $props.service.event.contact.name.last : `ADD CONTACT INFORMATION HERE` }}
    </h2>
  </div>
</template>

<script lang="ts">

export default {
  props: {
    service: {
      type: Object,
      required: true,
    },
  },
}
</script>

<style>
</style>
