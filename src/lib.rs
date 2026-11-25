//! The owner's DIE: DIE's own config, die-dev, drafted from the owner's Emacs `init.el`, with the
//! extensions `init.el`'s packages became on top. die-dev has DIE's keys, mono-dark for core's
//! faces, the status line and the arguments; this adds what isn't part of DIE (DIE's scope D14):
//! the extensions, their keys, their faces in mono-dark, and the jellyfish to open on, all
//! through [`die_dev::main_with`] and an [`Extra`], so nothing of die-dev is copied here.
//!
//! Installing an extension is its dependency line in `Cargo.toml` and its entry in
//! [`extensions`]; its keys go in [`bindings`] and its faces in [`theme::faces`]. Removing one is
//! deleting them.
#![warn(missing_docs)]

#[cfg(test)]
use die_test_cap as _;

pub mod theme;

use die::core::{Extension, Registry, State};
use die_dev::{Extra, Greeting};
use std::path::PathBuf;

/// What this config adds to die-dev's: the extensions, their keys, their faces and the
/// greeting. `main` is `die_dev::main_with(extra)`.
pub fn extra() -> Extra {
    Extra {
        extensions: extensions(),
        bindings: Some(Box::new(bindings)),
        theme: Some(Box::new(theme::faces)),
        greeting: Some(greeting()),
    }
}

/// Your config, for these files: die-dev's with [`extra`] on top, as `--check` and the tests
/// know it, a still text buffer with no greeting ([`die_dev::greeted`] adds that for a run you
/// look at).
pub fn config(files: Vec<PathBuf>) -> die::Config {
    die_dev::config_with(files, extra())
}

/// What a run you look at opens on when no file is named: your jellyfish, in the directory DIE
/// started in (`die_dev::greeted` says when).
pub fn greeting() -> Greeting {
    Greeting {
        command: "jellyfish".into(),
        buffer: die_jellyfish::NAME.into(),
    }
}

/// The packages your `init.el` loads, as DIE has them: emacs-orchid-client's port, with its
/// defaults, as you set none of its options. It finds `orchid` on your login shell's PATH, which
/// has `~/.local/bin`, as `init.el` adds it to `exec-path`. And the whiteboard, with its defaults:
/// PNGs at 2x on the theme's background, the fonts where DIE's are. And your jellyfish, with its
/// defaults: the water's dust and twinkles but no shoal, the window's whole width, and at rest
/// after five minutes without a key. Registering it shows nothing: the greeting does that. And
/// the markdown view, with its defaults: the mode on `.md` files and markdown-mode's other
/// extensions, front matter hidden as your preview's export strips it, links that aren't
/// files DIE opens going to `open`, and the rendered view a page drawn as GitHub draws one, in
/// mono-dark's colours. A file opens raw, as it always has: `\m` renders it. And fig, with its
/// defaults: times in the system's zone, the fonts where DIE's are, the chrome in mono-dark's
/// greys, a `.fig` or `.fig.json` file opening as its figure (`\v` flips it to its text and
/// back), and `y` in its detail panel copying as Evil's `y` does, to the clipboard and the kill
/// ring. Nothing of it runs until `\v` or a figure's file.
pub fn extensions() -> Vec<Box<dyn Extension>> {
    vec![
        Box::new(die_orchid_client::Orchid::default()),
        Box::new(die_whiteboard::Whiteboard::default()),
        Box::new(die_jellyfish::Jellyfish::default()),
        Box::new(die_markdown::Markdown::default()),
        Box::new(
            die_fig::Fig::new(die_fig::Options {
                open_fig_as: die_fig::OpenAs::Figure,
                ..die_fig::Options::default()
            })
            .copying_with(die::vim::kill_new),
        ),
    ]
}

/// The extensions' keys, declared after die-dev's on the same registry, so its leader `\` is
/// set and the extensions' commands are there to bind.
pub fn bindings(reg: &mut Registry) {
    let keys = reg.global_keymap();
    // emacs-orchid-client's session browser, in normal state, as `init.el` binds it.
    keys.bind(State::Normal, "<leader>o", "orchid");
    // The whiteboard: the board used last, or a scratch board in this window; from a board, a
    // new scratch board.
    keys.bind(State::Normal, "<leader>w", "whiteboard");
    // fig: this buffer's data as a figure, or from a figure its buffer again; in visual state,
    // the selection's. `\g`, `\d`, `\s` and `\t` are taken in init.el.
    keys.bind(State::Normal, "<leader>v", "fig-toggle");
    keys.bind(State::Visual, "<leader>v", "fig-toggle");
    // A markdown file's two views, rendered and raw. It's in the mode's keymap, so `\m` is a key
    // only where there's markdown to render, and in the page's, which is a buffer of its own
    // with no mode: `\m` there goes back to the file. Both only where the extension is.
    if let Some(keys) = reg.mode_keymap(die_markdown::MODE) {
        keys.bind(State::Normal, "<leader>m", "markdown-toggle");
        let keys = reg.kind_keymap(die_markdown::PAGE);
        keys.bind(State::Normal, "<leader>m", "markdown-toggle");
    }
}
