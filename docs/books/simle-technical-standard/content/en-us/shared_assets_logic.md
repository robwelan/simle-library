# **Shared Assets Logic**

This document defines the resolution, loading, and management strategies for global assets (Fonts, Icons, and Media) across the application ecosystem.

## **1\. Asset Resolution Strategy**

To ensure consistency between local development and production environments, a centralized pathing logic is used.

### **1.1 Directory Structure**

Assets are organized by type and optimized for web delivery:

* /assets/fonts/: WOFF2/VF files (subsetted for performance).  
* /assets/vectors/: SVG icons and illustrative branding.  
* /assets/branding/: Logos and high-resolution marks.

### **1.2 Path Mapping**

Asset paths are resolved relative to the CSS distribution layer or via a global base URL variable:

/\* Relative to build/css/ \*/  
src: url('../fonts/\[font-name\].woff2');

## **2\. High-Performance Typography Loading**

The typography system (defined in typography\_standard.md) utilizes a "Performance-First" loading sequence.

### **2.1 Preload Priorities**

Critical fonts must be preloaded in the HTML \<head\> to minimize Layout Shift (CLS):

1. **BaseSans (Latin):** UI interaction priority.  
2. **BaseSerif (Variable):** Primary reading content priority.  
3. **BaseMath:** Only preloaded on pages with math or formula selectors.

### **2.2 Variable Font Integration**

The system prioritizes **STIXTwoText\[wght\].woff2**.

* **Logic:** Use a single HTTP request for multiple weights (400–700) to reduce total payload size compared to individual weight files.

## **3\. Caching & Versioning**

### **3.1 Immutable Assets**

Fonts and brand assets are treated as immutable resources:

* **Header:** Cache-Control: public, max-age=31536000, immutable  
* **Versioning:** Filenames include version tags (e.g., ibm-plex-sans-v23-...) to allow for seamless cache busting during upgrades.

## **4\. Multi-Language Asset Logic**

The application dynamically adjusts asset loading based on the document's language attribute.

| Script Type | Loading Logic |
| :---- | :---- |
| **Latin/Pinyin** | Standard BaseSans subset. |
| **CJK/Arabic** | Lazy-loaded via unicode-range to prevent downloading 10MB+ files for users only viewing Latin text. |
| **Vietnamese** | Uses the specific vietnamese subset of IBM Plex to ensure diacritic clearance. |

## **5\. SVG & Icon Strategy**

* **Injection:** Standard UI icons are injected as inline SVGs to allow for CSS-based color manipulation (currentColor).  
* **Sprite System:** For secondary icons, a single SVG sprite is used to reduce individual network requests.

## **6\. Failover & Safety**

If the primary asset server is unreachable:

1. **Fonts:** Fall back to the system-ui stack defined in the variables.  
2. **Icons:** Use aria-label or alt text to maintain accessibility during asset failure.