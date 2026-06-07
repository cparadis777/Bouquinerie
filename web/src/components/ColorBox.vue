<template>
  <div :style="'background-color: var(' + props.color + ');'" />
</template>
<script setup lang="ts">
  const props = defineProps(["color"])
</script>
<style lang="css" scoped>
  div {
    width: 100px;
    height: 100px;
  }
</style>
