---
name: aomi-app-e2e-tester
description: Author and run an end-to-end behavioral test for an Aomi app. Reads the seed `user_story` from `apps/<platform>/test.json` (written earlier by `aomi-app-ux-tool-maker`), expands it into a multi-turn TDD spec (prompts, expected tool calls per turn, final assertions on `UserState` + tool responses), then invokes the generic harness in `aomi/crates/runtime/tests/local-app-e2e.rs` to run it. Reports a per-turn pass/fail table with cost. Triggers include "test this app", "/aomi-app-e2e-tester", "expand the test.json", "run e2e for X", "after curating tools, write the test."
---

# aomi-app-e2e-tester

You are turning a seed `user_story` into a complete, runnable e2e test for an Aomi app, then running it and reporting results.

## Position in the pipeline

```
aomi-build new-app → aomi-app-ux-tool-maker (curates + writes seed user_story)
                                            ↓
                                    YOU: expand test.json, run, report
```

When you start, `apps/<platform>/test.json` already exists with at minimum:
```json
{ "user_story": "..." }
```

Your job is to fill in the rest: `wallet_seed`, `turns`, `final_assertion`, `max_cost_usd`. Then run and report.

## Inputs

Required:
- `platform` — e.g. `lifi`, `dune`. The app at `apps/<platform>/` must compile and have a seed `test.json`.

Optional:
- Override the seed user_story (otherwise read from existing test.json).

If `apps/<platform>/test.json` doesn't exist or has no `user_story`, **stop and tell the user** to run `aomi-app-ux-tool-maker` first.

## Pre-reading (always)

1. `apps/<platform>/test.json` — the seed (currently just `user_story`)
2. `apps/<platform>/src/lib.rs` — PREAMBLE + tool list (your turn-budget should match what the LLM actually sees)
3. `apps/<platform>/src/tool.rs` — tool args, descriptions, what each one returns/mutates
4. `aomi/crates/tools/src/user_state.rs` — the `UserState` shape you're asserting against (`pending_txs`, `pending_eip712s`, `address`, `chain_id`, `ext`)
5. `aomi/crates/runtime/tests/local-app-e2e.rs` — the existing khalani test, as a reference for harness conventions
6. (If present) `apps/khalani/test.json` — once one exists in the new shape, use it as a quality benchmark

## The schema (canonical)

```jsonc
{
  "user_story": "Swap 100 USDC for ETH on Ethereum mainnet via LiFi",

  // Optional. Many flows assume a connected wallet. Harness pre-populates UserState.
  "wallet_seed": {
    "address": "0x1234567890123456789012345678901234567890",
    "chain_id": 1,
    "is_connected": true,
    "ens_name": null
  },

  "turns": [
    {
      "prompt": "I want to swap 100 USDC for ETH on Ethereum",
      // Exactly one of any_of / must_call / exactly:
      "expected_tools": {
        "any_of": ["lifi_get_swap_quote", "lifi_build_swap_tx"]
      }
    },
    {
      "prompt": "Looks good, build the tx",
      "expected_tools": {
        "must_call": ["lifi_build_swap_tx"]
      }
    }
  ],

  "final_assertion": {
    // Assert on UserState after all turns finish. All sub-fields optional.
    "user_state": {
      "pending_txs": {
        "min_count": 1,
        "max_count": 3,
        "expect_any": [
          { "kind": "swap", "chain_id": 1, "data_starts_with": "0x", "data_min_len": 10 }
        ],
        "expect_all": []
      },
      "pending_eip712s": { "max_count": 0 },
      "address_set": true,
      "chain_id": 1
    },

    // Assert on tool call traces / responses.
    "tool_responses": {
      "any_returned_substring": "USDC",          // optional content sniff
      "any_returned_keys": ["source"],           // every Aomi tool wraps with `source`
      "last_tool": "lifi_build_swap_tx"          // optional terminal-tool check
    },

    // Aomi runtime emits topics for tool calls + wallet events.
    "topics_emitted": ["wallet:tx_pending"],

    "no_errors": true,
    "max_turns": 6
  },

  "max_cost_usd": 0.50
}
```

### Field semantics

#### `expected_tools` (per turn, mixed strict/loose)

Pick exactly one mode per turn:

- **`any_of: [...]`** — *any* tool from this set must be called at least once during the turn. Use for **discovery/exploration turns** where the LLM may legitimately pick different tools.
- **`must_call: [...]`** — every tool in this set must be called (subset, not exact). Other tools may also be called. Use for **terminal/action turns** where a specific tool is non-negotiable.
- **`exactly: [...]`** — set of tools called must equal this set exactly (order ignored). Use sparingly — flaky.

