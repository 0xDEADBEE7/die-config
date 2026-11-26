# die-config

A ready-to-use [DIE](https://github.com/0xDEADBEE7/die): DIE's own config with a set of
extensions, their keys and their colours on top.

## What it does

- Starts from `die-dev`, DIE's base config: its keys (leader `\`), theme and status line.
- Adds seven extensions, each from its own repository.
- Binds each extension to a short leader key.
- Styles the extensions in the same mono-dark theme.
- Opens on an ASCII jellyfish when DIE starts with no file.

## What it installs

- [die-orchid-client](https://github.com/0xDEADBEE7/die-orchid-client): session browser and chats (`\o`)
- [die-whiteboard](https://github.com/0xDEADBEE7/die-whiteboard): freehand, shapes and arrows, saved as SVG (`\w`)
- [die-jellyfish](https://github.com/0xDEADBEE7/die-jellyfish): the start-up jellyfish (`:jellyfish`)
- [die-markdown](https://github.com/0xDEADBEE7/die-markdown): a rendered view of a markdown file (`\m`)
- [die-fig](https://github.com/0xDEADBEE7/die-fig): charts, graphs, gantt charts and dashboards (`\v`)
- [die-scribble](https://github.com/0xDEADBEE7/die-scribble): search your markdown notes by name and open one (`\s`)
- [die-delta](https://github.com/0xDEADBEE7/die-delta): your git history, its commits, the files each changed and their diffs (`\d`)

## Add or remove an extension

Add its line to `Cargo.toml` and its entry to `extensions()` in `src/lib.rs`:

```toml
die-fig = { git = "https://github.com/0xDEADBEE7/die-fig", branch = "main" }
```

```rust
Box::new(die_fig::Fig::default()),
```

Bind its keys in `bindings()` if it needs any. To remove one, delete the same lines.
Details: [docs/extensions.md](docs/extensions.md).

## Build

With this repository checked out as `die-config` beside a DIE checkout, run this in DIE:

```sh
cargo xtask app
```

## More

- [docs/extensions.md](docs/extensions.md): the extensions, the greeting, the theme, adding and removing
- [docs/building.md](docs/building.md): how DIE.app is built from this config, and `:rebuild`
- [docs/development.md](docs/development.md): building against local checkouts, and the tests

## Licence

MIT or Apache-2.0, at your option: `LICENSE-MIT`, `LICENSE-APACHE`.
