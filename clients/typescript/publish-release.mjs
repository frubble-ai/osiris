import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const channels = ["dev", "stg", "prd"];

// A branch publishes <version>-<channel>.g<12 hex>. The g keeps an all-digit
// commit id from being read as a numeric prerelease identifier. A tag v1.2.3
// publishes that version on latest. Anything after the patch number is refused.
// Point a channel back at a known version with:
// npm dist-tag add <name>@<version> <channel>
export function releaseTarget({ refType, refName, sha, version }) {
	if (refType === "tag") {
		if (!/^v(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/.test(refName)) {
			throw new Error(`release tag must look like v1.2.3, got ${refName}`);
		}
		return { version: refName.slice(1), tag: "latest", buildTag: "latest" };
	}
	if (!channels.includes(refName)) {
		throw new Error(`refused to publish from ${refName}`);
	}
	const id = sha.slice(0, 12);
	if (!/^[0-9a-f]{12}$/.test(id)) {
		throw new Error(`commit id must be 12 hex characters, got ${id}`);
	}
	return { version: `${version}-${refName}.g${id}`, tag: refName, buildTag: `g${id}` };
}

export function alreadyPublished(error) {
	const text = [error.stderr, error.stdout, error.message].map((part) => `${part ?? ""}`).join("\n");
	return text.includes("EPUBLISHCONFLICT") || text.includes("previously published");
}

// stdio inherit drops the child's output, so a publish conflict never reaches alreadyPublished.
export function commandError(error) {
	const stderr = `${error.stderr ?? ""}`;
	const stdout = `${error.stdout ?? ""}`;
	const wrapped = new Error(`${error.message ?? ""}\n${stderr}\n${stdout}`);
	wrapped.stderr = stderr;
	wrapped.stdout = stdout;
	return wrapped;
}

function runNpm(args) {
	try {
		const stdout = execFileSync("npm", args, { encoding: "utf8", stdio: ["inherit", "pipe", "pipe"] });
		if (stdout) process.stdout.write(stdout);
	} catch (error) {
		const wrapped = commandError(error);
		if (wrapped.stderr) process.stderr.write(wrapped.stderr);
		if (wrapped.stdout) process.stdout.write(wrapped.stdout);
		throw wrapped;
	}
}

// Publish the version either way. Move the branch channel only while this
// commit is still the branch head, before the upload and again before the move.
export async function publishCurrent({ refType, sha, target, heads, publishVersion, moveChannel }) {
	if (refType !== "tag" && (await heads()) !== sha) {
		return { action: "skip" };
	}
	try {
		await publishVersion(target.version, target.buildTag);
	} catch (error) {
		if (!alreadyPublished(error)) throw error;
	}
	if (refType === "tag") {
		return { action: "published", version: target.version, tag: target.tag };
	}
	if ((await heads()) !== sha) {
		return {
			action: "left-channel",
			version: target.version,
			tag: target.tag,
			restore: `npm dist-tag add ${target.name}@${target.version} ${target.tag}`,
		};
	}
	await moveChannel(target.version, target.tag, target.buildTag);
	return { action: "published", version: target.version, tag: target.tag };
}

export async function branchHead({ repository, refName, token, fetchImpl = globalThis.fetch }) {
	const response = await fetchImpl(`https://api.github.com/repos/${repository}/commits/${encodeURIComponent(refName)}`, {
		headers: {
			Authorization: `Bearer ${token}`,
			Accept: "application/vnd.github+json",
			"X-GitHub-Api-Version": "2022-11-28",
		},
	});
	if (!response.ok) {
		throw new Error(`reading ${refName} answered ${response.status}`);
	}
	const body = await response.json();
	if (typeof body.sha !== "string") {
		throw new Error(`reading ${refName} did not return a commit`);
	}
	return body.sha;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
	const pkg = JSON.parse(readFileSync(new URL("./package.json", import.meta.url), "utf8"));
	const sha = process.env.SHA;
	const refName = process.env.REF_NAME;
	const refType = process.env.REF_TYPE;
	const target = { ...releaseTarget({ refType, refName, sha, version: pkg.version }), name: pkg.name };
	const heads = () =>
		branchHead({
			repository: process.env.GITHUB_REPOSITORY,
			refName,
			token: process.env.GITHUB_TOKEN,
		});
	const result = await publishCurrent({
		refType,
		sha,
		target,
		heads,
		publishVersion(version, buildTag) {
			runNpm(["version", "--no-git-tag-version", "--allow-same-version", version]);
			runNpm(["publish", "--tag", buildTag]);
		},
		moveChannel(version, tag, buildTag) {
			runNpm(["dist-tag", "add", `${pkg.name}@${version}`, tag]);
			try {
				runNpm(["dist-tag", "rm", pkg.name, buildTag]);
			} catch {
				// The channel already points at this version. The build tag is only a spare pointer.
			}
		},
	});
	if (result.action === "skip") {
		console.log(`skipping publish; ${refName} has moved past ${sha}`);
	} else if (result.action === "left-channel") {
		console.log(`published ${result.version} without moving ${result.tag}; restore with: ${result.restore}`);
	} else {
		console.log(`published ${pkg.name}@${result.version} on ${result.tag}`);
	}
}
