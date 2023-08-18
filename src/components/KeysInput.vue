<script setup lang="ts">
import { config } from '@/stores/config';
import { Code2Number, Key2Number } from '@/utils/keyboard';
import { ref } from 'vue';

const focus = ref(false)
const KeysInpRef = ref<HTMLElement | null>(null)
const tmpHotkeys = ref('')
let keysPressed: Record<string, string> = {}
const onKeydown = (event: KeyboardEvent) => {
    event.preventDefault();
    event.stopPropagation();
    console.log('onKeydown: ', event)
    if (event.metaKey) {
        return
    }

    let key = event.code;
    if (key.startsWith('Key')) {
        key = key.slice(3)
    } else if (key.startsWith('Digit')) {
        key = key.slice(5)
    } else if (key.startsWith('Alt')) {
        key = 'Alt'
    } else if (key.startsWith('Control')) {
        key = 'Ctrl'
    } else if (key.startsWith('Shift')) {
        key = 'Shift'
    } else if (key.startsWith(' ')) {
        key = 'Space'
    }

    if (keysPressed[event.code] == null) { // 检测按键是否已被按下
        keysPressed[event.code] = key
    }

    tmpHotkeys.value = Object.keys(keysPressed).sort((a: string, b: string) => Code2Number[a as keyof typeof Code2Number] - Code2Number[b as keyof typeof Code2Number]).reduce((pre, key) => `${pre}+${keysPressed[key]}`, '').slice(1)
}
function clearKeys() {
    keysPressed = {};
}
const onClearTmpHotkeys = () => {
    tmpHotkeys.value = ''
    keysPressed = {}
}
const onConfirmHotkeys = () => {
    // TODO
    if (!KeysInpRef.value) {
        return
    }
    console.log('onConfirmHotkeys: ', tmpHotkeys.value)
    config.value.hotkeys.active = tmpHotkeys.value
    onClearTmpHotkeys()
    KeysInpRef.value.blur()
}
</script>
<template>
    <div class="keys-inp-container" ref="KeysInpRef" :tabindex="0" @focus="focus = true" @blur="focus = false"
        @keydown="onKeydown" @keyup="clearKeys">
        <div v-if="tmpHotkeys" class="hot-keys tmp-hot-keys">{{ tmpHotkeys }}</div>
        <div v-else class="hot-keys placehold-hot-keys">{{ config.hotkeys.active }}</div>
        <div class="cancel-icon" :class="{
            'active-cancel-icon': focus && tmpHotkeys,
            'unactive-cancel-icon': !tmpHotkeys,
            'blur-active-cancel-icon': !focus && tmpHotkeys
        }" @click="onConfirmHotkeys">
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24">
                <path
                    d="M12 2c5.523 0 10 4.477 10 10s-4.477 10-10 10S2 17.523 2 12S6.477 2 12 2zm0 1.5a8.5 8.5 0 1 0 0 17a8.5 8.5 0 0 0 0-17zm-1.25 9.94l4.47-4.47a.75.75 0 0 1 1.133.976l-.073.084l-5 5a.75.75 0 0 1-.976.073l-.084-.073l-2.5-2.5a.75.75 0 0 1 .976-1.133l.084.073l1.97 1.97l4.47-4.47l-4.47 4.47z">
                </path>
            </svg>
        </div>
        <div class="cancel-icon" :class="{
            'active-cancel-icon': focus && tmpHotkeys,
            'unactive-cancel-icon': !tmpHotkeys,
            'blur-active-cancel-icon': !focus && tmpHotkeys
        }" @click="onClearTmpHotkeys">
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24">
                <path
                    d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10s10-4.47 10-10S17.53 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8s8 3.59 8 8s-3.59 8-8 8zm3.59-13L12 10.59L8.41 7L7 8.41L10.59 12L7 15.59L8.41 17L12 13.41L15.59 17L17 15.59L13.41 12L17 8.41z">
                </path>
            </svg>
        </div>
    </div>
</template>

<style scoped>
.keys-inp-container {
    @apply px-3 w-full ease-in-out duration-300 flex flex-row justify-between items-center appearance-none bg-transparent outline-none rounded-lg;
    border: 1px solid rgb(145, 147, 145);
    cursor: text;
    height: 2.25rem;
    min-height: 2.25rem;
    max-height: 2.25rem;
    /* border-radius: 0.2rem; */
}

.keys-inp-container:hover {
    border: 1px solid rgb(199, 202, 198);
}

.keys-inp-container:focus {
    border: 1px solid rgb(96, 219, 184);
}

.hot-keys {
    color: rgb(199, 202, 198);
    @apply flex-auto ease-in-out duration-300;
}

.placehold-hot-keys {
    color: rgb(145, 147, 145);
}

.cancel-icon {
    height: 1.1rem;
    width: 1.1rem;
    min-width: 1.1rem;
    @apply flex-none rounded-full ease-in-out duration-300 flex justify-center items-center;
}

.cancel-icon:hover {
    cursor: pointer;
}

.cancel-icon svg {
    @apply ease-in-out duration-300;
}

.active-cancel-icon svg {
    height: 1.1rem;
    width: 1.1rem;
    min-width: 1.1rem;
    fill: rgba(96, 219, 184, 1);
}

.unactive-cancel-icon {
    width: 0px;
    min-width: 0px;
    height: 0px;
}

.unactive-cancel-icon svg {
    fill: rgba(96, 219, 184, 0);
}

.blur-active-cancel-icon svg {
    height: 1.1rem;
    width: 1.1rem;
    min-width: 1.1rem;
    fill: rgb(145, 147, 145);
}
</style>