# Building DIE.app

In a DIE checkout, `cargo xtask app` builds DIE.app from the first config it finds:

1. `--config DIR`;
2. else `$DIE_CONFIG`, if it's set and not empty;
3. else `die-config` beside the checkout (`../die-config`), if it has a `Cargo.toml`;
4. else DIE's own `apps/die-dev`, which has no extension.

So with this repository checked out as `die-config` beside a DIE checkout, a plain
`cargo xtask app` builds it. A config that's named has to be a crate, or the build stops and says
so.

Cargo runs in the config's own directory (`cargo build --release --bin die`), so this
repository's `.cargo/config.toml` applies, and the binary is taken from the target directory
`cargo metadata` names there (so `CARGO_TARGET_DIR` works).

The build isn't `--locked`, though this repository has a `Cargo.lock`: only DIE's own workspace,
whose lock DIE's commits keep, builds locked. Nothing keeps this one current, as DIE's crates and
the extensions move on `main` with their own commits, and with the dev config reach it by path
patches to checkouts, so a build brings it up to date, changing only what it must, rather than
stopping at a dependency one of them added or bumped.

`--home DIR` puts the bundle at `DIR/Applications/DIE.app` instead of `~/Applications/DIE.app`.

## `:rebuild`

The bundle records where it came from: `Contents/Resources/checkout` names the DIE checkout and
`Contents/Resources/config` this config, as an absolute path. `:rebuild` in DIE runs
`cargo xtask app --config <that config> --home <that home>` in that checkout, so DIE is built from
this config again, whichever file you're in. If the config isn't a crate any more, `:rebuild` says
so and builds nothing.
