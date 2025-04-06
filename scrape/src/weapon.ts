export type DamageType = 'phys' | 'fire' | 'cold' | 'lightning' | 'chaos';
export type FlatDamage = {
	damage_type: DamageType;
	range: [number, number];
};
export type WeaponBaseStats = {
	base: string;
	item_class: ItemClass;
	img: string;
	damages: Array<FlatDamage>;
	aps: number;
};

export const SUPPORTED_ITEM_CLASSES = [
	'One Hand Maces',
	'Two Hand Maces',
	'Quarterstaves',
	'Bows',
	'Crossbows',
	'Spears',
] as const;
export type ItemClass = (typeof SUPPORTED_ITEM_CLASSES)[number];
