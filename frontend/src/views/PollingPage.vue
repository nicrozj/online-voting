<script setup>
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { getPollingById, getOptionsByPollingId } from "../services/api";

const route = useRoute();
const polling = ref("");
const options = ref([]);

const getPollingOptions = async () => {
	try {
		const response = await getOptionsByPollingId(parseInt(route.params.id));
		options.value = response;
	}
	catch (err) {
		console.log(err);
	}
}

onMounted(async () => {
	try {
		const response = await getPollingById(parseInt(route.params.id));
		polling.value = response;
	}
	catch (err) {
		console.log(err);
	}
	getPollingOptions();
});

</script>

<template>
	<main class="min-h-screen bg-stone-100">
		<section
			class="pt-10 max-w-[1000px] mx-auto grid grid-cols-1 md:grid-cols-2 gap-4"
		>
			<div>
				<h1 class="text-3xl">{{ polling.title }}</h1>
				<p class="text-xl">{{ polling.description }}</p>
			</div>
			<div class="bg-white p-4 rounded-lg">
				<h1 class="text-3xl">
					Варианты ответов:
				</h1>
				<div v-for="option in options">
					<input name="option" type="radio">
					{{ option.option_text }}
				</div>
			</div>
		</section>
	</main>
</template>
