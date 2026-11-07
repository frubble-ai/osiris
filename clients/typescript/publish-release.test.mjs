import assert from "node:assert/strict";
import { test } from "node:test";
import { alreadyPublished, branchHead, commandError, publishCurrent, releaseTarget } from "./publish-release.mjs";

const sha = "27ff530956b3e38d57ad4d4c3e2ffc5634857b06";
const newer = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

test("a branch publishes a prerelease on its dist-tag", () => {
	for (const channel of ["dev", "stg", "prd"]) {
		assert.deepEqual(releaseTarget({ refType: "branch", refName: channel, sha, version: "0.1.0" }), {
			version: `0.1.0-${channel}.g27ff530956b3`,
			tag: channel,
			buildTag: "g27ff530956b3",
		});
	}
});

test("an all-digit commit id is not a numeric prerelease identifier", () => {
	const { version } = releaseTarget({
		refType: "branch",
		refName: "dev",
		sha: `123456789012${"0".repeat(28)}`,
		version: "0.1.0",
	});
	assert.equal(version, "0.1.0-dev.g123456789012");
});

test("a release tag publishes that version on latest", () => {
	assert.deepEqual(releaseTarget({ refType: "tag", refName: "v1.2.3", sha, version: "0.1.0" }), {
		version: "1.2.3",
		tag: "latest",
		buildTag: "latest",
	});
});

test("a tag that is not a plain release is refused", () => {
	for (const refName of ["v1.2.3garbage", "v1.2.3.4", "v1.2.3-rc.1", "v01.2.3"]) {
		assert.throws(() => releaseTarget({ refType: "tag", refName, sha, version: "0.1.0" }), /v1\.2\.3/);
	}
});

test("any other ref is refused", () => {
	assert.throws(() => releaseTarget({ refType: "branch", refName: "feature", sha, version: "0.1.0" }), /feature/);
});

test("a run whose branch has moved does not publish", async () => {
	let published = false;
	const result = await publishCurrent({
		refType: "branch",
		sha,
		target: { version: "0.1.0-dev.g27ff530956b3", tag: "dev", buildTag: "g27ff530956b3", name: "@frubble-ai/osiris-client" },
		heads: async () => newer,
		publishVersion() {
			published = true;
		},
		moveChannel() {
			throw new Error("channel moved");
		},
	});
	assert.equal(result.action, "skip");
	assert.equal(published, false);
});

test("a branch that is still current is published and then tagged", async () => {
	const moved = [];
	const result = await publishCurrent({
		refType: "branch",
		sha,
		target: { version: "0.1.0-dev.g27ff530956b3", tag: "dev", buildTag: "g27ff530956b3", name: "@frubble-ai/osiris-client" },
		heads: async () => sha,
		publishVersion() {},
		moveChannel(version, tag) {
			moved.push([version, tag]);
		},
	});
	assert.equal(result.action, "published");
	assert.deepEqual(moved, [["0.1.0-dev.g27ff530956b3", "dev"]]);
});

test("a branch that moves during upload does not move the channel", async () => {
	let reads = 0;
	const result = await publishCurrent({
		refType: "branch",
		sha,
		target: { version: "0.1.0-dev.g27ff530956b3", tag: "dev", buildTag: "g27ff530956b3", name: "@frubble-ai/osiris-client" },
		heads: async () => (reads++ === 0 ? sha : newer),
		publishVersion() {},
		moveChannel() {
			throw new Error("channel moved");
		},
	});
	assert.equal(result.action, "left-channel");
	assert.equal(result.restore, "npm dist-tag add @frubble-ai/osiris-client@0.1.0-dev.g27ff530956b3 dev");
});

test("a retry of a version already published still moves the channel", async () => {
	const moved = [];
	const result = await publishCurrent({
		refType: "branch",
		sha,
		target: { version: "0.1.0-dev.g27ff530956b3", tag: "dev", buildTag: "g27ff530956b3", name: "@frubble-ai/osiris-client" },
		heads: async () => sha,
		publishVersion() {
			const error = new Error("cannot publish over the previously published versions");
			error.stderr = "EPUBLISHCONFLICT";
			throw error;
		},
		moveChannel(version, tag) {
			moved.push([version, tag]);
		},
	});
	assert.equal(result.action, "published");
	assert.deepEqual(moved, [["0.1.0-dev.g27ff530956b3", "dev"]]);
	assert.equal(alreadyPublished({ stderr: "EPUBLISHCONFLICT" }), true);
});

test("a publish conflict is visible only once stderr is kept", () => {
	const failed = { message: "Command failed: npm publish --tag latest" };
	assert.equal(alreadyPublished(failed), false);
	assert.equal(
		alreadyPublished(commandError({ ...failed, stderr: "EPUBLISHCONFLICT previously published versions" })),
		true,
	);
});

test("a release tag does not consult the branch head", async () => {
	const result = await publishCurrent({
		refType: "tag",
		sha,
		target: { version: "1.2.3", tag: "latest", buildTag: "latest", name: "@frubble-ai/osiris-client" },
		heads: async () => {
			throw new Error("branch was read");
		},
		publishVersion() {},
		moveChannel() {
			throw new Error("channel moved");
		},
	});
	assert.deepEqual(result, { action: "published", version: "1.2.3", tag: "latest" });
});

test("a publish failure that is not a conflict stops the job before the channel moves", async () => {
	let moved = false;
	await assert.rejects(
		publishCurrent({
			refType: "branch",
			sha,
			target: { version: "0.1.0-dev.g27ff530956b3", tag: "dev", buildTag: "g27ff530956b3", name: "@frubble-ai/osiris-client" },
			heads: async () => sha,
			publishVersion() {
				const error = new Error("Command failed: npm publish --tag g27ff530956b3");
				error.stderr = "npm error code E401\nnpm error 401 Unauthorized";
				throw error;
			},
			moveChannel() {
				moved = true;
			},
		}),
		/Command failed/,
	);
	assert.equal(moved, false);
});

test("a GitHub Packages publish conflict counts as already published", () => {
	const stderr = [
		"npm error code E409",
		"npm error 409 Conflict - PUT https://npm.pkg.github.com/@frubble-ai%2fosiris-client - Cannot publish over existing version",
	].join("\n");
	assert.equal(alreadyPublished(commandError({ message: "Command failed: npm publish --tag g27ff530956b3", stderr })), true);
});

function answering(status, body) {
	const calls = [];
	const fetchImpl = async (url, init) => {
		calls.push({ url, init });
		return { ok: status >= 200 && status < 300, status, json: async () => body };
	};
	return { calls, fetchImpl };
}

test("the branch head is the sha the GitHub API returns", async () => {
	const { calls, fetchImpl } = answering(200, { sha });
	const head = await branchHead({ repository: "frubble-ai/osiris", refName: "dev", token: "t", fetchImpl });
	assert.equal(head, sha);
	assert.equal(calls[0].url, "https://api.github.com/repos/frubble-ai/osiris/commits/dev");
	assert.equal(calls[0].init.headers.Authorization, "Bearer t");
});

test("a failed branch read stops the job, naming the status", async () => {
	const { fetchImpl } = answering(404, { message: "Not Found" });
	await assert.rejects(branchHead({ repository: "frubble-ai/osiris", refName: "dev", token: "t", fetchImpl }), /404/);
});

test("a branch read without a commit sha stops the job", async () => {
	const { fetchImpl } = answering(200, { sha: 42 });
	await assert.rejects(
		branchHead({ repository: "frubble-ai/osiris", refName: "dev", token: "t", fetchImpl }),
		/did not return a commit/,
	);
});
