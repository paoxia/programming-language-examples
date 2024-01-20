<template>
    <div class="talk">
        <button @click="getLoveTalk">获取一句土味情话</button>
        <ul>
            <li v-for="talk in talkStore.talkList" :key="talk.id">{{ talk.title }}</li>

        </ul>
    </div>
</template>
    
<script setup lang='ts' name="LoveTalk">
import axios from 'axios';
import { reactive } from 'vue'
import { nanoid } from 'nanoid'
import { useLoveTalkStore } from '@/store/lovetalk';

const talkStore = useLoveTalkStore()

async function getLoveTalk() {
    // req
    // let res = await axios.get('https://api.uomg.com/api/rand.qinghua?format=json')
    // 将content映射成content
    let { data: { content: title } } = await axios.get('https://api.uomg.com/api/rand.qinghua?format=json')
    let obj = { id: nanoid(), title }
    talkStore.talkList.unshift(obj)
    console.log(title)
}

</script>
    
<style></style>