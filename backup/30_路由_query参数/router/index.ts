

// 引入 createRouter
import { createRouter, createWebHistory, createWebHashHistory } from "vue-router";
// 引入组件
import Home from '@/pages/Home.vue'
import News from '@/pages/News.vue'
import About from '@/pages/About.vue'
import Detail from '@/pages/Detail.vue'

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
            component: News,
            children: [
                {
                    name:'neirong',
                    path: 'detail',
                    component: Detail
                }
            ]
        },
        {
            name: 'guanyu',
            path: '/about',
            component: About
        }
    ]
})

export default router

