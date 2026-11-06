import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const channels = ["dev", "stg", "prd"];

// A branch publishes <version>-<channel>.g<12 hex> on that dist-tag.
// A v* tag publishes that version on latest. The g keeps an all-digit commit id
// from being read as a numeric prerelease identifier.
export function releaseTarget({ refType, refName, sha, version }) {
	if (refType === "tag") {
		if (!/^v\d+\.\d+\.\d+/.test(refName)) {
			throw new Error(`release tag must look like v1.2.3, got ${refName}`);
		}
		return { version: refName.slice(1), tag: "latest" };
	}
	if (!channels.includes(refName)) {
		throw new Error(`refused to publish from ${refName}`);
	}
	const id = sha.slice(0, 12);
	if (!/^[0-9a-f]{12}$/.test(id)) {
		throw new Error(`commit id must be 12 hex characters, got ${id}`);
	}
	return { version: `${version}-${refName}.g${id}`, tag: refName };
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
	const { version: base } = JSON.parse(
		readFileSync(new URL("./package.json", import.meta.url), "utf8"),
	);
	const { version, tag } = releaseTarget({
		refType: process.env.REF_TYPE,
		refName: process.env.REF_NAME,
		sha: process.env.SHA,
		version: base,
	});
	execFileSync("npm", ["version", "--no-git-tag-version", version], { stdio: "inherit" });
	execFileSync("npm", ["publish", "--tag", tag], { stdio: "inherit" });
}
