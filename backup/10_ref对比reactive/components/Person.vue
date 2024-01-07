<template>
    <div class="person">
        <h2>brand :{{ car.brand }}</h2>
        <h2>price :{{ car.price }}</h2>
        <button @click="changePrize">修改价格</button>
        <button @click="changeBrand">修改品牌</button>
        <button @click="changeCar">修改车</button>
        <hr>
        <h2>当前求和为: {{ sum }}</h2>
        <button @click="changeSum">sum+1</button>

        <hr>
    </div>
</template>



<script lang="ts" setup name="Person">

import { ref, reactive } from 'vue';

let car = ref({ brand: "huawei", price: 100 })
let sum = ref(0)


function changePrize() {
    car.value.price += 10

}

function changeBrand() {
    car.value.brand = '小米'

}

function changeCar() {
    car.value = { brand: "weilai", price: 99 } //ref可以直接修改
    // car = { brand: "weilai", price: 99 } // reactive页面不更新
    // reactive Object.assign(car, { brand: "weilai", price: 99 })
}

function changeSum() {
    // sum.value += 1
    // sum = ref(9) // 不能这么写
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
