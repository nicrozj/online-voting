<script setup lang="ts">
const props = defineProps<{
  title?: string ;
  type?: string;
  placeholder?: string;
  class?: string;
  autocomplete?: string;
}>();
const model: any = defineModel();
</script>
<template>
  <div>
    <label
      v-if="props.title"
      for="description"
      class="block text-sm font-medium text-gray-900"
      >{{ props.title }}</label
    >
    <textarea
      v-if="props.type == 'textarea'"
      :type="props.type"
      :class="props.class"
      id="description"
      v-model="model"
      :placeholder="props.placeholder"
      class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 w-full p-2"
    >
    </textarea>

    <input
      v-else
      :autocomplete="props.autocomplete"
      v-model="model"
      class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 w-full p-2"
      required
    />
  </div>
</template>
