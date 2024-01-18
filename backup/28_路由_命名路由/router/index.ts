

// 引入 createRouter
import { createRouter, createWebHistory, createWebHashHistory } from "vue-router";
// 引入组件
import Home from '@/pages/Home.vue'
import News from '@/pages/News.vue'
import About from '@/pages/About.vue'

// 第二步 创建路由器
const router = createRouter({
    history: createWebHistory(),
    // history:createWebHashHistory(),
    routes: [
        {
            name: 'zhuye',
            path: '/home',
            component: Home
        },
        {
            name: 'xinwen',
            path: '/news',
            component: News
        },
        {
            name: 'guanyu',
            path: '/about',
            component: About
        }
    ]
})

export default router

