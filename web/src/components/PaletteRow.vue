<template>
  <div style="display: flex;">
    <ColorBox v-for="shade in shades" :color=shade />
  </div>
</template>

<script setup lang="ts">
  import ColorBox from '../components/ColorBox.vue';

  const props = defineProps(["color", "count"])

  function shade_generator(color: string, ...values: number[]) {
    return values.map(v => `--${color}-${v}`)
  }


  const range = (start: number, end: number, step = 100) =>
    Array.from({ length: Math.floor((end - start) / step) + 1 }, (_, i) => start + i * step)

  const shades = shade_generator(props.color, ...range(100, 900, 100).reverse())

</script>
