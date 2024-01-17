<template>
    <div class="person">

        <h2>当前求和为:{{ sum }}</h2>
        <button @click="add">sum+1</button>
    </div>
</template>
  
<script lang="ts" setup name="Person">
import {
    ref, onBeforeMount, onMounted, onBeforeUpdate, onUpdated, onBeforeUnmount,
    onUnmounted
} from 'vue'

let sum = ref(0)

function add() {
    sum.value += 1
}

// 创建
console.log("创建")

// 挂载前
onBeforeMount(() => { console.log("挂载前") })

// 挂载后
onMounted(() => { console.log("挂载后") })

// 更新前
onBeforeUpdate(() => { console.log("更新前") })

// 更新后
onUpdated(() => { console.log("更新后") })

// 卸载前
onBeforeMount(() => {
    console.log("卸载前")
})

// 卸载后
onUnmounted(() => {
    console.log("卸载后")
})

</script>

<style scoped>
.person {
    box-shadow: 0 0 10px;
    border-radius: 10px;
    padding: 20px
}

button {
    margin: 0 5px;
}
</style>