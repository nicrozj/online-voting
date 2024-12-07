<script setup>
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { getVoteById, getOptionsByVoteId } from "../services/api";

const route = useRoute();
const vote = ref("");
const options = ref([]);

const getVoteOptions = async () => {
	try {
		const response = await getOptionsByVoteId(parseInt(route.params.id));
		options.value = response;
	}
	catch (err) {
		console.log(err);
	}
}

onMounted(async () => {
	try {
		const response = await getVoteById(parseInt(route.params.id));
		vote.value = response;
	}
	catch (err) {
		console.log(err);
	}
	getVoteOptions();
});

</script>

<template>
	<main class="min-h-screen bg-stone-100">
		<section
			class="pt-10 max-w-[1000px] mx-auto text-center flex flex-col gap-4"
		>
			<h1 class="text-3xl">{{ vote.title }}</h1>
			<p class="text-xl">{{ vote.description }}</p>

			<h1 class="text-3xl">
				Варианты ответов:
			</h1>
			<div v-for="option in options">
				{{ option.option_text }}
			</div>
		</section>
	</main>
</template>
