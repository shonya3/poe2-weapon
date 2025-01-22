import { createRouter, createWebHistory } from 'vue-router';

import HomeView from './views/HomeView.vue';
import ClipboardFlowView from './views/ClipboardFlowView.vue';

const routes = [
	{ path: '/', component: HomeView },
	{ path: '/clipboard-flow', component: ClipboardFlowView },
];

export const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes,
});
