<template>
    <div class="person">
        <h2>brand :{{ car.brand }}</h2>
        <h2>price :{{ car.price }}</h2>
        <button @click="changePrize">修改价格</button>
        <h2>game list</h2>
        <ul>
            <li v-for="g in games" v-bind:key="g.id">{{ g.name }}</li>
        </ul>

        <button @click="changeFirstGame">修改第一个游戏的名字</button>

        <hr>
        <h2>test:{{ obj.x }}</h2>
        <button @click="changeObj">test change abc</button>
    </div>
</template>



<script lang="ts" setup name="Person">

import { ref } from 'vue';

let car = ref({ brand: "huawei", price: 100 })
let games = ref([
    { id: 's01', name: 'wangzherongyao' },
    { id: 's02', name: 'yuanshen' },
    { id: 's03', name: 'sanguozhi' }
])

let obj = ref({
    x: 666
})

function changePrize() {
    car.value.price += 10
    console.log(car.value.price)
}

function changeFirstGame() {
    games.value[0].name = 'mario'
}

function changeObj() {
    obj.value.x = 999
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

li {
    font-size: 30px;
}
</style>
