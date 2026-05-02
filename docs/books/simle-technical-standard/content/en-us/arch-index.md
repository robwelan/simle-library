# Architecture: The CSS Cascade & Loading Order

This document is the **entry point** for understanding how the "Simple Technical
Standard" engine is assembled. To ensure the browser renders the book correctly,
the shared assets must be loaded in a specific, intentional sequence managed by
a single index.

## 1. The Power of CSS Layers (`@layer`)

Our architecture uses the modern **CSS Cascade Layers** feature. This allows us
to control the "priority" of our styles regardless of when they are loaded or
how specific the selectors are.

By using layers, we ensure that:

1. **Reset** always has the lowest priority (the foundation).
2. **Theme** and **Book-specific** styles always have the highest priority (the
   paint).
3. We avoid "CSS Specificity Wars" where developers have to use `!important` to
   fix layout issues.

---

## 2. The Master Entry Point (`base-index.css`)

To simplify implementation and maintain strict ordering, we use a master index
file. This file uses `@import` rules to pull in individual modules while
simultaneously assigning them to their respective CSS layers.

**Source Path:** `shared-assets/styles/base-index.css`

```css
@import url("base-reset.css") layer(reset);
@import url("base-typography.css") layer(typography);
@import url("base-layout.css") layer(base);
@import url("base-ruby.css") layer(ruby);
@import url("base-theme.css") layer(theme);
```
