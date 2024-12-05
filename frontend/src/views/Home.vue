<script setup lang="ts">
import Vote from "../components/Vote.vue";

import { ref, onMounted } from "vue";

const data = ref(null);

const fetchData = async () => {
	try {
		const response = await fetch("http://127.0.0.1:8080/votes");

		if (!response.ok) {
			throw new Error("Ошибка при получении данных");
		}

		const json = await response.json();
		data.value = json;
	} catch (error) {
		console.error("Ошибка при получении данных:", error);
	}
};

onMounted(fetchData);
</script>
<template>
	<main class="flex flex-col gap-4 bg-stone-100 min-h-screen items-center">
		<RouterLink
			to="/create"
			class="bg-stone-600 text-3xl rounded-xl text-white py-2 px-2 hover:scale-95 transition-transform"
		>
			Создать опрос
		</RouterLink>
		<section class="grid grid-cols-1 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 justify-center items-center w-[70%]">
			<Vote
				v-for="vote in data"
				:title="vote.title"
				:path="'/vote/' + vote.id"
			/>
		</section>
	</main>
</template>
