<template>
  <div style="display:flex; flex-direction: column; text-align: center;">
    <div class="box" :style="`background-color: var(--${props.color});`" />
    <span style="font-size: 90%">{{ props.color }}</span>
  </div>
</template>
<script setup lang="ts">
  const props = defineProps(["color"])
</script>

<style lang="css" scoped>
  .box {
    width: 100px;
    height: 100px;
  }
</style>
