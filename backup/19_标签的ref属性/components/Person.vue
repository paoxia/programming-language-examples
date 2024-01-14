<template>
    <div class="person">
        <h1 ref='title1'>china</h1>
        <h2 ref='title2'>zhejiang</h2>
        <h3 ref='title3'>hangzhou</h3>
        <button @click="showLog">click there show h2</button>
    </div>
</template>
  
<script lang="ts" setup name="Person">
import { ref, defineExpose } from 'vue'
let title2 = ref()

let name = ref('zhangsan')
let age = ref(18)
defineExpose({ name, age })

function showLog() {
    console.log(title2.value)
}
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