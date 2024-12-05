import { createApp } from "vue";
import { createMemoryHistory, createRouter } from "vue-router";
import globalComponents from "./components/global";

import App from "./App.vue";
import Home from "./views/Home.vue";
import CreateVote from "./views/CreateVote.vue";
import VotePage from "./views/VotePage.vue";
import "../public/tailwind.css";

const router = createRouter({
	history: createMemoryHistory(),
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
