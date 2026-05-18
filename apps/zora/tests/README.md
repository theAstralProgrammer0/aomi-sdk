# Zora app e2e specs

The harness at `aomi/crates/runtime/tests/local-app-e2e.rs` discovers
`apps/zora/test.json` by default. The files in this directory are
additional user-story specs you can run by pointing `AOMI_E2E_SPEC` at
them explicitly.

All anchor on real on-chain data (real coins, real creator handles) so the
LLM hits the live API and the simulator hits live Base mainnet via the
gateway RPC.

## How to run

```bash
# Run the default story (multi-hop ETH buy of nap creator coin)
cd /path/to/product-mono/aomi
AOMI_E2E_APP_PATH=/path/to/apps/zora/target/debug/libzora.dylib \
  cargo test -p aomi-runtime --test local-app-e2e app_e2e_specs -- --nocapture

# Run a specific story
AOMI_E2E_APP_PATH=/path/to/apps/zora/target/debug/libzora.dylib \
AOMI_E2E_SPEC=/path/to/apps/zora/tests/buy_with_zora.json \
  cargo test -p aomi-runtime --test local-app-e2e app_e2e_specs -- --nocapture
```

## Stories

| File | Surface exercised | Real anchor |
|---|---|---|
| `../test.json` | discovery + multi-hop V4 buy (`call_v4_multihop_swap`) with ETH input | `@nap` creator coin |
| `buy_with_zora.json` | discovery + single-hop V4 buy (`call_v4_swap`) with ZORA input | `@papanda`'s `zora` trend coin (`0x2748…`) |
| `research_trending.json` | read-only research: trends → coin → holders → price history → recommendation | top trending coin (whatever is most active that day) |
| `exact_out_limbo.json` | exact-out single-hop V4 (`call_v4_exact_out_swap`) — "want exactly N tokens" | `@basedflick`'s `limbo` trend coin (`0xbf7a…`) |
| `sell_creator_coin.json` | sell flow: balanceOf → Permit2 → single-hop V4 with zero_for_one flipped | `@papanda`'s personal creator coin (`0x4eaa…`) |
| `creator_forensics.json` | reverse lookup: address → profile → coin enumeration | wallet `0x812f11c3…` resolves to `@papanda` |

## What each story proves

- **`test.json`**: agent can compose the canonical "buy with ETH" flow via the V4 multi-hop encoder. Already proven against live Base mainnet — see the conversation history.
- **`buy_with_zora.json`**: the simpler single-hop path. No multi-hop encoding needed; tests the Permit2 ERC-20 input flow.
- **`research_trending.json`**: read-only surface coverage. No skill activation, no execution. Tests that the LLM can navigate Zora's read tools and synthesize a recommendation.
- **`exact_out_limbo.json`**: the exact-out direction. Distinct action codes (`SWAP_EXACT_OUT_SINGLE = 0x08`). Pins the receive amount, bounds the spend.
- **`sell_creator_coin.json`**: the inverse direction (coin → ZORA). Verifies `zero_for_one` flip handling + sell-side Permit2 sequence.
- **`creator_forensics.json`**: address-as-identifier resolution. Tests `zora_get_profile`'s tolerance for the three identifier types (handle / address / numeric id).

## Notes on simulation outcomes

The harness asserts `no_errors: true` against the **tool layer**, not the
on-chain simulation outcome. `call_v4_*` tools return their result inside
a JSON envelope with `success: true|false` + `revert_reason` rather than
returning `Err`, so a sim revert doesn't fail the test as long as the
agent reads the revert and bails cleanly rather than retrying garbage.

In practice some of these stories WILL revert at the on-chain simulation
step because the seed wallet (vitalik.eth) doesn't actually hold the
relevant tokens or have the Permit2 allowances set up. That's expected.
The test passes if the agent composes the right tool sequence and
surfaces the revert reason — that's what "this flow works end-to-end"
means in this harness.

For a green simulation (and a real broadcastable tx), run against a
wallet that's actually funded with the required tokens.
