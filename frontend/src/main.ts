import { createApp } from "vue";
import { createWebHistory, createRouter } from "vue-router";
import globalComponents from "./components/global";

import App from "./App.vue";
import Home from "./views/Home.vue";
import CreatePolling from "./views/CreatePolling.vue";
import PollingPage from "./views/PollingPage.vue";
import Registration from "./views/Registration.vue";
import Login from "./views/Login.vue";
import "../public/tailwind.css";

const router = createRouter({
	history: createWebHistory(),
	routes: [
		{
			path: "/",
			component: Home,
		},
		{
			path: "/create",
			component: CreatePolling,
		},
		{
			path: "/polling/:id",
			component: PollingPage,
		},
		{
			path: "/registration",
			component: Registration,
		},
		{
			path: "/login",
			component: Login,
		}
	],
});

const app = createApp(App);
app.use(router);
app.use(globalComponents);
app.mount("#app");
