<template>
	<header class="bg-stone-100">
		<div class="max-w-[1000px] mx-auto py-4 flex justify-between">
			<RouterLink class="text-2xl font-medium" to="/"> mrpenis.ru </RouterLink>
			<div class="flex gap-4">
				<RouterLink class="text-xl font-medium flex items-center bg-green-400 text-white px-4 py-1 rounded-lg hover:scale-95 transition-transform" to="/login">Вход</RouterLink>
				<RouterLink class="text-xl font-medium flex items-center text-green-400 border-2 px-4 py-1 border-green-400 rounded-lg hover:scale-95 transition-transform" to="/registration">Регистрация</RouterLink>
			</div>
		</div>
	</header>
</template>
