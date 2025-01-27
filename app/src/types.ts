export const SUPPORTED_ITEM_CLASSES = [
	'One Hand Maces',
	'Two Hand Maces',
	'Quarterstaves',
	'Bows',
	'Crossbows',
] as const;

export type ItemClass = (typeof SUPPORTED_ITEM_CLASSES)[number];

export const RUNES_VARIANTS = ['Iron', 'Desert', 'Glacial', 'Storm'] as const;
export type Rune = (typeof RUNES_VARIANTS)[number];

export const DAMAGE_TYPES_VARIANTS = ['physical', 'fire', 'cold', 'lightning', 'chaos'];
export type DamageType = (typeof DAMAGE_TYPES_VARIANTS)[number];

export type Range = [number, number];

export type FlatDamage = {
	damage_type: DamageType;
	range: Range;
};

export type Weapon = {
	base: string;
	item_class: ItemClass;
	quality: number;
	phys: number | null;
	atk_spd: number | null;
	flats: Array<FlatDamage>;
	runes: Array<Rune>;
};

export type Dps = {
	total: number;
	pdps: number;
	edps: number;
	cdps: number;
};

export type DpsWithRunes = {
	runes: [Rune] | [Rune, Rune];
	dps: Dps;
};

export type ClipboardFlowData = {
	weapon: WeaponWithCalculatedRunes;
	img: string;
	elapsed: number;
	weapon_q20?: WeaponWithCalculatedRunes;
};

export type WeaponWithCalculatedRunes = {
	weapon: Weapon;
	dps: Dps;
	dps_with_different_runes: Array<DpsWithRunes>;
};
