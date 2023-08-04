<script setup lang="ts">
import { onBeforeUnmount, onMounted, onUpdated } from 'vue';
import { RouterView } from 'vue-router'
import { useWindowSC, unuseWindowSC } from '@/services/shortcuts'
import { app, process } from '@tauri-apps/api'
import { appWindow } from '@tauri-apps/api/window'
import { cmd_print } from '@/services/cmds';
import { initCfg, useStoreCfg, useWatchCfg } from '@/stores/config';
import { recordsStoreInit } from '@/stores/records';

onMounted(async () => {
  await initCfg()

  await recordsStoreInit();
  useWatchCfg()

  await cmd_print('on mounted')
  useWindowSC()
})
onBeforeUnmount(async () => {
  await cmd_print('on before unmount')
  unuseWindowSC()
})

// onUpdated(async () => {
//   await cmd_print('on updated')
//   console.log(app)
//   console.log(process)
//   console.log(appWindow)
// })
</script>

<template>
  <div class="w-screen h-screen app-global">
    <Suspense>
      <RouterView v-slot="{ Component }">
        <!-- <KeepAlive>
          <component :is="Component" />
        </KeepAlive> -->
        <component :is="Component" />
      </RouterView>
    </Suspense>
  </div>
</template>

<style scoped>
.app-global {
  background-color: rgb(25, 28, 27);
}
</style>