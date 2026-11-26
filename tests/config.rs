//! This config: die-dev's with the extensions on top, through `die_dev::config_with`, as
//! `die_dev::main_with` makes it. Its extensions come in, in order, their keys are bound over
//! die-dev's, only bindings whose commands exist (C8, at every checkpoint), and every face its
//! theme styles is one someone declares. And `\d` runs delta on a repository made here.

use die_test_cap as _;

use die::core::{Ctx, Harness, KeyAction, parse_keys};
use die_config::config;
use die_orchid_client::Orchid;
use std::path::Path;
use std::process::Command;

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

/// The seven extensions in the order `extensions()` lists them, `\o`, `\w` and `\v` everywhere and
/// `\m` only where there's markdown, die-dev's own keys under them, and nothing said about a face:
/// the theme styles the extensions' faces, and each is declared.
#[test]
fn the_extensions_come_in_with_their_keys_over_die_devs_and_their_faces() {
    let config = config(vec![]);
    let names: Vec<&str> = config.extensions.iter().map(|e| e.name()).collect();
    assert_eq!(
        names,
        [
            "orchid",
            "whiteboard",
            "jellyfish",
            "markdown",
            "fig",
            "scribble",
            "delta"
        ],
        "this config's extensions"
    );
    assert!(config.files.is_empty());
    let ctx = die::start(config).unwrap();
    for (keys, cmd) in [
        ("\\o", "orchid"),
        ("\\w", "whiteboard"),
        ("\\v", "fig-toggle"),
        ("\\s", "scribble"),
        ("\\d", "delta"),
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
/// orchid's commands runs, and nothing reads `~/.config/orchid`.
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

/// git in `dir`, as Ada, with a HOME and a git config of `home`'s and nothing of this machine's.
fn git(dir: &Path, home: &Path, args: &[&str]) {
    let mut cmd = Command::new("git");
    cmd.args(args).current_dir(dir).env("HOME", home);
    cmd.env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null");
    for name in ["GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE"] {
        cmd.env_remove(name);
    }
    for (k, v) in [("NAME", "Ada Lovelace"), ("EMAIL", "ada@example.com")] {
        cmd.env(format!("GIT_AUTHOR_{k}"), v);
        cmd.env(format!("GIT_COMMITTER_{k}"), v);
    }
    let out = cmd.output().unwrap();
    let said = String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(), "git {args:?}: {said}");
}

/// `\d` runs delta through this config's keys: from a buffer in a repository made here, its
/// commits take the window, and `q` gives it back. delta is this config's but for where it finds
/// git: on this process's PATH, with the repository's HOME, where the config's asks the login
/// shell. Nothing it runs changes the repository.
#[test]
fn leader_d_shows_this_repositorys_commits_in_delta() {
    let root = std::env::temp_dir().join(format!("die-config-delta-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let (dir, home) = (root.join("shop"), root.join("home"));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::create_dir_all(&home).unwrap();
    git(&dir, &home, &["init", "-q", "-b", "main"]);
    std::fs::write(dir.join("notes.md"), "# Notes\n").unwrap();
    git(&dir, &home, &["add", "-A"]);
    git(&dir, &home, &["commit", "-q", "-m", "first: the notes"]);

    let delta = die_delta::Delta::new(die_delta::Options {
        path: std::env::var_os("PATH"),
        env: vec![
            ("HOME".into(), home.clone().into()),
            ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
            ("GIT_CONFIG_GLOBAL".into(), "/dev/null".into()),
        ],
        ..die_delta::Options::default()
    })
    .copying_with(die::vim::kill_new);
    let config = config(vec![]);
    let mut exts = die::built_in();
    for ext in config.extensions {
        if ext.name() != "delta" {
            exts.push(ext);
        }
    }
    exts.push(Box::new(delta));
    let mut h = Harness::with_config(exts, config.core).unwrap();
    let (notes, _) = h.open_text("notes", "my notes\n").unwrap();
    h.set_directory(notes, &dir);
    h.set_window_size(120, 40);
    let logs = h.type_keys("\\d").unwrap();
    assert!(logs.iter().all(|k| k.errors.is_empty()), "{logs:?}");
    h.run_until_idle();
    let log = h.find_buffer(die_delta::LOG_BUFFER);
    assert!(log.is_some(), "{:?}\n{}", h.messages.clone(), h.screen());
    assert_eq!(h.focused_buffer(), log, "in this window");
    let screen = h.screen();
    assert!(
        screen.contains("delta · shop") && screen.contains("first: the notes"),
        "{screen}"
    );
    let logs = h.type_keys("q").unwrap();
    assert!(logs.iter().all(|k| k.errors.is_empty()), "{logs:?}");
    assert_eq!(h.find_buffer(die_delta::LOG_BUFFER), None);
    assert_eq!(h.focused_buffer(), Some(notes), "back on the notes");
    let _ = std::fs::remove_dir_all(&root);
}
