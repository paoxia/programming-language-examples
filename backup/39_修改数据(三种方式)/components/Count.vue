<template>
    <div class="count">
        <h2>sum: {{ countStore.sum }}</h2>
        <hr>
        <h2>school:{{ countStore.school }}</h2>
        <h2>address:{{ countStore.address }}</h2>
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
import { ref, reactive } from "vue";
import { useCountStore } from "@/store/count";

const countStore = useCountStore();
console.log(countStore.sum);
console.log(countStore.$state.sum);
let n = ref(1);

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
    countStore.school += 'zhongguo'
    countStore.address += 'hz'
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
