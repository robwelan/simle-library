# Architecture: Global Reset (`base-reset.css`)

This document serves as the technical specification for the structural
foundation of the project. The `base-reset.css` is the primary "leveling" layer,
designed to neutralize browser inconsistencies and establish a
**Modern Logical** layout for all multilingual and technical content.

## 1. The Reset Philosophy

By utilizing the `@layer reset` declaration, this logic is placed at the very
bottom of the CSS Cascade. This architectural choice ensures that the reset
provides a "clean slate" while allowing book-specific styles to override these
rules effortlessly without high-specificity selectors or `!important` flags.

## 2. Technical Breakdown by Section

### Section 1: Box Sizing & Fluidity

This section standardizes the browser's box model. By applying
`box-sizing: border-box` to all elements, we ensure that `padding` and `border`
widths are included within the element's total width/height. This prevents
mathematical errors in layout and stops elements from "blowing out" the page
width on mobile devices.

### Section 2: Text Rendering & Smoothing

This section stabilizes the typography across different hardware:

- **Size Adjust:** Prevents mobile browsers from automatically inflating font
  sizes (`text-size-adjust`), ensuring our typographic scale remains intact.
- **Smoothing:** Forces `-webkit-font-smoothing: antialiased` to ensure
  high-resolution "Retina" screens render our technical fonts with maximum
  sharpness.
- **Hanging Punctuation:** Implements `hanging-punctuation`, a high-end feature
  that allows quotation marks to sit outside the text alignment, maintaining a
  perfect vertical "edge" for the reading block.

### Section 3: Modern Media Responsiveness

This section treats media elements (images, videos, SVGs) as block-level
components. Setting `max-width: 100%` ensures that a large technical diagram
never overflows its container, while `height: auto` preserves the intended
aspect ratio regardless of screen size.

### Section 4: List Neutralization

Default browser list styles (bullets and numbering) are removed from `ol`, `ul`,
and `menu`. This ensures that technical lists or navigation menus do not inherit
unwanted browser-specific styling, allowing the "Book Layer" to define precise,
themed bullet styles.

### Section 5: Modern Logical Properties (RTL Support)

This is the core of the project's multilingual capability. Instead of using
physical properties like `margin-left` or `padding-right`, we use
**Logical Properties** (`padding-inline`).

- **Directional Agnostic:** By using `inline` and `block` terminology, the
  margins and padding automatically flip their orientation when a book is
  switched from English (LTR) to Arabic (RTL).

### Section 6: Form Elements Standardization

This section forces buttons, inputs, and textareas to inherit the fonts and
colors defined in our "Base Plate." This prevents the browser from reverting to
low-quality system fonts inside interactive elements.

### Section 7: Accessible Motion

To support readers with vestibular disorders or motion sensitivity, smooth
scrolling is only enabled if the user has **not** explicitly requested "Reduced
Motion" in their operating system preferences.

## 3. Integration Requirements

To maintain the **Simple Technical Standard**, this file must be the first
stylesheet declared in the HTML template to ensure the cascade builds upward
correctly.

- **Storage Location:** `shared-assets/styles/base-reset.css`
- **Documentation Root:**
  `books/simple-technical-standard/content/en-us/arch-reset.md`
