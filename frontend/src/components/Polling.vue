<script setup lang="ts">
import { deletePollingById } from '../services/api';

const { title, id } = defineProps<{ title: string; id: number }>();

const deletePolling = async () => {
	console.log(id);
	await deletePollingById(id);
	window.location.reload();
}

</script>

<template>
	<div
		class="relative flex flex-col justify-between max-w-md h-full gap-4 bg-white rounded-xl border p-8"
	>
		<h1 class="font-medium text-xl text-center break-words">{{ title }}</h1>
		<RouterLink
			:to="'/polling/' + id"
			class="bg-green-400 py-2 rounded-xl text-white font-medium hover:scale-95 transition-transform text-center"
		>
			Проголосовать
		</RouterLink>
		<button 
			@click="deletePolling"
			class="absolute -right-4 -top-4 flex justify-center items-center size-8 bg-red-400 rounded-lg text-white hover:scale-90 transition-transform delay-75">
			<span class="material-symbols-outlined">
				close
			</span>
		</button>
	</div>
</template>
