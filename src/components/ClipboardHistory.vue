<script setup lang="ts">
import { nextTick, onBeforeMount, onMounted, ref, watch, watchEffect } from 'vue'
import { type Record, cmd_find_all_records, cmd_print } from '../services/cmds'
import { useScrollToSelectedItem } from '@/utils/scroll'
import { useEnterAndEsc } from '@/services/shortcuts'

const records = ref<Record[]>([])

onMounted(async () => {
  records.value = await cmd_find_all_records()
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

watch(currentFocusIndex, () => {
  scrollToSelectedItem();
});
const scrollToSelectedItem = () => {
  const selectedItem = HistoryCtnRef.value.querySelector(`li:nth-child(${currentFocusIndex.value + 1})`)
  useScrollToSelectedItem(HistoryCtnRef.value, selectedItem, 0.5)
};

const onKeyDown = (event: KeyboardEvent) => {
  if (event.key === 'ArrowLeft' || event.key === 'ArrowRight') {
    event.preventDefault()
    if (event.key === 'ArrowLeft') {
      setCurrentFocusIndex(currentFocusIndex.value - 1)
    } else {
      setCurrentFocusIndex(currentFocusIndex.value + 1)
    }
  }
}
const onEnter = async (event: KeyboardEvent) => {
  if (event.key !== 'Enter') return
  const record = records.value[currentFocusIndex.value]
  await cmd_print(JSON.stringify(record))
  await useEnterAndEsc(record.id)
}

const clickTimeout = ref<number>(-1)
const onSingleClick = async (idx: number) => {
  clearTimeout(clickTimeout.value);

  clickTimeout.value = setTimeout(async () => {
    await cmd_print("Single click");
    setCurrentFocusIndex(idx)
  }, 250);
}
const onDoubleClick = async (idx: number) => {
  clearTimeout(clickTimeout.value);
  setCurrentFocusIndex(idx)
  const record = records.value[currentFocusIndex.value]
  await cmd_print(JSON.stringify(record))
  await useEnterAndEsc(record.id)
}
</script>

<template>
  <div class="bg-slate-500 w-full h-full 
  flex justify-center items-center">
    <p v-if="records.length === 0" class="text-4xl">No clipboard history available.</p>
    <ul ref="HistoryCtnRef" v-else class="flex flex-row 
    space-x-4 overflow-x-auto
    p-4 pl-6 pr-6
     h-full
     histories-container hide-scrollbar
     ">
      <li v-for="(item, index) in records" :key="index" @click="() => onSingleClick(index)"
        @dblclick="() => onDoubleClick(index)" class="
      overflow-hidden
      rounded-md
      flex-none
      inline-flex
      justify-between
    odd:bg-green-200 even:bg-orange-200
      w-full h-full flex-col
      " :class="{ 'outline outline-blue-500': index === currentFocusIndex }">
        <p class="flex-none underline bg-red-300 roundedt-md">{{ index + 1 }}</p>
        <p v-if="item.data_type === 'text'" class="flex-auto 
        pl-4 pr-4
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

.hide-scrollbar::-webkit-scrollbar {
  display: none;
}

.hide-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
