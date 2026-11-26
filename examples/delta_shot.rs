//! `delta_shot`: die-delta's README animation, as this config shows delta. It makes a repository
//! with an invented history in a temporary directory: a small tide-table program by three people,
//! with two merged branches, two tags and two files changed but not committed. It opens delta on
//! it with `\d`, through this config's keys, theme and faces, and plays [`TOUR`]: a few moves
//! down the commits, a search, a commit's files and two of its diffs, back, and the query
//! cleared, so the loop starts where it ends. Each step is drawn through DIE's renderer as
//! DIE.app draws it (Ioskeley Mono Term at 16 pt on a 2x display, with the status line of a DIE
//! started in your home directory), and the keys that made it are captioned at the right of the
//! empty command line. The screens go to `OUT_DIR/shots/` with `OUT_DIR/manifest.json`, which
//! [filmstrip](https://github.com/0xDEADBEE7/filmstrip) films into a lossless animated WebP:
//!
//! ```sh
//! cargo run --release --example delta_shot -- target/delta-tour
//! filmstrip target/delta-tour/manifest.json -o ../die-delta/docs/images/tour.webp
//! ```
//!
//! The people, dates and code are made up. git runs with a HOME of its own and nothing of this
//! machine's git config.

use die::core::{Cell, Glyph, Harness, Scene};
use die::render::driver::{Config as Font, Renderer};
use die_test_cap as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The window's text area, in cells: delta's 80 columns and a margin of 3 each side.
const COLS: usize = 86;
const ROWS: usize = 26;

const MIRA: (&str, &str) = ("Mira Okafor", "mira@example.com");
const TOMAS: (&str, &str) = ("Tomás Reyes", "tomas@example.com");
const JUN: (&str, &str) = ("Jun Park", "jun@example.com");

/// The steps: the keys typed, their caption, and how long the screen they make stays, in
/// milliseconds.
const TOUR: &[(&str, &str, u16)] = &[
    ("\\d", "\\d", 1300),
    ("j", "j", 260),
    ("j", "j", 260),
    ("k", "k", 500),
    ("/", "/", 300),
    ("s", "/s", 90),
    ("t", "/st", 90),
    ("a", "/sta", 90),
    ("t", "/stat", 90),
    ("i", "/stati", 90),
    ("o", "/statio", 90),
    ("n", "/station", 900),
    ("<Esc>", "Esc", 300),
    ("j", "j", 260),
    ("j", "j", 260),
    ("j", "j", 450),
    ("<CR>", "RET", 1400),
    ("j", "j", 260),
    ("j", "j", 260),
    ("j", "j", 450),
    ("<CR>", "RET", 1400),
    ("n", "n", 1100),
    ("b", "b", 600),
    ("b", "b", 700),
    ("d", "d", 1300),
];

