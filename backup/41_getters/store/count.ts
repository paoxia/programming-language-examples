import { defineStore } from "pinia"

export const useCountStore = defineStore('count', {
    // actions放置动作函数方法
    actions: {
        increment(value: number) {
            console.log("increment" + value)
            if (this.sum < 10) {
                this.sum += value
            }
        }
    },
    // 存储数据的地方
    state() {
        return {
            sum: 6,
            address: "dizhi",
            school: "shehui"
        }
    },
    getters: {
        // bigSum(state) {
        //     return state.sum * 10
        // },
        // bigSum() {
        //     return this.sum * 10
        // },
        bigSum: state => state.sum * 10,
        // upperSchool(state) {
        //     return state.school.toUpperCase()
        // }

        upperSchool(): string {
            return this.school.toUpperCase()
        }
    }
})