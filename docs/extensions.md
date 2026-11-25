# Extensions

die-config is a binary crate named `die` that is DIE's own config, `apps/die-dev`, with the
extensions installed on top. die-dev has DIE's keys (leader `\`), mono-dark for core's faces, the
status line and the command line's arguments, and no extension. This crate adds what isn't part of
DIE: the extensions, their keys, their faces in mono-dark, and the jellyfish DIE opens on. It
doesn't copy die-dev: `main` is

```rust
die_dev::main_with(die_config::extra)
```

and `extra()` (`src/lib.rs`) is a `die_dev::Extra { extensions, bindings, theme, greeting }`. This
is the model DIE's scope sets out: a personal config as a binary crate, as in xmonad.

## What it installs

| Extension | What it is | Keys |
|---|---|---|
| [die-orchid-client](https://github.com/0xDEADBEE7/die-orchid-client) | emacs-orchid-client, ported: orchid's session browser and chats | `\o` in normal state |
| [die-whiteboard](https://github.com/0xDEADBEE7/die-whiteboard) | A whiteboard: freehand, shapes and arrows, saved as SVG, exported as PNG | `\w` in normal state |
| [die-jellyfish](https://github.com/0xDEADBEE7/die-jellyfish) | An ASCII jellyfish | the greeting; `:jellyfish` |
| [die-markdown](https://github.com/0xDEADBEE7/die-markdown) | A markdown file's rendered view, a page drawn as GitHub draws a README | `\m` in a markdown file and on its page |
| [die-fig](https://github.com/0xDEADBEE7/die-fig) | Figures from data: charts, network graphs, gantt charts and dashboards | `\v` in normal and visual state |

Each extension's README has its keys and how to use it. Here they're as die-dev had them before
the extensions left DIE: each with its defaults, but fig, which opens a `.fig` or `.fig.json` file
as its figure (`OpenAs::Figure`) and copies a yank in its detail panel through die-vim's kill ring
(`copying_with(die::vim::kill_new)`). [die-draw](https://github.com/0xDEADBEE7/die-draw), the
drawing library die-fig and die-markdown share, comes in through them.

## The greeting

A run you look at, with no file named, opens on the jellyfish in the directory DIE started in
(`die_dev::greeted` says when: never over a session's splits, a file, `--check` or a restart that
brought something back).

## The theme

die-dev's mono-dark styles core's faces only, since DIE reports a face a theme styles that no one
declared. `theme::faces` styles the extensions' faces in its palette (`die_dev::theme`): the
orchid faces no base face matches, the whiteboard dial's two, fig's chrome (its series and status
faces are left to fig's own colours) and every markdown face, after a markdown preview stylesheet.

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
