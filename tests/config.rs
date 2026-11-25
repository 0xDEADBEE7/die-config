//! Your config: die-dev's with the extensions on top, through `die_dev::config_with`, as
//! `die_dev::main_with` makes it. Its extensions come in, in order, their keys are bound over
//! die-dev's, only bindings whose commands exist (C8, at every checkpoint), and every face your
//! theme styles is one someone declares.

use die_test_cap as _;

use die::core::{Ctx, Harness, KeyAction, parse_keys};
use die_config::config;
use die_orchid_client::Orchid;

/// The command `keys` runs here, if a command.
fn command(ctx: &Ctx, keys: &str) -> Option<String> {
    let found = ctx.describe_key(&parse_keys(keys).unwrap())?;
    match found.action {
        KeyAction::Command { name, .. } => Some(name.to_string()),
        _ => None,
    }
}

#[test]
fn every_binding_names_a_command_that_exists() {
    die::start(config(vec![])).unwrap();
}

/// The five extensions in the order `extensions()` lists them, `\o`, `\w` and `\v` everywhere and
/// `\m` only where there's markdown, die-dev's own keys under them, and nothing said about a face:
/// the theme styles the extensions' faces, and each is declared.
#[test]
fn your_extensions_come_in_with_their_keys_over_die_devs_and_their_faces() {
    let config = config(vec![]);
    let names: Vec<&str> = config.extensions.iter().map(|e| e.name()).collect();
    assert_eq!(
        names,
        ["orchid", "whiteboard", "jellyfish", "markdown", "fig"],
        "your config's extensions"
    );
    assert!(config.files.is_empty());
    let ctx = die::start(config).unwrap();
    for (keys, cmd) in [
        ("\\o", "orchid"),
        ("\\w", "whiteboard"),
        ("\\v", "fig-toggle"),
        ("\\f", "find-file"),
        ("\\z", "toggle-max-width"),
    ] {
        assert_eq!(command(&ctx, keys).as_deref(), Some(cmd), "{keys}");
    }
    assert_eq!(command(&ctx, "\\m"), None, "no markdown here");
    let unknown: Vec<&String> = (ctx.messages.iter())
        .filter(|m| m.contains("no one declared"))
        .collect();
    assert!(unknown.is_empty(), "{unknown:?}");
    let shown = ctx.focused_buffer().unwrap();
    assert_eq!(
        ctx.buffer_name(shown),
        Some("*scratch*"),
        "no jellyfish outside `greeted`"
    );
}

/// emacs-orchid-client's port comes in through `die::Config.extensions`, its commands with it, and
/// `\o` runs `orchid` in normal state, as `init.el` binds it. Only `describe-key` asks, so none of
/// orchid's commands runs, and nothing reads your `~/.config/orchid`.
#[test]
fn orchid_comes_in_through_the_config_and_leader_o_opens_it() {
    let ctx = die::start(config(vec![])).unwrap();
    for name in [
        "orchid",
        "orchid-open-session",
        "orchid-new-session",
        "orchid-log-show",
    ] {
        assert_eq!(ctx.command_doc(name).map(|d| d.1), Some("orchid"), "{name}");
    }
    let config = config(vec![]);
    let mut exts = die::built_in();
    exts.extend(config.extensions);
    let mut h = Harness::with_config(exts, config.core).unwrap();
    assert!(h.with_ext(|_: &mut Orchid, _| ()).is_ok());
    h.open_text("t", "").unwrap();
    let logs = h.type_keys("\\hk\\o").unwrap();
    assert!(logs.iter().all(|k| k.errors.is_empty()), "{logs:?}");
    let help = h.find_buffer("*help*").expect("*help* is there");
    let help = h.text(help).unwrap().rope().to_string();
    assert!(
        help.contains("orchid") && help.contains("Open the orchid session browser"),
        "{help}"
    );
    assert_eq!(
        h.next_timer(),
        None,
        "and no timer of orchid's runs while nothing's open"
    );
}
