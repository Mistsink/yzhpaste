<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import OptGeneral from '../components/options/OptGeneral.vue'
import OptHotkey from '../components/options/OptHotkey.vue'
import OptSync from '../components/options/OptSync.vue'
import { useRouter } from 'vue-router';

const router = useRouter()

const tags = ['通用', '快捷键', '同步', '订阅']
const curTagIdx = ref(0)
const onChangeTag = (idx: number) => {
  curTagIdx.value = idx
}
const optView = computed(() => {
  switch (curTagIdx.value) {
    case 0:
      return OptGeneral
    case 1:
      return OptHotkey
    case 2:
      return OptSync
    default:
      return OptGeneral
  }
})
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="
    flex-none flex flex-row justify-center items-center
    p-4 h-16 min-h-16 max-h-16
    space-x-3 duration-300 ease-in-out
    ">
      <div class="
      absolute left-2
      unactive_tag
      p-2 h-8 min-h-8 max-h-8
      flex justify-center items-center
      cursor-pointer
      " @click="router.back()">
        Back
      </div>
      <button v-for="(tag, idx) in tags" :key="tag" class="
      p-1 px-3 h-8
      text-sm
      flex flex-row justify-center items-center space-x-2
    " :class="curTagIdx === idx ? 'active_tag' : 'unactive_tag'" @click="() => onChangeTag(idx)">
        <div class="w-3 h-3 bg-green-300 rounded-full">
        </div>
        <div class="">
          {{ tag }}
        </div>
      </button>
    </div>
    <div class="flex-auto overflow-hidden">
      <optView />
    </div>
  </div>
</template>

<style scoped></style>
