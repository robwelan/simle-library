# **SIMLE Shared Assets Changelog**

This file tracks the evolution of the SIMLE shared assets. Until version 0.1.0, this project is considered to be in a **Pre-Alpha / Draft** state.

## **[0.0.7-dev] - 2024-05-30**

### **Changed**
* **Deployment Concurrency**: Added concurrency controls to the workflow to handle the "double-push" sequence, resolving conflicts where initial build requests were superseded by automated sync commits.
* **Asset Cleanup**: Updated the sync script to perform a clean wipe (`rm -rf`) of `docs/books/` before copying, ensuring the web distribution is a 1:1 mirror of the root source.

## **[0.0.6-dev] - 2024-05-30**

### **Added**
* **Environment-Aware Pathing**: Implemented "Smart Path" logic in `index.html` and `diagnostic.html` to automatically toggle between local sibling paths (`../books`) and GitHub Pages deployment paths (`./books`).
* **Automated Asset Sync**: Updated GitHub Actions workflow to physically synchronize the root `books/` directory into the `docs/` folder, ensuring asset visibility on the production web server.

### **Fixed**
* **Diagnostic Accuracy**: Enhanced the asset debugger to provide real-time environment detection (GitHub vs. Local) and explicit URL probing for `library.json` and book assets.
* **Asset Accessibility**: Resolved 404 errors caused by GitHub Pages' restriction on accessing directories outside the site root.

## **\[0.0.5-dev\] \- 2024-05-29**

### **Status: Dynamic Configuration & Path Decoupling**

Decoupled the library interface from hardcoded directory structures by implementing a configuration-driven path resolution system.

### **Added**

* **Configuration-Driven Routing:** Integrated a fetch-based lookup of paths.json to dynamically determine the search\_dir, allowing the library to locate book modules even if the folder names are changed in the environment configuration.
* **Intelligent Path Normalization:** Added logic to resolve relative sibling paths (e.g., transitioning from /docs/ to /books/) based on the root manifest configuration.

### **Changed**

* **Dynamic Asset Resolution:** Refined the image and reader URL generation logic to use the baseBooksUrl derived from the environment config, eliminating "broken link" issues during cross-repository deployments.
* **Manifest Loading Logic:** Updated the initialization sequence to ensure configuration constants are resolved prior to parsing library.json.

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

* **Localized Asset Schema:** Integrated![][image1]
  into the meta.toml specification to allow for language-specific visual branding.
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

* **Simplified Hierarchy:** Removed the experimental library/![][image2]
  /![][image3]
  nested structure in favor of a flat /books/ directory to improve developer ergonomics.

[image1]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAmwAAAAvCAYAAABexpbOAAAEU0lEQVR4Xu3abajeYxwH8J0Z5THKms7Ozn3O2ZgWouPF8tjwQiJaG/JUGwrhhYeyhZqHspJYaE0WyeOaTHmcJYk1SjLmhSblYSlFK29Wmu+1XZf+uzFTVkufT/36/67fdf2v/3Xfr37973vCBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAPifGhkZub7X6/2UuKt/7r+WZ5yS2J54uX8OAIDdGB4enpc4u7++N5SGLU3iov46AAB/byBN1Pf9xb0lz/ohl4n9dQCAf2VoaOjo4eHhF0ZGRtakwXii1JIfnlia8ZeJz9va5G8kVpc895w/Ojp6RpvL+usSr2b+w8yNT58+fVrZr9aXpPZWnnV8W/9Xsv75xPqkA2VczpXxbbnOnzZt2ultXTlfeU45X/Jza+2BxGeJi8o5yz1tfcavJb5J3JfYXmpTp04dSr4x6y7O9aX62ebm/odzXZW4vd1f97gssTaxIrG41uYkVpc3domnkj83efLkQ8rc+Pj4/qVW161MvN/ZrtTKXu/keSPdOgDALtJATe3VJi3Nxam1qSq1Z9qa5CclFpbGJL3Fgl5teOo9V9Y1panb8SYp+WOpX1CatzpeW6/P7q45KQ1OuZbmppNvT4yWPHteVWvf9p2vrFmYcx+YNctnzpx56JQpUw5ObU6m90ttQ2e/1xObS1OV+pvJP0l82tnr1+6+Y2Njx9T8vMTVJS9NapkreT7Oovp97fhPXK4PDQ4OHlTz+xNzEw+W5jVrX2x71+9mUmIgjehxrQ4A8CdpJpaleZjdV7sntTPbOPmFaW6uLXmu72X+izaX8ax6z/bStJS3U61W1Gbmljb+J7n3iN6uTdPmlmfvY2ut/C/sj/OVcTtf8k2tXqQ+v8y3cfKticc747LXHW2c9a905r7u5OtnzJhxWM0XJ7Z15i5P03VyGzfZ96PM/Zi4s38utXvLs8u+/XMAALtIw/BBd1yakm4TURuun9u4Nhln1fyabr3lMZDG57RaXzFh55ukPZL77u7VZmhwcPDI5LeWPNe5nTUbWl7OV99W7Xjb13eO0jSVn3V/a+Oyd/mMo6OjJ9Txlgn1zWDWLig/D7c8MTt7jpe3cd19yx6ZW5Lrsjr+qs01mb+k3ZM9bs74qLZ36psyfqTOjY+NjQ137wUA2EVpJtLATC55/anv49RumjVr1gGllvG6XueNWmlC0mxcmjgx+cpOfWsnX524sebftfqeyLOvaI1OrjdkvLzWu2++tpTz1YZuXfkvWq0/mfilrSty3zltv+Tzkm+sP4/u+I9cPsfTbW2v8zNre27m19S5d+vUpLJfOWc962jbvyu1R9tZejv/Czcxey2t423D9f91I523ewAA+4w0KW/3R5qYVf3rAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA9g2/A8udAhy+0I6IAAAAAElFTkSuQmCC>

