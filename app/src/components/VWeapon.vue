<script setup lang="ts">
import { Dps, Weapon } from '../types';
import VDps from './VDps.vue';
import VRune from './VRune.vue';

defineProps<{ weapon: Weapon; img: string; dps: Dps }>();
</script>

<template>
	<div class="flex gap-2 items-center">
		<img :src="img" alt="" class="h-16" />
		<div class="flex pt-4 pb-4 items-center justify-between">
			<div>
				<span class="text-2xl text-stone-700">
					{{ weapon.base }}
				</span>
				<div class="text-stone-500 text-base leading-5 flex gap-2">
					<div class="flex gap-1">dps: <VDps class="text-stone-700" :dps="dps" /></div>
					<div>
						quality: <span class="text-stone-600">{{ weapon.quality }}</span>
					</div>

					<ul class="flex items-center">
						<li v-for="(rune, i) in weapon.runes">
							<VRune :size="30" :variant="rune" :key="i" />
						</li>
					</ul>
				</div>
			</div>
		</div>
	</div>
</template>
