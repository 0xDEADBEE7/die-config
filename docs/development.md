# Developing die-config

## Building against local checkouts

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
die-cores meet.

## Checks and tests

With the Rust that `rust-toolchain.toml` pins (1.96.0):

```sh
cargo fmt --check
cargo clippy --all-targets
DIE_UNSEEN=1 cargo test
```

The tests are die-dev's that exercised extensions before they left DIE: `\o` and a chat through
the config's keys, `\w` and a board drawn, exported and restored, `\m` and a page in the preview
stylesheet's colours, `\v` on CSVs, graphs and dashboards start to end, the jellyfish's greeting
through `die_dev::greeted`, and the theme. They run through die-core's harness and need no window;
`DIE_UNSEEN=1` keeps any that start DIE off the screen.

**They need `DIE_FONT_DIR`**: a figure's and a page's text fall back on DIE's fonts. The dashboard
test runs again in a process with a HOME of its own, so a live command's login shell reads no real
profile. `tests/data/` holds the die-fig examples the fig tests read (`graphs/services.json`,
`dash/`, `live/`), copied from die-fig's `examples/data` at the split.

## Pictures

`delta_shot` draws die-delta's README pictures as this config shows delta. It makes a repository
with an invented history in a temporary directory, opens delta on it with `\d`, and draws the
commits, a commit's files and a diff through DIE's renderer at 2x, as DIE.app does, with the
status line of a DIE started in your home directory. It needs a GPU and `DIE_FONT_DIR`:

```sh
cargo run --release --example delta_shot -- ../die-delta/docs/images
```
