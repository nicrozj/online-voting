<script setup lang="ts">
const { title, path } = defineProps<{ title: string; path: string }>();
</script>

<template>
	<div
		class="flex flex-col justify-between max-w-md h-full gap-4 bg-white rounded-xl border p-8"
	>
		<h1 class="font-medium text-xl text-center break-words">{{ title }}</h1>
		<RouterLink
			:to="path"
			class="bg-green-400 py-2 rounded-xl text-white font-medium hover:scale-95 transition-transform text-center"
		>
			Проголосовать
		</RouterLink>
	</div>
</template>
