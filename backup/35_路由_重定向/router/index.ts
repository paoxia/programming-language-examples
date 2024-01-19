

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
                    name: 'neirong',
                    path: 'detail/:id/:title/:content',
                    component: Detail,
                    // 第一种param传入
                    props: true

                    // 第二种 函数写法
                    // props(route) {
                    //     return route.query
                    // }

                    // 对象写法
                    // props: {
                    //     id: 100,
                    //     title: 1200,
                    //     content: 300
                    // }
                }
            ]
        },
        {
            name: 'guanyu',
            path: '/about',
            component: About
        },{
            path:'/',
            redirect:'/home'
        }
    ]
})

export default router

