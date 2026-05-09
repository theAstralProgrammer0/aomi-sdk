---
name: openapi-from-docs
description: Draft an OpenAPI 3.1 spec for a third-party platform (e.g. Binance, Dune, DefiLlama) by reading its public documentation and writing the result to ext/specs/<platform>.yaml. Use when the user wants to add or update an Aomi app for a platform that does NOT have an official OpenAPI spec discoverable via aomi-build gen-specs. Triggers include "draft openapi spec for X", "openapi-from-docs", "/openapi-from-docs", or "no spec found, run the skill".
---

# openapi-from-docs

You are drafting an OpenAPI 3.1 spec for a third-party platform from its docs. The output feeds the `aomi-build gen-client` step downstream — quality matters because Rust client codegen will rely on it.

## Scope

This skill is the FALLBACK for when `aomi-build gen-specs <platform>` fails (no APIs.guru / GitHub / Postman match). It is for HTTP REST APIs. **Do not use it for**:

- GraphQL APIs (different paradigm — say so and stop)
- WebSocket-only APIs
- Pure on-chain integrations (no HTTP surface)
- Platforms whose entire API is one POST endpoint with a discriminated `type` field — these can technically be modelled with `oneOf` discriminators but the codegen output is ugly. Surface this to the user and ask if they want to proceed anyway or keep the client hand-written.

## Inputs you need

Required:
- `platform` — short name used as the file stem (e.g. `dune`, `defillama`).
- At least one source: a docs URL, a Postman collection JSON, or a TypeScript/Python SDK source path.

If the user only gave a platform name, ask for at least a docs URL before doing anything else. Do not guess URLs — confirm with the user.

Preferred sources, in order of quality:
1. **Official TS/Python SDK source** — types in the SDK encode the API surface more precisely than markdown ever will. If the user can point you at one, use it as primary.
2. **Postman collection** — already has structured request/response examples.
3. **Reference docs page** with curl examples.
4. **Free-form docs** — last resort, lowest quality, hardest to get right.

## Workflow

### 1. Plan the surface area

- Read the docs landing page to get the list of endpoints (or the SDK's exported methods).
- Group by resource (e.g. `/v1/markets/*`, `/v1/orders/*`).
- Ask the user which groups to include. Do NOT spec endpoints you weren't asked for — drafting 100 endpoints when the user only needs 5 is waste.

### 2. Draft iteratively, not all at once

For each endpoint group:
- Fetch the relevant doc page (WebFetch).
- Extract: HTTP method, path, path/query/body params (with types), response schema, auth requirement.
- Append to the spec. Do not try to draft the whole spec in one pass — iterate.

### 3. Auth and signing

OpenAPI's `securitySchemes` only describes *what header to send*, not *how to compute it*. Use the standard OpenAPI security types where they fit:
- Bearer token: `securitySchemes.bearerAuth = { type: http, scheme: bearer }`
- API key in header: `securitySchemes.apiKey = { type: apiKey, in: header, name: X-API-KEY }`

If the platform requires HMAC signing or any other computed credential (Binance, OKX, Polymarket-style), describe what header(s) to send in `securitySchemes` and add a top-level comment under `info.description` like:

```
## Auth
This API uses HMAC-SHA256 over the query string with appended `&timestamp=...&signature=...`.
The signing logic is hand-written in `ext/src/<platform>/auth.rs` — the spec only describes
the resulting header (`X-MBX-APIKEY`) for codegen purposes.
```

Do not invent OpenAPI extensions unless absolutely necessary.

### 4. Validate before writing

Before writing the file, sanity-check yourself:
- Does it parse as YAML? (Mentally: are colons + indentation correct?)
- Does it have `openapi: 3.0.3`, `info: { title, version }`, and `paths`?
  **Use 3.0.3, not 3.1.x.** The downstream codegen (progenitor) only supports OpenAPI 3.0.x. If you write 3.1, `aomi-build gen-client` will downgrade it textually with a warning, but you should produce 3.0.3 directly.
- Does every operation have at least a 200 response with a schema?
- Are response schemas defined inline or referenced from `components/schemas`? Prefer `components/schemas` for any type used by more than one operation.
- Does every signed endpoint declare its `security` requirement?

### 5. Write the files

Write two files:

**`ext/specs/<platform>.yaml`** — the spec itself.

**`ext/specs/<platform>.meta.json`** — provenance sidecar with this exact shape:

```json
{
  "platform": "<platform>",
  "source": "skill",
  "source_url": "<primary docs URL or 'multiple'>",
  "fetched_at": "<ISO-8601 UTC timestamp>",
  "upstream_version": null,
  "completeness": {
    "endpoints_specced": <number>,
    "endpoints_skipped": <number>,
    "auth_described": <true|false>,
    "notes": "<one-liner about what's missing or rough>"
  }
}
```

The `completeness` block is skill-only — it tells future-you (and the human) what's incomplete so they don't trust the spec blindly.

Use `mkdir -p ext/specs` if needed before writing.

### 6. Report back

Print a short summary: how many endpoints, what was skipped, what's rough. End with the suggested next command:

```
Next: aomi-build gen-client <platform>
```

## Anti-patterns

- **Don't fabricate.** If the docs are unclear about a response field's type, mark it `type: object` with `additionalProperties: true` and note it in `completeness.notes`. Never guess.
- **Don't over-spec.** This is a starting point for codegen, not a contract. 80% accurate covering the endpoints the user wants is better than 95% accurate covering endpoints they don't.
- **Don't write the spec to `apps/`.** Specs always live in `ext/specs/`.
- **Don't run `cargo build` or codegen here.** This skill ends at the spec file. The user will run `aomi-build gen-client` next.

## Example invocations

```
/openapi-from-docs dune https://docs.dune.com/api-reference/overview
/openapi-from-docs defillama https://defillama.com/docs/api
/openapi-from-docs binance  # → you ask for docs URL
```
