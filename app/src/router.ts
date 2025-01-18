import { createRouter, createWebHistory } from 'vue-router';

import HomeView from './HomeView.vue';
import AboutView from './AboutView.vue';

const routes = [
	{ path: '/', component: HomeView },
	{ path: '/about', component: AboutView },
];

export const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes,
});
