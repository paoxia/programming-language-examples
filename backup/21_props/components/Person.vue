<template>
    <div class="person">

        <!-- <h2>{{ a }}</h2> -->
        <h2>{{ list }}</h2>
        <ul>
            <li v-for="p in list" :key="p.id">{{ p.name }}--{{ p.age }}</li>
        </ul>
    </div>
</template>
  
<script lang="ts" setup name="Person">
import { defineProps, withDefaults } from 'vue'
import { type Persons } from '@/types'

// 只接收list
// defineProps(['a', 'list'])

// 接收list + 限制类型
// defineProps<{ list: Persons }>()

// 接收list + 限制类型 + 限制必要性 + 指定默认值
// withDefaults(defineProps<{ list?: Persons }>(), {
//     list: () => [{ id: '123', name: 'wangwu', age: 10 }]
// })


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