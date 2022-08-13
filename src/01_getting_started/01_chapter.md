<!-- # Getting Started -->

# はじめに

> 訳注: この本は[Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/index.html)の***非公式***な日本語版です

> 訳注: この本は[一人](https://github.com/masan4444)で翻訳したもので、他の誰によっても校正・チェックされていません。
> 例えば、情報が間違っていたり、言葉が不自由だったり、他の種類の間違いがあるかもしれません。
> 訳の改善は、[Pull Request](https://github.com/masan4444/async-book-ja/pulls)まで。

<!-- 
Welcome to Asynchronous Programming in Rust! If you're looking to start writing
asynchronous Rust code, you've come to the right place. Whether you're building
a web server, a database, or an operating system, this book will show you
how to use Rust's asynchronous programming tools to get the most out of your
hardware. 
-->

Rustでの非同期プログラミングへようこそ！
もしあなたが非同期のRustコードを書き始めたいとお考えなら、この本が最適です。
Webサーバ、データベース、オペレーティングシステムなどを構築する場合において、
Rustの非同期プログラミングツールを使用してハードウェアを最大限に活用する方法を紹介します。

<!-- ## What This Book Covers -->

## この本で扱う内容

<!--
This book aims to be a comprehensive, up-to-date guide to using Rust's async
language features and libraries, appropriate for beginners and old hands alike.
-->

この本は、Rustの非同期言語機能とライブラリを利用するための
包括的で最新のガイドとなることを目的としており、初心者から熟練者まで幅広く対応しています。

<!--
- The early chapters provide an introduction to async programming in general,
and to Rust's particular take on it.-->

- 序章では、一般的な非同期プログラミングの紹介と、Rust特有の取り組みについて紹介します。

<!-- 
- The middle chapters discuss key utilities and control-flow tools you can use
when writing async code, and describe best-practices for structuring libraries
and applications to maximize performance and reusability.
-->

- 中章では、非同期コードを書く際に使用する主要なユーティリティと制御フローツールについて説明します。
また、性能と再利用性を最大化するためのライブラリやアプリケーションの構造化に関するベストプラクティスを解説します。

<!-- 
- The last section of the book covers the broader async ecosystem, and provides
a number of examples of how to accomplish common tasks.
-->

- この本の最後のセクションでは、より広範な非同期エコシステムを扱い、
一般的なタスクをどのように達成するかについて多くの例を示します。

<!-- 
With that out of the way, let's explore the exciting world of Asynchronous
Programming in Rust!
-->

それでは、Rustでの非同期プログラミングの世界を覗いてみましょう！
