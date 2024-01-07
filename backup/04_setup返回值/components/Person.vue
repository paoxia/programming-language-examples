<template>
    <div class="person">
        <h2>name :{{ name }}</h2>
        <h2>age :{{ age }}</h2>
        <button @click="changeName">修改名字</button>
        <button @click="changeAge">修改年龄</button>
        <button @click="showTel">查看联系方式</button>
    </div>
</template>

<script lang="ts">
export default {
    name: 'Person',
    beforeCreate() {
        console.log('beforeCreate')
    },
    setup() {
        // console.log(this)  this为undefined vue3中弱化this
        // 
        // data 
        let name = '张三' // 非响应式
        let age = 18 // 非响应式
        let tel = '12312312312' // 非响应式
        // function
        function changeName() {
            name = 'zhangsan' // 页面不会有变化
        }

        function changeAge() {
            age += 1
        }

        function showTel() {
            alert(tel)
        }
        return () => '哈哈' //setup返回值可以是函数
        // return { name, age, changeName, changeAge, showTel }
    },
};
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