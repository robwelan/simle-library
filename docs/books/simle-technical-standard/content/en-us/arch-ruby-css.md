# Architecture: Phonetic Notation (`base-ruby.css`)

This document specifies the technical implementation of phonetic annotations
(Ruby) used for Pinyin, Furigana, or specialized phonetic markers. The
`base-ruby.css` ensures that pronunciation guides are legible, visually
balanced, and non-disruptive to the reading flow.

## 1. Architectural Philosophy

Wrapped in the `@layer ruby` declaration, this logic focuses on the
micro-typography of language learning. Its primary goal is to provide a
"Phonetic Overlay" that assists the reader without causing uneven line spacing
or "jumping" text.

## 2. Technical Breakdown by Section

### Section 1: Ruby Container Logic

The `ruby` element acts as the wrapper for both the base character and the
phonetic guide:

- **Alignment:** `ruby-align: center` ensures that phonetics are perfectly
  centered over the character they describe.
- **Positioning:** `ruby-position: over` sets a project-wide standard that
  phonetics appear above the text, providing a consistent "scanning" pattern for
  students.
- **Line Stability:** `line-break: loose` is applied to prevent the presence of
  phonetics from forcing awkward line breaks or uneven vertical gaps between
  paragraphs.

### Section 2: The Phonetic Text (`rt`)

This section styles the "Ruby Text" itself to ensure it is subservient to the
main text while remaining functional:

- **Font Selection:** We use `system-ui, sans-serif` for phonetics. This
  provides a clean, neutral contrast to our primary IBM Plex or STIX fonts,
  making the pronunciation guide instantly recognizable as a "layer."
- **Scale and Spacing:** The size is reduced to `0.55em` with increased
  `letter-spacing` to maintain legibility at small sizes.
- **Color Logic:** Utilizing `color-mix`, the phonetics are rendered at 65%
  opacity. This makes them subtle enough to ignore during fluent reading but
  clear enough to use when needed.
- **Layout Safety:** A small `padding-inline` ensures that wide phonetic strings
  (like long Pinyin clusters) do not visually collide with neighboring
  characters.

### Section 3: Alternative Phonetics (`phonetic-alt`)

This class provides support for "emphasis" or "special" phonetic markers (e.g.,
German phonetic distinctions). It utilizes a CSS variable
(`--book-accent-color`) to allow specific books to highlight certain
pronunciation rules in their own brand colors.

### Section 4: Interaction Control (`.hide-phonetics`)

This section provides a "toggle" mechanism. By applying the `.hide-phonetics`
class to a parent element or the body, all `rt` elements are removed from the
display. This allows for a "Mastery Mode" where the student can hide the crutch
of Pinyin to test their knowledge of the characters.

## 3. Implementation Requirements

This file is specialized for language-learning books and technical standards
involving phonetics.

- **File Name:** `arch-ruby.md`
- **Storage Location:** `shared-assets/styles/base-ruby.css`
- **Documentation Root:**
  `books/simple-technical-standard/content/en-us/arch-ruby.md`
