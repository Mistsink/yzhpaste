import { ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { cmd_find_all_records, cmd_find_by_key, type Record, type RecordOrigin } from '@/services/cmds'

export const records = ref<Record[]>([])
const recordsStore = ref<Record[]>([])
export const recordsStoreInit = async () => {
    recordsStore.value = await cmd_find_all_records()
    records.value = recordsStore.value
}
export const recordsStoreFind = async (keyword: string) => {
    if (keyword === '') {
        currentFocusIndex.value = 0
        records.value = recordsStore.value
        return
    }
    const queryKey = {
        key: keyword,
        limit: null,
        pined: null,
        tags: null,
    }
    records.value = await cmd_find_by_key(queryKey)
    currentFocusIndex.value = 0
}


listen('on-records-update', (e) => {
    console.log('on-records-update', e)
    const records = e.payload as RecordOrigin[]
    recordsStore.value =  records.map((r) => {
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

listen('on-window-hide', () => currentFocusIndex.value = 0)
