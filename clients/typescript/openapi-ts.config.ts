// Generated from osiris's own spec: the types, a fetch client, and a Zod schema for every request
// and answer, which the SDK checks every answer against.
import { defineConfig } from "@hey-api/openapi-ts";

export default defineConfig({
	input: "../../api-reference/openapi.json",
	output: { path: "src/generated", postProcess: [] },
	plugins: [
		"@hey-api/client-fetch",
		"@hey-api/typescript",
		"zod",
		{ name: "@hey-api/sdk", validator: true },
	],
});
