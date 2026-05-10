---
name: aomi-app-ux-tool-maker
description: Rewrite an Aomi app's tool layer (apps/<platform>/src/tool.rs) so it's actually usable — turning the mechanical stubs from `aomi-build gen-tool` (one tool per endpoint, names like `dune_getv1_query_queryid_results`) into a curated set of user-centric tools backed by the generated client in ext/. Optionally takes a user intent ("collect token prices from defillama"); otherwise infers typical user stories from the platform category. Composes endpoints into composite tools where useful, prunes endpoints that aren't valuable as tools, names tools after user goals, writes domain-specific PREAMBLE. Triggers include "design tools for X", "make this app usable", "rewrite tool layer", "/aomi-app-ux-tool-maker", "after gen-app design the tools", "curate tools".
---

# aomi-app-ux-tool-maker

You are taking an Aomi app from "compiles" to "actually useful." The previous step (`aomi-build gen-tool`) emitted one mechanical stub per endpoint with operationId-derived names. Your job is to **rewrite the tool layer** so:

- Tool names match user goals, not endpoint paths.
- Tool descriptions tell the LLM *when to use* them, not just what they return.
- Composite tools chain multiple endpoints when one user goal needs several.
- Useless endpoints (admin, internal, niche) are dropped.
- The app's `PREAMBLE` is domain-specific — it tells the assistant what kinds of questions it can answer.

## Inputs

Required:
- `platform` — e.g. `dune`, `defillama`, `lifi`. The app at `apps/<platform>/` must already exist (run `aomi-build new-app` or at least `aomi-build gen-tool` first).

Optional:
- `intent` — one sentence about what the user wants this app to do, e.g.
  - "I want to query token price action and TVL trends from DefiLlama"
  - "I want to swap tokens cross-chain via LiFi"
  - "I want to monitor my Hyperliquid positions and place stop-losses"

If no intent is given, ask the user for one. **Do not skip this** — without an intent, you'll guess at user stories that may not match what the user actually wants.

## Pre-reading (always)

Before designing anything, read these to ground yourself:

1. `ext/specs/<platform>.yaml` — the API surface, with operation summaries/descriptions.
2. `ext/src/<platform>/client.rs` — the generated method names (you need these to call from tool.rs).
3. `apps/<platform>/src/tool.rs` (the current generated stub) — what's there now.
4. `apps/<platform>/src/lib.rs` — current PREAMBLE and tool list.
5. **One existing well-curated app for reference**: read `apps/binance/src/tool.rs` or `apps/hyperliquid/src/tool.rs` — both are hand-crafted with good naming, descriptions, and composition. Match this quality bar.

## Workflow

### 1. Categorise the platform

Pick the closest archetype. This shapes which user stories matter:

| Archetype | Examples | Typical user stories |
|---|---|---|
| Price oracle / data | DefiLlama, Dune, CoinGecko | "what's the current price/TVL of X", "show historical Y", "compare A vs B over time" |
| Swap / bridge | LiFi, CoW, 1inch, 0x, Across | "swap A for B", "find best route", "estimate fees", "build a tx I can sign" |
| Lending / yield | Morpho, Yearn, Aave | "show APY for asset", "view my positions", "deposit", "withdraw" |
| Perps / spot exchange | Binance, Bybit, OKX, dYdX, GMX, Hyperliquid | "show price/depth", "place order", "view balance", "view positions", "set stop-loss" |
| Prediction market | Kalshi, Manifold, Polymarket | "list active markets", "show market detail", "show my positions", "place bet" |
| Social | X, Neynar | "post", "search posts/users", "view profile/feed" |

Write down the archetype and 3–6 user stories. Show them to the user for sign-off if you weren't given an explicit intent.

### 2. Map user stories → tools

