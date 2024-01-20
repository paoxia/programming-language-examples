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
import { useLoveTalkStore } from '@/store/lovetalk';
import { storeToRefs } from 'pinia';

const talkStore = useLoveTalkStore()
const { talkList } = storeToRefs(useLoveTalkStore())

function getLoveTalk() {
    talkStore.getATalk()
}

</script>
    
<style></style>