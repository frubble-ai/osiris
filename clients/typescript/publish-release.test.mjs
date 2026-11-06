import assert from "node:assert/strict";
import { test } from "node:test";
import { releaseTarget } from "./publish-release.mjs";

const sha = "27ff530956b3e38d57ad4d4c3e2ffc5634857b06";

test("a branch publishes a prerelease on its dist-tag", () => {
	for (const channel of ["dev", "stg", "prd"]) {
		assert.deepEqual(releaseTarget({ refType: "branch", refName: channel, sha, version: "0.1.0" }), {
			version: `0.1.0-${channel}.g27ff530956b3`,
			tag: channel,
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
	assert.deepEqual(
		releaseTarget({ refType: "tag", refName: "v1.2.3", sha, version: "0.1.0" }),
		{ version: "1.2.3", tag: "latest" },
	);
});

test("any other ref is refused", () => {
	assert.throws(() => releaseTarget({ refType: "branch", refName: "feature", sha, version: "0.1.0" }), /feature/);
});
