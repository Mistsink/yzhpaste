<script setup lang="ts">
import { ref, watch } from 'vue';
import OptItem from './OptItem.vue';
import { NSwitch, NSlider } from 'naive-ui'

const general = ref({
    autoStart: false,
    directPaste: false,
    pureText: false,
    historyCapacityIdx: 1
})

const marks = {
    0: '天',
    20: '周',
    40: '月',
    60: '季',
    80: '年',
    100: '无限'
}


watch(() => ({ ...general.value }), (newConfig, oldConfig) => {
    console.log(newConfig, oldConfig)
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
                <NSwitch v-model:value="general.autoStart" />
                开机自启动 YzhPaste - {{ general.autoStart }}
            </div>
        </OptItem>
        <OptItem title="集成">
            <div>
                <NSwitch v-model:value="general.directPaste" />
                []开启 Direct Paste - {{ general.directPaste }}
            </div>
            <div>
                <NSwitch v-model:value="general.pureText" />
                []粘贴为纯文本 - {{ general.pureText }}
            </div>
        </OptItem>
        <OptItem title="历史记录容量">
            <NSlider class="w-full" v-model:value="general.historyCapacityIdx" :marks="marks" step="mark" :tooltip="false"/>
        </OptItem>
    </div>
</template>
