<script setup lang="ts">
import { onMounted, ref, watch, watchEffect } from 'vue';
import OptItem from './OptItem.vue';
import { NSwitch, NSlider, NInputNumber } from 'naive-ui'
import { config } from '@/stores/config'
import { storeToRefs } from 'pinia';
import type { CommonConfig } from '@/services/cmds';

// onMounted(() => {
//     window.cfg = cfg
// })

watchEffect((onCleanUp) => {
    console.log('[OptGeneral.vue] config changed', config.value)
    console.log('onCleanUp: ', onCleanUp)
})
</script>
<template>
    <div class="
    w-full h-full p-6
    flex flex-row justify-even items-center
    bg-green-200
    space-x-8
    ">
        <OptItem title="启动">
            <div>
                <NSwitch v-model:value="config.enable_auto_launch" />
                开机自启动 YzhPaste - {{ config.enable_auto_launch }}
            </div>
        </OptItem>
        <OptItem title="集成">
            <div>
                <NSwitch v-model:value="config.enable_auto_paste" />
                [TODO] 开启 Auto Paste - {{ config.enable_auto_paste }}
            </div>
            <div>
                <NSwitch v-model:value="config.enable_delete_confirm" />
                [TODO] 删除时需要确认 - {{ config.enable_delete_confirm }}
            </div>
        </OptItem>
        <OptItem title="历史记录时长(天)">
            <!-- <NSlider class="w-full" v-model:value="cfg.record_limit_days" :marks="marks" step="mark" :tooltip="false"/> -->
            <NInputNumber v-model:value="config.record_limit_days" ></NInputNumber>
            
        </OptItem>
    </div>
</template>