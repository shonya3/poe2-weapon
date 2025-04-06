<script setup lang="ts" generic="UNUSED">
import { RouterLinkProps } from 'vue-router';

export type Props =
	| { kind: 'app'; to: RouterLinkProps['to']; onClick?: never }
	| { kind: 'app'; onClick: () => void; to?: never }
	| { kind: 'browser'; to: string; onClick?: never };

defineProps<Props>();
</script>

<template></template>

<style scoped></style>
