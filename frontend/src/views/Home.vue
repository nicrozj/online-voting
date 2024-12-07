<script setup lang="ts">
import { ref, onMounted } from "vue";
import Vote from "../components/Vote.vue";
import { getVotes } from "../services/api";

const data = ref(null);

const fetchData = async () => {
	data.value = await getVotes();
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
				:id="vote.id"
			/>
		</section>
	</main>
</template>
