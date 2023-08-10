<script setup lang="ts">
import { onMounted, ref, watch, watchEffect, type CSSProperties } from 'vue';
import OptItem from './OptItem.vue';
import KeysInput from '../KeysInput.vue';
import NumInput from '../NumInput.vue';
import { NSwitch, NSlider, NInput, NInputNumber } from 'naive-ui'
import { config } from '@/stores/config'
import { storeToRefs } from 'pinia';
import type { CommonConfig } from '@/services/cmds';

watchEffect((onCleanUp) => {
    console.log('[OptGeneral.vue] config changed', config.value)
    console.log('onCleanUp: ', onCleanUp)
})

const railStyle = ({
        focused,
        checked
      }: {
        focused: boolean
        checked: boolean
      }) => {
        const style: CSSProperties = {}
        if (checked) {
          style.background = 'rgb(96, 219, 184)'
        } else {
          style.background = 'rgb(29, 38, 35)'
        }
        return style
      }


</script>
<template>
    <div class="opt-container">
        <OptItem title="启动">
            <div>
                <NSwitch :rail-style="railStyle" v-model:value="config.enable_auto_launch" />
                开机自启 YzhPaste
            </div>
        </OptItem>
        <OptItem title="集成">
            <div>
                <NSwitch :rail-style="railStyle" v-model:value="config.enable_auto_paste" />
                Auto Paste
            </div>
        </OptItem>
        <OptItem title="历史记录时长(天)">
            <NumInput v-model="config.record_limit_days" />
        </OptItem>
        <OptItem title="快捷键设置">
            <KeysInput />
        </OptItem>
    </div>
</template>

<style scoped>
.opt-container {
    @apply w-full h-full p-6 flex flex-row items-center space-x-8;
    background-color: rgb(25, 28, 27);
}
</style>