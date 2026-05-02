# Architecture: System Theming (`base-theme.css`)

This document specifies the adaptive color architecture for the project. The
`base-theme.css` utilizes modern CSS functions to handle Light and Dark modes
automatically, ensuring visual comfort and accessibility across all reading
environments.

## 1. Architectural Philosophy

Wrapped in the `@layer theme` declaration, this logic acts as the "Visual Skin"
of the system. Its primary goal is to provide high-contrast, legible
environments while respecting the user's system-level color preferences. By
using the modern `light-dark()` function, we eliminate the need for redundant
media queries throughout the codebase.

## 2. Technical Breakdown by Section

### Section 1: Color Scheme Declaration

The property `color-scheme: light dark;` is defined at the `:root`. This informs
the browser and operating system that our technical standard is fully optimized
for both modes, allowing the browser to adjust system elements (like scrollbars
and form controls) to match the theme automatically.

### Section 2: Palette Definitions

The system maintains two distinct palettes:

- **Light Mode:** Uses a pure white background (`#ffffff`) with high-contrast
  off-black text (`#1a1a1a`) to simulate a printed page.
- **Dark Mode:** Uses a "Midnight" grey-black (`#121212`) to reduce eye strain
  in low-light environments, paired with soft-white text (`#f0f0f0`) to prevent
  visual "haloing."

### Section 3: Adaptive "Magic" Variables

This is the core innovation of the theme layer. Instead of writing separate
styles for dark mode, we use the `light-dark()` function:

- **Automatic Switching:** The browser automatically selects the correct
  variable based on the user's OS settings.
- **Unified Logic:** Elements like `--page-bg` and `--main-text` serve as the
  single point of reference for the entire book, ensuring color consistency
  across all components.

### Section 4: Document Application

The theme is applied directly to the `body`:

- **Smooth Transitions:** A `0.3s ease` transition is applied to
  `background-color` and `color`. This prevents a "jarring" flash if the user
  toggles their system theme while the book is open.
- **Component Linking:** The theme explicitly links back to the **Ruby** and
  **Link** styles, ensuring that phonetic notations (`rt`) and hyperlinks (`a`)
  maintain accessible contrast ratios in both light and dark environments.

## 3. Implementation Requirements

This file should be loaded after the layout and typography layers to ensure that
color variables are available to all components.

- **File Name:** `arch-theme.md`
- **Storage Location:** `shared-assets/styles/base-theme.css`
- **Documentation Root:**
  `books/simple-technical-standard/content/en-us/arch-theme.md`
