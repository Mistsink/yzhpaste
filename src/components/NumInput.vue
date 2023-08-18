<script setup>
import { ref, toRefs, watch, watchEffect } from 'vue';

const focus = ref(false)
const props = defineProps({
    modelValue: {
        type: Number,
        default: 0
    },
    onConfirm: {
        type: Function,
        default: () => { console.log('onConfirm') }
    },
    onCancle: {
        type: Function,
        default: () => { console.log('onCancle') }
    },
    showConfirm: {
        type: Boolean,
        default: true
    }
})
const emit = defineEmits(['update:modelValue'])


const onAdd = () => {
    emit('update:modelValue', (parseInt(props.modelValue) || 0) + 1)
}
const onSub = () => {
    const newValue = (parseInt(props.modelValue) || 0) - 1;
    emit('update:modelValue', newValue < 0 ? 0 : newValue)
}
const validateInt = (event) => {
    let tVal = event.target.value.replace(/[^0-9]/g, '');
    tVal = tVal.replace(/^0+/, '');
    if (tVal === '') {
        tVal = '0';
    }
    emit('update:modelValue', parseInt(tVal))
}

</script>

<template>
    <div class="num-inp-container" :class="{
        'focus-num-inp-container': focus,
        'unfocus-num-inp-container': !focus
    }" ref="NumInpContainerRef" :tabindex="0" @focus="focus = true" @blur="focus = false">
        <input class="num-inp" :class="{ focus: focus }" type="text" @focus="() => focus = true" @blur="() => focus = false"
            :value="props.modelValue" @input="validateInt" maxlength="3" />
        <div class="icon" :class="{
            'active-icon': focus && props.showConfirm,
            'unactive-icon': (!focus) || (!props.showConfirm)
        }" @click="props.onConfirm">
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24">
                <path
                    d="M12 2c5.523 0 10 4.477 10 10s-4.477 10-10 10S2 17.523 2 12S6.477 2 12 2zm0 1.5a8.5 8.5 0 1 0 0 17a8.5 8.5 0 0 0 0-17zm-1.25 9.94l4.47-4.47a.75.75 0 0 1 1.133.976l-.073.084l-5 5a.75.75 0 0 1-.976.073l-.084-.073l-2.5-2.5a.75.75 0 0 1 .976-1.133l.084.073l1.97 1.97l4.47-4.47l-4.47 4.47z">
                </path>
            </svg>
        </div>
        <div class="icon" :class="{
            'active-icon': focus && props.showConfirm,
            'unactive-icon': (!focus) || (!props.showConfirm)
        }" @click="props.onCancle">
            <svg xmlns="http://www.w3.org/2000/svg"
                xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24">
                <path
                    d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10s10-4.47 10-10S17.53 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8s8 3.59 8 8s-3.59 8-8 8zm3.59-13L12 10.59L8.41 7L7 8.41L10.59 12L7 15.59L8.41 17L12 13.41L15.59 17L17 15.59L13.41 12L17 8.41z">
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
    @apply w-full h-full appearance-none bg-transparent outline-none ease-in-out duration-300;
}

.num-inp.focus {
    animation: focusCaretColor 0.3s ease-in-out forwards;
}

.num-inp:not(.focus) {
    animation: blurCaretColor 0.3s ease-in-out forwards;
}
</style>


