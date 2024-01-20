<template>
    <div class="count">
        <h2>sum: {{ sum }}, mul10 :{{ bigSum }}</h2>
        <hr>
        <h2>school:{{ school }},upperCase :{{ upperSchool }}</h2>
        <h2>address:{{ address }}</h2>
        <select v-model.number="n">
            <option value="1">1</option>
            <option value="2">2</option>
            <option value="3">3</option>
        </select>
        <button @click="Add">+</button>
        <button @click="Minus">-</button>
    </div>
</template>

<script setup lang="ts" name="Count">
import { ref, reactive, toRefs } from "vue";
import { useCountStore } from "@/store/count";
import { storeToRefs } from "pinia";


const countStore = useCountStore();
console.log(countStore.sum);
console.log(countStore.$state.sum);
let n = ref(1);
// 只关注store中的数据 不会对方法进行ref
const { sum, school, address, bigSum, upperSchool } = storeToRefs(countStore)
console.log(storeToRefs(countStore))

function Add() {
    // 第一种修改
    // countStore.sum += n.value
    // countStore.school += 'guigu'
    // countStore.address += 'zj'

    // 第二种修改
    // countStore.$patch({
    //     sum: 888,
    //     school: "xindongfang",
    //     address: "xian"
    // })

    // 第三种修改
    countStore.increment(n.value)

}

function Minus() {
    countStore.sum -= n.value
}
</script>

<style scoped>
.count {
    background-color: aqua;
    padding: 10px;
    border-radius: 10px;
    box-shadow: 0 0;
}

select,
button {
    margin: 0, 20px;
    height: 25px;
}
</style>
