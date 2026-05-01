import os
import json
import tomllib

def get_book_metadata(book_folder_path):
    """
    Parses books/[folder]/book.toml to find the locale list,
    then looks in books/[folder]/content/[locale]/meta.toml for details.
    """
    book_toml_path = os.path.join(book_folder_path, "book.toml")
    if not os.path.exists(book_toml_path):
        return None

    try:
        # 1. Load the main book.toml
        with open(book_toml_path, "rb") as f:
            book_config = tomllib.load(f)

        project = book_config.get("project", {})
        book_id = project.get("id", "unknown-id")

        # 2. Determine the locale (first item in locales array)
        structure = book_config.get("structure", {})
        locales = structure.get("locales", [])

        if not locales:
            print(f"  [!] No locales defined in {book_toml_path}")
            return None

        default_locale = locales[0]

        # 3. Path to meta.toml: books/[folder]/content/[locale]/meta.toml
        content_dir = os.path.join(book_folder_path, "content", default_locale)
        meta_path = os.path.join(content_dir, "meta.toml")

        title = project.get("title", "Unknown Title")
        author = "Unknown Author"
        description = "No description available."

        if os.path.exists(meta_path):
            with open(meta_path, "rb") as f:
                meta_data = tomllib.load(f)
                title = meta_data.get("title", title)
                author = meta_data.get("author", author)
                description = meta_data.get("description", description)
        else:
            print(f"  [!] Missing meta.toml at: {meta_path}")

        # 4. Resolve Cover Images
        cover_cfg = book_config.get("cover", {})
        ls_rel = cover_cfg.get("cover_background_landscape", "")
        pt_rel = cover_cfg.get("cover_background_portrait", "")

        # Use forward slashes for web compatibility
        cover_ls = f"content/{default_locale}/{ls_rel}".replace("\\", "/")
        cover_pt = f"content/{default_locale}/{pt_rel}".replace("\\", "/")

        return {
            "id": book_id,
            "version": project.get("version", "1.0.0"),
            "title": title,
            "author": author,
            "description": description,
            # Ensure path is relative to the library root
            "path": book_folder_path.replace("\\", "/"),
            "active_locale": default_locale,
            "cover": {
                "landscape": cover_ls,
                "portrait": cover_pt
            }
        }

    except Exception as e:
        print(f"  [!] Error processing {book_folder_path}: {e}")
        return None

def build_library(search_dir="books", output_dir="docs"):
    """
    Scans the 'books' directory for folders containing book.toml
    and writes library.json to the 'docs' directory.
    """
    if not os.path.exists(search_dir):
        print(f"Error: Directory '{search_dir}' not found.")
        return

    # Ensure the output directory (docs) exists
    if not os.path.exists(output_dir):
        print(f"Creating directory: {output_dir}")
        os.makedirs(output_dir)

    print(f"Scanning for books in: {os.path.abspath(search_dir)}")
    library = []

    # Iterate through folders inside 'books/'
    for item in os.listdir(search_dir):
        book_folder = os.path.join(search_dir, item)
        if os.path.isdir(book_folder):
            if os.path.exists(os.path.join(book_folder, "book.toml")):
                print(f"--> Found book folder: {item}")
                metadata = get_book_metadata(book_folder)
                if metadata:
                    library.append(metadata)

    # Use a dictionary as the root object
    output_data = library

    # Target path: docs/library.json
    output_file = os.path.join(output_dir, "library.json")

    with open(output_file, "w", encoding="utf-8") as f:
        json.dump(output_data, f, indent=4, ensure_ascii=False)

    print("-" * 30)
    print(f"Success: Generated {output_file} with {len(library)} books.")

if __name__ == "__main__":
    # Assumes script is run from project root where /books and /docs live
    build_library("books", "docs")
