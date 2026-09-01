//! Source gate: tag side effects must not elevate to System for history append.
#![cfg(feature = "ssr")]
#![allow(missing_docs)]

use std::path::PathBuf;

#[test]
fn tag_side_effects_must_not_elevate_to_system_for_history() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/side_effects");
    let writer =
        std::fs::read_to_string(root.join("history_writer.rs")).expect("read history_writer.rs");
    assert!(
        !writer.contains("with_actor(Actor::System")
            && !writer.contains("with_actor(valence::Actor::System"),
        "TagHistoryWriter must append under session Valence (no System elevate)"
    );
    assert!(
        !writer.contains("tag_history_append"),
        "remove System operation tag_history_append"
    );
}
