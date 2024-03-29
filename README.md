# autojump-rs  [![Crates.io version](https://img.shields.io/crates/v/autojump.svg)][cratesio] [![Crates.io downloads](https://img.shields.io/crates/dv/autojump.svg)][cratesio] [![Crates.io license](https://img.shields.io/crates/l/autojump.svg)](LICENSE) ![GitHub branch checks state](https://img.shields.io/github/checks-status/xen0n/autojump-rs/develop)

A port of the wildly popular helper application [`autojump`][aj] to Rust.

[aj]: https://github.com/wting/autojump
[cratesio]: https://crates.io/crates/autojump


## License

As this project is technically a fork, the license is the same as `autojump`,
which is GPL, either version 3 or any later version. See [LICENSE](LICENSE)
for details.


## Install

We have [prebuilt binaries available][releases] for a while now, thanks to
the [trust] project!

The package is a drop-in replacement of `autojump`. Assuming `autojump` is
already installed, or at least the shell script part of it has been properly
set up, and you have in `$PATH` `~/.cargo/bin` before the system binary
locations, all you have to do is to put [a binary of your choice architecture][releases]
in your PATH, overriding the original `autojump` script.

You may have to issue `hash -r` for the shell to forget previous
location of `autojump`, if you don't want to re-exec your shell.

(Manually cloning the repository and building is okay, of course.)

[releases]: https://github.com/xen0n/autojump-rs/releases
[trust]: https://github.com/japaric/trust


## Features

Why do a port when the original version works? Primarily for two reasons:

* The author is *really* tired of `autojump` breakage inside Python virtualenvs, and
* Rust is simply *awesome* for CLI applications, with its performance and (code) slickness!

Indeed, being written in a compiled language, **`autojump-rs` is very light on
modern hardware**. As the program itself is very short-running, the overhead of
setting up and tearing down a whole Python VM could be overwhelming,
especially on less capable hardware. With `autojump-rs` this latency is
greatly reduced. Typical running time is like this on the author's Core
i7-2670QM laptop, with a directory database of 1014 entries:

```
$ time ./autojump/bin/autojump au
/home/xenon/src/autojump-rs
./autojump/bin/autojump au  0.09s user 0.01s system 99% cpu 0.103 total

$ time ./autojump-rs/target/release/autojump au
/home/xenon/src/autojump-rs
./autojump-rs/target/release/autojump au  0.00s user 0.00s system 87% cpu 0.007 total
```

The time savings are more pronounced on less powerful hardware, where every
cycle shaved off counts. The running time on a 1.4GHz Loongson 3A3000 is
about 10ms, for example, which is very close to the x86 figures despite the
clock frequency difference.

And, of course, the program no longer interacts with Python in any way, so the
virtualenv-related crashes are no more. Say goodbye to the dreaded
`ImportError`'s *showing every `$PS1` in a virtualenv with the system-default
Python*!

```
# bye and you won't be missed!
Traceback (most recent call last):
  File "/usr/lib/python-exec/python2.7/autojump", line 43, in <module>
    from autojump_data import dictify
ImportError: No module named autojump_data
```


## Compatibility

All of the command line flags and arguments are now implemented, and behave
exactly like the original. Being a drop-in replacement, all other shell
features like tab completion should work too. (Except `jc` and `jco`; see
below.)

As for the text database, the on-disk format should be identical. (Actually
there is a little difference in the representation of floats, but it doesn't
matter.) However, as the author is developing and using this on Linux, other
platforms may need a little more love, although all the libraries used are
lovingly cross-platform. (Patches are welcome, of course!)

The Windows batch files shipped with the original `autojump` has Python
hard-coded into them, and obviously that won't work with `autojump-rs`.
Use the batch files in the `windows` directory instead; just replacing the
original files and putting `autojump.exe` along with them should work.
(Thanks @tomv564 for the Windows testing!)

That said, there're some IMO very minor deviations from the original Python
implementation. These are:

*   Argument handling and help messages.

    Original `autojump` uses Python's `argparse` to parse its arguments. There
    is [a Rust port of it][rust-argparse], but it's nowhere as popular as the
    [`docopt.rs`][docopt.rs] library, as is shown in `crates.io` statistics
    and GitHub activities. So it's necessary to re-arrange the help messages
    at least, as the `docopt` family of argument parsers mandate a specific
    style for them. However this shouldn't be any problem, just that it's
    different. Again, who looks at the usage screen all the day? XD

*   Different algorithm chosen for fuzzy matching.

    The Python version uses the [`difflib.SequenceMatcher`][difflib] algorithm
    for its fuzzy matches. Since it's quite a bit of complexity, I chose to
    leverage the [`strsim`][strsim-rs] library instead. The [Jaro-Winkler
    distance][jaro] is computed between every filename and the last part of
    query needles respectively, and results are filtered based on that.

*   `jc` may jump outside current directory.

    Exact reason may be different filtering logic involved, but I'm not very
    sure about this one. The behavior is also observed on original `autojump`,
    but the frequency seems to be lower, and both implementations actually
    don't check if the target is below current directory. However I only use
    plain `j` mostly, so if you're heavily reliant on `jc` and its friends
    please open an issue!


[rust-argparse]: https://github.com/tailhook/rust-argparse
[docopt.rs]: https://github.com/docopt/docopt.rs
[difflib]: https://docs.python.org/3.5/library/difflib.html
[strsim-rs]: https://github.com/dguo/strsim-rs
[jaro]: https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance


## Future plans

Now that platform support is mostly considered okay, next steps would be
more refactoring and bug fixing. The `jc` behavior differences are observed
on original `autojump` too, in that you could jump outside `$(pwd)`, but the
actual directory jumped to is different; this needs further investigation.
Hell I even want to write a `fasd` backend too, but I don't presently have
*that* much free time. Anyway, contributions and bug reports are welcome!


<!-- vim:set ai et ts=4 sw=4 sts=4 fenc=utf-8: -->
