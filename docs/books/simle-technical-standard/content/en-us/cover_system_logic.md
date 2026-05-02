# **SIMLE Cover & Branding Logic**

This document specifies the behavior of the dynamic cover system for the SIMLE
engine. The goal is to separate visual art from textual branding to support
localization, responsiveness, and "no-JS" styling.

## **1\. The Layered Architecture**

Covers are no longer single "flat" images. They are composed of three distinct
layers managed by the engine:

1. **Background Layer:** High-fidelity art (WebP). Supports orientation-specific
   files.
2. **Branding Layer (The Wordmark):** Transparent SVG or WebP containing the
   title/author logo.
3. **Overlay Engine:** A CSS-driven 3x3 grid that determines where the Branding
   Layer is anchored.

## **2\. The 3x3 Grid Specification**

The branding layer is positioned using a standard 9-zone grid. This is defined
in the book's manifest (book.toml).

| Zone ID | Horizontal | Vertical |
| :---- | :---- | :---- |
| top-left | Left | Top |
| top-center | Center | Top |
| top-right | Right | Top |
| mid-left | Left | Center |
| center | Center | Center |
| mid-right | Right | Center |
| bottom-left | Left | Bottom |
| bottom-center | Center | Bottom |
| bottom-right | Right | Bottom |

### **CSS Implementation (Abstract)**

```css
.cover-container {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  grid-template-rows: 1fr 1fr 1fr;
}

.wordmark-layer {
  /\* Logic for zone 'bottom-center' \*/
  grid-column: 2;
  grid-row: 3;
  align-self: end;
  justify-self: center;
}
```

## **3\. Orientation & Assets**

To prevent awkward cropping on mobile (portrait) vs. desktop (landscape), the
engine requests specific aspect ratios.

* **Portrait (art\_p):** Target 9:16. Optimized for mobile devices.
* **Landscape (art\_l):** Target 16:9. Optimized for desktop/tablet headers.
* **Fallback:** If only one is provided, the engine uses object-fit: cover.

## **4\. Localization Logic**

Because the **Wordmark** is a separate file:

1. The engine checks the user's system language.
2. If wordmark\_jp.svg exists and the user is in Japan, it swaps the branding
   layer automatically.
3. The **Background Layer** remains the same, saving significantly on bandwidth
   and storage.

## **5\. Metadata Integration**

The Branding Layer must be accompanied by alt text derived from the manifest's
title and author fields to ensure screen readers can identify the book even when
using graphical wordmarks.
