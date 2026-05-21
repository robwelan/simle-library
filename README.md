# **SIMLE: Simple Interactive Multilingual Learning Engine**

**SIMLE** is a high-performance, open-source technical documentation standard
designed for multilingual education.

## **🌐 Live Demo & Library**

[**Launch Live Demo & Library**](https://robwelan.github.io/simle-library/)

## **📈 Project Evolution & Readiness**

The SIMLE engine is currently in **Pre-Alpha**.

👉
[**View the Shared Assets Changelog**](http://docs.google.com/shared_assets/CHANGELOG.md)

## **🚀 Local Development**

To test the library locally, use the built-in development script:

**Run using npm:**

```bash
npm run dev
```

Access your local library at: [http://localhost:3000](http://localhost:3000)

## **🛠 Developer Workflow**

### **1\. Building the Library Manifest**

If you add or remove files in the books/ directory, regenerate the JSON
registry:

python build\_library.py

### **2\. Compiling the Engine (Rust/WASM)**

If you modify the Rust logic in src/, you must rebuild the WebAssembly binaries:

```bash
npm run build:wasm
```

### **3\. Deployment**

Deployment is handled by the gh-pages utility. This command bundles the web/
folder along with wasm, books, and shared\_assets.

```bash
npm run deploy
```

## **🤖 Automated Workflow (CI/CD)**

This repository uses **GitHub Actions** to automatically manage the engine. When
you push to main:

1. The **Library Manifest** is updated via Python.
2. The **Rust/WASM** engine is compiled.
3. The **package.json** version is patched.
4. The site is deployed to the gh-pages branch.

**Always pull before working** to stay in sync with automated version bumps:

```bash
git pull --rebase origin main
```

## **📖 How to Get the Books**

### **1\. The "Full Library" Experience (Git)**

```bash
git clone
\[https://github.com/robwelan/simle-library.git\](https://github.com/robwelan/simle-library.git)
cd simle-library/books
```

### **2\. Standalone Books (.simle)**

Download individual archives from our **Releases Page**.

## **🛠 Project Architecture**

* **18px Minimum Typography:** Cognitive accessibility floor.
* **Shared Assets:** Versioned CSS, Fonts, and Wasm logic.
* **Web Directory:** Primary frontend entry point (web/index.html).
* **Rust Core:** Performance-critical logic compiled to WASM.

## **🚀 Supporting the Project**

[**Help fund the SIMLE Project on Patreon**](https://patreon.com/BeAReactDev)

## **⚖️ License**

MIT License \- see [LICENSE](http://docs.google.com/LICENSE).

*Primary Maintainer: [robwelan](https://github.com/robwelan)*
