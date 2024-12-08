<script setup>

</script>
<template>
    <section class="max-w-[1000px] mx-auto flex justify-center items-center">
        <div class="flex flex-col items-center gap-4 bg-white p-4 rounded-lg mt-52">
            <UInput 
                type="text" 
                placeholder="Логин"/>
            <UInput 
                type="password" 
                placeholder="Пароль"/>
            <UInput 
                type="password" 
                placeholder="Повторите пароль"/>
            <p class="text-xs text-center">Уже зарегистрированы? <br/> <RouterLink to="/login" class="text-blue-800">Войти</RouterLink></p>
            <UButton>Войти</UButton>
        </div>
    </section>
</template>