<script setup lang="ts">
import { onMounted, ref, watch, watchEffect } from 'vue';
import OptItem from './OptItem.vue';
import { NSwitch, NSlider, NInput, NInputNumber } from 'naive-ui'
import { config } from '@/stores/config'
import { storeToRefs } from 'pinia';
import type { CommonConfig } from '@/services/cmds';

watchEffect((onCleanUp) => {
    console.log('[OptGeneral.vue] config changed', config.value)
    console.log('onCleanUp: ', onCleanUp)
})

const hotkeys = config.value.hotkeys.active
const tmpHotkeys = ref('')
const onKeydown = (event: KeyboardEvent) => {
    event.preventDefault();
    // event.stopPropagation();
    console.log('onKeydown: ', event)
    tmpHotkeys.value = event.key
}
const onFocus = (event: FocusEvent) => {
    console.log('onFocus: ', event)
    // tmpHotkeys.value = ''
}
const onClearTmpHotkeys = () => {
    tmpHotkeys.value = ''
}

</script>
<template>
    <div class="opt-container">
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
            <NInputNumber v-model:value="config.record_limit_days"></NInputNumber>

        </OptItem>
        <OptItem title="快捷键设置">
            <div class="keys-inp-container" :tabindex="0" @focus="onFocus" @keydown="onKeydown">
                <div v-if="tmpHotkeys" class="hot-keys tmp-hot-keys">{{ tmpHotkeys }}</div>
                <div v-else class="hot-keys placehold-hot-keys">{{ hotkeys }}</div>
                <div class="cancel-icon" :class="tmpHotkeys ? 'active-cancel-icon' : 'unactive-cancel-icon'" @click="onClearTmpHotkeys"><svg
                        xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24">
                        <path
                            d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10s10-4.47 10-10S17.53 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8s8 3.59 8 8s-3.59 8-8 8zm3.59-13L12 10.59L8.41 7L7 8.41L10.59 12L7 15.59L8.41 17L12 13.41L15.59 17L17 15.59L13.41 12L17 8.41z">
                        </path>
                    </svg></div>
            </div>
        </OptItem>
    </div>
</template>

<style scoped>
.opt-container {
    @apply w-full h-full p-6 flex flex-row items-center space-x-8;
    background-color: rgb(25, 28, 27);
}

.keys-inp-container {
    @apply px-3 w-full ease-in-out duration-300 flex flex-row justify-between items-center appearance-none bg-transparent outline-none;
    border: 1px solid rgb(96, 219, 184);
    cursor: text;
    height: 2.25rem;
    min-height: 2.25rem;
    max-height: 2.25rem;
}

.keys-inp-container:focus {
    border: 1px solid rgb(108, 96, 219);

}

.hot-keys {
    /* @apply   ; */
}

.cancel-icon {
    @apply w-3 h-3 rounded-full ease-in-out duration-300 flex justify-center items-center;
}

.active-cancel-icon:hover {
    cursor: pointer;
}

.cancel-icon svg {
    width: 2rem;
    height: 2rem;
    @apply ease-in-out duration-300;
}
.active-cancel-icon svg {
    fill: rgba(96, 219, 184, 1);
}
.unactive-cancel-icon svg {
    fill: rgba(96, 219, 184, 0);
}

.keys-inp {
    @apply w-full h-full appearance-none bg-transparent outline-none;
    caret-color: rgb(96, 219, 184);
}
</style>