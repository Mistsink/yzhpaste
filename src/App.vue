<script setup lang="ts">
import { onBeforeUnmount, onMounted } from 'vue';
import { RouterView } from 'vue-router'
import {useWindowSC, unuseWindowSC} from './services/shortcuts'

onMounted(useWindowSC)
onBeforeUnmount(unuseWindowSC)
</script>

<template>
  <div class="w-screen h-screen bg-gradient-to-r from-cyan-500 to-sky-500">
    <RouterView />
  </div>
</template>

<style scoped>
</style>