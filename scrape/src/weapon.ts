export type DamageType = 'phys' | 'fire' | 'cold' | 'lightning' | 'chaos';
export type FlatDamage = {
	damage_type: DamageType;
	value: [number, number];
};
export type WeaponStats = {
	base: string;
	img: string;
	damages: Array<FlatDamage>;
	aps: number;
};