fn main() {
    let out = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .expect("usage: delta_shot OUT_DIR");
    let _ = fs::remove_dir_all(out.join("shots"));
    fs::create_dir_all(out.join("shots")).unwrap();
    let root = std::env::temp_dir().join(format!("delta-shot-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let repo = Repo::new(&root);
    repo.history();

    let mut h = harness(&repo);
    let mut r = Renderer::new(&Font::term(16.0, 2.0)).unwrap();
    let (file, _) = h.open_text("main.rs", "").unwrap();
    h.set_directory(file, &repo.dir);
    h.set_window_size(COLS, ROWS);
    let mut steps = Vec::new();
    for (i, (keys, caption, hold)) in TOUR.iter().enumerate() {
        press(&mut h, keys);
        let mut scene = h.scene();
        caption_keys(&mut scene, caption);
        let shot = format!("shots/{i:03}.png");
        write_png(&out.join(&shot), &mut r, &scene);
        steps.push(format!(
            r#"{{"shot": "{shot}", "scale": 2, "hold": {hold}}}"#
        ));
    }
    // The tour ends where it starts but for the caption, so the loop is a cut.
    let manifest = format!(
        "{{\"scale\": 2, \"loop\": \"cut\", \"steps\": [\n  {}\n]}}\n",
        steps.join(",\n  ")
    );
    let path = out.join("manifest.json");
    fs::write(&path, manifest).unwrap();
    println!("{}: {} shots", path.display(), steps.len());
    let _ = fs::remove_dir_all(&root);
}

/// This config's editor, as `die` starts it, with delta finding git on this process's PATH and
/// the repository's HOME, and the status line DIE.app has, started in your home directory.
fn harness(repo: &Repo) -> Harness {
    let delta = die_delta::Delta::new(die_delta::Options {
        path: std::env::var_os("PATH"),
        env: repo.env(),
        ..die_delta::Options::default()
    })
    .copying_with(die::vim::kill_new);
    let config = die_config::config(vec![]);
    let mut exts = die::built_in();
    let others = config.extensions.into_iter();
    exts.extend(others.filter(|e| e.name() != "delta"));
    exts.push(Box::new(delta));
    let mut h = Harness::with_config(exts, config.core).unwrap();
    let home = &repo.home;
    h.set_status_text(Box::new(die_dev::status_text(home, Some(home))));
    h
}

/// Type `keys`, each of which must work, then wait for git.
fn press(h: &mut Harness, keys: &str) {
    for k in h.type_keys(keys).unwrap() {
        assert!(k.errors.is_empty(), "{}: {:?}", k.key, k.errors);
    }
    h.run_until_idle();
}

/// The keys that made this screen, dim, at the right of the command line, the frame's last row,
/// which is empty in the tour: a screencast's key caption, not part of DIE.
fn caption_keys(scene: &mut Scene, caption: &str) {
    let row = scene.rows - 1;
    let start = scene.cols - caption.chars().count() - 3;
    for (i, c) in caption.chars().enumerate() {
        let cell = &mut scene.cells[row * scene.cols + start + i];
        *cell = Cell::new(Glyph::Char(c), die_dev::theme::FG_MID, cell.bg);
    }
}

/// `scene` drawn as DIE.app draws it, saved as a PNG.
fn write_png(path: &Path, r: &mut Renderer, scene: &Scene) {
    let (w, h) = r.cell().grid(scene.cols, scene.rows);
    let image = r.render(scene, w, h).unwrap();
    let rgb: Vec<u8> = image
        .bgra
        .chunks_exact(4)
        .flat_map(|p| [p[2], p[1], p[0]])
        .collect();
    let mut bytes = Vec::new();
    let mut png = png::Encoder::new(&mut bytes, w, h);
    png.set_color(png::ColorType::Rgb);
    png.set_depth(png::BitDepth::Eight);
    png.set_compression(png::Compression::Fast);
    let mut writer = png.write_header().unwrap();
    writer.write_image_data(&rgb).unwrap();
    writer.finish().unwrap();
    fs::write(path, bytes).unwrap();
}

/// A repository, `harbor`, with a HOME of its own.
struct Repo {
    dir: PathBuf,
    home: PathBuf,
}

impl Repo {
    fn new(root: &Path) -> Repo {
        let (dir, home) = (root.join("harbor"), root.join("home"));
        fs::create_dir_all(&dir).unwrap();
        fs::create_dir_all(&home).unwrap();
        let repo = Repo { dir, home };
        repo.git(
            MIRA,
            "2026-09-01T09:00:00+01:00",
            &["init", "-q", "-b", "main"],
        );
        repo
    }

    /// git's environment: nothing of this machine's config.
    fn env(&self) -> Vec<(std::ffi::OsString, std::ffi::OsString)> {
        vec![
            ("HOME".into(), self.home.clone().into()),
            ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
            ("GIT_CONFIG_GLOBAL".into(), "/dev/null".into()),
        ]
    }

    /// git in the repository, as `who` at `date`.
    fn git(&self, who: (&str, &str), date: &str, args: &[&str]) {
        let mut cmd = Command::new("git");
        cmd.args(args).current_dir(&self.dir).envs(self.env());
        for name in ["GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE"] {
            cmd.env_remove(name);
        }
        for (k, v) in [("NAME", who.0), ("EMAIL", who.1), ("DATE", date)] {
            cmd.env(format!("GIT_AUTHOR_{k}"), v);
            cmd.env(format!("GIT_COMMITTER_{k}"), v);
        }
        let out = cmd.output().unwrap();
        let said = String::from_utf8_lossy(&out.stderr);
        assert!(out.status.success(), "git {args:?}: {said}");
    }

    fn write(&self, path: &str, text: &str) {
        let path = self.dir.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    fn commit(&self, who: (&str, &str), date: &str, message: &str) {
        self.git(who, date, &["add", "-A"]);
        self.git(who, date, &["commit", "-q", "-m", message]);
    }

    fn merge(&self, who: (&str, &str), date: &str, branch: &str, message: &str) {
        self.git(who, date, &["checkout", "-q", "main"]);
        self.git(
            who,
            date,
            &["merge", "-q", "--no-ff", branch, "-m", message],
        );
    }

    /// Fifteen commits down main's first parents, two of them merges, and two files changed.
    fn history(&self) {
        self.write("Cargo.toml", CARGO_1);
        self.write("README.md", README_1);
        self.write("src/main.rs", MAIN_1);
        self.write("src/tide.rs", TIDE_1);
        self.write("stations.toml", STATIONS_1);
        self.commit(
            MIRA,
            "2026-09-01T09:00:00+01:00",
            "first: a tide table for one station",
        );
        self.write("src/tide.rs", TIDE_2);
        let d = "2026-09-02T18:10:00+01:00";
        self.commit(MIRA, d, "tide: heights from the harmonic constants");
        self.write("README.md", README_2);
        self.commit(JUN, "2026-09-03T12:30:00+01:00", "readme: how to run it");
        self.write(".github/workflows/ci.yml", CI);
        self.commit(
            TOMAS,
            "2026-09-04T16:00:00+01:00",
            "ci: build and test on every push",
        );

        let d = "2026-09-08T11:15:00+01:00";
        fs::remove_file(self.dir.join("stations.toml")).unwrap();
        self.write("data/stations.toml", STATIONS_2);
        self.write("Cargo.toml", CARGO_2);
        self.write("docs/stations.md", STATIONS_DOC);
        self.write("src/main.rs", MAIN_2);
        self.write("src/station.rs", STATION_1);
        let message = "station: stations from a TOML file\n\n\
                       Each station has a name, a position and its harmonic constants.\n\
                       The table reads them at start: no rebuild for a new station.";
        self.commit(TOMAS, d, message);
        self.git(TOMAS, d, &["tag", "v0.1.0"]);

        let d = "2026-09-12T10:05:00+01:00";
        self.git(JUN, d, &["checkout", "-q", "-b", "ics"]);
        self.write("src/ics.rs", ICS_1);
        self.commit(JUN, d, "ics: export a week of tides as a calendar");
        self.write("src/ics.rs", ICS_2);
        self.commit(
            JUN,
            "2026-09-14T16:30:00+01:00",
            "ics: one event per high water",
        );
        self.git(MIRA, d, &["checkout", "-q", "main"]);
        self.write("src/tide.rs", TIDE_3);
        let d = "2026-09-15T08:50:00+01:00";
        self.commit(MIRA, d, "tide: round heights to the centimetre");
        let d = "2026-09-16T13:20:00+01:00";
        self.merge(TOMAS, d, "ics", "Merge 'ics': tides as calendar events");

        self.write("src/main.rs", MAIN_3);
        self.commit(
            MIRA,
            "2026-09-17T10:40:00+01:00",
            "main: --days and --ics flags",
        );
        self.write("src/station.rs", STATION_2);
        let d = "2026-09-18T15:45:00+01:00";
        self.commit(JUN, d, "station: search by name, in any case");
        self.write("src/tide.rs", TIDE_4);
        let d = "2026-09-22T09:05:00+01:00";
        self.commit(TOMAS, d, "tide: times in the station's own zone");

        let d = "2026-09-24T14:00:00+01:00";
        self.git(TOMAS, d, &["checkout", "-q", "-b", "newhaven"]);
        self.write("data/stations.toml", STATIONS_3);
        self.commit(TOMAS, d, "station: Newhaven, from the 2025 survey");
        let d = "2026-09-25T11:30:00+01:00";
        self.merge(MIRA, d, "newhaven", "Merge 'newhaven': a second station");

        self.write("README.md", README_3);
        let d = "2026-09-29T17:20:00+01:00";
        self.commit(MIRA, d, "readme: install with cargo install");
        self.write("README.md", README_4);
        let d = "2026-09-30T09:10:00+01:00";
        self.commit(JUN, d, "readme: a week at Dover, as it prints");
        self.write("src/tide.rs", TIDE_5);
        let d = "2026-10-02T16:45:00+01:00";
        self.commit(MIRA, d, "tide: steps of ten minutes");
        self.git(MIRA, d, &["tag", "v0.2.0"]);

        self.write("src/tide.rs", TIDE_6);
        self.write("README.md", README_5);
    }
}

const CARGO_1: &str = "[package]\nname = \"harbor\"\nversion = \"0.1.0\"\nedition = \"2024\"\n";

const CARGO_2: &str = "[package]\nname = \"harbor\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n\
                       [dependencies]\ntoml = \"0.9\"\n";

const README_1: &str = "# harbor\n\nA tide table for one station.\n";

const README_2: &str = "# harbor\n\nA tide table for one station.\n\n    cargo run -- dover\n";

const README_3: &str = "# harbor\n\nTide tables from harmonic constants.\n\n\
                        cargo install --path .\n    harbor dover\n";

const README_4: &str = "# harbor\n\nTide tables from harmonic constants.\n\n\
                        cargo install --path .\n    harbor dover --days 7\n\n\
                        Mon 21  02:14  6.1 m   08:31  0.9 m   14:39  6.3 m\n";

const README_5: &str = "# harbor\n\nTide tables from harmonic constants, for any station\n\
                        in `data/stations.toml`.\n\n\
                        cargo install --path .\n    harbor dover --days 7\n\n\
                        Mon 21  02:14  6.1 m   08:31  0.9 m   14:39  6.3 m\n\n\
                        `--ics` writes the week as calendar events.\n";

const CI: &str = "on: push\njobs:\n  test:\n    runs-on: ubuntu-latest\n    steps:\n      \
                  - uses: actions/checkout@v4\n      - run: cargo test\n";

const MAIN_1: &str = "mod tide;\n\nfn main() {\n    let days = 7;\n    \
                      for (hour, height) in tide::table(days) {\n        \
                      println!(\"{hour}  {height:.1} m\");\n    }\n}\n";

const MAIN_2: &str = "mod station;\nmod tide;\n\nfn main() {\n    \
                      let name = std::env::args().nth(1).unwrap_or(\"Dover\".into());\n    \
                      let text = std::fs::read_to_string(\"data/stations.toml\").unwrap();\n    \
                      let stations = station::load(&text).unwrap();\n    \
                      let here = station::find(&stations, &name).unwrap();\n    \
                      let days = 7;\n    \
                      for (hour, height) in tide::table(here, days) {\n        \
                      println!(\"{hour}  {height:.1} m\");\n    }\n}\n";

const MAIN_3: &str = "mod ics;\nmod station;\nmod tide;\n\nfn main() {\n    \
                      let args = Args::parse();\n    \
                      let text = std::fs::read_to_string(\"data/stations.toml\").unwrap();\n    \
                      let stations = station::load(&text).unwrap();\n    \
                      let here = station::find(&stations, &args.name).unwrap();\n    \
                      let table = tide::table(here, args.days);\n    \
                      if args.ics {\n        print!(\"{}\", ics::calendar(&table));\n    } else {\n        \
                      tide::print(&table);\n    }\n}\n";

const TIDE_1: &str = "/// A week of heights, an hour apart.\n\
                      pub fn table(days: u32) -> Vec<(u32, f64)> {\n    \
                      (0..days * 24).map(|hour| (hour, 3.0)).collect()\n}\n";

const TIDE_2: &str = "/// One constituent: its speed in degrees an hour, amplitude and phase.\n\
                      pub struct Constituent {\n    pub speed: f64,\n    pub amplitude: f64,\n    \
                      pub phase: f64,\n}\n\n\
                      /// The height at `hour`: the mean level plus each constituent's wave.\n\
                      pub fn height(mean: f64, waves: &[Constituent], hour: f64) -> f64 {\n    \
                      let wave = |c: &Constituent| c.amplitude * (c.speed * hour - c.phase).cos();\n    \
                      mean + waves.iter().map(wave).sum::<f64>()\n}\n";

const TIDE_3: &str = "/// One constituent: its speed in degrees an hour, amplitude and phase.\n\
                      pub struct Constituent {\n    pub speed: f64,\n    pub amplitude: f64,\n    \
                      pub phase: f64,\n}\n\n\
                      /// The height at `hour`, to the centimetre.\n\
                      pub fn height(mean: f64, waves: &[Constituent], hour: f64) -> f64 {\n    \
                      let wave = |c: &Constituent| c.amplitude * (c.speed * hour - c.phase).cos();\n    \
                      let metres = mean + waves.iter().map(wave).sum::<f64>();\n    \
                      (metres * 100.0).round() / 100.0\n}\n";

const TIDE_4: &str = "/// One constituent: its speed in degrees an hour, amplitude and phase.\n\
                      pub struct Constituent {\n    pub speed: f64,\n    pub amplitude: f64,\n    \
                      pub phase: f64,\n}\n\n\
                      /// The height at `hour`, to the centimetre.\n\
                      pub fn height(mean: f64, waves: &[Constituent], hour: f64) -> f64 {\n    \
                      let wave = |c: &Constituent| c.amplitude * (c.speed * hour - c.phase).cos();\n    \
                      let metres = mean + waves.iter().map(wave).sum::<f64>();\n    \
                      (metres * 100.0).round() / 100.0\n}\n\n\
                      /// A time in the station's zone, as the table prints it.\n\
                      pub fn local(utc: i64, zone: &Zone) -> String {\n    \
                      zone.format(utc + zone.offset_at(utc))\n}\n";

const TIDE_5: &str = "/// One constituent: its speed in degrees an hour, amplitude and phase.\n\
                      pub struct Constituent {\n    pub speed: f64,\n    pub amplitude: f64,\n    \
                      pub phase: f64,\n}\n\n\
                      /// The height at `hour`, to the centimetre.\n\
                      pub fn height(mean: f64, waves: &[Constituent], hour: f64) -> f64 {\n    \
                      let wave = |c: &Constituent| c.amplitude * (c.speed * hour - c.phase).cos();\n    \
                      let metres = mean + waves.iter().map(wave).sum::<f64>();\n    \
                      (metres * 100.0).round() / 100.0\n}\n\n\
                      /// Heights every ten minutes, so the turns land within five.\n\
                      pub fn steps(days: u32) -> impl Iterator<Item = f64> {\n    \
                      (0..days * 24 * 6).map(|step| step as f64 / 6.0)\n}\n";

const TIDE_6: &str = "/// One constituent: its speed in degrees an hour, amplitude and phase.\n\
                      pub struct Constituent {\n    pub speed: f64,\n    pub amplitude: f64,\n    \
                      pub phase: f64,\n}\n\n\
                      /// The height at `hour`, to the centimetre.\n\
                      pub fn height(mean: f64, waves: &[Constituent], hour: f64) -> f64 {\n    \
                      let wave = |c: &Constituent| c.amplitude * (c.speed * hour - c.phase).cos();\n    \
                      let metres = mean + waves.iter().map(wave).sum::<f64>();\n    \
                      (metres * 100.0).round() / 100.0\n}\n\n\
                      /// Heights every five minutes, so the turns land within three.\n\
                      pub fn steps(days: u32) -> impl Iterator<Item = f64> {\n    \
                      (0..days * 24 * 12).map(|step| step as f64 / 12.0)\n}\n\n\
                      /// The highs: steps higher than both neighbours.\n\
                      pub fn highs(heights: &[f64]) -> Vec<usize> {\n    \
                      (1..heights.len() - 1).collect()\n}\n";

const STATIONS_1: &str = "[[station]]\nname = \"Dover\"\nlatitude = 51.114\nlongitude = 1.322\n\
                          constituents = [[28.984, 2.25, 330.0], [30.0, 0.72, 13.0]]\n";

const STATIONS_2: &str = "# Each station's harmonic constants: [speed, amplitude, phase].\n\n\
                          [[station]]\nname = \"Dover\"\nlatitude = 51.114\nlongitude = 1.322\n\
                          constituents = [[28.984, 2.25, 330.0], [30.0, 0.72, 13.0]]\n";

const STATIONS_3: &str = "# Each station's harmonic constants: [speed, amplitude, phase].\n\n\
                          [[station]]\nname = \"Dover\"\nlatitude = 51.114\nlongitude = 1.322\n\
                          constituents = [[28.984, 2.25, 330.0], [30.0, 0.72, 13.0]]\n\n\
                          [[station]]\nname = \"Newhaven\"\nlatitude = 50.781\nlongitude = 0.057\n\
                          constituents = [[28.984, 2.21, 315.0], [30.0, 0.69, 2.0]]\n";

const STATIONS_DOC: &str = "# Stations\n\nEach `[[station]]` in `data/stations.toml` has a\n\
                            `name`, a `latitude` and `longitude` in degrees, and its\n\
                            `constituents` as `[speed, amplitude, phase]`.\n";

const STATION_1: &str = r#"//! Tide stations: where they are, and the constants that predict them.

use crate::tide::Constituent;

pub struct Station {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub constituents: Vec<Constituent>,
}

/// The station called `name`.
pub fn find<'a>(stations: &'a [Station], name: &str) -> Option<&'a Station> {
    stations.iter().find(|s| s.name == name)
}

/// The stations in a TOML file, in the order it lists them.
pub fn load(text: &str) -> Result<Vec<Station>, String> {
    let file: toml::Table = text.parse().map_err(|e| e.to_string())?;
    let rows = file.get("station").and_then(|s| s.as_array());
    rows.ok_or("no [[station]] tables")?.iter().map(station).collect()
}

/// One `[[station]]` table.
fn station(row: &toml::Value) -> Result<Station, String> {
    let field = |key: &str| row.get(key).ok_or(format!("no {key}"));
    let text = |key: &str| field(key)?.as_str().ok_or("not text");
    let degrees = |key: &str| field(key)?.as_float().ok_or("not degrees");
    Ok(Station {
        name: text("name")?.to_string(),
        latitude: degrees("latitude")?,
        longitude: degrees("longitude")?,
        constituents: vec![],
    })
}
"#;

const STATION_2: &str = r#"//! Tide stations: where they are, and the constants that predict them.

use crate::tide::Constituent;

pub struct Station {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub constituents: Vec<Constituent>,
}

/// The station called `name`, in any case: `dover` finds Dover.
pub fn find<'a>(stations: &'a [Station], name: &str) -> Option<&'a Station> {
    stations.iter().find(|s| s.name.eq_ignore_ascii_case(name.trim()))
}

