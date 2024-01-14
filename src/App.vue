<template>
  <Person a="haha" />
</template>


<script lang="ts" setup name="App">
import Person from './components/Person.vue'
import { reactive } from 'vue';
import { type Persons } from '@/types'

let person = reactive<Persons>([
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