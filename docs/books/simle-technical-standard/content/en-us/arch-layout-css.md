# Architecture: Global Layout (`base-layout.css`)

This document specifies the layout engine for the `simple-technical-standard`.
The `base-layout.css` is responsible for handling parallel text synchronization,
bidirectional support (RTL), and modern phonetic (Ruby) notations.

## 1. Architectural Philosophy

Wrapped in the `@layer base` declaration, this logic provides the "middle-tier"
structure. It sits between the **Reset** (foundational) and the **Book Layer**
(thematic). It uses CSS Grid to ensure that multi-column parallel translations
remain perfectly aligned regardless of text length.

## 2. Technical Breakdown by Section

### Section 1: The Page Layout Grid

The `.page-layout` class is the engine for parallel text synchronization:

- **Grid Auto-Flow:** By using `grid-auto-flow: column`, the system
  automatically scales. If you inject two languages (English/Chinese), it
  creates two columns; if you inject three, it creates three.
- **Equal Distribution:** `grid-auto-columns: 1fr` ensures that every language
  column occupies exactly the same amount of horizontal space, maintaining a
  clean, balanced aesthetic for comparative study.

### Section 2: Logical Bidirectional Support

This section handles the transition between Western (LTR) and Middle Eastern
(RTL) scripts:

- **Directional Switching:** The `[dir="rtl"]` selector applies global direction
  changes when an Arabic locale is detected.
- **Logical Alignment:** Using `text-align: start` ensures that text is
  right-aligned for Arabic and left-aligned for English without requiring
  separate alignment classes.

### Section 3: Modern Ruby (Phonetic) Styling

This section provides advanced support for Pinyin (Chinese) and Furigana
(Japanese):

- **Positioning:** `ruby-position: over` ensures that phonetic notations
  consistently appear above the base character, preventing "line-height
  jumping."
- **Visual Hierarchy:** Phonetic text (`rt`) is sized to `0.65em` to remain
  legible but subordinate to the main text.
- **Modern Color Mixing:** `color-mix` is used to subtly fade the phonetic
  notation (70% opacity) without needing a fixed hex code, allowing it to adapt
  to any theme color.
- **UX Protection:** `user-select: none` is applied to phonetic characters so
  that when a reader copies a sentence, they only copy the primary text—not the
  phonetic "noise."

## 3. Implementation Requirements

This file should be loaded after the reset and typography to provide the
structural scaffolding for the content.

- **File Name:** `arch-layout.md`
- **Storage Location:** `shared-assets/styles/base-layout.css`
- **Documentation Root:**
  `books/simple-technical-standard/content/en-us/arch-layout.md`
