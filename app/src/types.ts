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
	quality: number;
	phys: number | null;
	atk_spd: number | null;
	flats: Array<FlatDamage>;
	runes: [Rune, Rune];
};

export type Dps = {
	total: number;
	pdps: number;
	edps: number;
};

export type DpsWithRunes = {
	runes: [Rune, Rune];
	dps: Dps;
};

export type ClipboardFlowData = {
	weapon: WeaponWithCalculatedRunes;
	elapsed: number;
	weapon_q20?: WeaponWithCalculatedRunes;
};

export type WeaponWithCalculatedRunes = {
	weapon: Weapon;
	dps: Dps;
	dps_with_different_runes: Array<DpsWithRunes>;
};
