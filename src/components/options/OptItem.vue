<script setup lang="ts">
const props = defineProps({
    title: String
})
</script>

<template>
    <div class="option-item">
        <div class="
            item-title
            ">
            {{ props.title }}
        </div>
        <div class="item-content">
            <slot></slot>
        </div>
    </div>
</template>
<style scoped>
.option-item {
    background-color: rgb(29, 38, 35);
    color: rgb(225, 227, 224);
    @apply flex-1 h-full flex flex-col p-4 rounded-lg
}

.item-title {
    /* background-color: rgb(94, 115, 94); */
    @apply w-full text-2xl flex-none;
}

.item-content {
    background-color: rgb(29, 38, 35);
    color: rgb(225, 227, 224);
    @apply flex-auto p-4 pl-10 space-y-3;
}
</style>