# **Font Acquisition and Optimization Strategy**

This document details the process for sourcing, licensing, and preparing font
assets for the typography system.

## **1\. Primary Sourcing**

We prioritize Open Source and Variable fonts to ensure maximum flexibility and
zero licensing friction for web distribution.

### **Core Typefaces**

| Role | Font Family | Source | License | Format |
| :---- | :---- | :---- | :---- | :---- |
| **Serif** | STIX Two Text | [Google Fonts](https://fonts.google.com/) | SIL Open Font (OFL) | Variable (VF) |
| **Sans** | IBM Plex Sans | [IBM Type](https://github.com/IBM/plex) | SIL Open Font (OFL) | WOFF2 |
| **Mono** | IBM Plex Mono | [IBM Type](https://github.com/IBM/plex) | SIL Open Font (OFL) | WOFF2 |

## **2\. Obtaining WOFF2 Files from Google Fonts**

Google Fonts typically serves .ttf files via their "Download All" button, which
are not optimized for web use. To obtain the production-ready .woff2 files that
Google uses on its own CDN, developers should use one of the following methods:

### **Method A: google-webfonts-helper (Recommended)**

The easiest way to get the files and the necessary CSS is the
[google-webfonts-helper](https://google-webfonts-helper.herokuapp.com/fonts)
tool.

1. Search for the font (e.g., "STIX Two Text").
2. Select the specific charsets (e.g., latin, latin-ext).
3. Select the styles (e.g., regular, 700, italic).
4. Download the .zip containing optimized .woff2 files.
5. Copy the generated CSS directly into our typography.css.

### **Method B: Manual CLI Extraction**

If you need the absolute latest variable font version not yet on helpers:

1. Construct the Google Fonts API URL (e.g.,
   <https://fonts.googleapis.com/css2?family=Inter:wght@400;700>\&display=swap).
2. Use curl with a modern User-Agent to trick Google into serving the WOFF2 CSS:
   curl \-H "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64)
   AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36" \\
   "\[https://fonts.googleapis.com/css2?family=Inter:wght@400;700\](https://fonts.googleapis.com/css2?family=Inter:wght@400;700)"
   \> fonts.css
3. Open fonts.css, find the src: url(...) links, and download the .woff2 files
   manually.

## **3\. Acquisition Pipeline**

To maintain performance, we do not link directly to external CDNs. All fonts
must be self-hosted.

1. **Download Source:** Obtain files via Method A or B above.
2. **Subsetting:** Use pyftsubset (fonttools) only if the downloaded files still
   contain unused glyph ranges (e.g., Cyrillic or Greek if not needed).
3. **Directory Mapping:** Place assets in /assets/fonts/.

## **4\. Optimization Checklist**

* **Format:** Only use .woff2.
* **Font-Display:** Always use font-display: swap;.
* **Preloading:** Add
  \<link rel="preload" href="/assets/fonts/font-name.woff2" as="font" type="font/woff2" crossorigin\>
  to the \<head\>.
* **Variable Fonts:** Prefer a single \[wght\].woff2 file over multiple static
  files to save on total payload size.

## **5\. Licensing Compliance**

* **OFL (SIL Open Font License):** Allows bundling and redistribution.
* **Attribution:** Ensure the LICENSE.txt for each font family is preserved in
  the /assets/fonts/ directory.

## **6\. Fallback Strategy**

* **Serif Fallback:** Georgia, Times New Roman, serif
* **Sans Fallback:** system-ui, \-apple-system, BlinkMacSystemFont, Segoe UI,
  Roboto, Arial, sans-serif
