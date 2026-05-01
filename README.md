# **SIMLE: Simple Interactive Multilingual Learning Engine**

**SIMLE** is a high-performance, open-source technical documentation standard designed for multilingual education.

## **🌐 Live Demo & Library**

[**Launch Live Demo & Library**](https://robwelan.github.io/simle-library/) *(Opens in new tab)*

## **📈 Project Evolution & Readiness**

The SIMLE engine is currently in **Pre-Alpha**.

👉 [**View the Shared Assets Changelog**](http://docs.google.com/shared-assets/CHANGELOG.md)

## **🛠 Developer Workflow (Important)**

This repository uses **GitHub Actions** to automatically compile the Rust/Wasm engine.

**Note on Syncing:**

When you push code to main, the GitHub Action may compile the engine and commit the new .wasm binaries back to the repository. This will cause your local branch to be out of sync.

**Always pull before pushing:**

git pull \--rebase origin main

## **🚀 Supporting the Project**

[**Help fund the SIMLE Project on Patreon**](https://patreon.com/BeAReactDev)

## **📖 How to Get the Books**

### **1\. The "Full Library" Experience (Git)**

git clone \[https://github.com/robwelan/simle-library.git\](https://github.com/robwelan/simle-library.git)  
cd simle-library/books

### **2\. Standalone Books (.simle)**

Download individual archives from our **Releases Page**.

## **🛠 Project Architecture**

* **18px Minimum Typography:** Cognitive accessibility floor.  
* **Shared Assets:** Versioned CSS, Fonts, and Wasm logic.  
* **Direct Shadowing:** Filename-based parallel synchronization.

## **📜 Key Documentation**

1. [Project Standard](http://docs.google.com/core-docs/simle-technical-standard/content/en-us/project_standard.md)  
2. [File Structure](http://docs.google.com/core-docs/simle-technical-standard/content/en-us/file_structure.md)  
3. [Locale Standards](http://docs.google.com/core-docs/simle-technical-standard/content/en-us/locales_standard.md)

## **🤝 Contributing**

1. Fork the repo.  
2. Create a feature branch.  
3. Submit a Pull Request.

## **⚖️ License**

MIT License \- see [LICENSE](http://docs.google.com/LICENSE).

*Standard Version: 1.2.0*

*Primary Maintainer: [robwelan](https://github.com/robwelan)*