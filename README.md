# pytokei
---
Pytokei is a python binding to [tokei](https://github.com/XAMPPRocky/tokei):
Note that this is a small upgrade of the original project Pytokei. If you want to use the original package, run the command 
```python 
pip install pytokei
```
instead of pytokei_new

<p align="center">
    Tokei is a program that displays statistics about your code. Tokei will show the number of files, total lines within those files and code, comments, and blanks grouped by language.
</p>

--- 

This wrapper allows to obtain the same reports directly from python.

```python
$ python
>>> import pytokei_new
>>> from rich import print
>>> langs = pytokei_new.Languages()
>>> langs.get_statistics(["."], ["tests/data", "requirements"], pytokei_new.Config()) # Use ['all'] in place of ignore to load default ignore file methods
>>> print(langs.report_compact_plain())
{
    'YAML': {'blanks': 6, 'code': 63, 'comments': 0, 'files': 1, 'lines': 69},
    'Python': {'lines': 376, 'blanks': 89, 'files': 2, 'code': 280, 'comments': 7},
    'Makefile': {'code': 18, 'lines': 26, 'comments': 0, 'blanks': 8, 'files': 1},
    'Markdown': {'code': 0, 'blanks': 37, 'files': 10, 'comments': 52, 'lines': 89},
    'Rust': {'blanks': 23, 'comments': 23, 'code': 317, 'lines': 363, 'files': 7},
    'TOML': {'code': 14, 'comments': 2, 'lines': 20, 'blanks': 4, 'files': 2}
}
```

It includes a small CLI for simple use cases (run against the pytokei's folder):

```console
$ pytokei pytokei
                        pytokei                         
┏━━━━━━━━━━━━┳━━━━━━━┳━━━━━━━┳━━━━━━┳━━━━━━━━━━┳━━━━━━━━┓
┃ language   ┃ Files ┃ Lines ┃ Code ┃ Comments ┃ Blanks ┃
┡━━━━━━━━━━━━╇━━━━━━━╇━━━━━━━╇━━━━━━╇━━━━━━━━━━╇━━━━━━━━┩
│ Rust       │     9 │  1011 │  846 │       49 │    116 │
│ Python     │     5 │   568 │  436 │       13 │    119 │
│ Markdown   │    11 │   423 │  123 │      179 │    121 │
│ Plain Text │     4 │   133 │    0 │      133 │      0 │
│ TOML       │     3 │    75 │   59 │        6 │     10 │
│ YAML       │     1 │    69 │   63 │        0 │      6 │
│ Makefile   │     1 │    26 │   18 │        0 │      8 │
│ Dockerfile │     1 │    16 │    7 │        3 │      6 │
│ Shell      │     3 │    12 │   12 │        0 │      0 │
│ Autoconf   │     3 │     7 │    7 │        0 │      0 │
└────────────┴───────┴───────┴──────┴──────────┴────────┘
```

For more information about `tokei`, please visit the original repo.

[![PyPI pyversions](https://img.shields.io/pypi/pyversions/pytokei.svg)](https://pypi.org/project/pytokei/)
![example workflow](https://github.com/plaguss/pytokei/actions/workflows/ci.yml/badge.svg)
[![license](https://img.shields.io/github/license/plaguss/pytokei.svg)](https://github.com/plaguss/pytokei/blob/main/LICENSE)


## Installation

```bash
pip install pytokei_new
```

Requires Python >= 3.7.

Binaries are available for:

* **Linux**: `x86_64`, `aarch64`, `i686`, `armv7l`, `musl-x86_64` & `musl-aarch64`
* **MacOS**: `x86_64` & `arm64` (except python 3.7)
* **Windows**: `amd64` & `win32`

Otherwise, you can install from source which requires Rust stable to be installed.

## Why this library?

Wanted to practice rust, and taking this library to python seemed like a good opportunity. It's awesome, and maybe more people coming from python will find something useful to do with it.

But really? Just for fun :)

## [Documentation](https://plaguss.github.io/pytokei/)

## What times should you expect?

Running `Languages.get_statistics` against [cpython](https://github.com/python/cpython) takes a little less than 200 milliseconds.

Some more info should be found in the [docs](https://plaguss.github.io/pytokei/#time-comparison-tokei-and-pytokei).

## Development

You will need:

- [maturin](https://www.maturin.rs/installation.html) to compile the library

- `maturin develop` / `make develop` to compile the code.

From python side:

Run `make install-dev` inside a virtual environment, `make test`, `make mypy` and `make format` to ensure everything is as expected, and `make docs` to build the documentation.

*There are some problems when building the docs with mkdocstrings, a reminder is in the following [github issue](https://github.com/mkdocstrings/mkdocstrings/issues/404). For the moment, it seems that the best option is to remove the .so file and build the docs without it.*

To create a new release:

- Update the version in [Cargo.toml](./Cargo.toml).
- Create a new tag to run the github action workflow.
- git push --atomic origin main tag-name 