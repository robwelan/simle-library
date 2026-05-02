# **Typography Standard: The Universal Base Plate**

This document specifies the font architecture for the simple-technical-standard
and explains the philosophy behind our shared asset system.

## **1\. The "Base Plate" Philosophy**

The directory structure of this project is designed so that shared-assets acts
as a **Universal Base Plate**.

### **Why a Shared Foundation?**

* **Systemic Consistency:** By forcing every book to link back to the root
  shared-assets, we ensure that the "Standard" is enforced globally. Every book
  inherits the same high-performance typographic engine.
* **Single Source of Truth:** Maintenance happens in one place. An update to a
  font file or a line-height tweak in base-typography.css propagates to every
  book instantly.
* **Resource Efficiency:** Browsers cache these shared assets. Once a user loads
  one book, the fonts for every other book in the library are already stored
  locally.

## **2\. Selection Rationale: High-Performance Readability**

Every font in this kit was selected to reduce "Cognitive Load"—the mental effort
required to decode text. This is especially critical in technical and
multilingual contexts.

### **Clarity and Distinction**

In technical documentation, confusing characters lead to errors. We use
**IBM Plex** because it excels at "Glyph Distinction":

* The uppercase I, lowercase l, and number 1 are visually distinct.
* The zero 0 is clearly differentiated from the letter O.
* **Vietnamese Support:** We include specific Vietnamese subsets to ensure that
  complex stacked diacritics (like *ổ* or *ặ*) are rendered with native-level
  precision rather than falling back to poorly scaled system fonts.

### **Literary Flow**

For long-form prose and technical explanations, we utilize **STIX Two**.

* The serif design provides "horizontal flow," helping the eye travel across
  lines of text.
* It provides the "Academic/LaTeX" aesthetic that signals high-quality,
  trustworthy information.

### **Global Equality**

We treat all languages as "First-Class Citizens." **Noto (Arabic/CJK)** fonts
match our Latin fonts in visual weight, preventing the "jittery" feeling that
occurs when different scripts look unbalanced on the same page.

## **3\. Physical Asset Inventory (15 Files)**

These files are located in ../../../../shared-assets/fonts/.

| File Name | Category | Role |
| :---- | :---- | :---- |
| ibm-plex-sans-v23-latin-regular.woff2 | Sans | Main UI / Pinyin |
| ibm-plex-sans-v23-latin-ext-regular.woff2 | Sans | Extended Latin (European Tones) |
| ibm-plex-sans-v23-vietnamese-regular.woff2 | Sans | Vietnamese Tone Marks |
| ibm-plex-mono-v20-latin-regular.woff2 | Mono | Code / Technical Notation |
| ibm-plex-mono-v20-latin-ext-regular.woff2 | Mono | Technical Phonetics |
| ibm-plex-mono-v20-vietnamese-regular.woff2 | Mono | Vietnamese Technical Code |
| STIXTwoText\[wght\].woff2 | Serif | Variable Prose (Regular) |
| STIXTwoText-Italic\[wght\].woff2 | Serif | Variable Prose (Italic) |
| STIXTwoMath-Regular.woff2 | Math | Scientific Operators & LaTeX |
| noto-naskh-arabic-v44-arabic-regular.woff2 | Arabic | Standard RTL Text |
| noto-naskh-arabic-v44-math-regular.woff2 | Arabic | RTL Math Logic |
| noto-naskh-arabic-v44-symbols-regular.woff2 | Arabic | RTL Punctuation |
| noto-sans-sc-v40-chinese-simplified-regular.woff2 | CJK | Simplified Chinese |
| noto-sans-jp-v56-japanese-regular.woff2 | CJK | Japanese (Hiragana/Katakana) |
| noto-sans-kr-v39-korean-regular.woff2 | CJK | Korean (Hangul) |

## **4\. Technical Integration & Variables**

The logic is housed in shared-assets/styles/base-typography.css.

### **Fluid Font Sizing**

We use a clamp() function to ensure readability across all devices. We start at
**18px** (1.125rem) to ensure that text is never "too small" for modern
high-density mobile screens.

\--fs-base: clamp(1.125rem, 1.08rem \+ 0.22vw, 1.25rem);

### **Font Stacks and Cascading Logic**

The system uses "Optimized Stacks." By defining multiple fonts under one
variable, the browser "falls forward" to the next available script.

* **\--font-sans**: Loads IBM Plex first. If it sees Chinese characters, it
  falls forward to BaseCJK. If it sees Arabic, it falls to BaseArabic.
* **\--font-serif**: Prioritizes the literary STIX Two for prose, with the same
  global fallback logic.
* **\--font-mono**: Strictly for code blocks and phonetic guides.

### **Complex Script Spacing**

Languages like Vietnamese, Arabic, and Chinese require more vertical "breathing
room." The standard implements a :lang() selector that automatically increases
the line-height from 1.6 to 1.9 for these scripts to prevent diacritic
collision.
