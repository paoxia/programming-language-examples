<template>
    <div class="home">
        <img src="https://images.dog.ceo/breeds/pointer-german/n02100236_5698.jpg" alt="">
    </div>
</template>
  
<script setup lang="ts" name="Home">
import { onUnmounted } from 'vue'
onUnmounted(() => {
    console.log("home卸载了")
})

</script>
  
<style scoped>
.home {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
}

image {
    width: 50%;
}
</style>