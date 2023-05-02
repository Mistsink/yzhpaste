<script setup lang="ts">
import ClipboardHistory from '@/components/ClipboardHistory.vue';
import OptionButton from '../components/OptionButton.vue'
import { onMounted, ref } from 'vue';

const getTags = async (): Promise<Array<string>> => {
  return ['tag1', 'tag2', 'tag3']
}

const tags = ref<Array<string>>(['剪切板历史'])
const curTagIdx = ref(0)

const onChangeTag = (idx: number) => {
  curTagIdx.value = idx
  isSearching.value = false
}


const searchText = ref('')
const isSearching = ref(false)
const useToggleSearching = () => {
  isSearching.value = !isSearching.value
}

const onClickSearIcon = () => {
  if (!searchText.value) useToggleSearching()
}

onMounted(async () => {
  tags.value.push(...await getTags())
})

</script>
<template>
  <div class="flex flex-col h-full">
    <div class="
    flex-none flex flex-row justify-center items-center
    p-4 h-16 min-h-16 max-h-16
    space-x-3 duration-300 ease-in-out
    ">
      <div
        class="
        h-8 px-1
        flex justify-center items-center
        cursor-pointer
        "
       :class="isSearching ? 'active_tag' : 'unactive_tag'"
        @click="onClickSearIcon"
      >🔍</div>
      <div v-if="isSearching">
        <input v-model="searchText" />
      </div>
      <button v-for="(tag, idx) in tags" :key="tag" class="
      p-1 px-3 h-8
      text-sm
      flex flex-row justify-center items-center space-x-2
    " :class="curTagIdx === idx && !isSearching ? 'active_tag' : 'unactive_tag'" @click="() => onChangeTag(idx)">
        <div class="w-3 h-3 bg-green-300 rounded-full">
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
</style>
