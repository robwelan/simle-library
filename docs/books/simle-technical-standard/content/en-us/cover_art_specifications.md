# **Cover Art Asset Specifications**

This document defines the technical requirements for visual assets within the SIMLE engine. These specifications ensure high visual fidelity on high-DPI (Retina) displays while maintaining optimal performance.

## **1\. Asset Logic: book.toml vs. meta.toml**

The SIMLE engine separates shared visual assets from locale-specific branding to support a seamless multi-lingual experience.

* **Global Assets (book.toml):** Defines shared, non-textual assets like the cover\_background\_landscape and cover\_background\_portrait.  
* **Locale Assets (meta.toml):** Located in content/\[locale\]/meta.toml. This file defines the text strings for metadata and maps the paths to the localized wordmark images (book\_wordmark\_title and book\_wordmark\_author).

## **2\. Background Art (The Canvas)**

Background art must be "clean" (free of text) to allow for dynamic wordmark placement and localization.

### **Portrait Assets (cover\_background\_portrait)**

* **Aspect Ratio:** Fixed **9:16**  
* **Recommended Resolution:** 1440 × 2560 px  
* **CSS Variable:** \--bg-portrait

### **Landscape Assets (cover\_background\_landscape)**

* **Aspect Ratio:** Fixed **16:9**  
* **Recommended Resolution:** 2560 × 1440 px  
* **CSS Variable:** \--bg-landscape

## **3\. Wordmark Asset Specifications**

Wordmarks are visual overrides for the metadata strings. They must be localized to match the language of the current meta.toml.

### **Title Wordmark (book\_wordmark\_title)**

* **Target Size:** 1200 px width.  
* **Canvas Padding:** Crop tightly to the glyphs with 0px transparent padding to ensure correct alignment within the SIMLE **3x3 responsive grid**.  
* **Format:** SVG (preferred) or WebP (lossless) with alpha transparency.

### **Author Wordmark (book\_wordmark\_author)**

* **Target Size:** 800 px to 1000 px width.  
* **Format:** SVG (preferred) or WebP (lossless) with alpha transparency.

## **4\. Implementation Example**

### **Step 1: Global Config in book.toml**

\[cover\]  
book\_cover\_layout \= "layout-classic"  
cover\_background\_landscape \= "assets/images/cover\_art\_landscape.webp"  
cover\_background\_portrait \= "assets/images/cover\_art\_portrait.webp"

### **Step 2: Localized Config in content/en-us/meta.toml**

title \= "SIMLE Technical Standard"  
author \= "Robert Michael Welan"  
description \= "Defining the blueprint for the next generation of digital storytelling..."

\[cover\_wordmarks\]  
book\_wordmark\_title \= "assets/images/en-us/title.webp"  
book\_wordmark\_author \= "assets/images/en-us/author.webp"

## **5\. Technical Checklist for Authors**

| Requirement | Specification |
| :---- | :---- |
| **No Baked-in Text** | Background art must be clean. No language-specific text in backgrounds. |
| **Transparency** | Wordmarks MUST have a transparent alpha channel. |
| **Locale Isolation** | Store localized wordmarks in assets/images/\[locale\]/ to prevent language clash. |
| **Format** | Use WebP (lossy) for backgrounds and WebP (lossless) or SVG for wordmarks. |

## **6\. Directory Structure**

├── book.toml  
├── assets/images/  
│   ├── cover\_art\_landscape.webp  
│   ├── cover\_art\_portrait.webp  
│   └── en-us/  
│       ├── title.webp  
│       └── author.webp  
└── content/en-us/  
    └── meta.toml  
