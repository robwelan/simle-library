# **Branding & Wordmarks**

This document outlines the design standards and technical implementation for title treatments and author branding within the cover system.

## **1\. Wordmark Architecture**

Wordmarks are treated as independent layers to allow for responsive scaling and localized language support without requiring unique background art for every market.

### **SVG Requirements**

* **Path Optimization:** All text must be converted to outlines/paths to ensure font consistency across devices.  
* **ViewBox:** Set the viewBox attribute tightly to the bounds of the art to ensure predictable scaling.  
* **Color Handling:** Use fill="currentColor" for primary elements to allow the CSS engine to toggle between light and dark themes (e.g., swapping white text for black text based on background luminance).

### **Composition Rules**

* **Title Hierarchy:** The primary title should occupy approximately 60-70% of the wordmark's vertical height.  
* **Subtitle/Author:** Secondary text should be positioned with a consistent gap (minimum 4px in vector space) from the primary title.

## **2\. Dynamic Placement Logic**

The wordmark is placed according to the grid system defined in the core layout specs.

| Orientation | Grid Anchor | Justification | Scaling Limit |
| :---- | :---- | :---- | :---- |
| **Portrait** | Center-Center | Center | Max 90% of container width |
| **Landscape** | Center-Left | Left | Max 40% of container width |

### **Safe Zone Management**

To prevent the wordmark from clashing with the focal point of the background art:

* **Focal Point Exclusion:** Background art should ideally have its "visual weight" in the lower third or right half to accommodate the wordmark.  
* **CSS Drop Shadows:** Apply a standard filter to the SVG container rather than the paths themselves to maintain legibility on busy backgrounds:  
  .wordmark-layer {  
    filter: drop-shadow(0 2px 4px rgba(0,0,0,0.5));  
  }

## **3\. Localization Standards**

When creating wordmarks for different languages:

* **Height Matching:** Ensure the visual "heaviness" of translated titles matches the original English version to maintain the intended layout balance.  
* **Language Tags:** Files must follow the ISO 639-1 naming convention (e.g., wordmark\_en.svg, wordmark\_fr.svg).

## **4\. Scaling Behavior**

Wordmarks utilize fluid scaling. The width is relative to the viewport size but constrained by a maximum pixel width to prevent the branding from becoming overwhelming on 4K monitors.

* **Min-Width:** 280px (Mobile)  
* **Max-Width:** 800px (Desktop / 4K)