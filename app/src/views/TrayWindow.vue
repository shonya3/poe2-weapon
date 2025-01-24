<script setup lang="ts"></script>

<template>
	<div class="py-4 rounded-lg">
		<div class="flex items-center mb-2">
			<span class="text-2xl mr-2">⚔️</span>
			<span class="text-base font-bold text-gray-700">
				Status: <span class="text-green-600">Ready to capture.</span>
			</span>
		</div>

		<p class="text-sm text-gray-600 mb-3">
			Hit <span class="font-medium text-gray-800">CTRL+C</span> when you're ready.
		</p>

		<div class="flex items-center text-sm text-gray-500 mt-4">
			<span class="mr-2 text-blue-500">✔️</span>
			<p>It's safe to close this window. The app will keep running in the tray.</p>
		</div>
	</div>
</template>
