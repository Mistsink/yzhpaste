<script setup lang="ts">
import { computed, ref, watch, onBeforeMount, nextTick, onMounted } from 'vue';
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
      " @click="router.push('/config')">
        Back
      </div>
      <button v-for="(tag, idx) in tags" :key="tag" class="
      tag
    " :class="curTagIdx === idx ? 'active-tag' : 'unactive-tag'" @click="() => onChangeTag(idx)">
        <div class="tag-point" :class="curTagIdx === idx ? 'active-tag-point' : 'unactive-tag-point'">
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

<style scoped>
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
  /* min-width: 2.25rem;
  width: auto; */
  width: 2.25rem;
  color: rgb(191, 201, 195);
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
