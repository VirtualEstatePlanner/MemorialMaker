<template>
  <li
    :key="agendaItem.key"
    :agendaItem="$props.agendaItem"
  >
    {{ agendaItem.description !== `undefined` || `` ? agendaItem.description : `ADD THIS AGENDA ITEM'S DESCRIPTION` }} - {{ agendaItem.value !== `undefined` || `` ? agendaItem.value : `ADD THIS AGENDA ITEM'S VALUE` }}.
  </li>
</template>

<script lang="ts">

export default {
  props: {
    agendaItem: {
      type: Object,
      required: true,
    },
  },
}
</script>

<style>
</style>
