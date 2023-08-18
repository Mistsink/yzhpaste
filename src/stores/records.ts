import { ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import {
  cmd_delete_by_id,
  cmd_find_all_records,
  cmd_find_by_key,
  type Record,
  type RecordOrigin
} from '@/services/cmds'

export const records = ref<Record[]>([])
const recordsStore = ref<Record[]>([])
export const recordsStoreInit = async () => {
  recordsStore.value = await cmd_find_all_records()
  records.value = recordsStore.value
}
export const recordsStoreFind = async (keyword: string) => {
  console.log('recordsStoreFind', keyword)
  if (keyword === '') {
    currentFocusIndex.value = 0
    records.value = recordsStore.value
    return
  }
  const queryKey = {
    key: keyword,
    limit: null,
    pined: null,
    tags: null
  }
  records.value = await cmd_find_by_key(queryKey)
  currentFocusIndex.value = 0
}

listen('on-records-update', (e) => {
  console.log('on-records-update', e)
  const records = e.payload as RecordOrigin[]
  recordsStore.value = records.map((r) => {
    if (r.data_type === 'image') r.content = JSON.parse(r.content)
    return r as Record
  })

  currentFocusIndex.value = 0
})

export const currentFocusIndex = ref(0)
export const setCurrentFocusIndex = (newIndex: number) => {
  console.log('setCurrentFocusIndex', newIndex)
  if (newIndex >= 0 && newIndex < records.value.length) {
    currentFocusIndex.value = newIndex
  }
}

// 窗口隐藏 n 秒后若仍未再次唤醒，删除本地record记录，以节约内存
const timeout = 5000
let timer: number
listen('on-window-show', async () => {
  if (timer) {
    clearTimeout(timer)
    await recordsStoreInit()
  }
})

listen('on-window-hide', () => {
  currentFocusIndex.value = 0
  timer = setTimeout(() => {
    recordsStore.value = []
    console.log('recordsStore clear')
  }, timeout)
})

export const deleteRecord = async (idxInRecords: number) => {
  const record = records.value[idxInRecords]
  await cmd_delete_by_id(record.id)

  records.value.splice(idxInRecords, 1)
  // 从 recordsStore 中找到id一致的记录后，再删除
  const idxInRecordsStore = recordsStore.value.findIndex((r) => r.id === record.id)
  if (idxInRecordsStore !== -1) {
    recordsStore.value.splice(idxInRecordsStore, 1)
  }

  // 更新 currentFocusIndex
  if (currentFocusIndex.value === records.value.length) {
    currentFocusIndex.value--
  }
}
