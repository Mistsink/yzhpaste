<script setup lang='ts'>
import { onMounted, ref } from 'vue';

interface ClipboardContent {
    text: string;
    html: string;
    image: string;
}

const useClipboardContent = (str: string): ClipboardContent => {
    return {
        text: str,
        html: 'html',
        image: 'image'
    }
}

const histories = ref<Array<ClipboardContent>>([])


onMounted(async () => {
    await loadClipboardHistory()
})

const loadClipboardHistory = async () => {
    histories.value = [useClipboardContent('Example text 1'), useClipboardContent('Example text 2')]
}

</script>

<template>
    <div class="clipboard-history">
        <h2>Clipboard History</h2>
        <div v-if="histories.length === 0">
            No clipboard history available.
        </div>
        <ul v-else>
            <li v-for="(item, index) in histories" :key="index">
                <div class="clipboard-item">
                    <span>{{ item.text }}</span>
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
</style>
  