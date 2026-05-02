# **The SIMLE Project Standard: Philosophy & Golden Rules**

The **SIMLE** (Simple Interactive Multilingual Learning Engine) standard is a
foundational specification designed to ensure high-performance, accessible, and
perfectly synchronized technical documentation.

**Terminology Note:** While **SIMLE** is the acronym for the engine and
standard, **.simle** is the reserved file extension used for the project
archives.

## **1\. The Core Philosophy: "Cognitive Accessibility"**

The primary goal of the SIMLE format is to minimize the mental effort required
to decode complex technical information, especially when navigating multiple
languages or scripts. We believe that clarity of form leads directly to clarity
of understanding.

### **Why the Format Exists**

Traditional digital publishing formats often impose significant friction on both
creators and readers:

* **The EPUB Legacy:** EPUB remains a fragmented and overly verbose standard. It
  often relies on outdated subsets of HTML and CSS, leading to inconsistent
  rendering across devices. The reliance on complex XML namespaces and lack of a
  unified modern direction makes it difficult for authors to produce consistent
  results.
* **The Kindle Ecosystem:** Building and formatting for Kindle is famously
  cumbersome and clunky, often requiring proprietary tools and dealing with
  unpredictable conversion errors.
* **The SIMLE Alternative:** This system is built on **modern HTML5 and CSS3**
  only. It intentionally abandons XML, legacy codebases, and the "metadata
  overkill" that plagues traditional ebook formats. By using standard web
  technologies, SIMLE provides a streamlined, developer-friendly authoring
  experience without annoying versioning conflicts.

Furthermore, traditional formats struggle with
**"Parallel Synchronization"**—the ability to show two or more languages
side-by-side without breaking the reading flow. SIMLE was built to solve this
specifically for technical and linguistic education, ensuring that translations
are not just available, but perfectly aligned in real-time.

## **2\. The Golden Rules**

Every SIMLE project must adhere to these three foundational pillars to be
considered compliant with the standard.

### **Rule I: The 18px Baseline Rationale**

The standard defines a strict "Floor" for body typography. No body text may ever
be smaller than **18px** (1.125rem).

* **The Rationale:** Modern high-density mobile screens and diverse multilingual
  glyphs (such as complex Chinese characters or Arabic diacritics) require more
  physical pixels to remain legible. Smaller font sizes lead to "glyph blurring"
  and increased eye strain in complex scripts.
* **Implementation:** We utilize fluid typography via the CSS clamp() function
  to ensure that 18px is the minimum starting point, scaling smoothly up to 20px
  on larger viewports.

### **Rule II: Shared Assets Architecture (The Universal Base Plate)**

To prevent "Flash of Unstyled Text" (FOUT) and keep file sizes manageable,
individual books do not embed their own font files. Instead, they link to a
root-level shared-assets/ directory.

* **The Rationale:** This architecture allows for extreme browser caching. Once
  a user loads any single book in a library, the 15-file font "Base Plate" is
  already stored in the browser cache. Subsequent books load nearly
  instantaneously because the heaviest assets are already present on the device.

### **Rule III: Direct Shadowing (Parallel Sync)**

Multilingualism is achieved through directory structure, not complex database
IDs or proprietary tagging systems.

* **The Rationale:** If a file named introduction.md exists in the /en-us/
  folder, it must also exist in the /zh-py/ folder. This "Shadowing" allows the
  engine to switch languages or scroll them in parallel by simply swapping the
  folder path in the URI while keeping the filename constant.

## **3\. High-Level Logic**

### **ISO 639-1 & BCP 47 Compliance**

Naming conventions for locale folders are strictly enforced. We follow
international standards for language tagging to ensure the engine can
automatically:

* Detect text direction (LTR vs RTL).
* Apply appropriate typographic overrides (e.g., increasing line-height for
  scripts with high vertical diacritics like Vietnamese or Arabic).

### **First-Class Citizens**

No language is treated as a "secondary" translation. The engine is designed to
render CJK (Chinese, Japanese, Korean) and Arabic with the same visual weight,
performance, and importance as Latin scripts.

## **4\. Manifest Dominance**

The logical flow of the book is decoupled from the physical file system.

* The engine strictly follows the sequence defined in the root book.toml
  manifest.
* This allows authors to reorganize chapters, insert new sections, or change the
  reading order without renaming physical files or moving folders on the disk.
