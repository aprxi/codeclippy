
Codeclippy
==========

Codeclippy is a utility to scan, map, and search code objects. It helps
prepping/ compressing a codebase for LLM (Language Level Model) services.

It is currently Rust-only to start, but we'll hope to extend it with other
languages as well.

Prerequisites
-------------
- Rust build tools (cargo, rustc, etc.)

Installation
------------

Rust (CLI)
~~~~~~~~~~~~~~~~~~~~

Clone the repository and compile the project using Cargo:

.. code-block:: console

    git clone https://github.com/aprxi/codeclippy.git
    cargo build --release

Next, copy the binary from `./target/release/codeclippy` to your local path.

Usage
-----

Quickstart
~~~~~~~~~~~~~~

List
^^^^
.. code-block:: console

    codeclipy --help

    List code objects

    Example:
     codeclippy ls src/

    Usage:

    Commands:
      ls    List code objects
      help  Print this message or the help of the given subcommand(s)

    Options:
      -h, --help     Print help
      -V, --version  Print version


Contributing
------------

Contributions to Codeclippy are welcome. Please open an issue or submit a pull request on the GitHub repository.

License
-------

Codeclippy is released under the MIT license. See LICENSE for more details.

