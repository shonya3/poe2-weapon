import { createRouter, createWebHistory } from 'vue-router';

import ClipboardFlowView from './views/ClipboardFlowView.vue';
import UpdateView from './views/UpdateView.vue';
import TrayWindow from './views/TrayWindow.vue';

const routes = [
	{ path: '/clipboard-flow', component: ClipboardFlowView },
	{ path: '/update/:new_version', component: UpdateView },
	{ path: '/tray-window', component: TrayWindow },
];

export const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes,
});
