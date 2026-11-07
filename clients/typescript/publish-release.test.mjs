import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";
import {
	alreadyPublished,
	changesClient,
	clientPaths,
	commandError,
	laterClientChange,
	npmSteps,
	publishCurrent,
	releaseTarget,
} from "./publish-release.mjs";

const sha = "27ff530956b3e38d57ad4d4c3e2ffc5634857b06";

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

test("a commit id that is not 12 hex characters is refused", () => {
	for (const bad of ["xyz", "27ff530956b", "27ff530956bZ"]) {
		assert.throws(() => releaseTarget({ refType: "branch", refName: "dev", sha: bad, version: "0.1.0" }), /12 hex/);
	}
});

test("any other ref is refused", () => {
	assert.throws(() => releaseTarget({ refType: "branch", refName: "feature", sha, version: "0.1.0" }), /feature/);
});

test("a run superseded by a later client change does not publish", async () => {
	let published = false;
	const result = await publishCurrent({
		refType: "branch",
		target: { version: "0.1.0-dev.g27ff530956b3", tag: "dev", buildTag: "g27ff530956b3", name: "@frubble-ai/osiris-client" },
		superseded: async () => true,
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
		target: { version: "0.1.0-dev.g27ff530956b3", tag: "dev", buildTag: "g27ff530956b3", name: "@frubble-ai/osiris-client" },
		superseded: async () => false,
		publishVersion() {},
		moveChannel(version, tag) {
			moved.push([version, tag]);
		},
	});
	assert.equal(result.action, "published");
	assert.deepEqual(moved, [["0.1.0-dev.g27ff530956b3", "dev"]]);
});

test("a client change that lands during upload does not move the channel", async () => {
	let reads = 0;
	const result = await publishCurrent({
		refType: "branch",
		target: { version: "0.1.0-dev.g27ff530956b3", tag: "dev", buildTag: "g27ff530956b3", name: "@frubble-ai/osiris-client" },
		superseded: async () => reads++ > 0,
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
		target: { version: "0.1.0-dev.g27ff530956b3", tag: "dev", buildTag: "g27ff530956b3", name: "@frubble-ai/osiris-client" },
		superseded: async () => false,
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

test("a release tag does not consult the branch", async () => {
	const result = await publishCurrent({
		refType: "tag",
		target: { version: "1.2.3", tag: "latest", buildTag: "latest", name: "@frubble-ai/osiris-client" },
		superseded: async () => {
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
			target: { version: "0.1.0-dev.g27ff530956b3", tag: "dev", buildTag: "g27ff530956b3", name: "@frubble-ai/osiris-client" },
			superseded: async () => false,
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

const compare = (body) => laterClientChange({ repository: "frubble-ai/osiris", refName: "dev", sha, token: "t", ...answering(200, body) });

test("the branch is compared from the run's commit to its head", async () => {
	const { calls, fetchImpl } = answering(200, { status: "identical", files: [] });
	assert.equal(await laterClientChange({ repository: "frubble-ai/osiris", refName: "dev", sha, token: "t", fetchImpl }), false);
	assert.equal(calls[0].url, `https://api.github.com/repos/frubble-ai/osiris/compare/${sha}...dev`);
	assert.equal(calls[0].init.headers.Authorization, "Bearer t");
});

test("later commits that leave the client alone keep the channel with this run", async () => {
	assert.equal(await compare({ status: "ahead", files: [{ filename: ".github/workflows/claude-code-review.yml" }] }), false);
});

test("a later commit that changes the client hands the channel to its own run", async () => {
	for (const filename of ["clients/typescript/src/index.ts", "api-reference/openapi.json", ".github/workflows/client.yml"]) {
		assert.equal(await compare({ status: "ahead", files: [{ filename: "README.md" }, { filename }] }), true);
	}
});

test("a commit no longer on the branch counts as superseded", async () => {
	for (const status of ["behind", "diverged"]) {
		assert.equal(await compare({ status, files: [] }), true);
	}
});

test("a comparison too large to list in full counts as superseded", async () => {
	const files = Array.from({ length: 300 }, (_, i) => ({ filename: `docs/${i}.md` }));
	assert.equal(await compare({ status: "ahead", files }), true);
});

test("a failed comparison stops the job, naming the status", async () => {
	const { fetchImpl } = answering(404, { message: "Not Found" });
	await assert.rejects(laterClientChange({ repository: "frubble-ai/osiris", refName: "dev", sha, token: "t", fetchImpl }), /404/);
});

test("a comparison without a file list stops the job", async () => {
	await assert.rejects(compare({ status: "ahead" }), /did not list the changed files/);
});

test("a path matches only inside the client paths", () => {
	assert.equal(changesClient("clients/typescript/package.json"), true);
	assert.equal(changesClient("clients/typescript-old/package.json"), false);
	assert.equal(changesClient(".github/workflows/client.yml.bak"), false);
});

test("the client paths match the workflow's push trigger", () => {
	const workflow = readFileSync(new URL("../../.github/workflows/client.yml", import.meta.url), "utf8");
	const push = workflow.slice(workflow.indexOf("  push:"));
	const paths = JSON.parse(push.match(/^\s+paths: (\[.*\])$/m)[1]);
	assert.deepEqual(paths, clientPaths);
});

const name = "@frubble-ai/osiris-client";

test("a version is set and then uploaded under its build tag", () => {
	const calls = [];
	npmSteps({ name, run: (args) => calls.push(args) }).publishVersion("0.1.0-dev.g27ff530956b3", "g27ff530956b3");
	assert.deepEqual(calls, [
		["version", "--no-git-tag-version", "--allow-same-version", "0.1.0-dev.g27ff530956b3"],
		["publish", "--tag", "g27ff530956b3"],
	]);
});

test("moving a channel points it at the version and then drops the build tag", () => {
	const calls = [];
	npmSteps({ name, run: (args) => calls.push(args) }).moveChannel("0.1.0-dev.g27ff530956b3", "dev", "g27ff530956b3");
	assert.deepEqual(calls, [
		["dist-tag", "add", `${name}@0.1.0-dev.g27ff530956b3`, "dev"],
		["dist-tag", "rm", name, "g27ff530956b3"],
	]);
});

test("a failed build tag removal does not fail the move", () => {
	const calls = [];
	const run = (args) => {
		calls.push(args);
		if (args[1] === "rm") throw new Error("Command failed: npm dist-tag rm");
	};
	npmSteps({ name, run }).moveChannel("0.1.0-dev.g27ff530956b3", "dev", "g27ff530956b3");
	assert.equal(calls.length, 2);
});

test("a failed channel move fails the job", () => {
	const run = (args) => {
		if (args[1] === "add") throw new Error("Command failed: npm dist-tag add");
	};
	assert.throws(() => npmSteps({ name, run }).moveChannel("0.1.0-dev.g27ff530956b3", "dev", "g27ff530956b3"), /dist-tag add/);
});
