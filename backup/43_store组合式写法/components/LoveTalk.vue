<template>
    <div class="talk">
        <button @click="getLoveTalk">获取一句土味情话</button>
        <ul>
            <li v-for="talk in talkList" :key="talk.id">{{ talk.title }}</li>

        </ul>
    </div>
</template>
    
<script setup lang='ts' name="LoveTalk">
import axios from 'axios';
import { reactive } from 'vue'
import { nanoid } from 'nanoid'
import { useTalkStore } from '@/store/lovetalk';
import { storeToRefs } from 'pinia';

const talkStore = useTalkStore()
const { talkList } = storeToRefs(useTalkStore())

talkStore.$subscribe((mutate, state) => {
    console.log('talkStore数据发生了变化', mutate, state)
    localStorage.setItem('talkList', JSON.stringify(state.talkList))

})

function getLoveTalk() {
    talkStore.getATalk()
}

</script>
    
<style></style>