/// The stations in a TOML file, in the order it lists them.
pub fn load(text: &str) -> Result<Vec<Station>, String> {
    let file: toml::Table = text.parse().map_err(|e| e.to_string())?;
    let rows = file.get("station").and_then(|s| s.as_array());
    rows.ok_or("no [[station]] tables")?.iter().map(station).collect()
}

/// One `[[station]]` table.
fn station(row: &toml::Value) -> Result<Station, String> {
    let field = |key: &str| row.get(key).ok_or(format!("no {key}"));
    let text = |key: &str| field(key)?.as_str().ok_or("not text");
    let degrees = |key: &str| field(key)?.as_float().ok_or("not degrees");
    Ok(Station {
        name: text("name")?.trim().to_string(),
        latitude: degrees("latitude")?,
        longitude: degrees("longitude")?,
        constituents: vec![],
    })
}
"#;

const ICS_1: &str = "//! A week of tides as an iCalendar file.\n\n\
                     /// One all-day event a day, its summary the day's high waters.\n\
                     pub fn calendar(days: &[(String, String)]) -> String {\n    \
                     let event = |(date, highs): &(String, String)| event(date, highs);\n    \
                     wrap(days.iter().map(event).collect())\n}\n";

const ICS_2: &str = "//! A week of tides as an iCalendar file.\n\n\
                     /// One ten-minute event per high water, its summary the height.\n\
                     pub fn calendar(highs: &[(String, f64)]) -> String {\n    \
                     let event = |(start, height): &(String, f64)| event(start, *height);\n    \
                     wrap(highs.iter().map(event).collect())\n}\n";
