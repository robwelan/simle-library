# **SIMLE Shared Assets Changelog**

This file tracks the evolution of the SIMLE shared assets. Until version 0.1.0, this project is considered to be in a **Pre-Alpha / Draft** state.

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

### **To-Do / Next Steps**

* Implement the actual base-typography.css file.  
* Compile the first iteration of engine\_core.wasm.  
* Verify font loading across mobile and desktop environments.

*Standard Version: 0.0.2-dev*

*File Name: CHANGELOG.md*