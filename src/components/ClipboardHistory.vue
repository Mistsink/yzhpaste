<script setup lang="ts">
import { nextTick, onBeforeMount, onMounted, ref, watchEffect } from 'vue'
import { type Record, cmd_find_all_records } from '../services/cmds'

const records = ref<Record[]>([])

onMounted(async () => {
  records.value = await cmd_find_all_records()
  console.log(records.value)
  window.data = records.value


  nextTick(() => {
    // 设置第一个li元素的焦点
    if (HistoryCtnRef.value) {
      const firstLi = HistoryCtnRef.value.querySelector('li:first-child')
      if (firstLi) {
        firstLi.focus()
      }
    }
  })

  window.addEventListener('keydown', onKeyDown)
  window.addEventListener('keyup', onEnter)
})

onBeforeMount(() => {
  window.removeEventListener('keydown', onKeyDown)
  window.removeEventListener('keyup', onEnter)
})

//  for the li height
const HistoryCtnRef = ref()
watchEffect(() => {
  if (HistoryCtnRef.value) {
    const liHeight = HistoryCtnRef.value.clientHeight;
    HistoryCtnRef.value.style.setProperty('--li-height', `${liHeight * 0.9}px`);
  }
})

const currentFocusIndex = ref(0)
const setCurrentFocusIndex = (newIndex: number) => {
  if (newIndex >= 0 && newIndex < records.value.length) {
    currentFocusIndex.value = newIndex
  }
}

const onKeyDown = (event: KeyboardEvent) => {
  if (event.key === 'ArrowLeft' || event.key === 'ArrowRight') {
    event.preventDefault()
    if (event.key === 'ArrowLeft') {
      setCurrentFocusIndex(currentFocusIndex.value - 1)
    } else {
      setCurrentFocusIndex(currentFocusIndex.value + 1)
    }
    const newFocusElement = HistoryCtnRef.value.querySelector(`li:nth-child(${currentFocusIndex.value + 1})`)
    if (newFocusElement) {
      newFocusElement.focus()
    }
  }
}
const onEnter = (event: KeyboardEvent) => {
  if (event.key === 'Enter') {
    const focusElement = document.activeElement
    const focusIndex = Array.from(HistoryCtnRef.value.children).indexOf(focusElement)
    const focusRecord = records.value[focusIndex]
    console.log('当前焦点元素：', focusElement, '对应的记录数据：', focusRecord)
  }
}
</script>

<template>
  <div class="bg-slate-500 w-full h-full 
  flex justify-center items-center">
    <p v-if="records.length === 0" class="text-4xl">No clipboard history available.</p>
    <ul ref="HistoryCtnRef" v-else 
    class="flex flex-row 
    space-x-4 overflow-x-auto
    p-4
     h-full
     snap-x snap-proximity
     histories-container
     ">
      <li v-for="(item, index) in records" :key="index" :tabindex="index" class="
      rounded-md
      flex-none
      inline-flex
      justify-between
    odd:bg-green-200 even:bg-orange-200
      p-4 w-full h-full flex-col
      focus:outline focus:outline-blue-500 
      snap-start
      " @keyup.enter="onEnter">
        <p class="flex-none underline bg-red-300">{{ index + 1 }}</p>
        <p v-if="item.data_type === 'text'" class="flex-auto 
          bg-sky-200  
        break-words 
          whitespace-pre-wrap
          tracking-wide
          overflow-hidden
          ">{{ item.content }}</p>
        <img v-else :src="`data:image/jpeg;base64,${item.content.base64}`" />
        <div class="flex-none
          bg-yellow-200
          flex
          justify-center
          items-center
          shadow-top
        ">
          {{ item.data_type === 'text' ? `${item.content.length}个字符` : `${item.content.width} x ${item.content.height} 像素`
          }}
        </div>
      </li>
    </ul>
  </div>
</template>

<style scoped>
.histories-container {
  --li-height: 0;
  /* width: calc(100% - (1 * 2)rem);
  padding: 1rem 0 1rem; */
}

li {
  height: 100%;
  width: var(--li-height);
}

.shadow-top {
  box-shadow: 0 -10px 20px rgba(254, 240, 138, 0.9);
}
</style>
