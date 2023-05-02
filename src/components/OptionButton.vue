<script setup lang="ts">
import { ref, watchEffect } from 'vue';
import { useClickOutside } from '../utils/useClickOutside'

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
    <ul v-if="isClicked" 
    class="
        w-32 min-w-32 max-w-32
        absolute top-8 right-0 z-50
        bg-green-300
        flex flex-col justify-center items-start
    ">
        Many options
        <li>
           <button>Option 1 </button>
        </li>
        <li>
           <button>Option 2 </button>
        </li>
        <li>
           <button>Option 3 </button>
        </li>
    </ul>
</template>