# **SIMLE Shared Assets Changelog**

This file tracks the evolution of the SIMLE shared assets. Until version 0.1.0, this project is considered to be in a **Pre-Alpha / Draft** state.

## **\[0.0.4-dev\] \- 2024-05-28**

### **Status: Interactive Library Interface**

Developed the front-end interface for the SIMLE Library, providing a visual gateway to the distributed book modules.

### **Added**

* **Interactive Library UI:** Created a responsive, grid-based web interface (index.html) using Tailwind CSS to display books from the generated manifest.  
* **Defensive Asset Loading:** Implemented a robust path-resolution script that dynamically constructs image URLs and handles fallback states (color/icon placeholders) if assets are missing or improperly path-referenced.  
* **Dynamic Reader Integration:** Established a direct "Launch" pipeline that maps library items to their respective /books/{book-folder}/index.html entry points.

### **Changed**

* **Manifest Consumption:** Switched from a static file view to a dynamic fetch model with cache-busting logic to ensure the library always reflects the latest CI/CD manifest updates.  
* **UI Polish:** Integrated hover transitions, glassmorphism effects for book covers, and a "Mobile-First" responsive design pattern.

## **\[0.0.3-dev\] \- 2024-05-27**

### **Status: Automation & Distribution Pipeline**

Implemented automated manifest generation and optimized the directory structure for web-based library distribution via GitHub Pages.

### **Added**

* **Manifest Build Script:** Created build\_library.py to recursively parse meta.toml files and generate a unified library.json manifest.  
* **Automated CI/CD Workflow:** Integrated a GitHub Action (build-library.yaml) to automatically rebuild and commit the library manifest on every push.  
* **Web Distribution Path:** Established the /docs/ directory as the primary target for build artifacts to support direct hosting via GitHub Pages.

### **Changed**

* **Output Relocation:** Moved the generated library.json from the root directory to /docs/library.json to isolate distribution assets from source code.  
* **Workflow Optimization:** Updated the build-library workflow with intelligent change detection to prevent redundant "empty" commits when manifest data remains unchanged.

## **\[0.0.2-dev\] \- 2024-05-24**

### **Status: Localization Logic Finalized**

Refined the asset mapping strategy to support multi-lingual covers by decoupling global backgrounds from locale-specific wordmarks.

### **Added**

* **Localized Asset Schema:** Integrated \[cover\_wordmarks\] into the meta.toml specification to allow for language-specific visual branding.  
* **Asset Specification Document:** Created comprehensive technical requirements for background art (9:16/16:9) and wordmark transparency.  
* **3x3 Grid Alignment Standards:** Established "tight-crop" rules for wordmark assets to ensure predictable layout positioning.

### **Changed**

* **Metadata Relocation:** Moved wordmark file path definitions from book.toml (Global) to meta.toml (Locale) to ensure the correct visual branding loads automatically with the selected language.  
* **Directory Structure:** Updated the recommended asset pathing to include locale subdirectories (e.g., assets/images/en-us/).

## **\[0.0.1-dev\] \- 2024-05-22**

### **Status: Architectural Draft (Non-Functional)**

Finalized the simplified repository hierarchy and asset management strategy. This version focuses on a "Flat-Library" model where author-level collisions are handled at the Git repository level rather than through complex internal subdirectories.

### **Added**

* **Final Repository Schema:** Established the core root structure:  
  * /books/ \- Contains the standard and library books.  
  * /shared-assets/ \- Evergreen CSS, Fonts, and Wasm.  
  * /local-only/ \- Directory for non-committed drafts and experiments.  
  * README.md \- Root project documentation.  
* **Namespace Strategy:** Confirmed that name clashing will be managed by hosting distinct books in separate repositories.  
* **Style Definitions (Draft):** Outlined the 18px baseline typography rules and the "Base Plate" font list.  
* **Engine Specification:** Defined the requirements for the Rust-based Wasm engine for manifest parsing and direct shadowing logic.

### **Changed**

* **Simplified Hierarchy:** Removed the experimental library/\[category\]/\[author\] nested structure in favor of a flat /books/ directory to improve developer ergonomics.

*Standard Version: 0.0.4-dev* *File Name: CHANGELOG.md*