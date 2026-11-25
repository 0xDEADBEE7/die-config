# die-config

The owner's [DIE](https://github.com/0xDEADBEE7/die): a binary crate named `die` that is DIE's
own config, `apps/die-dev`, with the extensions installed on top. die-dev has DIE's keys (leader
`\`), mono-dark for core's faces, the status line and the command line's arguments, and no
extension. This crate adds what isn't part of DIE: the extensions, their keys, their faces in
mono-dark, and the jellyfish DIE opens on. It doesn't copy die-dev: `main` is

```rust
die_dev::main_with(die_config::extra)
```

and `extra()` (src/lib.rs) is a `die_dev::Extra { extensions, bindings, theme, greeting }`. This
is the model DIE's scope sets out (§5.10): a personal config as a binary crate, as in xmonad.

## What it installs

| Extension | What it is | Keys |
|---|---|---|
| [die-orchid-client](https://github.com/0xDEADBEE7/die-orchid-client) | The owner's emacs-orchid-client, ported: orchid's session browser and chats | `\o` in normal state |
| [die-whiteboard](https://github.com/0xDEADBEE7/die-whiteboard) | A whiteboard: freehand, shapes and arrows, saved as SVG, exported as PNG | `\w` in normal state |
| [die-jellyfish](https://github.com/0xDEADBEE7/die-jellyfish) | The owner's ASCII jellyfish | the greeting; `:jellyfish` |
| [die-markdown](https://github.com/0xDEADBEE7/die-markdown) | A markdown file's rendered view, a page drawn as GitHub draws a README | `\m` in a markdown file and on its page |
| [die-fig](https://github.com/0xDEADBEE7/die-fig) | Figures from data: charts, network graphs, gantt charts and dashboards | `\v` in normal and visual state |

Each extension's README has its keys and how to use it. Here they're as die-dev had them before
the extensions left DIE: each with its defaults, but fig, which opens a `.fig` or `.fig.json` file
as its figure (`OpenAs::Figure`) and copies a yank in its detail panel through die-vim's kill ring
(`copying_with(die::vim::kill_new)`). [die-draw](https://github.com/0xDEADBEE7/die-draw), the
drawing library die-fig and die-markdown share, comes in through them.

**The greeting.** A run you look at, with no file named, opens on the jellyfish in the directory
DIE started in (`die_dev::greeted` says when: never over a session's splits, a file, `--check`
or a restart that brought something back).

**The theme.** die-dev's mono-dark styles core's faces only, since DIE reports a face a theme
styles that no one declared. `theme::faces` styles the extensions' faces in its palette
(`die_dev::theme`): the orchid faces no base face matches, the whiteboard dial's two, fig's chrome
(its series and status faces are left to fig's own colours) and every markdown face, from the
owner's markdown preview stylesheet.

## Building DIE.app from it

In a DIE checkout, `cargo xtask app` builds DIE.app from the first config it finds:

1. `--config DIR`;
2. else `$DIE_CONFIG`, if it's set and not empty;
3. else `die-config` beside the checkout (`../die-config`), if it has a `Cargo.toml`;
4. else DIE's own `apps/die-dev`, which has no extension.

So with this repository checked out as `die-config` beside a DIE checkout, a plain
`cargo xtask app` builds it. A config that's named has to be a crate, or the build stops and says
so. Cargo runs in the config's own directory (`cargo build --release --bin die`), so this
repository's `.cargo/config.toml` applies, and the binary is taken from the target directory
`cargo metadata` names there (so `CARGO_TARGET_DIR` works). It isn't `--locked`, though this
repository has a `Cargo.lock`: only DIE's own workspace, whose lock DIE's commits keep, builds
locked. Nothing keeps this one current, as DIE's crates and the extensions move on `main` with
their own commits, and with the dev config reach it by path patches to checkouts, so a build
brings it up to date, changing only what it must, rather than stopping at a dependency one of them
added or bumped. `--home DIR` puts the bundle at `DIR/Applications/DIE.app` instead of
`~/Applications/DIE.app`.

The bundle records where it came from: `Contents/Resources/checkout` names the DIE checkout and
`Contents/Resources/config` this config, as an absolute path. `:rebuild` in DIE runs
`cargo xtask app --config <that config> --home <that home>` in that checkout, so DIE is built from
this config again, whichever file you're in. If the config isn't a crate any more, `:rebuild` says
so and builds nothing.

## Installing and removing an extension

Installing one is four small edits:

- its dependency line in `Cargo.toml`, from its repository at `main`:

  ```toml
  die-fig = { git = "https://github.com/0xDEADBEE7/die-fig", branch = "main" }
  ```

- its entry in `extensions()` in `src/lib.rs`, with the options you want:

  ```rust
  Box::new(die_fig::Fig::default()),
  ```

- its keys, if it leaves any to the config, in `bindings()`, which runs after die-dev's so the
  leader is set and the extension's commands exist (every binding has to name a command that
  exists, or DIE won't start, and `tests/config.rs` checks):

  ```rust
  keys.bind(State::Normal, "<leader>v", "fig-toggle");
  ```

- its faces, if it declares any the theme should style, in `theme::faces`.

Removing one is deleting the same lines, and the tests here that exercise it. Every dependency
from DIE or an extension is at `branch = "main"`, never a `rev`, so every repository in one build
resolves to the same die-core and the extensions' `Extension` is DIE's.

## Developing it

Without a dev config, DIE's crates and the extensions come from GitHub. To build against checkouts
on the machine instead, copy `dev-config.example.toml` to `.cargo/config.toml`, which git ignores.
Its paths are relative, for checkouts of DIE and the extensions beside this one (`../die`,
`../die-fig`, ...), which cargo reads from this repository's root; change any that's elsewhere:

```sh
mkdir -p .cargo
cp dev-config.example.toml .cargo/config.toml   # for checkouts beside this one
```

- `[patch."https://github.com/0xDEADBEE7/die"]` points every DIE crate the build takes (die,
  die-dev, die-core, die-render, die-vim, die-term, die-test-cap) at a DIE checkout;
- `[patch."https://github.com/uzaaft/libghostty-rs"]` points `libghostty-vt-sys` at the same
  checkout's `crates/die-term-sys`. `Cargo.toml` patches that package to DIE's git source, as
  DIE's own manifest does with a path, since a `[patch]` in DIE's manifest doesn't reach a crate
  outside DIE's workspace; a patch in a configuration file takes precedence over the manifest's
  for the same package, so the checkout's is the one built;
- one `[patch]` for each extension and die-draw points it at its checkout;
- `[env]` sets `DIE_FONT_DIR` to DIE's `fonts/` (`cargo xtask fonts` in DIE's checkout fetches
  them), where die-draw and the whiteboard find DIE's fonts: a repository outside DIE has no
  `fonts/` of its own.

Patch every repository in one build (this one and each extension's) to the same checkouts, or two
die-cores meet. Then, with the Rust that `rust-toolchain.toml` pins (1.96.0):

```sh
cargo fmt --check
cargo clippy --all-targets
DIE_UNSEEN=1 cargo test
```

The tests are die-dev's that exercised extensions before they left DIE: `\o` and a chat through
the config's keys, `\w` and a board drawn, exported and restored, `\m` and a page in the preview
stylesheet's colours, `\v` on CSVs, graphs and dashboards start to end, the jellyfish's greeting
through `die_dev::greeted`, and the theme. They run through die-core's harness and need no window;
`DIE_UNSEEN=1` keeps any that start DIE off the screen. **They need `DIE_FONT_DIR`**: a figure's
and a page's text fall back on DIE's fonts. The dashboard test runs again in a process with a HOME
of its own, so a live command's login shell reads no real profile. `tests/data/` holds the die-fig
examples the fig tests read (`graphs/services.json`, `dash/`, `live/`), copied from die-fig's
`examples/data` at the split.

## Licence

MIT or Apache-2.0, at your option: `LICENSE-MIT`, `LICENSE-APACHE`.
