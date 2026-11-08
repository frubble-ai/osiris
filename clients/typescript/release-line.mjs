import { readFileSync, readdirSync } from "node:fs";
import { pathToFileURL } from "node:url";

// The client follows osiris's releases. It sends the release it was generated for, and its major
// version is that release's number (`info.x-osiris-release-number`: anubis 1, bastet 2, never
// reused, kept when older releases are removed), so a client for a release that broke something is
// never an upgrade within the same major. While the first release is still a preview there is no
// stable line yet, so the major is 0. A preview's client is only published on
// the branch channels as a prerelease, never on latest from a tag.

const versions = new URL("../../api-reference/versions/", import.meta.url);
const current = new URL("../../api-reference/openapi.json", import.meta.url);

// Every release osiris serves, oldest first, with its stability, from api-reference/versions/.
export function readReleases(directory = versions) {
	return readdirSync(directory)
		.filter((file) => file.endsWith(".json"))
		.sort()
		.map((file) => {
			const spec = JSON.parse(readFileSync(new URL(file, directory), "utf8"));
			return {
				name: file.slice(0, -".json".length),
				number: spec.info["x-osiris-release-number"],
				stability: spec.info["x-osiris-stability"] ?? "preview",
			};
		});
}

// The major version the client must carry for `release`, one of `releases`.
export function expectedMajor(releases, release) {
	const listed = releases.find((candidate) => candidate.name === release);
	if (!listed) throw new Error(`${release} is not a release osiris serves`);
	if (!Number.isInteger(listed.number)) throw new Error(`${release}'s spec names no x-osiris-release-number`);
	if (listed.number === 1 && listed.stability === "preview") return 0;
	return listed.number;
}

// The problems with publishing `version` (package.json's) for `osirisVersion` (OSIRIS_VERSION),
// generated from a spec at `specVersion`, from a ref of `refType`; empty when there are none.
export function releaseLineProblems({ releases, version, osirisVersion, specVersion, refType }) {
	const problems = [];
	if (osirisVersion === undefined) {
		problems.push('src/index.ts declares no OSIRIS_VERSION = "<release>"');
	} else if (osirisVersion !== specVersion) {
		problems.push(`OSIRIS_VERSION is ${osirisVersion}, but the client is generated for ${specVersion}`);
	}
	const release = releases.find((listed) => listed.name === specVersion);
	if (!release) {
		problems.push(`${specVersion} has no spec in api-reference/versions/`);
		return problems;
	}
	const major = Number(version.split(".")[0]);
	const expected = expectedMajor(releases, specVersion);
	if (major !== expected) {
		problems.push(`the package is ${version}, but a client for ${specVersion} is major ${expected}`);
	}
	if (refType === "tag" && release.stability !== "stable") {
		problems.push(`${specVersion} is a preview: its client is only published on the branch channels`);
	}
	return problems;
}

// The problems with the client as it is in the repository, for `refType`.
export function checkRepository(refType = "branch") {
	const pkg = JSON.parse(readFileSync(new URL("./package.json", import.meta.url), "utf8"));
	const index = readFileSync(new URL("./src/index.ts", import.meta.url), "utf8");
	const osirisVersion = /OSIRIS_VERSION = "([^"]+)"/.exec(index)?.[1];
	const specVersion = JSON.parse(readFileSync(current, "utf8")).info.version;
	return releaseLineProblems({ releases: readReleases(), version: pkg.version, osirisVersion, specVersion, refType });
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
	const problems = checkRepository(process.env.REF_TYPE ?? "branch");
	for (const problem of problems) console.error(problem);
	if (problems.length > 0) process.exit(1);
}
