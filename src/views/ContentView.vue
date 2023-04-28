<script setup lang="ts">
import ClipboardHistory from '@/components/ClipboardHistory.vue';
import OptionButton from '../components/OptionButton.vue'
</script>
<template>
  <div class="flex flex-col h-full">
    <div class="flex-none flex flex-row justify-center items-center">
      <h1 class="">Clipboard Contents</h1>
      <div class="absolute right-1">
        <OptionButton />
      </div>
    </div>
    <div class="flex-auto overflow-hidden relative">
      <ClipboardHistory />
    </div>
  </div>
</template>

<style scoped></style>
