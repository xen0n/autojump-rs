# autojump-rs

A port of the wildly popular helper application [`autojump`][aj] to Rust.

[aj]: https://github.com/wting/autojump


## License

As this project is technically a fork, the license is the same as `autojump`,
which is GPL, either version 3 or any later version. See [LICENSE](LICENSE)
for details.


## FAQ

#### Why Rust?

Primarily for two reasons:

* The author is really tired of `autojump` breakage inside Python virtualenvs, and
* Rust is simply *awesome* for CLI applications, with its performance and (code) slickness!


#### Is the port compatible, bug-for-bug?

The on-disk format of the text database should be identical. That said, edge
cases certainly exist. The author is developing and using this on Linux, so
other platforms may need a little more love.

That said, there're some IMO very minor deviations from the original Python
implementation. These are:

*   Argument handling and help messages.

    Original `autojump` uses Python's `argparse` to parse its arguments. There
    is [a Rust port of it][rust-argparse], but it's nowhere as popular as the
    [`docopt.rs`][docopt.rs] library, as is shown in `crates.io` statistics
    and GitHub activities. So it's necessary to re-arrange the help messages
    at least, as the `docopt` family of argument parsers mandate a specific
    style for them.

    Also, due to [a limitation][docopt-limitation] optional flag arguments
    are not supported, so it's required to provide the weights when you do
    `--increase` or `--decrease`. The original defaults are `10` and `15`
    respectively, so you can manually specify them in the meantime.


[rust-argparse]: https://github.com/tailhook/rust-argparse
[docopt.rs]: https://github.com/docopt/docopt.rs
[docopt-limitation]: https://github.com/docopt/docopt.rs/issues/167


<!-- vim:set ai et ts=4 sw=4 sts=4 fenc=utf-8: -->
