# async-book-ja
Asynchronous Programming in Rust 日本語版

- [Asynchronous Programming in Rust](https://github.com/rust-lang/async-book)の有志による日本語訳です
- 本レポジトリのオーナーは[個人](https://github.com/masan4444)で、Rust公式入門書の和訳を手掛けておられる
  [rust-lang-ja](https://github.com/rust-lang-ja)とは無関係ですが、
  レポジトリの運営方針や約語などは参考にしています
- 訳の追加、改善などは[Pull Request](https://github.com/masan4444/async-book-ja/pulls)までお願いします。
  簡単なtypoの修正から、章全体の和訳まで、大小様々なPull Requestを歓迎します。
  Pull Requestの書き方なども自由です
- ビルド済みドキュメントは本家同様、[Github Pages](https://masan4444.github.io/async-book-ja/)にホストされており、
  masterブランチにソースコードがコミットされると、自動的に更新されます

## Requirements
The async book is built with [`mdbook`], you can install it using cargo.

```
cargo install mdbook
cargo install mdbook-linkcheck
```

[`mdbook`]: https://github.com/rust-lang/mdBook

## Building
To create a finished book, run `mdbook build` to generate it under the `book/` directory.
```
mdbook build
```

## Development
While writing it can be handy to see your changes, `mdbook serve` will launch a local web
server to serve the book.
```
mdbook serve
```
