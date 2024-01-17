<template>
  <h2 a="1+1" :b="1 + 1" c="x" :d="x"></h2>
  <Person a="haha" :list="personList" />
  <!-- :来接数据 -->
</template>


<script lang="ts" setup name="App">
import Person from './components/Person.vue'
import { reactive } from 'vue';
import { type Persons } from '@/types'

let x = 9
let personList = reactive<Persons>([
  { id: '123', name: 'zhangsan', age: 18 },
  { id: '124', name: 'lisi', age: 19 },
  { id: '125', name: 'wangwu', age: 20 }
])

</script>

<style>
.app {
  background-color: blue;
  box-shadow: 0 0 10px;
  border-radius: 10px;
  padding: 20px
}
</style>