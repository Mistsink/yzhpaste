<script setup lang="ts">
import { ref, watchEffect, type CSSProperties } from 'vue';
import OptItem from './OptItem.vue';
import KeysInput from '../KeysInput.vue';
import NumInput from '../NumInput.vue';
import { NSwitch } from 'naive-ui'
import { config } from '@/stores/config'

watchEffect((onCleanUp) => {
    console.log('[OptGeneral.vue] config changed', config.value)
    console.log('onCleanUp: ', onCleanUp)
})

const onRecordLimitDaysConfirm = () => {
    config.value.record_limit_days = tmp_record_limit_days.value
}
const onRecordLimitDaysCancle = () => {
    tmp_record_limit_days.value = config.value.record_limit_days
}


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

    style.border = '1px solid rgb(145, 147, 145)'
    return style
}

const tmp_record_limit_days = ref(config.value.record_limit_days)


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
            <NumInput v-model="tmp_record_limit_days" :onConfirm="onRecordLimitDaysConfirm"
                :onCancle="onRecordLimitDaysCancle" :showConfirm="tmp_record_limit_days !== config.record_limit_days" />
        </OptItem>
        <OptItem title="快捷键设置[TODO]">
            <KeysInput />
        </OptItem>
    </div>
</template>

<style scoped>
.opt-container {
    @apply w-full h-full px-6 py-4 flex flex-row items-center space-x-8;
    background-color: rgb(25, 28, 27);
}
</style>