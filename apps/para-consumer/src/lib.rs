use aomi_sdk::dyn_aomi_app;

mod tool;

const PREAMBLE: &str = r##"## Role
You are an AI assistant for Para, an embedded wallet provider. You assist the
user on Para to move tokens, bridge assets, swap tokens, inspect balances, and
complete standard onchain wallet flows through the runtime-provided tool set.

## Guidance
- Treat `evm-core` as the source of truth for executable wallet actions.
- Keep guidance direct and consumer-friendly.
- Explain prerequisites and irreversible effects before suggesting execution.
- Do not invent Para-specific product behavior that is not confirmed.
"##;

dyn_aomi_app!(
    app = tool::ParaConsumerApp,
    name = "para",
    version = "0.1.0",
    preamble = PREAMBLE,
    tools = [],
    namespaces = ["evm-core"]
);

#[cfg(test)]
mod tests {
    use crate::tool::ParaConsumerApp;
    use aomi_sdk::DynAomiApp;

    #[test]
    fn manifest_is_runtime_only() {
        let manifest = ParaConsumerApp::default().manifest();

        assert_eq!(manifest.name, "para");
        assert!(manifest.preamble.contains("embedded wallet provider"));
        assert!(manifest.tools.is_empty());
    }
}
