<script setup lang="ts">
import ClipboardHistory from './ClipboardHistory.vue'
import { ref } from 'vue'

const isVisible = ref(true)

const showPopup = () => {
  isVisible.value = true
}

const hidePopup = () => {
  isVisible.value = false
}

const togglePopup = () => {
  isVisible.value = !isVisible.value
}
defineExpose({ showPopup, hidePopup, togglePopup })
</script>

<template>
  <transition name="slide-up">
    <div v-if="isVisible" class="clipboard-popup">
      <!-- 弹窗内容 -->
      <ClipboardHistory />
    </div>
  </transition>
</template>

<style scoped>
.slide-up-enter-active,
.slide-up-leave-active {
  transition: all 0.5s ease;
}

.slide-up-enter-from,
.slide-up-leave-to {
  transform: translateY(100%);
}

.clipboard-popup {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  background-color: #fff;
  /* 更多样式 */
}
</style>
