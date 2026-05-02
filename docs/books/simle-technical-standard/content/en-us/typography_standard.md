# **Typography Standard**

This document defines the typographic systems, font stacks, and implementation rules for the digital cover and content ecosystem.

## **1\. Core Font System**

The system utilizes a multi-layered font-face strategy to support global character sets while maintaining high-fidelity rendering for technical and academic content.

### **1.1 Primary Typeface Families**

| Alias | Typeface | Primary Use Case | Features |
| :---- | :---- | :---- | :---- |
| **BaseSans** | IBM Plex Sans | UI, Metadata, Pinyin | Low-contrast, high legibility at small sizes. |
| **BaseSerif** | STIX Two Text | Long-form Prose, LaTeX | Variable weight support, optimized for screen reading. |
| **BaseMath** | STIX Two Math | Formulas & Operators | Specialized glyphs for complex mathematical notation. |
| **BaseMono** | IBM Plex Mono | Code, Phonetics | Consistent width for technical alignment. |
| **BaseArabic** | Noto Naskh | Arabic Text & Math | Optimized for RTL flow and script-specific symbols. |
| **BaseCJK** | Noto Sans (SC/JP/KR) | CJK Characters | Unified weight matching for simplified/traditional/kana/hangul. |

## **2\. Optimized Font Stacks**

Fonts are arranged in stacks to ensure seamless fallbacks across complex scripts.

* **Sans-Serif (General UI):** 'BaseSans', 'BaseCJK', 'BaseArabic', system-ui, sans-serif;  
* **Serif (Narrative/Reading):** 'BaseSerif', 'BaseCJK', 'BaseArabic', serif;  
* **Monospace (Technical):** 'BaseMono', monospace;  
* **Math (Scientific):** 'BaseMath', 'BaseArabic', serif;

## **3\. Global Fluid Scaling**

To maintain readability across devices, the base font size is calculated using a responsive clamp:

* **Logic:** clamp(1.125rem, 1.08rem \+ 0.22vw, 1.25rem)  
* **Base (Mobile):** 18px (1.125rem)  
* **Scaling Factor:** \+0.22% of viewport width.  
* **Maximum (Desktop):** 20px (1.25rem)

## **4\. Complex Script & Language Handling**

Specialized rules are applied to accommodate the vertical height and diacritic requirements of specific scripts.

### **4.1 Diacritic & Script Clearance**

Languages requiring extended vertical space for accents (Vietnamese) or complex strokes (CJK, Arabic) utilize the lh-loose token.

* **Default Leading:** 1.6 (--lh-base)  
* **Expanded Leading:** 1.9 (--lh-loose)  
* **Triggers:** :lang(zh), :lang(jp), :lang(kr), :lang(ar), :lang(vi), and \[dir="rtl"\].

### **4.2 CJK Character Ranges**

Unicode ranges are strictly defined to prevent "mixed-font" visual artifacts within the CJK stack:

* **Simplified Chinese:** U+4E00-9FFF, U+3000-303F  
* **Japanese:** U+3040-309F, U+30A0-30FF, U+FF00-FFEF  
* **Korean:** U+AC00-D7AF, U+1100-11FF

## **5\. Mathematical Rendering**

Mathematical notation must utilize BaseMath or BaseArabic (for Arabic-specific math symbols).

* **Numeric Alignment:** Must use lining-nums and tabular-nums for consistent alignment in tables and equations.  
* **Rendering Mode:** optimizeLegibility is enabled globally to ensure proper kerning for STIX Two ligatures.

## **6\. Implementation Notes**

* **Font Display:** All @font-face declarations must use font-display: swap to prevent FOIT (Flash of Invisible Text).  
* **Text Adjustment:** \-webkit-text-size-adjust: 100% is enforced to prevent mobile browsers from overriding the fluid scale logic.