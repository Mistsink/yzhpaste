<script setup lang="ts">
const props = defineProps({
    title: String
})
</script>

<template>
    <div class="option-item">
        <div class="
            bg-red-100
            w-full text-xl
            flex-none
            ">
            {{ props.title }}
        </div>
        <div class="
        bg-yellow-200
        flex-auto p-4 pl-10
        space-y-3
        ">
            <slot></slot>
        </div>
    </div>
</template>
<style scoped>
.option-item {
    @apply bg-blue-300 flex-1 h-full flex flex-col p-4 rounded-lg
}
</style>