<template>
    <div class="about">
        <h2>weclome</h2>
    </div>
</template>
  
<script setup lang="ts" name="About">

import { onUnmounted } from 'vue'

onUnmounted(() => {
    console.log("about卸载了")
})

</script>
  
<style scoped>
.about {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    color: rgb(85, 84, 84);
    font-size: 18px;
}
</style>