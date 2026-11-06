// Osiris's client. `createOsirisClient` sets the origin, the key and the API version every
// request carries; every operation of the spec is a typed function whose answer is checked
// against its Zod schema before it is returned.

import { createClient, createConfig } from "./generated/client/index.js";

export * from "./generated/index.js";
export * as schemas from "./generated/zod.gen.js";

/** The API version this package was generated for: sent on every request. */
export const OSIRIS_VERSION = "2026-11-04.anubis";

export function createOsirisClient(options: { baseUrl: string; apiKey: string; fetch?: typeof fetch }) {
	return createClient(
		createConfig({
			baseUrl: options.baseUrl,
			auth: () => options.apiKey,
			headers: { "Osiris-Version": OSIRIS_VERSION },
			...(options.fetch ? { fetch: options.fetch } : {}),
		}),
	);
}
