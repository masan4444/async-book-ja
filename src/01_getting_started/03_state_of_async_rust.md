<!-- # The State of Asynchronous Rust -->

# 非同期Rustの現状

<!--
Parts of async Rust are supported with the same stability guarantees as
synchronous Rust. Other parts are still maturing and will change
over time. With async Rust, you can expect:
-->

非同期Rustは、同期Rustと同様の安定性保証が一部サポートされています。
その他の部分はまだ成熟中であり、時間の経過とともに変化していくでしょう。
非同期Rustにおいて、予想されることは:

<!--
- Outstanding runtime performance for typical concurrent workloads.
- More frequent interaction with advanced language features, such as lifetimes
  and pinning.
- Some compatibility constraints, both between sync and async code, and between
  different async runtimes.
- Higher maintenance burden, due to the ongoing evolution of async runtimes
  and language support.
-->

- 典型的な並行処理ワークロードに対する卓越したランタイム性能。

- ライフタイムやピン留めといった高度な言語機能とのインタラクションをより頻繁に行うことができます。

- 同期と非同期のコード間、および異なる非同期ランタイム間に一部互換性の制約があります。

- 非同期ランタイムや言語サポートが進化し続けるため、メンテナンスの負担が大きいです。

<!--
In short, async Rust is more difficult to use and can result in a higher
maintenance burden than synchronous Rust,
but gives you best-in-class performance in return.
All areas of async Rust are constantly improving,
so the impact of these issues will wear off over time.
-->

つまり、非同期Rustは、同期Rustよりも扱いにくく、メンテナンスの負担も大きくなります。
その代わり、最高クラスのパフォーマンスを得ることができます。
非同期Rustは、すべての領域において、常に改善され続けています。
そのため、これらの問題の影響は時間の経過とともに薄れていくでしょう。

<!-- ## Language and library support -->

## 言語・ライブラリの対応

<!--
While asynchronous programming is supported by Rust itself,
most async applications depend on functionality provided
by community crates.
As such, you need to rely on a mixture of
language features and library support:
-->

非同期プログラミングはRust自体でサポートされていますが、ほとんどの非同期アプリケーションは、
コミュニティのクレートが提供する機能に依存しています。
そのため、言語機能とライブラリのサポートを組み合わせて利用する必要があります:

<!--
- The most fundamental traits, types and functions, such as the
  [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) trait
  are provided by the standard library.
- The `async/await` syntax is supported directly by the Rust compiler.
- Many utility types, macros and functions are provided by the
  [`futures`](https://docs.rs/futures/) crate. They can be used in any async
  Rust application.
- Execution of async code, IO and task spawning are provided by "async
  runtimes", such as Tokio and async-std. Most async applications, and some
  async crates, depend on a specific runtime. See
  ["The Async Ecosystem"](../08_ecosystem/00_chapter.md) section for more
  details.
-->

- [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html)トレイトのような
  基本的なトレイト、型、関数は標準ライブラリで提供されています。

- `async/await`構文は、Rustコンパイラで直接サポートされています。

- 多くの有用な型、マクロ、関数が[`futures`](https://docs.rs/futures/)クレートによって提供されています。
  これらは全ての非同期Rustアプリケーションで使用することができます。

- 非同期コードの実行、I/O、タスクの起動は、Tokio や async-std のような
  "非同期ランタイム" によって提供されます。
  ほとんどの非同期アプリケーションといくつかの非同期クレートは、特定のランタイムに依存しています。
  詳しくは["非同期エコシステム"](../08_ecosystem/00_chapter.md)の項をご覧ください。

<!--
Some language features you may be used to from synchronous Rust are not yet
available in async Rust. Notably, Rust does not let you declare async
functions in traits. Instead, you need to use workarounds to achieve the same
result, which can be more verbose.
-->

同期Rustで使い慣れた言語機能の中には、非同期Rustではまだ利用できないものがあります。
特筆すべき点として、トレイト内で非同期関数を宣言することができません。
その代わりに、同じ結果を得るために回避策を使用する必要があり、より冗長になる可能性があります。

## Compiling and debugging

For the most part, compiler- and runtime errors in async Rust work
the same way as they have always done in Rust. There are a few
noteworthy differences:

### Compilation errors

Compilation errors in async Rust conform to the same high standards as
synchronous Rust, but since async Rust often depends on more complex language
features, such as lifetimes and pinning, you may encounter these types of
errors more frequently.

### Runtime errors

Whenever the compiler encounters an async function, it generates a state
machine under the hood. Stack traces in async Rust typically contain details
from these state machines, as well as function calls from
the runtime. As such, interpreting stack traces can be a bit more involved than
it would be in synchronous Rust.

### New failure modes

A few novel failure modes are possible in async Rust, for instance
if you call a blocking function from an async context or if you implement
the `Future` trait incorrectly. Such errors can silently pass both the
compiler and sometimes even unit tests. Having a firm understanding
of the underlying concepts, which this book aims to give you, can help you
avoid these pitfalls.

## Compatibility considerations

Asynchronous and synchronous code cannot always be combined freely.
For instance, you can't directly call an async function from a sync function.
Sync and async code also tend to promote different design patterns, which can
make it difficult to compose code intended for the different environments.

Even async code cannot always be combined freely. Some crates depend on a
specific async runtime to function. If so, it is usually specified in the
crate's dependency list.

These compatibility issues can limit your options, so make sure to
research which async runtime and what crates you may need early.
Once you have settled in with a runtime, you won't have to worry
much about compatibility.

## Performance characteristics

The performance of async Rust depends on the implementation of the
async runtime you're using.
Even though the runtimes that power async Rust applications are relatively new,
they perform exceptionally well for most practical workloads.

That said, most of the async ecosystem assumes a _multi-threaded_ runtime.
This makes it difficult to enjoy the theoretical performance benefits
of single-threaded async applications, namely cheaper synchronization.
Another overlooked use-case is _latency sensitive tasks_, which are
important for drivers, GUI applications and so on. Such tasks depend
on runtime and/or OS support in order to be scheduled appropriately.
You can expect better library support for these use cases in the future.
