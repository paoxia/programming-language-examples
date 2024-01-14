<template>
    <div class="person">

    </div>
</template>
  
<script lang="ts" setup name="Person">
let person = {
    id: '123123',
    name: 'zhangsan',
    age: 60,
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