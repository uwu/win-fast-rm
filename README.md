# win-fast-rm

The actually faster way to delete files on Windows.

> [!CAUTION]
> I do NOT claim responsibility for accidentally deleted files. They're not recoverable.

## Motivation

I recently came across [this video](https://youtu.be/CzgzED0VR5A), and came to the conclusion that this guy has no idea what he's talking about.

I wanted to prove him wrong, and that Windows _does_ have ways to delete files very quickly, if you just look for them.

His solution is to just throw 3000 threads at it. My solution uses the Windows APIs as intended.

## Benchmarking

TODO

![103GB deleted in just 790ms](https://github.com/uwu/win-fast-rm/blob/main/benchmarks/demo.png)

On my system (Ryzen 5 5600x, Crucial P5 Plus @ PCIe Gen3 speeds, ~3500MBps max write speed) and Windows 11, it deletes 103GB of data in > 800ms.

## Using this project

The project is distributed as a library, allowing you to delete files are fast as possible in your applications, or as an executable for testing purposes.
