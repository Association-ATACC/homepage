# public/

Everything cargo-leptos finds here is copied as-is to the site root, so
files land at `/<filename>`.

## Logo

The header references `/atacc_logo.svg`. I couldn't fetch the binary SVG
content from `https://atacc.org/atacc_logo.svg` while scaffolding this
project (my web-fetch tool can retrieve pages but not image bytes), so
that file isn't here yet.

Drop the real file at `public/atacc_logo.svg` (same filename, or update
the `src` in `src/components/header.rs` if you rename it) and it'll show
up automatically — no code changes needed beyond that.
