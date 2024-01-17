<template>
  <Person v-if="isShow" />
  <!-- :来接数据 -->
</template>


<script lang="ts" setup name="App">
import Person from './components/Person.vue'
import { reactive, onMounted } from 'vue';
import { type Persons } from '@/types'

let isShow = true
// 挂载后
onMounted(() => { console.log("父-挂载后") })
</script>

<style>
.app {
  background-color: blue;
  box-shadow: 0 0 10px;
  border-radius: 10px;
  padding: 20px
}
</style>