For each user story, decide:
- **Tool name** — verb-led, snake_case, prefixed with platform: `defillama_get_token_price`, `lifi_quote_swap`, `binance_place_limit_order`. Avoid endpoint-derived names like `dune_getv1_query_queryid_results`.
- **Args struct** — only the fields the user actually thinks about. If an endpoint has 12 params and 9 are knobs, keep the 3 that matter and hardcode reasonable defaults for the rest. Required vs optional reflects user mental model, not the spec.
- **Description** — start with "Use when…" or "Returns…". This is what the LLM sees when deciding whether to call your tool. Make it concrete.
- **Composition** — if the user goal needs multiple endpoints (e.g. quote → approve → tx for a swap), compose them in one tool. Don't make the LLM orchestrate.

Where the spec has overlapping endpoints (e.g. DefiLlama has `/protocols` and `/protocol/{id}`), pick the one that matches the user's intent. Don't expose both unless they serve genuinely different stories.

### 3. Cross-check API coverage

After drafting the tool list, walk back through the spec's operations and ask:
- Is there an op the user would clearly want that I missed?
- Is there a tool I drafted that doesn't actually serve a real user need?

Adjust. Print a one-line diff per change.

### 4. PREAMBLE

Rewrite `PREAMBLE` in `apps/<platform>/src/lib.rs`. Aim for ~25–40 lines. Sections:

- **Role** — "You are an AI assistant specialized in <platform> for <archetype use case>."
- **Capabilities** — bulleted list of what the user can do, mapping to your tools.
- **Important constraints** — auth requirements, rate limits, units, conventions (e.g. "all addresses are checksummed Ethereum addresses").
- **Workflow guidance** — when to chain tools (e.g. "for a swap: first call `lifi_quote_swap` to find the best route, then `lifi_build_tx` to get the transaction the user signs").
- **Formatting** — how to present results (e.g. "format prices in USD with 2 decimals; format APYs as percentages").

Look at `apps/binance/src/lib.rs` and `apps/hyperliquid/src/lib.rs` for length/tone reference.

### 5. Implement

Rewrite `apps/<platform>/src/tool.rs` from scratch with the curated tools. Patterns and discipline:

#### Get the workflow right (research if needed)

Before writing a composite tool body, you must know the **actual** sequence the platform expects. Spec summaries don't always say "after `execute`, poll `/status` until `STATE_COMPLETED`, then call `/results`." Read the platform's own docs / quickstart / SDK README to confirm.

- Use `WebFetch` or `WebSearch` against the platform's official docs (`docs.<platform>.com`, official GitHub README, official tutorials). Skip third-party blog posts unless nothing else exists.
- For each composite tool, explicitly verify: endpoint order, required intermediate state values (e.g. exact string of "completed" status), what counts as a terminal failure, recommended polling cadence.
- If docs disagree with the spec, the **docs win for behavior**, the **spec wins for types**. Note the discrepancy in a code comment.
- Don't guess polling intervals or timeout values. The platform usually documents them ("queries typically complete in 5–30 seconds; poll every 2 seconds"). Match that.

A composite that calls the right endpoints in the wrong order is worse than a stub that does nothing — the LLM will confidently call it and the user will see opaque errors.

#### Trim the arg surface

The spec's parameter list is an API contract; the user's mental model is much smaller. Aim for **5–6 args per tool, max**, even when the API takes 10+. Three rules of thumb:

1. **Compute, don't ask.** If a value can be derived in Rust — current timestamp, sensible page size, polling cadence, retry budget, default chain ID — compute it. Don't expose it. Every arg the LLM has to fill is a chance to fill it wrong.
2. **Keep what users actually think about.** "Token symbol" yes. "`include_metadata: bool`" no. "`order_by: Option<SortDirection>`" no. Pick a default the user would pick.
3. **Collapse related args.** If the API has `start_time` and `end_time`, consider exposing just `lookback_hours` and computing both. If it has `chain_id` and `chain_name`, expose `chain` (string) and resolve internally.

