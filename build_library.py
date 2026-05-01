import os
import json
import tomllib

# ==========================================
# CONFIGURATION / CONSTANTS
# ==========================================

# Since the script is inside /src/, and books/docs are also in /src/:
SEARCH_DIR = "books"
OUTPUT_DIR = "docs"

# Internal Book Structure
BOOK_TOML_NAME = "book.toml"
CONTENT_DIR_NAME = "content"
META_TOML_NAME = "meta.toml"
ASSETS_DIR_NAME = "assets"
IMAGES_DIR_NAME = "images"

# Fallbacks
DEFAULT_ICON = "📚"
DEFAULT_THEME = "#0984e3"

def get_book_metadata(book_folder_path):
    """Parses a book directory for metadata and localized strings."""
    book_toml_path = os.path.join(book_folder_path, BOOK_TOML_NAME)
    if not os.path.exists(book_toml_path):
        return None

    try:
        # 1. Load the main configuration
        with open(book_toml_path, "rb") as f:
            book_config = tomllib.load(f)

        project = book_config.get("project", {})
        book_id = project.get("id", "unknown-id")

        # 2. Determine the locale for the preview
        structure = book_config.get("structure", {})
        locales = structure.get("locales", [])

        if not locales:
            print(f"  [!] No locales defined in {book_toml_path}")
            return None

        default_locale = locales[0]

        # 3. Handle localized metadata
        meta_path = os.path.join(
            book_folder_path,
            CONTENT_DIR_NAME,
            default_locale,
            META_TOML_NAME
        )

        title = project.get("title", "Unknown Title")
        author = project.get("author", "Unknown Author")
        description = "No description available."

        if os.path.exists(meta_path):
            with open(meta_path, "rb") as f:
                meta_data = tomllib.load(f)
                title = meta_data.get("title", title)
                author = meta_data.get("author", author)
                description = meta_data.get("description", description)

        # 4. Resolve Image Filenames
        # Note: We only store the filename here to avoid path duplication in the UI
        cover_cfg = book_config.get("cover", {})
        ls_filename = cover_cfg.get("cover_background_landscape", "")
        pt_filename = cover_cfg.get("cover_background_portrait", "")

        return {
            "id": book_id,
            "version": project.get("version", "1.0.0"),
            "title": title,
            "author": author,
            "description": description,
            "icon": project.get("icon", DEFAULT_ICON),
            "cover_color": project.get("theme_color", DEFAULT_THEME),
            "path": os.path.basename(book_folder_path),
            "active_locale": default_locale,
            "landscape": ls_filename, # Pure filename
            "portrait": pt_filename    # Pure filename
        }

    except Exception as e:
        print(f"  [!] Error processing {book_folder_path}: {e}")
        return None

def build_library():
    """Scans the search directory and writes library.json."""

    # Get the directory where THIS script is located
    script_dir = os.path.dirname(os.path.abspath(__file__))

    # Resolve absolute paths to sibling folders
    abs_search_dir = os.path.join(script_dir, SEARCH_DIR)
    abs_output_dir = os.path.join(script_dir, OUTPUT_DIR)

    print("--- PATH DEBUGGING ---")
    print(f"Script Dir: {script_dir}")
    print(f"Looking in: {abs_search_dir}")
    print(f"Writing to: {abs_output_dir}")
    print("----------------------")

    if not os.path.exists(abs_search_dir):
        print(f"ERROR: '{abs_search_dir}' not found.")
        return

    if not os.path.exists(abs_output_dir):
        print(f"Creating directory: {abs_output_dir}")
        os.makedirs(abs_output_dir)

    library = []

    # Iterate through folders in /src/books/
    for item in sorted(os.listdir(abs_search_dir)):
        book_folder = os.path.join(abs_search_dir, item)
        if os.path.isdir(book_folder):
            if os.path.exists(os.path.join(book_folder, BOOK_TOML_NAME)):
                print(f"--> Found: {item}")
                metadata = get_book_metadata(book_folder)
                if metadata:
                    library.append(metadata)

    # Output to /src/docs/library.json
    output_file = os.path.join(abs_output_dir, "library.json")
    with open(output_file, "w", encoding="utf-8") as f:
        json.dump(library, f, indent=4, ensure_ascii=False)

    print("-" * 30)
    print(f"Success: Generated {output_file} with {len(library)} books.")

if __name__ == "__main__":
    build_library()
