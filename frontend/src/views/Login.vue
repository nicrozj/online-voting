<script setup>

</script>
<template>
    <section class="max-w-[1000px] mx-auto flex justify-center items-center">
        <div class="flex flex-col items-center gap-4 bg-white p-4 rounded-lg mt-60">
            <UInput 
                type="text" 
                placeholder="Логин"/>
            <UInput 
                type="text" 
                placeholder="Пароль"/>
            <p class="text-xs text-center">Еще не зарегистрированы? <br/> <RouterLink to="/registration" class="text-blue-800">Регистрация</RouterLink></p>
            <UButton>Войти</UButton>
        </div>
    </section>
</template>