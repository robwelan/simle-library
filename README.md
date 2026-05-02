# **SIMLE: Simple Interactive Multilingual Learning Engine**

**SIMLE** is a high-performance, open-source technical documentation standard
designed for multilingual education.

## **🌐 Live Demo & Library**

[**Launch Live Demo & Library**](https://robwelan.github.io/simle-library/)

## (Opens in new tab)

## **📈 Project Evolution & Readiness**

The SIMLE engine is currently in **Pre-Alpha**.

👉

[**View the Shared Assets Changelog**](http://docs.google.com/shared-assets/CHANGELOG.md)

## **🚀 Local Development**

To test the library locally and avoid browser security restrictions (CORS) when
loading .json or .toml files, use a local HTTP server.

**Run using npx:**

```bash
npx http-server . \-p 8000 \--cors \-c-1
```

* **\-p 8000**: Runs the server on port 8000\.
* **\--cors**: Enables Cross-Origin Resource Sharing.
* **\-c-1**: Disables caching so you see your changes immediately.

Access your local library at: [http://localhost:8000](http://localhost:8000)

## **🛠 Developer Workflow (Important)**

### **1\. Compiling the Engine**

If you modify the Rust logic in src/lib.rs, you must rebuild the WebAssembly
binaries so the browser can see the changes.

```bash
# 1. Compile the Rust code to WASM
wasm-pack build --target web

# 2. Sync the output to your docs folder
# (Adjust paths if your structure differs slightly)
cp pkg/simle_engine_core_bg.wasm docs/wasm/
cp pkg/simle_engine_core.js docs/wasm/
```

### **2\. Note on Syncing**

This repository uses **GitHub Actions** to automatically compile the Rust/Wasm
engine on deployment.

When you push code to main, the GitHub Action may compile the engine and commit
the new .wasm binaries back to the repository. This will cause your local branch
to be out of sync.

**Always pull before pushing:**

```bash
git pull \--rebase origin main
```

### **3\. gh-pages new**

```bash
npx gh-pages -d . -m "Publish SIMLE Book"
```

## **🚀 Supporting the Project**

[**Help fund the SIMLE Project on Patreon**](https://patreon.com/BeAReactDev)

## **📖 How to Get the Books**

### **1\. The "Full Library" Experience (Git)**

```bash
git clone \[https://github.com/robwelan/simle-library.git\](https://github.com/robwelan/simle-library.git)
cd simle-library/books
```

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

*Primary Maintainer: [robwelan](https://github.com/robwelan)*
