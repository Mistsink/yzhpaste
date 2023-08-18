<script setup lang="ts">
import { nextTick, onUnmounted, onMounted, onUpdated, ref, watch, watchEffect, onActivated, onDeactivated } from 'vue'
import { useScrollToSelectedItem } from '@/utils/scroll'
import { useEnterAndEsc } from '@/services/shortcuts'
import { currentFocusIndex, records, setCurrentFocusIndex, deleteRecord } from '@/stores/records'


onMounted(async () => {
  console.log('ClipboardHistory mounted');

  window.addEventListener('keydown', onKeyDown)
  window.addEventListener('keyup', onEnter)
})

onUnmounted(() => {
  console.log('ClipboardHistory on unmounted');
  console.log(window.removeEventListener('keydown', onKeyDown))
  console.log(window.removeEventListener('keyup', onEnter))
})

// onActivated(() => {
//   if (HistoryCtnRef.value) {
//     // HistoryCtnRef.value.scrollLeft = scrollPosition.value
//   }
//   console.log('ClipboardHistory activated');
//   window.addEventListener('keydown', onKeyDown)
//   window.addEventListener('keyup', onEnter)
// })

// onDeactivated(() => {
//   console.log('ClipboardHistory deactivated');
//   window.removeEventListener('keydown', onKeyDown)
//   window.removeEventListener('keyup', onEnter)
// })



//  for the li height
const HistoryCtnRef = ref()
watchEffect(() => {
  if (HistoryCtnRef.value) {
    const liHeight = HistoryCtnRef.value.clientHeight;
    HistoryCtnRef.value.style.setProperty('--li-height', `${liHeight * 0.9}px`);
  }
})

watch(currentFocusIndex, () => {
  scrollToSelectedItem();
});
const scrollPosition = ref(0)
const scrollToSelectedItem = () => {
  const selectedItem = HistoryCtnRef.value.querySelector(`li:nth-child(${currentFocusIndex.value + 1})`)
  const scrollTo = useScrollToSelectedItem(HistoryCtnRef.value, selectedItem, 0.5)
  if (scrollTo != -1) {
    scrollPosition.value = scrollTo
  }
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
  console.log('onEnter', record)
  await useEnterAndEsc(record.id)
}

const onSingleClick = async (idx: number) => {
  setCurrentFocusIndex(idx)
}
const onDoubleClick = async (idx: number) => {
  setCurrentFocusIndex(idx)
  const record = records.value[currentFocusIndex.value]
  await useEnterAndEsc(record.id)
}
const onWheel = (event: WheelEvent) => {
  if (event.deltaY === -0) {
    return;
  }; // 触摸板滚动事件

  event.preventDefault();
  if (event.deltaY < 0) {
    setCurrentFocusIndex(currentFocusIndex.value - 1);
  } else {
    setCurrentFocusIndex(currentFocusIndex.value + 1);
  }
};
const enableWheel = () => {
  window.addEventListener('wheel', onWheel);
};

const disableWheel = () => {
  window.removeEventListener('wheel', onWheel);
};

const onDelete = async (event: Event, index: number) => {
  event.stopPropagation();
  await deleteRecord(index)
}
</script>

<template>
  <div class="w-full h-full
  content-container
  flex justify-center items-center" @mouseover="enableWheel" @mouseleave="disableWheel">
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
      rounded-lg
      flex-none
      inline-flex
      justify-between
      w-full h-full flex-col
      " :class="index === currentFocusIndex ? 'active-item' : ''">
        <p class="flex-none item-title">{{ item.data_type.toUpperCase() }}</p>
        <p v-if="item.data_type === 'text'" class="
          item-content
          flex-auto 
          px-4 py-1
          break-words 
          whitespace-pre-wrap
          tracking-wide
          overflow-hidden
          ">{{ item.content }}</p>
        <img v-else :src="`data:image/jpeg;base64,${item.content.base64}`" class="
          img-container
          p-4
          " />
        <div class="flex-none
          flex
          justify-center
          items-center
          shadow-top
          item-footer
          relative
        ">
          {{ item.data_type === 'text' ? `${item.content.length} 字符` : `${item.content.width} x ${item.content.height} 像素`
          }}

          <div class="icon" @click="e => onDelete(e, index)">
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24">
              <path
                d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10s10-4.47 10-10S17.53 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8s8 3.59 8 8s-3.59 8-8 8zm3.59-13L12 10.59L8.41 7L7 8.41L10.59 12L7 15.59L8.41 17L12 13.41L15.59 17L17 15.59L13.41 12L17 8.41z">
              </path>
            </svg>
          </div>
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
  box-shadow: 0 -10px 20px rgba(29, 38, 35, 0.9);
}

.hide-scrollbar::-webkit-scrollbar {
  display: none;
}

.hide-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.content-container {
  background-color: rgb(25, 28, 27);
}

.active-item {
  outline-width: 0.23rem;
  outline-style: solid;
  /* outline-color: rgb(15, 103, 200); */
  outline-color: rgb(96, 219, 184);
  outline-offset: 0.08rem;
}

.unactive-item {
  outline-width: 0.23rem;
  outline-style: solid;
  /* outline-color: rgb(15, 103, 200); */
  outline-color: rgb(225, 227, 224);
  outline-offset: 0.08rem;
}

.item-title {
  font-size: 1.2rem;
  padding-left: 1.25rem;
  /* background-color: rgb(115,162,115); */
  background-color: rgb(40, 75, 94);
  color: rgb(196, 231, 255);
  @apply leading-10
}

.item-content {
  color: rgb(225, 227, 224);
  background-color: rgb(29, 38, 35);
}

.img-container {
  flex: 1 1 auto;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  background-color: rgb(29, 38, 35);
}

.img-container img {
  max-width: 100%;
  max-height: 100%;
  width: auto;
  height: auto;
  object-fit: contain;
}


.item-footer {
  font-size: 0.88rem;
  color: rgba(225, 227, 224, 0.7);
  background-color: rgb(29, 38, 35);
  padding: 0.15rem 0;
}

li:hover .icon, li:hover .icon svg {
  height: 1.2rem;
  width: 1.2rem;
  min-width: 1.2rem;
}

.icon, .icon svg {
  width: 0px;
  min-width: 0px;
  height: 0px;
  @apply ease-in-out duration-300;
}

.icon {
  @apply absolute right-4;
}

.icon svg {
  fill: rgba(96, 219, 184, 1);
}
</style>
