<script setup lang="ts">
import { onBeforeUnmount, onMounted, onUpdated } from 'vue';
import { RouterView } from 'vue-router'
import { useWindowSC, unuseWindowSC } from './services/shortcuts'
import { app, process } from '@tauri-apps/api'
import { appWindow } from '@tauri-apps/api/window'
import { cmd_print } from './services/cmds';

onMounted(async () => {
  await cmd_print('on mounted')
  await useWindowSC()
})
onBeforeUnmount(async () => {
  await cmd_print('on before unmount')
  await unuseWindowSC()
})

onUpdated(async () => {
  await cmd_print('on updated')
  console.log(app)
  console.log(process)
  console.log(appWindow)
})
</script>

<template>
  <div class="w-screen h-screen bg-gradient-to-r from-cyan-500 to-sky-500">
    <RouterView />
  </div>
</template>

<style scoped></style>