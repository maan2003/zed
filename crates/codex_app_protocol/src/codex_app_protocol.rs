//! Local app-server protocol types for Zed integration.
//!
//! This crate intentionally checks in generated types locally so this
//! workspace does not depend on `~/src/codex` crates.

// Regeneration:
//   RUSTFMT_BIN="$(nix shell nixpkgs#rustfmt -c which rustfmt)"
//   RUSTFMT="$RUSTFMT_BIN" cargo typify /home/maan2003/src/codex/codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.v2.schemas.json -o crates/codex_app_protocol/src/full_types_generated.rs -B
include!("full_types_generated.rs");