[image2]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAmwAAAAvCAYAAABexpbOAAAC+klEQVR4Xu3bO4hdVRQG4IlvRPA5Ru7cO3fOzIBhwEIGRVBSaCPxhZhGBEEQFAIKqQxiYaGiYGMlWhiDiZUptPGBjYVipYUWVoIPglqIhYha6L/Gs5nDqWJEGDLfB5u99tp7nzvlzznMwgIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADsEPP5/N5xDwCAHSSB7bNxDwCAHWJ5efmWjLvGfQAA/j/nrqysPJIQ9mnN1ZhMJhenPjKfzz9J/7H2CTTzB1l/U3ON9oDUBzI+HL95yzP2Zbyd/lsZxwb957P+OOOL1Levra3NUr+a+tGMp/Mb702n0+tS35P+ycw31b30rmg1AMCukUD0x2w2W6s6Qem5vvdX13XX9vWT6+vrF1adsHRR1r9v397a/zX9axK6rm6Br+//kvFM1XnuauoH+v63gzPX129VqOvXWyEw8/H0VnLv5vrbUj7RP+douwsAsCtMp9OlhKN3hr3JZHJVAtJHbZ2Q9O6gvq0CVlsnyC1m/WO9Fcv8Ysatba/OLS0tTfv64RYKh/f7N2hb6/5vOdz2mvzms+059XZvvA8AcFZLQDqQEHRnW3ddtz+9OyqY1Toh6+4KVOnfmPlQBbl6C7axsXFBfbasEJb5we0nbsvZ1/vyvNz5ofXr02ur0/958HbtlTrb9kr/Rm8r0OXe48OwBwCwKywuLl6SEPRC1aurq5emPpGQdFnC0cHqZf15haSMYxldxtcZL2X/5cz35cie1K8t/BO09qT3VHt2nvN+zfUZcxi0Up+qud7kpf5y8Bbuu3Zm4JzWz/x9xk/jAwAA/EcV9s70U+ZsNpsk0F1ZdYW+hMCHxmcAAPiXNjc3z0+4+q2tU3/Vdd3e4ZnTlbt/1ifXBLX7E95uGO8DAHCGErTeqH9WyHwy8+Xj/dOV+4czTmS8Od4DAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAOBv8DbG3jttdx/uGAAAAAElFTkSuQmCC>

[image3]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAmwAAAAvCAYAAABexpbOAAACTElEQVR4Xu3cz4uNURgHcAZFfm6mKc3cO/c2dRdKaYqNhEKxUdiILCysyNYkZaEsZSWllAXJjx0xfmTDZlYWFmTjD5CNhZH4Hp1Xb7dZqqE+n3o6z3mec953+/Q2c5csAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgH/cxMTExva+2+3eS/zsdDqb2nUAABZJGdAWqH0crgEAsAg6nc728jWtXev1eptTO9CuAQDwF2TIOpqYTVxPzDT1DGV3sn/e7CcnJ5/W9VR6n9KbL/f6/X6n1s+mfji1u4mHiS3N3VhW3zGXc+fq+cuJt4mDqT/O3SOt8wAAFBmU9idOlrzX6+3o1q9mGaJ2Zlma/YfW2T9f1JJ/S1xq9jm/Mvuvrf7rxP2az6b/vtX7nOHs5vj4+Kqs1waDwdqxsbHVqe9qzgAAUGVIejM1NbWu5jOJ+aaXIWtfe0grX9WavNQz4O1t9fYMDXTfUzvdnE2cb/V+JG7V/F1TBwBgyOjo6JqhIWs+Q9rFrFfLPgPXq+SPSp7hbGt6J1K7UM/O1jNPsizPfi7xoPWsF2Ut/0la3tHv99fX1kgZ/PKsDVmPt98PAMACMjC9rGkZuspPchwrUQr1b9ieJR3JejtD1rbUpuu9MtSVO4fqvtz9/UWtDILJz2TIG9Re+do2HSvyjCvZ7671G4kvJQcAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA4L/2CyOEb+y1CuseAAAAAElFTkSuQmCC>
