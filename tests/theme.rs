//! This config's theme, die-dev's mono-dark with the extensions' faces: every base and UI face, the
//! orchid faces whose Emacs faces no base face matches, the whiteboard dial's, fig's chrome, the
//! markdown view's and delta's, and only faces core or one of its extensions declares.

use die_test_cap as _;

use die::core::{BASE_FACES, UI_FACES};
use die_config::theme::mono_dark;
use die_dev::theme::{GREEN, RED};

/// die-delta's faces, which it declares in `register` without listing them: the config's DIE holds
/// each to a declaration below.
const DELTA: [&str; 13] = [
    "delta-search",
    "delta-commit",
    "delta-merge",
    "delta-hash",
    "delta-date",
    "delta-ref",
    "delta-banner",
    "delta-added",
    "delta-removed",
    "delta-added-word",
    "delta-removed-word",
    "delta-hunk",
    "delta-file",
];

#[test]
fn mono_dark_styles_every_core_face_and_the_extensions_and_only_those_declared() {
    let t = mono_dark();
    for (name, _) in BASE_FACES.iter().chain(UI_FACES) {
        assert!(
            t.faces.contains_key(*name),
            "mono-dark doesn't style {name}"
        );
    }
    let core = BASE_FACES.iter().chain(UI_FACES).map(|f| f.0);
    let orchid = die_orchid_client::FACES.iter().map(|f| f.0);
    let markdown = die_markdown::faces();
    let whiteboard = [
        die_whiteboard::face::DIAL,
        die_whiteboard::face::DIAL_ACCENT,
    ];
    let fig = die_fig::faces().iter().copied();
    let known: Vec<&str> = core
        .chain(orchid)
        .chain(markdown.map(|f| f.0))
        .chain(whiteboard)
        .chain(fig)
        .chain(DELTA)
        .collect();
    for name in t.faces.keys() {
        assert!(
            known.contains(&name.as_str()),
            "{name} is no core, orchid, markdown, whiteboard, fig or delta face"
        );
    }
    // The markdown view draws in the stylesheet's colours: each of its faces is styled here.
    for (name, ..) in markdown {
        assert!(t.faces.contains_key(name), "mono-dark doesn't style {name}");
    }
    // And the whiteboard's dial is in its grey and its red.
    for name in whiteboard {
        assert!(t.faces.contains_key(name), "mono-dark doesn't style {name}");
    }
    // And delta's are each styled, but its search line, which takes `highlight` as scribble's does.
    for name in DELTA {
        let styled = name != "delta-search";
        assert_eq!(t.faces.contains_key(name), styled, "{name}");
    }
}

/// What `extra()` gives `die_dev::config_with` is the same theme.
#[test]
fn the_config_is_drawn_in_this_theme() {
    let ctx = die::start(die_config::config(vec![])).unwrap();
    let t = mono_dark();
    for name in ["fig-grid", "markdown-h1", "whiteboard-dial"] {
        let got = ctx.face_style(name).unwrap().fg;
        assert_eq!(got, t.faces[name].fg, "{name}");
    }
    let got = ctx.face_style("orchid-seek-search").unwrap().bg;
    assert_eq!(got, t.faces["orchid-seek-search"].bg);
}

/// delta's faces are each declared, and drawn as the theme has them over the base faces they're
/// declared on: its lines plain in the green and the red, though success and error are bold, the
/// words that changed bold, a branch or tag in the ANSI yellow, and the search line on
/// `highlight`'s ground.
#[test]
fn delta_is_drawn_in_mono_darks_green_red_and_yellow() {
    let ctx = die::start(die_config::config(vec![])).unwrap();
    let style = |name: &str| {
        let style = ctx.face_style(name);
        style.unwrap_or_else(|| panic!("{name} isn't declared"))
    };
    for name in DELTA {
        style(name);
    }
    for (name, fg, bold) in [
        ("delta-added", GREEN, false),
        ("delta-removed", RED, false),
        ("delta-added-word", GREEN, true),
        ("delta-removed-word", RED, true),
        ("delta-commit", GREEN, false),
    ] {
        let got = style(name);
        assert_eq!((got.fg, got.bold), (Some(fg), Some(bold)), "{name}");
    }
    let yellow = mono_dark().ansi[3];
    assert_eq!(style("delta-ref").fg, Some(yellow), "the ANSI yellow");
    assert_eq!(style("delta-search").bg, style("highlight").bg);
}
