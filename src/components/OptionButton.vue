<script setup lang="ts">
import { ref, watchEffect } from 'vue';
import { useClickOutside } from '../utils/useClickOutside'
import { useRouter } from 'vue-router';

const router = useRouter()

const selfRef = ref<null | HTMLElement>(null)
const isClickOutside = useClickOutside(selfRef)

watchEffect(() => {
    if (isClickOutside.value) {
        isClicked.value = false
    }
})

const isClicked = ref(false)


const onClick = () => {
    isClicked.value = !isClicked.value
}
</script>

<template>
    <div ref="selfRef" class="
    h-8 mr-1 p-1
    flex justify-center items-center
    
    " :class="isClicked ? 'active_tag' : 'unactive_tag'" @click="onClick">
        <button>OptionButton</button>
    </div>
    <ul v-if="isClicked" class="
        options-container
        w-32 min-w-32 max-w-32
        absolute top-8 right-1 z-50
        bg-green-300
        flex flex-col justify-center items-start
        rounded-md p-2
    ">
        <li class="option-item" @click="router.push('config')">
            <button>偏好设置</button>
        </li>
        <li class="option-item">
            <button>Option 2 </button>
        </li>
        <li class="option-item">
            <button>Option 3 </button>
        </li>
    </ul>
</template>
<style scoped>
.options-container {}

.option-item {
    cursor: pointer;
    @apply unactive_tag w-full p-1
}
</style>