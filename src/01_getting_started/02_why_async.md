<!-- # Why Async? -->

# なぜ非同期？

<!-- 
We all love how Rust empowers us to write fast, safe software.
But how does asynchronous programming fit into this vision?
-->

Rustは、高速で安全なソフトウェアを書くことを可能にします。
しかし、非同期プログラミングは、このビジョンにどのようにフィットするのでしょうか？

<!--
Asynchronous programming, or async for short, is a _concurrent programming model_
supported by an increasing number of programming languages.
It lets you run a large number of concurrent
tasks on a small number of OS threads, while preserving much of the
look and feel of ordinary synchronous programming, through the
`async/await` syntax.
-->

非同期（`訳注`: asynchronous）プログラミング、略してasyncは、
多くのプログラミング言語でサポートされている並行プログラミングモデルです。
`async/await` 構文によって、通常の同期型プログラミングのlook＆feelの多くを維持しながら
少数のOSスレッドで多数のタスクを同時に実行することができます。

<!-- ## Async vs other concurrency models -->

## 非同期 vs 他の並行処理モデル

<!--
Concurrent programming is less mature and "standardized" than
regular, sequential programming. As a result, we express concurrency
differently depending on which concurrent programming model
the language is supporting.
A brief overview of the most popular concurrency models can help
you understand how asynchronous programming fits within the broader
field of concurrent programming:
-->

並行プログラミングは、通常の逐次プログラミングに比べると成熟度が低く、「標準化」されていません。

<!-- - **OS threads** don't require any changes to the programming model,
  which makes it very easy to express concurrency. However, synchronizing
  between threads can be difficult, and the performance overhead is large.
  Thread pools can mitigate some of these costs, but not enough to support
  massive IO-bound workloads.
- **Event-driven programming**, in conjunction with _callbacks_, can be very
  performant, but tends to result in a verbose, "non-linear" control flow.
  Data flow and error propagation is often hard to follow.
- **Coroutines**, like threads, don't require changes to the programming model,
  which makes them easy to use. Like async, they can also support a large
  number of tasks. However, they abstract away low-level details that
  are important for systems programming and custom runtime implementors.
- **The actor model** divides all concurrent computation into units called
  actors, which communicate through fallible message passing, much like
  in distributed systems. The actor model can be efficiently implemented, but it leaves
  many practical issues unanswered, such as flow control and retry logic. -->

- **OS スレッド**はプログラミングモデルを変更する必要がないため、並行処理の表現が非常に簡単です。
  しかし、スレッド間の同期が難しい場合があり、パフォーマンスのオーバーヘッドも大きくなります。
  スレッドプールはこれらのコストを一部軽減することができますが、
  大量のI/Oバウンド（訳注：処理に掛かる時間がI/Oに依存する）なワークロードをサポートするには十分ではありません。

- **イベント駆動型プログラミング**は、_コールバック_ と組み合わせることで非常に高いパフォーマンスを発揮しますが、
  冗長で"非線形"な制御フローになる傾向があります。
  データの流れやエラーの伝搬が分かりにくいことが多いです。

- **コルーチン**は、スレッドのようにプログラミングモデルを変更する必要がないため、使い勝手が良いです。
  また、非同期と同様に、多数のタスクを扱うことができます。
  しかし、システムプログラミングやカスタムランタイムの実装を行う者にとって重要な
  低レベルの詳細が抽象化されています。

- **アクターモデル**は、すべての並行計算をアクターと呼ばれる単位に分割し、
  fallibleなメッセージの受け渡しによって通信を行います。
  アクターモデルは効率的な実装が可能ですが、フロー制御や再試行ロジックなど、
  実用上の課題が多く残されています。

<!--
In summary, asynchronous programming allows highly performant implementations
that are suitable for low-level languages like Rust, while providing
most of the ergonomic benefits of threads and coroutines.
-->

要約すると、非同期プログラミングは、Rustのような低級言語に適した高性能な実装を可能にする一方で、
スレッドやコルーチンの人間工学的な利点をほぼ満たすことができます。

<!-- ## Async in Rust vs other languages -->

## Rustでの非同期 vs 他の言語

<!--
Although asynchronous programming is supported in many languages, some
details vary across implementations. Rust's implementation of async
differs from most languages in a few ways:
-->

非同期プログラミングは多くの言語でサポートされていますが、詳細は実装によって異なります。
Rustの非同期の実装は、大多数の言語といくつかの点で異なっています:

<!-- 
- **Futures are inert** in Rust and make progress only when polled. Dropping a
  future stops it from making further progress.
- **Async is zero-cost** in Rust, which means that you only pay for what you use.
  Specifically, you can use async without heap allocations and dynamic dispatch,
  which is great for performance!
  This also lets you use async in constrained environments, such as embedded systems.
- **No built-in runtime** is provided by Rust. Instead, runtimes are provided by
  community maintained crates.
- **Both single- and multithreaded** runtimes are available in Rust, which have
  different strengths and weaknesses.
  -->

- Rustでは、**futureは不活性** で、ポーリングされたときだけ進行します。
  futureをドロップすると、それ以降の進行が停止します。

- Rustでは、**非同期はゼロコスト**です。つまり、使う分だけコストを支払えばいいのです。
  具体的には、ヒープ割り当てや動的ディスパッチなしで非同期が使えるようになります。
  これはパフォーマンスにとって素晴らしいことです！
  これにより、組み込みシステムのような制約のある環境でも非同期を使用することができます。

- Rustには、**組み込みのランタイムはありません**。
  その代わり、ランタイムはコミュニティがメンテナンスするクレートによって提供されます。

- Rustでは、**シングルスレッドとマルチスレッドの両方のランタイム**が利用可能であり、それぞれ長所と短所があります。

