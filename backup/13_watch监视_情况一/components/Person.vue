<template>
    <div class="person">
        <h2> current_sum :{{ sum }}</h2>
        <button @click="changeSum">sum+1</button>
    </div>
</template>



<script lang="ts" setup name="Person">

import { ref, watch } from 'vue'
let sum = ref(0)

function changeSum() {
    sum.value += 1
}

const x = watch(sum, (newValue, oldValue) => {
    console.log('sum变化了', newValue, oldValue)
    if (newValue >= 3) {
        x()
    }
})

console.log(x)

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
