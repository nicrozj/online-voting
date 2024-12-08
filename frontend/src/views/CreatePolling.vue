<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";

import { addPolling } from "../services/api";
import UInput from "@/components/global/UInput.vue";

const titleInput = ref("");
const descriptionInput = ref("");
const data = ref(null);

const router = useRouter();

const handleAddPolling = async () => {
	try {
		data.value = await addPolling(titleInput.value, descriptionInput.value, options.value);
		titleInput.value = "";
		descriptionInput.value = "";
		router.push("/");
	} catch (err) {
		console.log(err);
	}
};

const optionInput = ref("");
const options = ref([]);

const addOption = () => {
	if (!optionInput.value) {
		console.log("Ошибка добавления варианта! Текст не может быть пуст!")
	};
	options.value.push(optionInput.value);
	optionInput.value = "";
}

const deleteOption = (id: number) => {
	options.value = options.value.filter((option, idOption) => {
		return idOption != id;
	})
}
</script>

<template>
	<Header />
	<section class="flex flex-col items-center gap-10 max-w-[1000px] mx-auto">
		<div class="grid grid-cols-1 md:grid-cols-2 gap-10 bg-white rounded-lg p-4">
			<div class="flex flex-col gap-4 items-center max-w-md">
				<h1 class="text-xl">Добавить голосование</h1>
				<UInput v-model="titleInput" placeholder="Введите название" />
				<UInput v-model="descriptionInput" type="textarea" placeholder="Введите описание" />
			</div>
			<div class="flex flex-col items-center gap-4">
				<h1 class="text-xl">Добавить вариант</h1>
				<div class="flex flex-col gap-2">
					<div class="flex gap-2">
						<UInput v-model="optionInput" placeholder="Введите вариант"/>
						<UButton @click="addOption" class="flex items-center">
							<span class="material-symbols-outlined">
								add
							</span>
						</UButton>
					</div>
					<div class="flex items-center justify-between bg-stone-100 rounded-lg p-1"
						v-for="(option, id) in options">
						<span>{{ id + 1 }}. {{ option }}</span>
						<button @click="deleteOption(id)"
							class="flex items-center justify-center text-white bg-red-400 size-8 rounded-lg hover:scale-90 transition-transform">
							<span class="material-symbols-outlined text-md">
								close
							</span>
						</button>
					</div>
				</div>
			</div>
			<UButton @click="handleAddPolling">
				Сохранить
			</UButton>
		</div>
	</section>
</template>