<!-- ## Async vs threads in Rust -->

## Rustでの非同期 vs スレッド

<!-- 
The primary alternative to async in Rust is using OS threads, either
directly through [`std::thread`](https://doc.rust-lang.org/std/thread/)
or indirectly through a thread pool.
Migrating from threads to async or vice versa
typically requires major refactoring work, both in terms of implementation and
(if you are building a library) any exposed public interfaces. As such,
picking the model that suits your needs early can save a lot of development time.
-->

Rustにおける非同期の主な代替手段は、OSのスレッドを使用することです。
[`std::thread`](https://doc.rust-lang.org/std/thread/)を直接使用するか、
スレッドプールを介して間接的に使用します。スレッドから非同期への移行、またはその逆は、
通常、実装と（ライブラリを構築している場合は）公開されているパブリックインターフェースの両方において、
大きなリファクタリング作業を必要とします。そのため、ニーズに合ったモデルを早めに選ぶことで、
開発期間を大幅に短縮することができます。

<!--
**OS threads** are suitable for a small number of tasks, since threads come with
CPU and memory overhead. Spawning and switching between threads
is quite expensive as even idle threads consume system resources.
A thread pool library can help mitigate some of these costs, but not all.
However, threads let you reuse existing synchronous code without significant
code changes—no particular programming model is required.
In some operating systems, you can also change the priority of a thread,
which is useful for drivers and other latency sensitive applications.
-->

**OS スレッド**はCPUとメモリのオーバーヘッドが伴うので、少数のタスクに適しています。
アイドル状態のスレッドでさえシステムリソースを消費するため、スレッドの生成と切り替えは、非常に高価です。
スレッドプールライブラリは、これらのコストの一部を軽減するのに役立ちますが、すべてではありません。
しかし、スレッドを使えば、既存の、同期的に動作するコードを大幅に変更することなく再利用することができ、
特定のプログラミングモデルは必要ありません。OSによっては、スレッドの優先度を変更することもできます。
これは、ドライバなど遅延が重要となるアプリケーションに便利な機能です。

<!--
**Async** provides significantly reduced CPU and memory
overhead, especially for workloads with a
large amount of IO-bound tasks, such as servers and databases.
All else equal, you can have orders of magnitude more tasks than OS threads,
because an async runtime uses a small amount of (expensive) threads to handle
a large amount of (cheap) tasks.
However, async Rust results in larger binary blobs due to the state
machines generated from async functions and since each executable
bundles an async runtime.
-->

**非同期**により、CPUとメモリのオーバーヘッドが大幅に削減されます。
特にサーバーやデータベースなど、I/Oバウンドなタスクが大量に発生するワークロードにおいて顕著です。
非同期ランタイムは、大量の（安価な）タスクを処理するために少量の（高価な）スレッドを使用するため、
条件が同じであれば、OSスレッドよりも桁外れに多くのタスクを扱うことができます。
しかし、非同期Rustは、非同期関数から生成されるステートマシンと、
各実行ファイルが非同期ランタイムを含むことにより、バイナリサイズが大きくなってしまいます。

<!--
On a last note, asynchronous programming is not _better_ than threads,
but different.
If you don't need async for performance reasons, threads can often be
the simpler alternative.
-->

最後に、非同期プログラミングはスレッドより _優れている_ わけではなく、異なるものです。
もしパフォーマンス上の理由で非同期を必要としないのであれば、スレッドの方がよりシンプルな選択肢になることが多いでしょう。

<!-- ### Example: Concurrent downloading -->

### 例: 並行ダウンロード

<!-- 
In this example our goal is to download two web pages concurrently.
In a typical threaded application we need to spawn threads
to achieve concurrency:
-->

この例では、2つのウェブページを同時にダウンロードすることを目標としています。
典型的なスレッドアプリケーションでは、並行処理を実現するためにスレッドを生成する必要があります:

```rust,ignore
{{#include ../../examples/01_02_why_async/src/lib.rs:get_two_sites}}
```

<!--
However, downloading a web page is a small task; creating a thread
for such a small amount of work is quite wasteful. For a larger application, it
can easily become a bottleneck. In async Rust, we can run these tasks
concurrently without extra threads:
-->

しかし、Webページのダウンロードは小さな作業であり、
このような小さな作業のためにスレッドを作成することは非常に無駄があります。
大規模なアプリケーションの場合、ボトルネックになりやすいのです。
非同期Rustでは、余分なスレッドなしにこれらのタスクを同時に実行することができます:

```rust,ignore
{{#include ../../examples/01_02_why_async/src/lib.rs:get_two_sites_async}}
```

<!-- 
Here, no extra threads are created. Additionally, all function calls are statically
dispatched, and there are no heap allocations!
However, we need to write the code to be asynchronous in the first place,
which this book will help you achieve.
-->

ここでは、余分なスレッドは生成されません。
さらに、すべての関数呼び出しは静的にディスパッチされ、ヒープの割り当てもありません！
しかし、そもそも非同期になるようにコードを書く必要があります。この本がその手助けになるでしょう。

<!-- ## Custom concurrency models in Rust -->

## Rustでのカスタム並行処理モデル

<!--
On a last note, Rust doesn't force you to choose between threads and async.
You can use both models within the same application, which can be
useful when you have mixed threaded and async dependencies.
In fact, you can even use a different concurrency model altogether,
such as event-driven programming, as long as you find a library that
implements it.
-->

最後に、Rustはスレッドか非同期かの二者択一を迫るものではありません。
同じアプリケーション内で両方のモデルを使用することができます。
スレッド依存と非同期依存が混在している場合に便利です。
実際、イベント駆動型プログラミングのような全く別の並行処理モデルも、
それを実装したライブラリさえあれば使うことができます。
