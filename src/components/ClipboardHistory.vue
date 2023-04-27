<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { type Record, cmd_find_all_records } from '../services/cmds'

const records = ref<Record[]>([])

onMounted(async () => {
  records.value = await cmd_find_all_records()
  console.log(`records: ${records.value}`)
})
</script>

<template>
  <div class="clipboard-history">
    <h2>Clipboard History</h2>
    <div v-if="records.length === 0">No clipboard history available.</div>
    <ul v-else class="histories">
      <li v-for="(item, index) in records" :key="index">
        <div class="clipboard-item">
          <span>{{ item.content }}</span>
        </div>
      </li>
    </ul>
  </div>
</template>

<style scoped>
.clipboard-history {
  max-width: 600px;
  margin: 0 auto;
}

.clipboard-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px;
  border: 1px solid #ccc;
  margin-bottom: 8px;
}

button {
  padding: 4px 8px;
  background-color: #2c3e50;
  color: #fff;
  border: none;
  cursor: pointer;
}

.histories {
  display: flex;
  flex-direction: row;
}
</style>
