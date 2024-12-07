import { createApp } from "vue";
import { createWebHistory, createRouter } from "vue-router";
import globalComponents from "./components/global";

import App from "./App.vue";
import Home from "./views/Home.vue";
import CreateVote from "./views/CreateVote.vue";
import VotePage from "./views/VotePage.vue";
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
			component: CreateVote,
		},
		{
			path: "/vote/:id",
			component: VotePage,
		},
	],
});

const app = createApp(App);
app.use(router);
app.use(globalComponents);
app.mount("#app");
