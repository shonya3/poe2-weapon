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
	await update_app_package_json(new_version);
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

async function update_app_package_json(new_version: string) {
	const package_json_path = node_path.join(PROJECT_ROOT, 'app', 'package.json');
	console.log(`Updating ${package_json_path}`);

	try {
		const package_json = await node_fs_promises.readFile(package_json_path, 'utf-8');
		const package_data = JSON.parse(package_json);

		package_data.version = new_version;

		await node_fs_promises.writeFile(package_json_path, JSON.stringify(package_data, null, '\t') + '\n', 'utf-8');

		console.log(`Updated version to ${new_version} in package.json.`);
	} catch (err) {
		throw new Error(`Failed to update package.json: ${err}`);
	}
}
