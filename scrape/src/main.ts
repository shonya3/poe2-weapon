import playwright from 'playwright';
import node_fs_promises from 'node:fs/promises';
import node_path from 'node:path';

import { scrap_page, WEAPON_TYPES } from './scrap_page.ts';

async function main() {
	const browser = await playwright.chromium.launch({ timeout: 30000, headless: true });
	const context = await browser.newContext();
	const promises = WEAPON_TYPES.map(async weapon_type => {
		const page = await context.newPage();
		return await scrap_page(weapon_type, page);
	});
	const results = (await Promise.all(promises)).flat();

	let rs_str = `pub const BASES: [&str; ${results.length}] = [\n`;
	results.forEach(({ base }) => (rs_str += `\t"${base}",\n`));
	rs_str += '];\n';

	await browser.close();
	await node_fs_promises.writeFile(
		node_path.resolve(import.meta.dirname, '../../crates/parser/src/bases.rs'),
		rs_str
	);
}

main();
