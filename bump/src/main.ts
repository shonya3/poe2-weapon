import node_path from 'node:path';
import node_fs_promises from 'node:fs/promises';

const PROJECT_ROOT = node_path.resolve(import.meta.dirname, '../../');
async function main() {
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
	await update_root_cargo_toml(new_version);
}
main().catch(err => {
	console.error(`Error: ${err.message}`);
	process.exit(1);
});

async function update_root_cargo_toml(new_version: string) {
	const cargo_toml_path = node_path.join(PROJECT_ROOT, 'Cargo.toml');
	console.log(`Updating ${cargo_toml_path}`);

	try {
		// Read the Cargo.toml file
		const cargo_toml = await node_fs_promises.readFile(cargo_toml_path, 'utf-8');

		// Replace the version in [workspace.package]
		const updated_cargo_toml = cargo_toml.replace(
			/\[workspace\.package\]\s*version\s*=\s*"([^"]+)"/,
			`[workspace.package]\nversion = "${new_version}"`
		);

		// Write the updated content back to Cargo.toml
		await node_fs_promises.writeFile(cargo_toml_path, updated_cargo_toml, 'utf-8');
		console.log(`Updated version to ${new_version} in Cargo.toml.`);
	} catch (err) {
		throw new Error(`Failed to update Cargo.toml: ${err}`);
	}
}
