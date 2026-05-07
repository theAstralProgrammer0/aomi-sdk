# byreal-perps-cli — sequence flows

How the byreal CLI (`byreal-perps-cli`) orchestrates its three actors: the local CLI process, byreal's Privy server-signing proxy, and Hyperliquid itself.

## Flow 1 — `account init` (server-signing path)

The only flow that touches byreal's backend. After this completes, the agent's private key lives locally and the backend is never spoken to again until the agent expires.

```mermaid
sequenceDiagram
    autonumber
    actor User
    participant CLI as byreal-perps-cli
    participant Privy as Privy proxy
    participant HL as Hyperliquid
    participant DB as local SQLite

    User->>CLI: account init
    CLI->>CLI: read ~/.claw/config.json
    Note right of CLI: baseUrl, agentToken, masterAddress
    CLI->>CLI: viem.generatePrivateKey()
    Note right of CLI: agentPrivateKey, agentAddress

    CLI->>HL: ExchangeClient.approveAgent
    Note over CLI,HL: SDK builds EIP-712 typed-data for the L1 ApproveAgent action

    HL-->>CLI: typed-data signing callback
    CLI->>Privy: POST /sign/evm-typed-data
    Note over Privy: Privy custodies the master key. User never sees the master PK.
    Privy-->>CLI: signature

    CLI->>HL: POST /exchange action+signature+nonce
    HL-->>CLI: status ok, agent valid 7d
    CLI->>DB: INSERT account row
    Note right of DB: masterAddress, agentPrivateKey, agentAddress, expiresAt
    CLI-->>User: account initialized
```

Key points:

- The **master key never leaves Privy's custody.** byreal's backend is a thin Privy passthrough.
- The **agent key is generated locally** by viem and never seen by the backend.
- The Privy proxy is called **exactly once** per agent (~once a week).

## Flow 2 — `order market buy` (steady state, no backend)

After init, every trading command is local-key + direct-to-Hyperliquid. byreal's backend is not involved.

```mermaid
sequenceDiagram
    autonumber
    actor User
    participant CLI as byreal-perps-cli
    participant DB as local SQLite
    participant HL as Hyperliquid

    User->>CLI: order market buy 0.01 BTC --tp 110000

    CLI->>DB: SELECT default account
    DB-->>CLI: agentPrivateKey, masterAddress, expiresAt
    CLI->>CLI: assert expiresAt is in the future
    Note right of CLI: else delete and force re-init

    par Read state in parallel
        CLI->>HL: WS subscribe clearinghouseState
        HL-->>CLI: positions, margin, free collateral
    and
        CLI->>HL: POST /info type=meta
        HL-->>CLI: universe, szDecimals, maxLeverage
    and
        CLI->>HL: POST /info type=allMids
        HL-->>CLI: BTC mid price
    end

    CLI->>CLI: compute size and slippage limit price
    Note right of CLI: round to szDecimals, validate min notional
    CLI->>CLI: build Actions::Order BulkOrder
    Note right of CLI: orders = [main, tp_trigger], grouping = positionTpsl
    CLI->>CLI: nonce = Date.now()
    CLI->>CLI: connection_id = keccak msgpack action plus nonce
    CLI->>CLI: typed_data = L1Agent source a, connection_id
    CLI->>CLI: signature = ECDSA.sign EIP-712 digest with agentPrivateKey
    Note over CLI: All signing is in-process with the agent key from SQLite. No network call to byreal or Privy.

    CLI->>HL: POST /exchange action+signature+nonce
    HL-->>CLI: status ok, statuses = [filled, resting]
    CLI-->>User: filled 0.01 BTC, TP order resting
```

Key points:

- **Zero backend dependency at trade time.** byreal's value-add is one-time onboarding via Privy, plus local-first CLI ergonomics around Hyperliquid.
- The **agent's 7-day expiry** is what forces users back through Flow 1 periodically — the only place byreal's backend matters in steady state.
- **Reads use WebSocket first, HTTP fallback.** Both go straight to Hyperliquid (never via byreal).

## How this maps to our aomi `byreal` app

Our app collapses Flow 2 into routed `build_*` / `submit_*` pairs and replaces the in-process ECDSA step with a hop through `commit_eip712` to the host wallet — same architecture, signer extracted into a separate trust domain. We deliberately skipped Flow 1 entirely (no agent approval), so today the host wallet signs every trade as if it were the master.

```mermaid
sequenceDiagram
    autonumber
    actor LLM
    participant App as aomi byreal app
    participant Host as host wallet
    participant HL as Hyperliquid

    LLM->>App: byreal_build_order coin sz tif
    App->>HL: POST /info type=meta
    HL-->>App: asset universe
    App->>App: build Actions::Order via hl_ranger
    App->>App: nonce, connection_id, typed_data
    App-->>LLM: ToolReturn preview plus route
    Note right of App: route adds host CommitEip712 then awaits master_signature

    LLM->>Host: commit_eip712 typed_data
    Host->>Host: user approves and wallet signs
    Host-->>LLM: signature

    LLM->>App: byreal_submit_order action nonce master_signature
    App->>App: parse_signature into r s v
    App->>HL: POST /exchange action+signature+nonce
    HL-->>App: status ok with oid
    App-->>LLM: response
```
