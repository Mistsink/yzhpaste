import { defineStore } from 'pinia'
import { cmd_get_common_config, cmd_set_common_config, type CommonConfig } from '@/services/cmds'
import { ref, reactive, watchEffect, watch } from 'vue'
import { isEnabled, enable, disable } from "tauri-plugin-autostart-api";


export const useStoreCfg = defineStore('config', () => {
  const config = ref({} as CommonConfig)
  const initCfg = async () => {
    console.log('store init');
    
    config.value = reactive(await cmd_get_common_config())
    console.log(config);
    
  }

  return {
    config,
    initCfg
  }
})

export const useWatchCfgStore = () => {
  const cfgStore = useStoreCfg()
  watch(cfgStore.config, async (newCfg: CommonConfig, oldCfg: CommonConfig) => {
    console.log('watch cfg', newCfg, oldCfg);
    await cmd_set_common_config(newCfg)
  })
}

export const config = ref({} as CommonConfig)
export const initCfg = async () => {
  config.value = await cmd_get_common_config()
  updateAutostart(config.value.enable_auto_launch)
}

const updateAutostart = async (cfg_enable: boolean) => {
  if (cfg_enable === await isEnabled()) return;

  if (cfg_enable) {
    await enable();
  } else {
    await disable();
  }
}

export const useWatchCfg = () => {
  watch(config.value, async (newCfg: CommonConfig, oldCfg: CommonConfig) => {
    console.log('[store/config] watch cfg', newCfg, oldCfg);
    await updateAutostart(newCfg.enable_auto_launch);
    await cmd_set_common_config(newCfg);
  })
}