Default pattern: **early turns use `any_of`, the final action turn uses `must_call`**.

#### `final_assertion.user_state.pending_txs.expect_any`

Each entry is a partial pattern; a tx matches if it satisfies every specified field. Available fields:
- `kind: "swap" | "approve" | "native_transfer" | "deposit" | ...` — exact match against `PendingSimTx.kind`
- `chain_id: <number>` — exact
- `to: "0x..."` — exact (lowercased)
- `to_contains: "0x..."` — substring (lowercased)
- `data_starts_with: "0x..."`
- `data_min_len: <number>` — bytes-as-hex string length
- `value_non_zero: true` — value present and > 0

`min_count` / `max_count` constrain the queue size before pattern matching.

#### `final_assertion.user_state.pending_eip712s.expect_any`

- `primary_type: "Order" | "OrderBatch" | ...`
- `domain_chain_id: <number>`
- `domain_name_contains: "..."`

#### `final_assertion.tool_responses`

- `any_returned_substring` — at least one tool's JSON-serialized response contains this substring
- `any_returned_keys` — at least one tool's response object has all these top-level keys (sanity check on the `ok()` wrapper)
- `last_tool` — the name of the final tool called

#### `max_cost_usd`

Hard ceiling. The harness tracks LLM token costs across turns and aborts if exceeded. Default `0.50`. Set higher for tests that legitimately need many turns; never above `2.00` without a reason in the user_story.

## Workflow

### 1. Read context

Load all pre-reading files. From `tool.rs`, build a mental list of:
- Tool names + their descriptions (verbatim — these are what the LLM sees)
- Which tools mutate `UserState` (those that build txs / sigs) vs read-only data tools
- Which composites exist (and what sub-tools they wrap internally)

Categorize the platform — same archetypes as `aomi-app-ux-tool-maker`: price-oracle / swap-bridge / lending-yield / perps-cex / prediction / social.

### 2. Decompose the user_story into turns

Aim for **2–4 turns**. More than 4 means either the story is too big (split it) or you're micro-managing the LLM. Common shapes by archetype:

| Archetype | Typical turn shape |
|---|---|
| Price oracle / data | 1–2 turns: ask → answer. Final assertion is on tool_responses, no user_state changes. |
| Swap / bridge | 2–3 turns: discovery (quote/route) → confirm → build tx. Final assertion: `pending_txs.min_count >= 1` with kind/chain. |
| Lending / yield | 2–3 turns: explore vaults/markets → pick → deposit/borrow. Same as swap. |
| Perps / CEX (read-only) | 1–2 turns: same as data. |
| Perps / CEX (write) | 2–3 turns: check price → place order → (optional) verify. Final: `pending_txs` for on-chain DEXes; `tool_responses.last_tool` for off-chain CEX. |
| Prediction | 2–3 turns: search → view → bet. Final: `pending_eip712s` (Polymarket-style L2 sigs) or `pending_txs`. |
| Social | 1–2 turns: search/lookup → present. Pure read. |

Each turn is a **terse natural-language prompt** the user might actually type. Don't fabricate elaborate scenarios — match the user_story.

### 3. Pick `expected_tools` per turn

Use the default pattern: `any_of` for early turns, `must_call` for the final action turn. If the curated tool set has obvious discovery siblings (e.g. `lifi_get_swap_quote` and `lifi_build_swap_tx`), the discovery turn's `any_of` lists both.

Refuse to use `exactly` unless a turn has exactly one obvious tool and you're confident the LLM won't add a sibling call.

### 4. Build `final_assertion`

