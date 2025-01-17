import type { Page } from 'playwright';
import type { WeaponStats, FlatDamage, DamageType } from './weapon';

export const WEAPON_TYPES = ['maces', 'quarterstaves', 'bows', 'crossbows'] as const;
export type WeaponType = (typeof WEAPON_TYPES)[number];

function page_url(weapon_type: WeaponType): string {
	return `https://www.poe2wiki.net/wiki/List_of_${weapon_type}`;
}

export async function scrap_page(weapon_type: WeaponType, page: Page): Promise<Array<WeaponStats>> {
	await page.goto(page_url(weapon_type));
	await page.waitForSelector('table', { timeout: 30000 });
	const result = await page.evaluate<Array<WeaponStats>>(() => {
		function parse_first_td(td: HTMLTableCellElement): { title: string; img: string } {
			const hoverbox_activator = td.querySelector('span.c-item-hoverbox__activator')!;
			const title = hoverbox_activator.querySelector('a')!.getAttribute('title')!;
			const src = hoverbox_activator.querySelector('img')!.getAttribute('src')!;
			return {
				title,
				img: `https://www.poe2wiki.net/${src}`,
			};
		}

		function parse_damage_td(td: HTMLTableCellElement): Array<FlatDamage> {
			const selectors: Record<DamageType, string> = {
				phys: '.-value',
				fire: '.-fire',
				lightning: '.-lightning',
				cold: '.-cold',
				chaos: '.-chaos',
			} as const;
			const entries = Object.entries(selectors) as Array<[DamageType, string]>;

			return Array.from(td.querySelectorAll('em')).map(em => {
				for (const [damage_type, selector] of entries) {
					const range = em.textContent!.trim().split('-').map(Number);
					if (range.length !== 2) {
						throw new Error(`Range length is not 2`);
					}

					if (em.matches(selector)) {
						return {
							damage_type,
							range: range as [number, number],
						};
					}
				}

				throw new Error('No damage type');
			});
		}

		function column_indexes(table: HTMLTableElement): { damage: number; attacks_per_second: number } {
			let column_index = 0;
			let damage = 0;
			let attacks_per_second = 0;

			const thead = table.querySelector('thead');
			if (!thead) {
				console.log(table);
				// throw new Error(`Scraping ${weapon_type} Now table thead`);
			}

			for (const th of table.querySelector('thead')!.querySelector('tr')!.querySelectorAll('th')) {
				if (th.textContent?.trim() === 'Damage') {
					damage = column_index;
				}
				if (th.textContent?.trim() === 'APS') {
					attacks_per_second = column_index;
				}

				column_index += 1;
			}

			return { damage, attacks_per_second };
		}

		return (
			Array.from(document.querySelectorAll('table'))
				// List of bases tables always have thead, but others don't
				.filter(table => table.querySelector('thead'))
				.flatMap(table => {
					const indexes = column_indexes(table);
					return Array.from(table.querySelector('tbody')!.querySelectorAll('tr')).map(tr => {
						const cells = tr.querySelectorAll('td');
						const { title, img } = parse_first_td(cells[0]);
						const aps = Number(cells[indexes.attacks_per_second].textContent);
						const damages = parse_damage_td(cells[indexes.damage]);

						return {
							base: title,
							img: img,
							aps,
							damages,
						};
					});
				})
		);
	});

	return result;
}
