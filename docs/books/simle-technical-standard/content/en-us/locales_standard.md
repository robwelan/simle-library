# **Language & Locale Standard: Transliteration & Romanization**

The **SIMLE** engine treats transliteration as a first-class citizen. This document defines the standards for "Romanization" (phonetic tracks), maps supported languages, and establishes the naming convention for directory structures to ensure perfect synchronization across the ecosystem.

## **1\. Naming Standard: ISO 639-1 & BCP 47**

To ensure global interoperability and prevent directory conflicts, the SIMLE format strictly adheres to **ISO 639-1** (two-letter language codes) and **BCP 47** (language tags with script and regional sub-tags).

### **1.1 Directory Structure Rules**

All folders within the /content/ directory must follow these naming conventions:

* **Primary Language:** Generic two-letter codes (e.g., en, fr) are discouraged for major languages. Authors must specify a region or script.  
* **Regional Variation:** Use the language code followed by a hyphen and a regional sub-tag (e.g., en-us, en-gb, pt-br).  
* **Script Distinction:** Mandatory for languages with multiple writing systems (e.g., zh-hans for Simplified, zh-hant for Traditional).  
* **Phonetic Tracks:** Use the language code followed by a custom suffix (e.g., zh-py for Pinyin).

## **2\. The "Phonetic Track" Standard**

When a script (like Chinese, Japanese, or Arabic) is converted into the Latin alphabet for pronunciation, we use specific **Romanization** codes. SIMLE handles these as separate, parallel content folders.

### **2.1 Global Romanization (Standard)**

| Source Language | System Name | Short Notation | Example |
| :---- | :---- | :---- | :---- |
| **Chinese** | **Pinyin** | zh-py | Nǐ hǎo |
| **Japanese** | **Hepburn Romaji** | ja-ro | Konnichiwa |
| **Arabic** | **ALA-LC / Roman** | ar-ro | Marhaba |
| **Korean** | **Revised Romanization** | ko-ro | Annyeong |

### **2.2 Localized Phonetic Adaptations**

SIMLE supports language-specific phonetic tracks to better match the native phonetics of the reader.

* **zh-de-py**: Pinyin adapted for German speakers (e.g., using "dsch" for "j").  
* **ja-fr-ro**: Romaji adapted for French phonetic rules.

## **3\. The IPA Fallback System**

The **IPA (International Phonetic Alphabet)** is a specialized technical track used when standard Romanization is insufficient or when the primary language already uses a Latin script.

### **3.1 Scientific Pronunciation vs. Romanization**

While Romanization (like Pinyin) is designed for general readability, the IPA track is designed for **scientific accuracy**.

* **The Problem:** Standard Latin-script languages (English, French, German) don't have a "Romanization" because they are already Romanized. However, their spelling often doesn't match their sounds (e.g., the English "thought").  
* **The Solution:** We use the /content/ipa/ directory as a universal "fallback" to provide the exact phonetic transcription using the International Phonetic Alphabet.

### **3.2 Implementation Example**

In a multilingual book, the engine can display the primary language alongside the IPA track to help the student master the sounds.

| Folder Path | Context | Content Example |
| :---- | :---- | :---- |
| /content/en-us/ | Primary Text | "Through the woods." |
| /content/ipa/ | **Phonetic Fallback** | /θruː ðə wʊdz/ |

## **4\. Implementation Guide: Chinese Tracks**

Because Chinese has various scripts and phonetic systems, authors must be precise with folder naming:

* **/zh-hans/**: Simplified Chinese (Mainland China, Singapore).  
* **/zh-hant/**: Traditional Chinese (Taiwan, Hong Kong).  
* **/zh-py/**: Standard Mandarin Pinyin (with tone marks).  
* **/zh-yue/**: Cantonese (Traditional script).

## **5\. Standard Locale Directory Naming (Common List)**

| Category | Locale Directory | Description |
| :---- | :---- | :---- |
| **Native** | en-us | English (United States) |
| **Native** | zh-hans | Simplified Chinese |
| **Native** | ar-eg | Arabic (Egyptian) |
| **Phonetic** | zh-py | Universal Pinyin (Chinese) |
| **Phonetic** | ja-ro | Universal Romaji (Japanese) |
| **Technical** | ipa | **Universal Phonetic Fallback** |

## **6\. Rule: No "Country-Only" Folders**

Folders named solely after a country (e.g., /us/, /cn/, /gb/) are **invalid**. The engine requires the language prefix to correctly load the appropriate font stacks and text-direction (LTR/RTL) logic.