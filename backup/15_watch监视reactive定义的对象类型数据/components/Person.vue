<template>
    <div class="person">
        <h2>name : {{ person.name }}</h2>
        <h2>age : {{ person.age }}</h2>
        <button @click="changeName">changeName</button>
        <button @click="changeAge">changeAge</button>
        <button @click="changePerson">changePerson</button>
        <hr>
        <h2>{{ obj.a.b.c }}</h2>
        <button @click="test">changePerson</button>
    </div>
</template>



<script lang="ts" setup name="Person">

import { reactive, watch } from 'vue'
let person = reactive({
    name: 'zhangsan',
    age: 18,
})

let obj = reactive({
    a: {
        b: {
            c: 666
        }
    }
})


function changeName() {
    person.name += '-'
}

function changeAge() {
    person.age += 1
}

function changePerson() {
    Object.assign(person, { name: "lisi", age: 19 })

}

function test() {
    obj.a.b.c = 999
}

// 默认深度watch
watch(person, (newValue, oldValue) => {
    console.log('person changed', newValue, oldValue)
})
// 修改对象的属性值，newValue和oldValue是同一个值


watch(obj, (newValue, oldValue) => {
    console.log('obj changed', newValue, oldValue)
})
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
