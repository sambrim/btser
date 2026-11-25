# Bulk TV Show Episode Renamer

A lightweight Rust utility designed to automatically parse and standardize TV show video filenames in your local directory. It uses regular expressions to detect various episode naming conventions (such as `S01E10`, `1x10`, or `Season 1 Episode 10`) and renames them into a clean, uniform `sXXeYY` format.

---

## Features

* **Flexible Pattern Matching:** Detects multiple common naming conventions using regular expressions, including standard tags, written-out formats, multiplier formats, and more.
* **Standardized Naming:** Automatically converts detected season and episode numbers into a neat, zero-padded format (`s01e05.ext`).
* **Conflict Prevention:** Checks if the target filename already exists or matches the current name before renaming, preventing accidental overwrites or duplicate errors.
* **Safe Skipping:** Gracefully skips files that do not contain recognizable season/episode indicators or are already correctly formatted.

---

## Supported Formats

The script detects and processes video files with the following extensions:

* `.mkv`
* `.mp4`
* `.avi`
* `.mov`
* `.wmv`
* `.m4v`
* `.ts`
* `.m2ts`
* `.webm`
* `.flv`

---

## How to Use

* Run the script in the same directory as the video files.

```bash
btser

```

*(Note: Replace `script_name.py` with the actual filename of your script).*
