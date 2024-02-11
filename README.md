# win-fast-rm

The actually faster(?) way to delete files on Windows.

> [!CAUTION]
> I do NOT claim responsibility for accidentally deleted files. They're not recoverable.

## Motivation

I recently came across [this video](https://youtu.be/CzgzED0VR5A), and ~~came to the conclusion that this guy has no idea what he's talking about.~~ turns out, I was the one who had no idea what I was talking about 🙃

I want to prove that Windows does have ways to delete files very fast, but it's definitely not as straight forward as I previously thought.
I am annoyed by the lack of variety in the testing used in the video, so I will also experiment with different APIs and methods, possibly even hybrid solutions that use multiple threads combined with Windows APIs.

## Benchmarking

TODO

## Using this project

The project is distributed as a library, allowing you to delete files are fast as possible in your applications, or as an executable for testing purposes.

You can get it on cargo with `cargo add win_fast_rm`
