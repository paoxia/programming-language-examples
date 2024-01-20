import { defineStore } from "pinia"
import axios from 'axios';
import { nanoid } from 'nanoid'

export const useLoveTalkStore = defineStore('lovetalk', {
    actions: {
        async getATalk() {
            let { data: { content: title } } = await axios.get('https://api.uomg.com/api/rand.qinghua?format=json')
            let obj = { id: nanoid(), title }
            this.talkList.unshift(obj)
            // console.log(title)
        }
    },
    // 存储数据的地方
    state() {
        return {
            "talkList": JSON.parse(localStorage.getItem('talkList') as string) || []
        }
    }
})