If you find yourself writing more than 6 fields on an Args struct, stop and ask: which 4 of these would the user actually care about? Hardcode or compute the rest.

#### Code patterns

- Keep the `<Platform>App` marker struct (it's the type the macro references).
- Keep helpers: `ok()` (wraps result with `source`), `rt()` (tokio runtime), `resolve_key()` (env-var fallback). Copy from current generated tool.rs or from `apps/binance/src/tool.rs`.
- Auth: use `aomi_sdk::resolve_secret_value` with the env var `<PLATFORM>_API_KEY`. One auth resolution per tool, even if the API has multiple credential slots.
- Calls into the generated client are async — wrap with `runtime.block_on(async move { client.method(args).await }).map_err(...)?`.
- Composite tools sequence multiple `.await` calls and combine the results in the JSON return.

#### Safety rails (no infinite loops, no bloat)

These are non-negotiable for any composite or polling tool:

- **Every loop has a deadline.** Polling loops use `Instant::now() + Duration::from_secs(N)` and check on each iteration. Default max-wait should be small (30–120s); expose as an Args field only if the user genuinely needs to extend it.
- **Every loop has a max iteration count too.** Even with a deadline, cap iterations as a belt-and-suspenders guard against `tokio::time::sleep` mocking weirdness or system clock skew.
- **Composite tools have a step budget.** A tool that orchestrates "execute → poll → fetch" is fine. A tool that calls 8 endpoints and dispatches based on intermediate results probably isn't a tool — it's an agent. Don't write that.
- **No new dependencies without strong justification.** The standard kit (aomi-sdk, aomi-ext, schemars, serde, serde_json, tokio) covers 99% of cases. If you reach for `regex`, `chrono`, `url`, etc., ask whether you can do it with `std`. If you must add a dep, mention it explicitly in the report.
- **Don't construct Tokio runtimes inside loops.** One per tool call, at the top.
- **Sleep with `tokio::time::sleep`, not `std::thread::sleep`** — never block the runtime thread.
- **Sanity-check error paths.** If an endpoint returns non-success, the tool returns `Err(format!("[platform] op X: {e}"))`. Don't retry indefinitely on errors. Don't swallow errors and return empty results.

Update `apps/<platform>/src/lib.rs` with the new PREAMBLE and the new tool list in `dyn_aomi_app!`.

Don't touch `Cargo.toml` unless you're adding a dep (rare — usually the gen-tool Cargo.toml is fine).

### 5.5. Reconcile PREAMBLE with the implementation

After step 5, go back to the PREAMBLE you drafted in step 4 and check:

- **Does every tool you actually shipped have a line in Capabilities?** Add missing ones; remove lines for tools you ended up dropping.
- **Does the Workflow guidance match what your composite tools actually do?** If `dune_run_query` waits for completion internally, the PREAMBLE shouldn't tell the LLM to call status separately.
- **Are the conventions in PREAMBLE consistent with the args you exposed?** If you renamed `query_id` to just `query`, fix the PREAMBLE example.
- **Did pruning change the platform's "shape"?** A platform you described as "full SQL platform" might now be "read-mostly SQL runner" if you dropped the CRUD endpoints. Update the Role line.

The PREAMBLE is the LLM's only orientation to your tools. Drift between PREAMBLE and tool reality is the most common cause of LLMs picking the wrong tool.

### 6. Verify

```
cd apps/<platform> && cargo build
```

If it fails:
- Most likely cause: arg type mismatch with the generated client's positional API. Look at the client.rs signature for the method you called and adjust your `client.method(...)` call.
- Second most likely: missing `&` / `.as_str()` / `.as_deref()` for string args, since progenitor's API takes `&'a str` not `String`.
- Fix and re-run. Iterate until clean.

### 7. Report

Print a tight summary:
- Number of tools before/after.
- Names of new composite tools (these are the highest-value additions).
- Names of dropped endpoints (and one-line reasons).
- One sentence on what the user can now do that they couldn't before.

Suggest the user run `cargo test -p <platform>` if there are tests, or load the dylib and try a tool from the Aomi CLI.

## Anti-patterns

- ❌ **Preserving all generated stubs.** Most APIs have 30–60+ ops. A useful Aomi app is usually 5–12 tools. Be aggressive about pruning.
- ❌ **Endpoint-derived names.** `dune_getv1_query_queryid_results_csv` tells the LLM nothing. `dune_get_query_results` does.
- ❌ **Generic descriptions.** "Get protocol detail" — useless. "Returns TVL, chains, and historical TVL series for a protocol slug. Use when the user asks about a specific protocol's size or trend." — useful.
- ❌ **Letting the LLM orchestrate composites.** If a user goal needs three endpoints, the tool wraps all three. Don't make the model figure out the sequence.
- ❌ **Exposing admin/internal endpoints.** If the spec includes `/admin/users`, `/internal/health`, etc., skip them.
- ❌ **Args structs that mirror the spec exactly.** The spec's params are an API surface; the user's mental model is different. Translate.
- ❌ **Skipping the user-intent question.** Without intent, tools will be generic and probably miss the point.
- ❌ **Touching `ext/src/<platform>/`.** Generated client is regenerated by `aomi-build gen-client`; never hand-edit it.
- ❌ **Asking the user (LLM) for values you can compute.** Timestamps, polling intervals, page sizes, default chains, retry budgets — derive these in Rust.
- ❌ **Unbounded loops.** Every poll loop has a deadline AND an iteration cap. No `loop { … sleep(2).await }` without an exit condition that fires on a wall-clock deadline.
- ❌ **PREAMBLE drift.** Don't write tool descriptions and forget to update PREAMBLE. After each tool change, re-read PREAMBLE and confirm it still describes what you shipped.

## Examples (illustrative, don't blindly copy)

### DefiLlama, intent = "track token prices and TVL trends"

**Before** (gen-tool stubs): 21 mechanical tools — `defillama_getProtocols`, `defillama_getProtocolDetail`, `defillama_getCurrentPrices`, `defillama_getHistoricalPrices`, `defillama_getPriceChangePercentage`, `defillama_getYieldPools`, …

**After** (curated): 6 tools

| Tool | Wraps | Description |
|---|---|---|
| `defillama_get_token_price` | `getCurrentPrices` | Use when the user asks the current price of a token. Accepts `chain:address` or `coingecko:id`. |
| `defillama_get_price_history` | `getHistoricalPrices` + `getPriceChangePercentage` | Returns price at a past timestamp and % change since. Use when the user asks "how much has X moved". |
| `defillama_list_protocols` | `getProtocols` | Lists DeFi protocols by current TVL, optionally filtered by category. |
| `defillama_get_protocol_tvl` | `getProtocolDetail` | TVL + historical series for one protocol. |
| `defillama_top_yield_pools` | `getYieldPools` | Top yield opportunities, filterable by chain/asset/min APY. |
| `defillama_get_chain_tvl` | `getChainsTvl` + `getHistoricalChainTvl` | Current and historical TVL for a chain. |

Dropped: `getStablecoinHistory`, `getStablecoinChains`, `getStablecoins`, `getDexOverview`, `getFeesOverview`, etc. — out of scope for the intent.

### LiFi, intent = "swap tokens cross-chain"

Likely curated tools: `lifi_get_quote`, `lifi_build_swap_tx` (composite: quote + approve + swap), `lifi_check_status`, `lifi_list_chains`, `lifi_list_tokens_on_chain`. Drop everything else.

## When to refuse

If the platform is not categorisable (no clear user stories), or if the user's intent doesn't fit the platform's actual API surface, surface this and ask the user to either:
- Reframe the intent
- Pick a different platform
- Accept that the existing generated stubs will stay until intent is clearer

Don't fabricate a "use case" to justify pretty tool names on a misfit API.
