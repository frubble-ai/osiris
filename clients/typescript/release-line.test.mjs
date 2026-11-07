import assert from "node:assert/strict";
import { test } from "node:test";
import { checkRepository, expectedMajor, releaseLineProblems } from "./release-line.mjs";

const anubis = "2026-11-04.anubis";
const bastet = "2027-03-01.bastet";

test("the first release's client is major 0 while it is a preview, and 1 once stable", () => {
	assert.equal(expectedMajor([{ name: anubis, stability: "preview" }], anubis), 0);
	assert.equal(expectedMajor([{ name: anubis, stability: "stable" }], anubis), 1);
});

test("a later release's client is its place in the list, preview or not", () => {
	const releases = [
		{ name: anubis, stability: "stable" },
		{ name: bastet, stability: "preview" },
	];
	assert.equal(expectedMajor(releases, bastet), 2);
});

test("a client must send the release it was generated for", () => {
	const problems = releaseLineProblems({
		releases: [{ name: anubis, stability: "preview" }],
		version: "0.1.0",
		osirisVersion: bastet,
		specVersion: anubis,
		refType: "branch",
	});
	assert.equal(problems.length, 1);
	assert.match(problems[0], /OSIRIS_VERSION/);
});

test("a client whose major doesn't match its release is refused", () => {
	const problems = releaseLineProblems({
		releases: [
			{ name: anubis, stability: "stable" },
			{ name: bastet, stability: "preview" },
		],
		version: "1.4.0",
		osirisVersion: bastet,
		specVersion: bastet,
		refType: "branch",
	});
	assert.deepEqual(problems, [`the package is 1.4.0, but a client for ${bastet} is major 2`]);
});

test("a preview's client is never published from a tag", () => {
	const problems = releaseLineProblems({
		releases: [{ name: anubis, stability: "preview" }],
		version: "0.1.0",
		osirisVersion: anubis,
		specVersion: anubis,
		refType: "tag",
	});
	assert.deepEqual(problems, [`${anubis} is a preview: its client is only published on the branch channels`]);
});

test("the client in the repository follows its release", () => {
	assert.deepEqual(checkRepository("branch"), []);
});
