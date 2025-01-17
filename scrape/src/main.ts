import playwright from 'playwright';
import { scrap_page, WEAPON_TYPES } from './scrap_page';

async function main() {
	const browser = await playwright.chromium.launch({ timeout: 30000, headless: true });
	const context = await browser.newContext();
	const page = await context.newPage();

	const promises = WEAPON_TYPES.map(async weapon_type => {
		const page = await context.newPage();
		return await scrap_page(weapon_type, page);
	});
	const results = await Promise.all(promises);

	await browser.close();
	await Bun.write('attack_weapon_bases.json', JSON.stringify(results));
}

main();
