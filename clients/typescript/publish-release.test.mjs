import assert from "node:assert/strict";
import { test } from "node:test";
import { alreadyPublished, publishCurrent, releaseTarget } from "./publish-release.mjs";

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
		sha: "0123456789abcdef",
		version: "0.1.0",
	});
	assert.equal(version, "0.1.0-dev.g0123456789ab");
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
