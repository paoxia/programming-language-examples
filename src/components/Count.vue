<template>
    <div class="count">
        <h2>sum: {{ sum }}</h2>
        <select v-model.number="n">
            <option value="1">1</option>
            <option value="2">2</option>
            <option value="3">3</option>
        </select>
        <button @click="Add">+</button>
        <button @click="Minus">-</button>
    </div>
</template>
    
<script setup lang='ts' name="Count">
import { ref } from "vue"
let sum = ref(1)
let n = ref(1)

function Add() {
    sum.value += n.value
}


function Minus() {
    sum.value -= n.value
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