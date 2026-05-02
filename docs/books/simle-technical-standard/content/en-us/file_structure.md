# **Architecture: The SIMLE File Structure**

The **SIMLE** (Simple Interactive Multilingual Learning Engine) format is a specialized ZIP-compressed archive (using the .simle extension). It utilizes a dual-manifest system to decouple global project logic from localized identity.

## **1\. Global Controller: book.toml**

The book.toml file resides at the root of the archive. It acts as the master blueprint, defining technical identifiers and the **logical reading order**.

**Location:** /book.toml

\# Root Manifest: Global identity for the entire publication  
\[project\]  
id \= "physics-101"              \# Unique slug/ID for library indexing  
isbn \= "978-3-16-148410-0"      \# Global ISBN (one per .simle file)  
version \= "2.1.0"               \# Current edition version

\[structure\]  
\# The engine scans these folders in the /content/ directory.  
\# The first locale in this list is the default view and primary language.  
\# Naming Standard: ISO 639-1 & BCP 47  
locales \= \["en-us", "zh-hans", "ar-eg"\]

\# THE MASTER SEQUENCE: Defines the exact page order.  
\# Filenames must exist in every folder listed above.  
content \= \[  
  "project\_standard.md",   
  "file\_structure.md"  
\]

## **2\. Localized Identity: meta.toml**

**Requirement:** Every language directory listed in locales **must** contain its own meta.toml.

**Location:** /content/\[locale\]/meta.toml

### **2.1 Metadata Fields**

| Field | Type | Description |
| :---- | :---- | :---- |
| locale\_name | String | **Native Label:** The language name as it appears in the UI switcher (e.g., "日本語" instead of "ja-jp"). |
| title | String | The translated title of the book. |
| subtitle | String | (Optional) The translated subtitle. |
| author | String | The author's name (localized or romanized). |
| translator | String | (Optional) Credit for this specific language track. |
| description | String | A short localized blurb for the library view. |
| publisher | String | Localized publisher branch name. |
| keywords | Array | Localized search terms for the reader engine. |

**Note on Redundancy:** While the folder name (e.g., zh-hans) identifies the locale to the *system*, the locale\_name field is required for the *user interface*. It ensures that a user can find "简体中文" in a menu even if they don't know the BCP 47 code.

## **3\. Media Assets & Rich Content**

Media is handled via the /assets/ directory or external links to maintain performance.

### **3.1 The Assets Folder**

Located at the root: /assets/. This is the **Single Source of Truth** for media.

* **Images:** Must be in **WebP** format only.  
* **Audio:** Supports **MP3** or **Ogg** (Opus) in /assets/audio/.  
* **Referencing:** Images are linked in Markdown using relative paths: \!\[\](../../assets/diagram.webp).

### **3.2 Shared Assets & Styles**

Shared styles and fonts are stored in a root-level shared\_assets folder (often outside the specific .simle file in a repository).

* **Linking:** Content files hook into the "Universal Base Plate" CSS.  
* **Layout Safety Nets:** Any LaTeX formula or Markdown table is automatically wrapped in a scroll-x container to prevent layout breakage on mobile devices.

## **4\. Directory Visual Map**

SIMLE uses **Direct Shadowing**. Filenames in content/ must be identical across all locale folders.

📂 physics.simle (Zip Archive)  
├── 📄 book.toml            \<-- Global Identifiers  
├── 📂 assets/              \<-- Universal Media (WebP, MP3)  
└── 📂 content/  
    ├── 📂 en-us/  
    │   ├── 📄 meta.toml    \<-- English Identity ("English")  
    │   ├── 📄 project\_standard.md  
    │   └── 📄 file\_structure.md  
    └── 📂 zh-hans/  
        ├── 📄 meta.toml    \<-- Chinese Identity ("简体中文")  
        ├── 📄 project\_standard.md  
        └── 📄 file\_structure.md

## **5\. Critical Implementation Rules**

1. **Direct Shadowing:** The engine fetches the same filename from active locale folders simultaneously for parallel viewing.  
2. **Invalid Formats:** The engine errors out on .jpg, .png, or .gif to enforce the WebP standard.  
3. **Distribution:** Books can be consumed as standalone .simle (ZIP) files or by cloning the entire Git repository for a synchronized local library.