<!-- # `async`/`.await` Primer -->

# `async`/`.await` 入門 

<!-- `async`/`.await` is Rust's built-in tool for writing asynchronous functions
that look like synchronous code. `async` transforms a block of code into a
state machine that implements a trait called `Future`. Whereas calling a
blocking function in a synchronous method would block the whole thread,
blocked `Future`s will yield control of the thread, allowing other
`Future`s to run. -->

`async`/`.await` は、Rust の組み込みツールで、非同期関数でありながら同期的にふるまう処理を記述できます。`async` はコードブロックを `Future` トレイトを実装したステートマシンに変換します。  
一方で、同期メソッドでブロッキング関数を呼び出すと、スレッド全体がブロックされてしまいます。ブロックされた `Future` はスレッドの制御を放棄し、他の `Future` が実行できるようになります。


<!-- Let's add some dependencies to the `Cargo.toml` file: -->

それでは、`Cargo.toml` ファイルにいくつかの依存関係を追加してみましょう:

```toml
{{#include ../../examples/01_04_async_await_primer/Cargo.toml:9:10}}
```

<!-- To create an asynchronous function, you can use the `async fn` syntax: -->

非同期関数を作成するには、`async fn` 構文を使用します:

```rust,edition2018
async fn do_something() { /* ... */ }
```

<!-- The value returned by `async fn` is a `Future`. For anything to happen,
the `Future` needs to be run on an executor. -->

`async fn` の戻り値は `Future` であり、`Future`　は `executor` 上で実行される必要があります。

```rust,edition2018
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:hello_world}}
```

<!-- Inside an `async fn`, you can use `.await` to wait for the completion of
another type that implements the `Future` trait, such as the output of
another `async fn`. Unlike `block_on`, `.await` doesn't block the current
thread, but instead asynchronously waits for the future to complete, allowing
other tasks to run if the future is currently unable to make progress. -->

`async fn` 内で `.await` を使用すると、 `Future` トレイトを実装している別の型の完了を待つことができます。  
`block_on` とは異なり、 `.await` は現在のスレッドをブロックするのではなく、非同期に `Future` が完了するのを待ちます。`Future` が完了すると、他のタスクを実行できるようになります。


<!-- For example, imagine that we have three `async fn`: `learn_song`, `sing_song`,
and `dance`: -->

例えば、3 つの `async fn`として `learn_song`(訳注:歌を覚える)、`sing_song`(訳注:歌を歌う)、`dance`(訳注:踊る)がある場合を考えます:

```rust,ignore
async fn learn_song() -> Song { /* ... */ }
async fn sing_song(song: Song) { /* ... */ }
async fn dance() { /* ... */ }
```

<!-- One way to do learn, sing, and dance would be to block on each of these
individually: -->

歌を覚える、歌を歌う、踊る動作を扱うために、それぞれ個別のブロック処理とします。

```rust,ignore
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:block_on_each}}
```

<!-- However, we're not giving the best performance possible this way—we're
only ever doing one thing at once! Clearly we have to learn the song before
we can sing it, but it's possible to dance at the same time as learning and
singing the song. To do this, we can create two separate `async fn` which
can be run concurrently: -->

しかし、この方法では、最高のパフォーマンスを発揮することはできません。なぜなら、一度に 1 つの処理しか行えないからです!  
一方、私たちは歌を覚えてから歌うのは当然ですが、歌を覚えて歌を歌うのと同時に踊ることも可能です。  
この方法を実現するためには、2つの独立した `async fn` を作成し、それらを同時実行します。  

```rust,ignore
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:block_on_main}}
```

<!-- In this example, learning the song must happen before singing the song, but
both learning and singing can happen at the same time as dancing. If we used
`block_on(learn_song())` rather than `learn_song().await` in `learn_and_sing`,
the thread wouldn't be able to do anything else while `learn_song` was running.
This would make it impossible to dance at the same time. By `.await`-ing
the `learn_song` future, we allow other tasks to take over the current thread
if `learn_song` is blocked. This makes it possible to run multiple futures
to completion concurrently on the same thread. -->

この例では、歌を覚えるのは歌を歌う前でなければなりませんが、歌を覚えるのも歌を歌うのも踊る処理と同時に行うことができます。
もし、 `learn_and_sing` の関数内で `learn_song().await` ではなく `block_on(learn_song())` を使用した場合、 `learn_song` が実行されている間、スレッドは他の処理ができなくなります。これでは、同時に踊ることは不可能です。  
`.await` の場合、 `learn_song` がブロックされた際に、他のタスクが現在のスレッドを引き継ぐことができます。したがって、同じスレッドで複数の `Future` を同時に実行できます。
