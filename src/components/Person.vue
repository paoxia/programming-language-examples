<template>
    <div class="person">
        <h2>{{ a }}</h2>
    </div>
</template>
  
<script lang="ts" setup name="Person">
import { defineProps } from 'vue'

defineProps(['a'])

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