# **SIMLE: Simple Interactive Multilingual Learning Engine**

**SIMLE** is a high-performance, open-source technical documentation standard designed for multilingual education. It solves the "Parallel Content" problem by allowing users to view multiple languages, phonetic scripts (like Pinyin or Romaji), and technical notations (IPA) side-by-side in a synchronized, distraction-free environment.

## **🚀 Supporting the Project**

This project is built to provide high-quality, accessible technical knowledge to the world for free. If you find this standard useful or would like to support the ongoing development of the engine and the documentation, please consider becoming a patron.

[**Help fund the SIMLE Project on Patreon**](https://patreon.com/BeAReactDev?utm_medium=unknown&utm_source=join_link&utm_campaign=creatorshare_creator&utm_content=copyLink)

## **📖 How to Get the Books**

There are two primary ways to consume SIMLE content depending on your needs:

### **1\. The "Full Library" Experience (Git)**

Best for developers, contributors, or students who want a synchronized local library that updates as we release new content.

\# Clone the entire repository  
git clone \[https://github.com/your-repo/simle-library.git\](https://github.com/your-repo/simle-library.git)

\# Navigate to the books directory  
cd simle-library/books

*By cloning the repo, you get the shared-assets folder (fonts/styles) which allows all books to load instantly using a local cache.*

### **2\. Standalone Books (.simle)**

Best for quick reading or mobile use. You can download individual .simle files from our \[Releases Page\].

* **Note:** A .simle file is a ZIP archive containing the book's specific logic.  
* **Usage:** Drag and drop the .simle file into any compatible SIMLE reader app or web interface.

## **🛠 Project Architecture**

SIMLE is built on a **Universal Base Plate** philosophy:

* **18px Minimum Typography:** Designed for cognitive accessibility and mobile-first readability.  
* **Shared Assets:** All books link to a common directory for fonts (**IBM Plex**, **STIX Two**, **Noto**) and CSS logic.  
* **Direct Shadowing:** Multilingual content is synced by matching filenames across locale-specific folders.

### **Directory Overview**

* /shared-assets: The global engine (Fonts, CSS, Standard Logic).  
* /books: The library of individual project folders.  
* /books/simle-technical-standard: The official documentation for the SIMLE standard (rendered as a SIMLE book).

## **📜 Key Documentation**

The technical specification for this project is maintained as a SIMLE book. You can find the source files for the standard here:

1. [Project Standard](http://docs.google.com/books/simle-technical-standard/content/en-us/project_standard.md) \- The "Golden Rules."  
2. [File Structure](http://docs.google.com/books/simle-technical-standard/content/en-us/file_structure.md) \- How to organize a .simle archive.  
3. [Locale Standards](http://docs.google.com/books/simle-technical-standard/content/en-us/locales_standard.md) \- Naming rules for BCP 47 and Pinyin tracks.  
4. [Typography Guide](http://docs.google.com/books/simle-technical-standard/content/en-us/typography_standard.md) \- Our rationale for high-readability font selection.

## **🤝 Contributing**

We welcome contributions to the standard and the translation tracks\!

1. Fork the repo.  
2. Create a new branch for your locale or content.  
3. Submit a Pull Request.

## **⚖️ License**

This project is licensed under the MIT License \- see the [LICENSE](http://docs.google.com/LICENSE) file for details.

*Standard Version: 1.2.0*

*Primary Maintainer: [robwelan](https://github.com/robwelan)*