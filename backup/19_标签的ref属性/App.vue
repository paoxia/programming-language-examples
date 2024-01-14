<template>
  <div>
    <h2 ref='title2'>zhejiang</h2>
    <button @click="showLog">click there show h2</button>
    <hr>
    <Person ref="ren" />
    <button @click="showRen">click there show ren</button>
  </div>
</template>


<script lang="ts" setup name="App">
import Person from './components/Person.vue'
import { ref } from 'vue'
let title2 = ref()
let ren = ref()

function showLog() {
  console.log(title2.value)
}

function showRen() {
  console.log(ren.value)
  console.log(ren.value.age)
  console.log(ren.value.name)
}

</script>

<style>
.app {
  background-color: blue;
  box-shadow: 0 0 10px;
  border-radius: 10px;
  padding: 20px
}
</style>