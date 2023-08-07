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
        option-button
        mr-1 px-1
        flex justify-center items-center
    " @click="router.push('/')">
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
.option-button {
  font-size: 0.8rem;
  background-color: rgb(25,28,27);
  color: rgb(225,227,224);
  border-radius: 1.125rem;
  @apply px-3 flex flex-row justify-center items-center space-x-2 leading-9 ease-in-out duration-300
}
.option-button:hover {
    background-color: rgb(52,76,67);
}
.options-container {}

.option-item {
    cursor: pointer;
    @apply unactive_tag w-full p-1
}
</style>