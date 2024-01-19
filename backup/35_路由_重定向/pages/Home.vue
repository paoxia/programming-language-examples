<template>
    <div class="home">
        <img src="https://images.dog.ceo/breeds/pointer-german/n02100236_5698.jpg" alt="">
    </div>
</template>
  
<script setup lang="ts" name="Home">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router';

const router = useRouter()
onMounted(() => {
    setTimeout(
        () => {
            // console.log("jump")
            // 在此处编写一段代码,直接跳转
            router.push('/news')
        }, 3000)
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