//! Your theme, die-dev's mono-dark with the extensions' faces: every base and UI face, the orchid
//! faces whose Emacs faces no base face matches, the whiteboard dial's, fig's chrome and the
//! markdown view's, and only faces core or one of your extensions declares.

use die_test_cap as _;

use die::core::{BASE_FACES, UI_FACES};
use die_config::theme::mono_dark;

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
        .collect();
    for name in t.faces.keys() {
        assert!(
            known.contains(&name.as_str()),
            "{name} is no core, orchid, markdown, whiteboard or fig face"
        );
    }
    // The markdown view draws in your stylesheet's colours: each of its faces is styled here.
    for (name, ..) in markdown {
        assert!(t.faces.contains_key(name), "mono-dark doesn't style {name}");
    }
    // And the whiteboard's dial is in its grey and its red.
    for name in whiteboard {
        assert!(t.faces.contains_key(name), "mono-dark doesn't style {name}");
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
