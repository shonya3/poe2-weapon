function main() {
	const new_version = process.argv[2];

	if (!new_version) {
		throw new Error('Provide a new version');
	}

	console.log(`New version: ${new_version}`);

	/**
	 * Things to update:
	 * - Cargo.toml
	 * - app/package.json
	 * - app/scr-tauri/tauri.conf.json
	 */
}

main();
