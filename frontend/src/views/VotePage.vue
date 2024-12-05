<script setup>
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { fetchVoteById } from "../services/api";

const route = useRoute();
const vote = ref("");

const fetchData = async () => {
	try {
		const response = await fetchVoteById(parseInt(route.params.id));
		console.log('Голосование успешно изъято: ', response);
		vote.value = response;
	}
	catch (err) {
		console.log(err);
	}
}
onMounted(fetchData);

</script>

<template>
	<main class="min-h-screen bg-stone-100">
		<section
			class="pt-10 max-w-[1000px] mx-auto text-center flex flex-col gap-4"
		>
			<h1 class="text-3xl">{{ vote.title }}</h1>
			<p class="text-xl">{{ vote.description }}</p>
		</section>
	</main>
</template>
