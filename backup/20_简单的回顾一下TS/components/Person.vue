<template>
    <div class="person">

    </div>
</template>
  
<script lang="ts" setup name="Person">
import { type Persons } from '@/types'
// let person: Array<PersonInter> = [
//     { id: '123', name: 'zhangsan', age: 18 },
//     { id: '124', name: 'lisi', age: 19 },
//     { id: '125', name: 'wangwu', age: 20 }
// ]

let person: Persons = [
    { id: '123', name: 'zhangsan', age: 18 },
    { id: '124', name: 'lisi', age: 19 },
    { id: '125', name: 'wangwu', age: 20 }
]



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