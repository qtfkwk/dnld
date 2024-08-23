# About

Rust library for simple downloading

# Usage

```Rust
// Create a client
let download = dnld::Client::new("dnld").unwrap();

// Download a URL to a string
let url = "https://server.tld/path/to/file";
//let contents = download.to_string(url).unwrap();

// Download a URL to a file
let dir = std::path::Path::new(".");
//let path = download.to_file(url, Some(&dir)).unwrap();
```

# Changelog

Please find the [`CHANGELOG.md`] in the [repository].

[`CHANGELOG.md`]: https://github.com/qtfkwk/dnld/blob/main/CHANGELOG.md
[repository]: https://github.com/qtfkwk/dnld/

