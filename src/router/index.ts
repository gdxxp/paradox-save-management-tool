import { createRouter, createWebHistory } from 'vue-router'

import HomeView from '../view/HomeView.vue'
import SplashscreenView from '../view/SplashscreenView.vue'

import SavefileList from '../components/SavefileList.vue'
import ChooseGame from '../components/ChooseGame.vue'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'HomeView',
            component: HomeView,
            redirect: '/choice',
            children: [
                {
                    path: '/choice',
                    name: 'choice',
                    component: ChooseGame,
                    meta: {
                        keepAlive: true
                    }
                },
                {
                    path: '/eu4',
                    name: 'eu4',
                    component: SavefileList,
                    meta: {
                        keepAlive: true
                    }
                },
                {
                    path: '/ck3',
                    name: 'ck3',
                    component: SavefileList,
                    meta: {
                        keepAlive: true
                    }
                },
                {
                    path: '/stellaris',
                    name: 'stellaris',
                    component: SavefileList,
                    meta: {
                        keepAlive: true
                    }
                },
                {
                    path: '/hoi4',
                    name: 'hoi4',
                    component: SavefileList,
                    meta: {
                        keepAlive: true
                    }
                },
                {
                    path: '/v3',
                    name: 'v3',
                    component: SavefileList,
                    meta: {
                        keepAlive: true
                    }
                },
                {
                    path: '/ck2',
                    name: 'ck2',
                    component: SavefileList,
                    meta: {
                        keepAlive: true
                    }
                }
            ]
        },
        {
            path: '/splashscreen',
            component: SplashscreenView
        }
    ],
})

export default router
