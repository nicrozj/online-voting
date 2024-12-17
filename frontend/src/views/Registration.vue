<script setup>
import { ref } from 'vue';
import { registration } from '../services/api';

const loginInput = ref('');
const passwordInput = ref('');
const repeatPasswordInput = ref('');

const validatePasswordAndSubmit = async () => {
    if (passwordInput.value != repeatPasswordInput.value) { return "Пароли не совпадают!" }

    registration(loginInput.value, passwordInput.value);
}
 
</script>
<template>
    <section class="max-w-[1000px] mx-auto flex justify-center items-center">
        <div class="flex flex-col items-center gap-4 bg-white p-4 rounded-lg mt-52">
            <UInput 
                type="text" 
                placeholder="Логин"
                v-model="loginInput" />
            <UInput 
                type="password" 
                placeholder="Пароль"
                v-model="passwordInput" />
            <UInput 
                type="password" 
                placeholder="Повторите пароль"
                v-model="repeatPasswordInput" />
            <p class="text-xs text-center">Уже зарегистрированы? <br/> <RouterLink to="/login" class="text-blue-800">Войти</RouterLink></p>
            <UButton @click="validatePasswordAndSubmit">Войти</UButton>
        </div>
    </section>
</template>