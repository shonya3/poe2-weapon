import playwright from 'playwright';
import node_fs_promises from 'node:fs/promises';
import node_path from 'node:path';

import { scrap_page, WEAPON_TYPES } from './scrap_page.ts';
import type { WeaponStats } from './weapon.ts';

async function main() {
	const browser = await playwright.chromium.launch({ timeout: 30000, headless: true });
	const context = await browser.newContext();
	const promises = WEAPON_TYPES.map(async weapon_type => {
		const page = await context.newPage();
		return await scrap_page(weapon_type, page);
	});
	const results = (await Promise.all(promises)).flat();
	await browser.close();

	await write_bases_rs_into_parser_crate(results);
	await node_fs_promises.writeFile(
		node_path.resolve(import.meta.dirname, '../../crates/weapon/data/bases.json'),
		JSON.stringify(results)
	);
}

async function write_bases_rs_into_parser_crate(weapons: Array<WeaponStats>): Promise<void> {
	let rs_str = `pub const BASES: [&str; ${weapons.length}] = [\n`;
	weapons.forEach(({ base }) => (rs_str += `\t"${base}",\n`));
	rs_str += '];\n';
	await node_fs_promises.writeFile(
		node_path.resolve(import.meta.dirname, '../../crates/parser/src/bases.rs'),
		rs_str
	);
}

main();
