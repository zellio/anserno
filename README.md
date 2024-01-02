# Anserno

Light, opinionated web view for a [Calibre][1] library.

## Usage

Anserno is designed to serve a read-only view of a Calibre library. That's it.
If you want access controls, put it behind a proxy. If you want faster
downloads, put it behind a CDN. If you want to read your ebooks, use calibre.

## Usage

Either build directly via cargo:

```bash
cargo build --release
```

And serve a local library:

```bash
target/release/anserno-server --library-url file:///path/to/library
```

Or download a release and do the same:

```bash
curl --url https://github.com/zellio/anserno/releases/download/$VERSION/anserno-server-x86_64-unknown-linux-gnu --output anserno-server

chmod +x anserno-server

anserno-server --library-url file:///path/to/library
```

## Contributing

Bug reports and pull requests are welcome on GitHub at
https://github.com/[USERNAME]/anserno. This project is intended to be a safe,
welcoming space for collaboration, and contributors are expected to adhere to
the [Contributor Covenant](http://contributor-covenant.org) code of conduct.

## Copyright

The MIT License (MIT)
Copyright (C) 2023 Zachary Elliott <contact(at)zell.io>

[1]: https://calibre-ebook.com/
