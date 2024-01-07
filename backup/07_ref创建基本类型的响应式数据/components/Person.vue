<template>
    <div class="person">
        <h2>name :{{ name }}</h2>
        <h2>age :{{ age }}</h2>
        <h2>address :{{ address }}</h2>
        <button @click="changeName">修改名字</button>
        <button @click="changeAge">修改年龄</button>
        <button @click="showTel">查看联系方式</button>
    </div>
</template>



<script lang="ts" setup name="Person">
import { ref } from 'vue'
let name = ref("张三"); // 响应式
let age = ref(18); // 响应式
let tel = "12312312312"; // 非响应式
let address = '杭州'
// function
function changeName() {
    name.value = "zhangsan"; // 页面不会有变化
}

function changeAge() {
    age.value += 1;
}

function showTel() {
    alert(tel);
}
</script>

<style scoped>
.person {
    box-shadow: 0 0 10px;
    border-radius: 10px;
    padding: 20px;
}

button {
    margin: 0 5px;
}
</style>
