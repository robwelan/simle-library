# **Base Typography CSS Implementation**

This file contains the core CSS architecture for the typography system,
utilizing CSS Variables (Custom Properties) to ensure consistent scaling and
accessibility across the application.

## **1\. Font Face Declarations**

These declarations handle the loading of local and hosted font assets, utilizing
font-display: swap to ensure text remains visible during download.

```css
/* Variable Font - STIX Two Text (Weight range 400-700) */
@font-face {
  font-family: 'BaseSerif';
  src: url('/assets/fonts/STIXTwoText-Variable.woff2') format('woff2-variations');
  font-weight: 400 700;
  font-style: normal;
  font-display: swap;
}

/* Sans Serif - IBM Plex Sans */
@font-face {
  font-family: 'BaseSans';
  src: url('/assets/fonts/IBMPlexSans-Regular.woff2') format('woff2');
  font-weight: 400;
  font-style: normal;
  font-display: swap;
}

@font-face {
  font-family: 'BaseSans';
  src: url('/assets/fonts/IBMPlexSans-Medium.woff2') format('woff2');
  font-weight: 500;
  font-style: normal;
  font-display: swap;
}
```

## **2\. Typography Variables**

The system uses a fluid scale where the font size adjusts based on the viewport
width using clamp().

```css
:root {
  /* Font Family Stacks */
  --font-serif: 'BaseSerif', 'Noto Serif', Georgia, serif;
  --font-sans: 'BaseSans', 'Noto Sans', system-ui, -apple-system, sans-serif;
  --font-mono: 'IBM Plex Mono', 'Noto Sans Mono', monospace;

  /* Font Scale (Fluid) */
  --text-xs: clamp(0.75rem, 0.7rem + 0.25vw, 0.875rem);
  --text-sm: clamp(0.875rem, 0.8rem + 0.35vw, 1rem);
  --text-base: clamp(1rem, 0.95rem + 0.25vw, 1.125rem);
  --text-lg: clamp(1.125rem, 1.05rem + 0.4vw, 1.25rem);
  --text-xl: clamp(1.25rem, 1.15rem + 0.6vw, 1.5rem);
  --text-2xl: clamp(1.5rem, 1.3rem + 1vw, 2rem);
  --text-3xl: clamp(2rem, 1.8rem + 1.5vw, 3rem);

  /* Line Heights */
  --leading-tight: 1.2;
  --leading-snug: 1.4;
  --leading-normal: 1.6;
  --leading-relaxed: 1.8;

  /* Letter Spacing */
  --tracking-tight: -0.02em;
  --tracking-normal: 0;
  --tracking-wide: 0.05em;
}
```

## **3\. Global Styles & Utilities**

The following styles apply the defaults to the document and provide common
utility classes for typesetting.

```css
body {
  font-family: var(--font-serif);
  font-size: var(--text-base);
  line-height: var(--leading-normal);
  color: \#1a1a1a;
  \-webkit-font-smoothing: antialiased;
  \-moz-osx-font-smoothing: grayscale;
}

h1, h2, h3, h4 {
  font-family: var(--font-sans);
  font-weight: 500;
  line-height: var(--leading-tight);
  letter-spacing: var(--tracking-tight);
  margin-bottom: 1rem;
}

/\* Typography Utilities \*/
.font-sans { font-family: var(--font-sans); }
.font-serif { font-family: var(--font-serif); }
.font-mono { font-family: var(--font-mono); }

.text-xs { font-size: var(--text-xs); }
.text-sm { font-size: var(--text-sm); }
.text-lg { font-size: var(--text-lg); }
.text-xl { font-size: var(--text-xl); }

.italic-serif {
  font-family: var(--font-serif);
  font-style: italic;
  font-feature-settings: "ital" 1;
}
```

## **4\. Accessibility & Readability Logic**

To ensure high readability, the maximum width of text containers is limited to
maintain an optimal character count per line (65–75 characters).

```css
.prose-container {
  max-width: 70ch;
  margin-left: auto;
  margin-right: auto;
}

@media (prefers-reduced-motion: reduce) {
  \* {
    transition: none \!important;
    animation: none \!important;
  }
}
```
