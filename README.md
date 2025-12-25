scrytui
=======

A fast Terminal User Interface (TUI) for browsing Magic: The Gathering cards
using the Scryfall API.

Installation
------------
### From crates.io (coming soon)
        cargo install scrytui

### From source
        cargo install --git https://github.com/Njoter/scrytui

Features
--------
*   Search MTG cards by name
*   Display card details

Usage
-----
### search for cards
        scrytui "pack rat"

### Output JSON
        scrytui "Rodolf" --json

### Pagination
        scrytui "Swamp --page 2"

License
-------
This project is licensed under either of

*   [Apache License, Version 2.0](LICENSE-APACHE)
*   [MIT License](LICENSE-MIT)

at your option.
