import UButton from "./UButton.vue";
import UInput from "./UInput.vue";
import Header from "./Header.vue";

const components = [
	{ name: "UButton", component: UButton },
	{ name: "UInput", component: UInput },
	{ name: "Header", component: Header },
];

export default {
	install(app: any) {
		components.forEach(({ name, component }) => {
			app.component(name, component);
		});
	},
};
