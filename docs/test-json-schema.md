# `apps/<platform>/test.json` schema

End-to-end behavioral test spec for an Aomi app. Authored by the
`aomi-app-e2e-tester` skill (seeded by `aomi-app-ux-tool-maker`),
consumed by the generic harness in `aomi/crates/runtime/tests/local-app-e2e.rs`.

One file per app: `apps/<platform>/test.json`.

## Top-level shape

```jsonc
{
  "user_story": "string",                // seed — written by aomi-app-ux-tool-maker
  "wallet_seed": { ... },                // optional — UserState seeded before turn 1
  "turns": [ ... ],                      // 2–4 turns; each is a prompt + expected_tools
  "final_assertion": { ... },            // assertions on UserState + tool responses
  "max_cost_usd": 0.50                   // hard cost ceiling (LLM tokens)
}
```

## `wallet_seed` (optional)

Pre-populates `UserState` before the first turn. Many tools assume a connected wallet.

```json
{
  "address": "0x1234567890123456789012345678901234567890",
  "chain_id": 1,
  "is_connected": true,
  "ens_name": null
}
```

All fields optional. Omit the block entirely for read-only / data apps.

## `turns[]`

Ordered list. Each turn is one user prompt + one assertion on which tools were called during that turn.

```jsonc
{
  "prompt": "I want to swap 100 USDC for ETH",
  "expected_tools": {
    // exactly one of:
    "any_of":    ["lifi_get_swap_quote", "lifi_build_swap_tx"],
    "must_call": ["lifi_build_swap_tx"],
    "exactly":   ["lifi_build_swap_tx"]
  }
}
```

### `expected_tools` modes (mixed by design)

| Mode | Semantics | When to use |
|---|---|---|
| `any_of: [...]` | At least one of these tools must be called during this turn. | **Discovery turns** — LLM may legitimately pick different tools. |
| `must_call: [...]` | Every listed tool must be called (subset; others may also be called). | **Terminal action turns** — a specific tool is non-negotiable. |
| `exactly: [...]` | Set of tools called must equal this set exactly (order ignored). | Sparingly — flaky; LLMs add sibling calls. |

**Default pattern**: `any_of` for early turns, `must_call` for the final action turn.

## `final_assertion`

Run after all turns complete. Every sub-block is optional.

### `user_state`

Constraints on `UserState` (`aomi/crates/tools/src/user_state.rs`):

```jsonc
{
  "pending_txs": {
    "min_count": 1,
    "max_count": 3,
    "expect_any": [
      // Tx matches if it satisfies every specified field. Skip what you don't care about.
      {
        "kind": "swap",                  // exact match against PendingSimTx.kind
        "chain_id": 1,                   // exact
        "to": "0x...",                   // exact (lowercased)
        "to_contains": "0x",             // substring (lowercased)
        "data_starts_with": "0x",
        "data_min_len": 10,              // hex string length
        "value_non_zero": true
      }
    ],
    "expect_all": []                     // every tx must match (rare)
  },
  "pending_eip712s": {
    "min_count": 1,
    "max_count": 1,
    "expect_any": [
      {
        "primary_type": "Order",
        "domain_chain_id": 137,
        "domain_name_contains": "Polymarket"
      }
    ]
  },
  "address_set": true,                   // UserState.address.is_some()
  "chain_id": 1                          // UserState.chain_id == 1
}
```

### `tool_responses`

Light content checks across all tool calls in the session:

```json
{
  "any_returned_substring": "USDC",
  "any_returned_keys": ["source"],
  "last_tool": "lifi_build_swap_tx"
}
```

`any_returned_keys` is the cheapest sanity check — every Aomi tool's `ok()` helper adds a `source` key. Use this for read-only apps that have no UserState changes.

### `topics_emitted`

Aomi runtime emits topics for tool calls + wallet events. List ones that must appear:

```json
["wallet:tx_pending", "lifi_build_swap_tx"]
```

### Cross-cutting

```json
{
  "no_errors": true,                     // any tool returning Err(...) fails the test
  "max_turns": 6                         // upper bound on session length (catches LLM loops)
}
```

## `max_cost_usd`

Hard ceiling on LLM token cost across the whole test. Harness aborts if exceeded.

| Default | When |
|---|---|
| `0.50` | Standard 2–4 turn flows |
| `0.75` | Apps with unusually long PREAMBLE (Hyperliquid, Bybit) |
| `1.00` | Genuinely needs >4 turns (rethink first) |
| `2.00` (max) | Document the reason in `user_story` |

## Anti-patterns

- **Don't assert on volatile values** in `pending_txs.expect_any`: prices, gas, exact `to:` (router upgrades), exact `value:`, exact `data:`.
- **Don't use `exactly` for tool calls** unless one tool is genuinely the only choice.
- **Don't include error-path tests.** This schema is happy-path only by design.
- **Don't set `max_turns` too tight.** `len(turns) + 2` buffer for re-asks.

## Example: complete LiFi swap test

```json
{
  "user_story": "Swap 100 USDC for ETH on Ethereum mainnet via LiFi",
  "wallet_seed": {
    "address": "0x1234567890123456789012345678901234567890",
    "chain_id": 1,
    "is_connected": true
  },
  "turns": [
    {
      "prompt": "I want to swap 100 USDC for ETH on Ethereum",
      "expected_tools": {
        "any_of": ["lifi_get_swap_quote", "lifi_build_swap_tx"]
      }
    },
    {
      "prompt": "Looks good — build the tx",
      "expected_tools": {
        "must_call": ["lifi_build_swap_tx"]
      }
    }
  ],
  "final_assertion": {
    "user_state": {
      "pending_txs": {
        "min_count": 1,
        "expect_any": [
          { "kind": "swap", "chain_id": 1, "data_starts_with": "0x", "data_min_len": 10 }
        ]
      },
      "pending_eip712s": { "max_count": 0 }
    },
    "tool_responses": {
      "any_returned_keys": ["source"]
    },
    "no_errors": true,
    "max_turns": 4
  },
  "max_cost_usd": 0.50
}
```

## Example: Defillama read-only test

```json
{
  "user_story": "Get the current TVL of Aave on Ethereum",
  "turns": [
    {
      "prompt": "What's Aave's TVL right now?",
      "expected_tools": {
        "any_of": ["defillama_get_protocol_tvl", "defillama_list_protocols"]
      }
    }
  ],
  "final_assertion": {
    "tool_responses": {
      "any_returned_substring": "aave",
      "any_returned_keys": ["source"]
    },
    "no_errors": true,
    "max_turns": 3
  },
  "max_cost_usd": 0.30
}
```
