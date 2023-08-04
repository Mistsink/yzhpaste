<script setup lang="ts">
import ClipboardHistory from '@/components/ClipboardHistory.vue';
import OptionButton from '../components/OptionButton.vue'
import { nextTick, onActivated, onDeactivated, onMounted, onUnmounted, ref, watch, watchEffect } from 'vue';
import { recordsStoreFind } from '@/stores/records';
import { listen } from '@tauri-apps/api/event';

const getTags = async (): Promise<Array<string>> => {
  return []
}

const tags = ref<Array<string>>(['剪切板历史'])
const curTagIdx = ref(0)

const onChangeTag = (idx: number) => {
  curTagIdx.value = idx
  isSearching.value = false
  searchText.value = ''
}

const isWinShow = ref(false)

const searchText = ref('')
const isSearching = ref(false)
listen('on-window-hide', () => {
  console.log('on-window-hide');
  isWinShow.value = false
  onSearchInpBlur()
})
listen('on-window-show', () => {
  console.log('on-window-show');
  isWinShow.value = true
})
watchEffect(async () => {
  console.log('searchText changed');
  await recordsStoreFind(searchText.value)
})

const onClickSearIcon = () => {
  isSearching.value = true
}

onMounted(async () => {
  tags.value.push(...await getTags())

  isWinShow.value = true
  window.addEventListener('keydown', onSearch)

  nextTick(() => {
    SearchInpRef.value.focus()
  })
})
onUnmounted(() => {
  isWinShow.value = false
  window.removeEventListener('keydown', onSearch)
})
// onActivated(() => {
//   isWinShow.value = true
//   onSearchInpBlur()
//   window.addEventListener('keydown', onSearch)
// })
// onDeactivated(() => {
//   window.removeEventListener('keydown', onSearch)
// })

const SearchInpRef = ref()
let cnt = 0
const onSearch = (event: KeyboardEvent) => {
  // 检查按键是否为可输入文字的按键
  console.log(cnt, 'onSearch', event.key, isSearching.value, isWinShow.value)
  if (!isSearching.value && isWinShow.value && (event.key == 'Process' || event.key.length === 1 || event.key === 'Backspace')) {
    console.log('inner onSearch');

    onClickSearIcon();
  }
  cnt ++
}
const onSearchInpBlur = () => {
  console.log('onSearchInpBlur, isWinShow', isWinShow.value);
  if (isWinShow.value && SearchInpRef.value) {
    SearchInpRef.value.focus()
    console.log('onSearchInpBlur, focus');
  }
  isSearching.value = false
  searchText.value = ''
}

watch(isSearching, () => {
  if (isSearching.value) {
    SearchInpRef.value.focus();
    // nextTick(() => {
    //   SearchInpRef.value.focus();
    // })
  }
})

</script>
<template>
  <div class="flex flex-col h-full">
    <div class="
    flex-none flex flex-row justify-center items-center
    p-4 pb-0 h-16 min-h-16 max-h-16
    space-x-3
    ">
      <div class="
          search
          p-1
          flex justify-center items-center
          cursor-pointer
        " :class="isSearching ? 'active-search' : 'unactive-search'" @click="onClickSearIcon">
        <svg t="1690870647825" class="search-icon" :class="isSearching ? 'active-search-icon' : ''"
          viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="1561">
          <path
            d="M689.067 631.467L889.6 832c38.4 38.4-19.2 96-57.6 57.6L631.467 689.067C576 731.733 505.6 757.333 430.933 757.333 249.6 757.333 102.4 610.133 102.4 428.8s147.2-326.4 328.533-326.4 328.534 147.2 328.534 328.533c-2.134 74.667-27.734 145.067-70.4 200.534z m-258.134 44.8c136.534 0 245.334-110.934 245.334-245.334S565.333 183.467 430.933 183.467 183.467 294.4 183.467 430.933 294.4 676.267 430.933 676.267z"
            p-id="1562"></path>
        </svg>
        <input ref="SearchInpRef" class="search-inp" :class="isSearching ? 'active-search-inp' : 'unactive-search-inp'"
          v-model="searchText" @blur="onSearchInpBlur" />
      </div>

      <button v-for="(tag, idx) in tags" :key="tag" class="tag"
        :class="curTagIdx === idx && !isSearching ? 'active-tag' : 'unactive-tag'" @click="() => onChangeTag(idx)">
        <div class="tag-point" :class="curTagIdx === idx && !isSearching ? 'active-tag-point' : 'unactive-tag-point'">
        </div>
        <div v-if="!isSearching" class="">
          {{ tag }}
        </div>
      </button>
      <div class="absolute right-1">
        <OptionButton />
      </div>
    </div>
    <div class="flex-auto overflow-hidden relative">
      <ClipboardHistory />
    </div>
  </div>
</template>

<style scoped>
.search {
  min-width: 2.25rem;
  min-height: 2.25rem;
  max-height: 2.25rem;
  @apply rounded-lg flex flex-row justify-center items-center leading-9 ease-in-out duration-300;
}

.active-search {
  cursor: default;
  background-color: rgb(25, 28, 27);
  color: rgb(225, 227, 224);
  border: 0.12em solid rgb(137, 147, 142);
}

.unactive-search {
  border: 0rem solid rgba(137, 147, 142, 0);
}

.unactive-search:hover {
  background-color: rgb(52, 76, 67);
}

.search-icon {
  fill: rgb(225, 227, 224);
  width: 1.25rem;
  height: 1.25rem;
  margin: 0 0.3rem;
  @apply leading-9 ease-in-out duration-300;
}

.active-search-icon {
  fill: rgb(96, 219, 184);
}

.search-inp-container {}

.search-inp {
  height: calc(2.25rem - 0.12em);
  margin-right: 0.25rem;
  @apply appearance-none border-none bg-transparent outline-none ease-in-out duration-300;
  caret-color: rgb(96, 219, 184);
  border-bottom: 0.12em solid rgba(96, 219, 184, 0);
}

.search-inp:focus {
  border-bottom: 0.12em solid rgba(96, 219, 184, 1);
}

.active-search-inp {
  width: 12rem;
}

.unactive-search-inp {
  width: 0rem;
  padding: 0;
  margin: 0;
}

.tag {
  width: auto;
  min-width: 2.25rem;
  font-size: 0.8rem;
  min-height: 2.25rem;
  max-height: 2.25rem;
  @apply px-3 rounded-lg flex flex-row justify-center items-center space-x-2 leading-9 ease-in-out duration-300;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.active-tag {
  width: 8rem;
  background-color: rgb(25, 28, 27);
  color: rgb(225, 227, 224);
  border: 0.12rem solid rgba(137, 147, 142, 1);
  cursor: default;
}

.unactive-tag {
  width: 2.25rem;
  border: 0rem solid rgba(137, 147, 142, 0);
}

.unactive-tag:hover {
  background-color: rgb(52, 76, 67);
}

.tag-point {
  @apply w-3 h-3 rounded-full ease-in-out duration-300;
}

.active-tag-point {
  background-color: rgb(96, 219, 184);
}

.unactive-tag-point {
  background-color: rgb(52, 76, 67);
}

.unactive-tag:hover .unactive-tag-point {
  background-color: rgb(96, 219, 184);
}
</style>
