//! The extensions' faces in mono-dark, the owner's Emacs theme: die-dev's `mono_dark` styles core's
//! faces only, as DIE reports a face a theme styles that no one declared, so each extension's faces
//! are styled here, in mono-dark's palette (`die_dev::theme`). They're as DIE's config had them
//! before the extensions left it, each noting the Emacs face or stylesheet rule it comes from.

use die::core::{Rgb, Style, Theme, Underline};
use die_dev::theme::{BG_HL, BG_SUBTLE, BORDER, FG, FG_BRIGHT, FG_DIM, FG_MID, RED};

/// fig's gridlines: a step off the ground, under the border's grey.
const GRID: Rgb = Rgb::hex("#2a2a2a");

/// highlight.js's grayscale-dark, which the owner's Emacs markdown preview colours code blocks with
/// (`markdown-xwidget`'s `highlight_css/grayscale-dark.css`): its base16 greys by the role it gives
/// each. Comments are base03's #525252 lifted to #545454, the least that reads at 2:1 on a page's
/// code panel, as `Palette::mono_dark` has it.
const CODE_COMMENT: Rgb = Rgb::hex("#545454"); // base03
const CODE_FUNCTION: Rgb = Rgb::hex("#686868"); // base0D: functions, headings
const CODE_META: Rgb = Rgb::hex("#5e5e5e"); // base0F
const CODE_KEYWORD: Rgb = Rgb::hex("#747474"); // base0E: keywords, types
const CODE_VARIABLE: Rgb = Rgb::hex("#7c7c7c"); // base08: variables, tags, deletions
const CODE_STRING: Rgb = Rgb::hex("#8e8e8e"); // base0B: strings, additions
const CODE_CONSTANT: Rgb = Rgb::hex("#999999"); // base09: numbers, literals, attributes

/// This config's theme: die-dev's mono-dark with the extensions' faces.
pub fn mono_dark() -> Theme {
    faces(die_dev::theme::mono_dark())
}

/// `theme` with the extensions' faces styled: the orchid faces whose Emacs faces no base face
/// matches, the whiteboard dial's two, fig's chrome, and the markdown view's, from the stylesheet
/// the owner's Emacs preview uses (`markdown-xwidget`'s `resources/mono-dark.css`). A markdown page
/// takes its colours from the same faces, and `tests/markdown.rs` holds them to the stylesheet's.
/// fig's series and status faces are left unstyled, so a figure's data takes fig's validated
/// colours for a dark ground.
pub fn faces(theme: Theme) -> Theme {
    let s = Style::new;
    theme
        // font-lock-string-face, which emacs-orchid-client's chat header shows its workspace in.
        // Its other faces come out as mono-dark has them from the base faces they inherit.
        .face("orchid-chat-header-workspace", s().fg(FG_MID))
        // seek's search line, `(:background "#3a3a3a")`, which the session browser draws while
        // you type a query.
        .face("orchid-seek-search", s().bg(BORDER))
        // The whiteboard's tool dial, in the grey and the one accent mono-dark already has:
        // its names, readout keys and line art in the mid grey; the hovered name's chip, the
        // hovered stretch of its ring, the needle's dot and the readout's value in the red.
        .face("whiteboard-dial", s().fg(FG_MID))
        .face("whiteboard-dial-accent", s().fg(RED))
        // fig's chrome, quiet so the data is the only loud thing: gridlines a step off the
        // ground, the zero line and the hovered point's ring in the border's grey, ticks in the
        // mid grey, labels and titles as text and bold text are, the card on the subtle ground,
        // and the cursor's chip in the brightest grey.
        .face("fig-grid", s().fg(GRID))
        .face("fig-baseline", s().fg(BORDER))
        .face("fig-tick", s().fg(FG_MID))
        .face("fig-text", s().fg(FG))
        .face("fig-strong", s().fg(FG_BRIGHT))
        .face("fig-tooltip", s().fg(BG_SUBTLE))
        .face("fig-ring", s().fg(BORDER))
        .face("fig-cursor", s().fg(FG_BRIGHT))
        // h1 and h2: bright and bold over a border-coloured line. In the view in place a cell
        // has one size of letter, so the two lines stand in for 2em, and one for 1.5em.
        .face(
            "markdown-h1",
            heading(FG_BRIGHT).underline(Underline::Double),
        )
        .face(
            "markdown-h2",
            heading(FG_BRIGHT).underline(Underline::Single),
        )
        // h3 to h5 are bright and bold there too, each smaller, as a page draws them; h6 is
        // the dimmer one, as GitHub's is.
        .face("markdown-h3", heading(FG_BRIGHT))
        .face("markdown-h4", heading(FG_BRIGHT))
        .face("markdown-h5", heading(FG_BRIGHT))
        .face("markdown-h6", s().fg(FG_MID))
        // strong, em and del
        .face("markdown-bold", s().fg(FG_BRIGHT).bold())
        .face("markdown-italic", s().fg(FG_MID).italic())
        .face("markdown-strike", s().fg(FG_DIM).strike())
        // code, and pre with its own text colour. Inline code is on the highlight grey, a
        // shade over a block's, so it reads as a pill on a table's striped row too.
        .face("markdown-code", s().fg(FG_MID).bg(BG_HL))
        .face("markdown-code-block", s().bg(BG_SUBTLE))
        // a
        .face(
            "markdown-link",
            s().fg(RED).bold().underline(Underline::Single),
        )
        // blockquote's text; its bar, hr and a table's borders are the border colour
        .face("markdown-quote", s().fg(FG_MID).italic())
        .face("markdown-rule", s().fg(BORDER))
        // th, without its background: a row's padding is drawn by its borders
        .face("markdown-table-header", s().fg(FG_BRIGHT).bold())
        // What a browser draws in the text's colour, dimmed here so the text leads: bullets,
        // numbers and checkboxes; and what it doesn't draw at all: HTML, a fence's language.
        .face("markdown-marker", s().fg(FG_MID))
        .face("markdown-meta", s().fg(FG_DIM))
        .face("markdown-alert", s().fg(FG_BRIGHT).bold())
        // A page's code blocks, in grayscale-dark's greys; what's none of these is the block's
        // own text, the default's.
        .face("markdown-code-keyword", s().fg(CODE_KEYWORD))
        .face("markdown-code-string", s().fg(CODE_STRING))
        .face("markdown-code-comment", s().fg(CODE_COMMENT))
        .face("markdown-code-number", s().fg(CODE_CONSTANT))
        .face("markdown-code-function", s().fg(CODE_FUNCTION))
        .face("markdown-code-type", s().fg(CODE_KEYWORD))
        .face("markdown-code-constant", s().fg(CODE_CONSTANT))
        .face("markdown-code-meta", s().fg(CODE_META))
        .face("markdown-code-variable", s().fg(CODE_VARIABLE))
        .face("markdown-code-tag", s().fg(CODE_VARIABLE))
        .face("markdown-code-attr", s().fg(CODE_CONSTANT))
        .face("markdown-code-inserted", s().fg(CODE_STRING))
        .face("markdown-code-deleted", s().fg(CODE_VARIABLE))
        .face("markdown-code-heading", s().fg(CODE_FUNCTION))
}

/// A heading: bold in `fg`, and any underline in the border's colour, as the stylesheet's
/// `border-bottom` is.
fn heading(fg: Rgb) -> Style {
    Style::new().fg(fg).bold().underline_color(BORDER)
}