#### Tx-building app
- `pending_txs.min_count: 1` (or higher if the flow naturally produces multiple, like an approve+swap pair).
- `expect_any` with the most specific, most stable fields:
  - `kind` — always include if you know it (look at `tool.rs`'s `QueuedTxPayload.kind` literals)
  - `chain_id` — include if user_story implies a chain
  - `data_starts_with: "0x"` + `data_min_len: 10` — sanity check (calldata is non-empty, hex-shaped)
  - **Avoid**: exact `to:`, exact `value:`, exact `data:` — these change with prices, fees, contract upgrades
- `pending_eip712s.max_count: 0` if no sigs expected (catches stray sig-prompts)

#### Sig-building app
- `pending_eip712s.min_count: 1`, `expect_any` with `primary_type` and `domain_chain_id`
- `pending_txs.max_count: 0` (or higher if the flow also builds txs)

#### Read-only app
- Skip `user_state` block entirely
- Use `tool_responses.any_returned_keys: ["source"]` (every Aomi tool's `ok()` helper adds `source`)
- Use `tool_responses.any_returned_substring` ONLY if the user_story names a specific token/value the response should mention. Don't assert on volatile content (prices, current TVL).

#### Always include
- `no_errors: true` — any tool returning `Err(...)` fails the test
- `max_turns: <N>` — set to `len(turns) + 2` (buffer for one re-ask round)

### 5. Set cost guard

`max_cost_usd: 0.50` for typical 2–4 turn flows. Set higher only if:
- The PREAMBLE is unusually long (Hyperliquid, Bybit) — round to `0.75`
- The flow needs `>4` turns (rare; rethink first) — round to `1.00`

Never above `2.00`. Document the override in a `// reason: ...` JSONC comment if your tool supports comments, or in the `user_story` text otherwise.

### 6. Write the expanded test.json

Write to `apps/<platform>/test.json`, preserving the original `user_story`. Pretty-print with 2-space indent. Strip JSONC comments before writing (JSON doesn't support them).

### 7. Run the harness

```
cd /Users/cecilia/Code/aomi-apps && cargo test \
  --test local-app-e2e \
  --features generic-test-runner \
  -- --nocapture <platform>
```

(The exact harness invocation may differ — check `aomi/crates/runtime/tests/local-app-e2e.rs` for the current generic-runner entry point. The user is adapting it to read these JSON files.)

The harness:
- Loads `apps/<platform>/<lib>.dylib` (must be built first — run `cargo build -p <platform>` if needed)
- Seeds `UserState` from `wallet_seed`
- Runs each turn through the LLM, asserting `expected_tools` after each
- After the final turn, asserts `final_assertion`
- Tracks LLM cost; aborts if `max_cost_usd` exceeded
- Returns structured pass/fail per assertion

### 8. Report (markdown table)

After the harness completes, emit a per-test-step results table:

```
| Step                                    | Result | Detail |
|---|---|---|
| Build app dylib                         | ✅     | `cargo build -p lifi` |
| Load app (manifest validates)           | ✅     | 6 tools registered |
| Turn 1: discovery                       | ✅     | called `lifi_get_swap_quote` (matched `any_of`) |
| Turn 2: build                           | ✅     | called `lifi_build_swap_tx` (matched `must_call`) |
| `pending_txs.min_count >= 1`            | ✅     | actual: 1 |
| `pending_txs.expect_any[0]` matches     | ❌     | tx kind=`approve` (expected `swap`) |
| `tool_responses.any_returned_substring` | ✅     | found "USDC" in response of `lifi_build_swap_tx` |
| `no_errors`                             | ✅     | 0 tool errors |
| `max_turns <= 6`                        | ✅     | actual: 2 |
| Cost                                    | ✅     | $0.04 / $0.50 budget |
```

If anything failed:
- Show the actual values vs expected
- Don't auto-fix the test.json — surface the failure to the user (might be a real app bug, a flaky LLM, or a too-strict assertion)

If everything passed: print one-line summary `✅ <platform> e2e: 5/5 turns pass, $0.X cost`.

## Anti-patterns

- **Don't add error-path tests.** This skill writes ONE happy-path test per app. Error handling (chain not supported, bad API key, etc.) is out of scope. The user explicitly opted out.
- **Don't assert on volatile values** in `expect_any`: token prices, gas costs, exact `to:` addresses (router contracts upgrade), exact `value:` (depends on slippage/quote timing).
- **Don't use `exactly` mode for tool calls** unless one tool is obviously the only choice. LLMs add sibling calls (`get_meta` before `get_l2_book`, `lookup_chain` before `swap`) — these are correct behavior, not bugs.
- **Don't set `max_cost_usd > 1.00`** without a reason. Tests that need more cost are usually badly scoped.
- **Don't run the test if the app doesn't build first.** Explicit `cargo build -p <platform>` before invoking the harness.
- **Don't fix failing assertions silently.** A failure means: real bug, flaky LLM, or wrong assertion. The user reads the table and decides which.

## Refusal cases

Stop and tell the user if:
- `apps/<platform>/test.json` doesn't exist or has no `user_story` (run `aomi-app-ux-tool-maker` first)
- The app doesn't compile (`cargo build -p <platform>` fails)
- `ANTHROPIC_API_KEY` isn't set (the harness needs it)
- The user_story is fundamentally untestable e2e (e.g., asks for live blockchain interaction we can't sandbox)
- Required platform credentials missing (e.g. `LIFI_API_KEY`) and the app won't function without them
