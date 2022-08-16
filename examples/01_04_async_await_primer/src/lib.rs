#![cfg(test)]

use futures::executor::block_on;

mod first {
// ANCHOR: hello_world
// `block_on` は、提供された Future が実行されるまで、現在のスレッドをブロックします。
// 完了するまで現在のスレッドをブロックします。他の executers は、より複雑な動作を提供します。
// 複数の futures を同じスレッドにスケジューリングするような、より複雑な動作を提供します。
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world(); // 何も表示されない
    block_on(future); // `future` が実行され、"hello, world!" が出力される
}
// ANCHOR_END: hello_world

#[test]
fn run_main() { main() }
}

struct Song;
async fn learn_song() -> Song { Song }
async fn sing_song(_: Song) {}
async fn dance() {}

mod second {
use super::*;
// ANCHOR: block_on_each
fn main() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}
// ANCHOR_END: block_on_each

#[test]
fn run_main() { main() }
}

mod third {
use super::*;
// ANCHOR: block_on_main
async fn learn_and_sing() {
    // 歌を学ぶまで待ってから歌います。
    // ここでは `block_on` ではなく `.await` を使用し、スレッドのブロックを防いでいます。
    // スレッドをブロックしないようにするため、`.await` を使用しています。これにより、`dance` を同時に行うことができます。
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` は `.await` のようなものだが、複数の futures を同時に待つことができます。
    // `learn_and_sing` の future で一時的にブロックされた場合、 `dance` が現在のスレッドを引き継ぎます。
    // もし `dance` がブロックされた場合、future が現在のスレッドを引き継ぎます。
    // もし両方の未来がブロックされた場合、`learn_and_sing` が引き継ぐことができます。
    // `async_main` がブロックされ、executor に引き継がれます。
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
// ANCHOR_END: block_on_main

#[test]
fn run_main() { main() }
}
