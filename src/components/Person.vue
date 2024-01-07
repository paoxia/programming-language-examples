<template>
    <div class="person">
        <h2>name : {{ person.name }}</h2>
        <h2>age : {{ person.age }}</h2>
        <button @click="changeName">修改名字</button>
        <button @click="changeAge">修改年龄</button>
    </div>
</template>



<script lang="ts" setup name="Person">

import { ref, reactive } from 'vue';


let person = reactive({
    name: 'zhangsan',
    age: 18,
})

function changeName() {
    person.name += '-'
}

function changeAge() {
    person.age += 1
}

</script>


<style scoped >
.person {
    box-shadow: 0 0 10px;
    border-radius: 10px;
    padding: 20px;
}

button {
    margin: 0 5px;
}
</style>
