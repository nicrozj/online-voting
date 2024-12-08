<script setup lang="ts">
import { ref, onMounted } from "vue";
import Polling from "../components/Polling.vue";
import { getPollings } from "../services/api";

const data = ref(null);

const fetchData = async () => {
	data.value = await getPollings();
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
			<polling
				v-for="polling in data"
				:title="polling.title"
				:id="polling.id"
			/>
		</section>
	</main>
</template>
