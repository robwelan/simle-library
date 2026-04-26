# **SIMLE Shared Assets Changelog**

This file tracks the evolution of the SIMLE shared assets. Until version 0.1.0, this project is considered to be in a **Pre-Alpha / Draft** state.

## **\[0.0.1-dev\] \- 2024-05-22**

### **Status: Architectural Draft (Non-Functional)**

Finalized the simplified repository hierarchy and asset management strategy. This version focuses on a "Flat-Library" model where author-level collisions are handled at the Git repository level rather than through complex internal subdirectories.

### **Added**

* **Final Repository Schema:** Established the core root structure:  
  * /books/ \- Contains the standard and library books.  
  * /shared-assets/ \- Evergreen CSS, Fonts, and Wasm.  
  * /local-only/ \- Directory for non-committed drafts and experiments.  
  * README.md \- Root project documentation.  
* **Namespace Strategy:** Confirmed that name clashing will be managed by hosting distinct books in separate repositories, allowing for a flatter and more readable /books/ directory within this specific library.  
* **Style Definitions (Draft):** Outlined the 18px baseline typography rules and the "Base Plate" font list.  
* **Engine Specification:** Defined the requirements for the Rust-based Wasm engine for manifest parsing and direct shadowing logic.

### **Changed**

* **Simplified Hierarchy:** Removed the experimental library/\[category\]/\[author\] nested structure in favor of a flat /books/ directory to improve developer ergonomics for single-repo or small-team use cases.

### **To-Do / Next Steps**

* Implement the actual base-typography.css file.  
* Compile the first iteration of engine\_core.wasm.  
* Verify font loading across mobile and desktop environments.

*Standard Version: 0.0.1-dev*

*File Name: CHANGELOG.md*