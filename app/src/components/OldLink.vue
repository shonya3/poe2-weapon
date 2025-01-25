<script setup lang="ts">
import { computed } from 'vue';
import { RouterLinkProps } from 'vue-router';

type Props = { kind: 'app'; to?: RouterLinkProps['to'] } | { kind: 'browser'; to: string };

const props = defineProps<Props>();

defineEmits<{
	click: [];
}>();

const css_class = '';

const isBrowserLink = (props: Props): props is { kind: 'browser'; to: string } =>
	props.kind === 'browser' && typeof props.to === 'string';

/** That's a Browser href helper, because vue template fails to infer
 *  if app === "browser", then "to" can be only a string
 */
const href = computed(() => {
	if (isBrowserLink(props)) {
		return props.to;
	}
	return null;
});
</script>

<template>
	<template v-if="kind === 'app'">
		<RouterLink :class="css_class" :to="to ?? '#'" @click.native="$emit('click')">
			<slot></slot>
		</RouterLink>
	</template>
	<template v-if="kind === 'browser' && href">
		<BrowserLink :class="css_class" :href="href">
			<slot></slot>
		</BrowserLink>
	</template>
</template>
