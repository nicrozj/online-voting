<script setup lang="ts">
import { ref } from "vue";

import { fetchAddVote } from "../services/api";

const titleInput = ref("");
const descriptionInput = ref("");
const data = ref(null);

const handleAddVote = async () => {
  try {
    data.value = await fetchAddVote(titleInput.value, descriptionInput.value);
    console.log('Голосование добавлено: ', data.value);
    titleInput.value = '';
    descriptionInput.value = '';
  } 
  catch (err) {
    console.log(err);
  }
}
</script>

<template>
  {{ data }}
  <main class="bg-stone-100 min-h-screen">
    <section class="max-w-[1000px] mx-auto flex justify-center items-center">
      <div class="flex flex-col gap-4 items-center max-w-md">
        <UInput
          v-model="titleInput"
          title="Название"
          placeholder="Введите название"
        />
        <UInput
          v-model="descriptionInput"
          type="textarea"
          title="Описание"
          placeholder="Введите описание"
        />
        <button
          @click="handleAddVote"
          class="bg-blue-500 text-white px-4 py-2 rounded-lg hover:scale-95 transition-transform"
        >
          Добавить
        </button>
      </div>
    </section>
  </main>
</template>
