<script setup lang="ts">
import { toRefs, ref, watch, watchEffect, defineProps } from 'vue';
const { modelValue } = defineProps({
    modelValue: {
        type: Number,
        default: 0
    }
})
const emit = defineEmits(['update:modelValue'])

const val = ref('' + modelValue)
watchEffect(() => {
    console.log('val changed', val.value)
    emit('update:modelValue', parseInt(val.value))
})
watchEffect(() => {
    console.log('modelValue changed', modelValue)
    val.value = '' + modelValue;
})

const focus = ref(false)

const onAdd = () => {
    console.log('onAdd', val.value)
    val.value = '' + ((parseInt(val.value) || 0) + 1);
    // emit('update:modelValue', modelValue + 1)
    // modelValue.value = '' + ((parseInt(modelValue.value) || 0) + 1);
}
const onSub = () => {
    console.log('onSub', val.value)
    const newValue = (parseInt(val.value) || 0) - 1;
    val.value = newValue < 0 ? '0' : '' + newValue;
    // emit('update:modelValue', newValue < 0 ? 0 : newValue)
}
const validateInt = (event: Event) => {
    let tVal = event.target.value.replace(/[^0-9]/g, '');
    tVal = tVal.replace(/^0+/, '');
    // 如果结果为空，则设置为 '0'
    if (tVal === '') {
        tVal = '0';
    }
    val.value = tVal;
}

</script>

<template>
    <div class="num-inp-container" :class="{
        'focus-num-inp-container': focus,
        'unfocus-num-inp-container': !focus
    }" ref="NumInpContainerRef" :tabindex="0" @focus="focus = true" @blur="focus = false">
        <!-- <input class="num-inp" :class="{ focus: focus }" type="text" @focus="() => focus = true" @blur="() => focus = false"
            :value="modelValue" @input="validateInt" maxlength="3"/> -->
            <input class="num-inp" :class="{ focus: focus }" type="text" @focus="() => focus = true" @blur="() => focus = false"
            v-model="val" @input="validateInt" maxlength="3"/>
        <div class="icon" :class="{
            'active-icon': focus,
            'unactive-icon': !focus
        }">
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24">
                <path
                    d="M12 2c5.523 0 10 4.477 10 10s-4.477 10-10 10S2 17.523 2 12S6.477 2 12 2zm0 1.5a8.5 8.5 0 1 0 0 17a8.5 8.5 0 0 0 0-17zm-1.25 9.94l4.47-4.47a.75.75 0 0 1 1.133.976l-.073.084l-5 5a.75.75 0 0 1-.976.073l-.084-.073l-2.5-2.5a.75.75 0 0 1 .976-1.133l.084.073l1.97 1.97l4.47-4.47l-4.47 4.47z">
                </path>
            </svg>
        </div>
        <div class="icon" :class="{
            'active-icon': focus,
            'unactive-icon': !focus
        }" @click="onSub">
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 28 28">
                <path
                    d="M14 2c6.627 0 12 5.373 12 12s-5.373 12-12 12S2 20.627 2 14S7.373 2 14 2zm0 1.5C8.201 3.5 3.5 8.201 3.5 14S8.201 24.5 14 24.5S24.5 19.799 24.5 14S19.799 3.5 14 3.5zm5.25 9.75a.75.75 0 0 1 0 1.5H8.75a.75.75 0 0 1 0-1.5h10.5z">
                </path>
            </svg>
        </div>
        <div class="icon" :class="{
            'active-icon': focus,
            'unactive-icon': !focus
        }" @click="onAdd">
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 28 28">
                <path
                    d="M14 2c6.627 0 12 5.373 12 12s-5.373 12-12 12S2 20.627 2 14S7.373 2 14 2zm0 1.5C8.201 3.5 3.5 8.201 3.5 14S8.201 24.5 14 24.5S24.5 19.799 24.5 14S19.799 3.5 14 3.5zM14 8a.75.75 0 0 1 .75.75v4.5h4.5a.75.75 0 0 1 0 1.5h-4.5v4.5a.75.75 0 0 1-1.5 0v-4.5h-4.5a.75.75 0 0 1 0-1.5h4.5v-4.5A.75.75 0 0 1 14 8z">
                </path>
            </svg>
        </div>
    </div>
</template>

<style scoped>
.num-inp-container {
    @apply px-3 w-full ease-in-out duration-300 flex flex-row justify-between items-center appearance-none bg-transparent outline-none rounded-lg;
    height: 2.25rem;
    min-height: 2.25rem;
    max-height: 2.25rem;
}

.focus-num-inp-container {
    border: 1px solid rgb(96, 219, 184);
}

.unfocus-num-inp-container {
    border: 1px solid rgb(145, 147, 145);
}

.unfocus-num-inp-container:hover {
    border: 1px solid rgb(199, 202, 198);
}

@keyframes focusCaretColor {
    from {
        caret-color: rgb(145, 147, 145);
    }

    to {
        caret-color: rgb(96, 219, 184);
    }
}

@keyframes blurCaretColor {
    from {
        caret-color: rgb(96, 219, 184);
    }

    to {
        caret-color: rgb(145, 147, 145);
    }
}

.num-inp {
    @apply h-full appearance-none bg-transparent outline-none ease-in-out duration-300;
}

.num-inp.focus {
    animation: focusCaretColor 0.3s ease-in-out forwards;
}

.num-inp:not(.focus) {
    animation: blurCaretColor 0.3s ease-in-out forwards;
}
</style>