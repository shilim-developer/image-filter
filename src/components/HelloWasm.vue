<template>
  <button @click="greet('webAssembly')">hello wasm</button>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import init, { greet } from 'wasm-image-filter';

export default defineComponent({
  name: 'HelloWasm',
  setup: () => {
    // init wasm
    init();
    return { greet }
  }
})
</script>
