import { createRouter, createWebHistory } from 'vue-router';

import HomeView from './views/HomeView.vue';
import ClipboardFlowView from './views/ClipboardFlowView.vue';
import UpdateView from './views/UpdateView.vue';

const routes = [
	{ path: '/', component: HomeView },
	{ path: '/clipboard-flow', component: ClipboardFlowView },
	{ path: '/update/:new_version', component: UpdateView },
];

export const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes,
});
