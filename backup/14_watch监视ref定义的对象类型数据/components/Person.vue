<template>
    <div class="person">
        <h2>name : {{ person.name }}</h2>
        <h2>age : {{ person.age }}</h2>
        <button @click="changeName">changeName</button>
        <button @click="changeAge">changeAge</button>
        <button @click="changePerson">changePerson</button>
    </div>
</template>



<script lang="ts" setup name="Person">

import { ref, watch } from 'vue'
let person = ref({
    name: 'zhangsan',
    age: 18,
})


function changeName() {
    person.value.name += '-'
}

function changeAge() {
    person.value.age += 1
}

function changePerson() {
    person.value = { name: "lisi", age: 19 }
}

// 监视的是对象地址值 想要监视对象内部属性的变化 需要开始深度监视
watch(person, (newValue, oldValue) => {
    console.log('person changed', newValue, oldValue)
}, { deep: true })
// 修改对象的属性值，newValue和oldValue是同一个值


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
