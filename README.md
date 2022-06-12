# Rust Basics FEM Course

There are [slides](https://docs.google.com/presentation/d/1kkTsCrMIVtxYef9T7SV-MWS-nQlnmTniAGaTl6L9Fe4/edit?usp=sharing) that go with this repo, as well as a [companion website](https://rtfeldman-rust-workshop.netlify.app/).

## Getting Started

1. Install [Rust](https://www.rust-lang.org/tools/install) 1.51 or higher

2. Install [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) 1.51 or higher

3. Not required, but **highly** recommended: install [`rustfmt`](https://github.com/rust-lang/rustfmt#on-the-stable-toolchain) and configure your editor to run it on save.

Verify that you have the right version of cargo by running:

```shell
cargo --version
```

It should print a version number of 1.51 or higher.

Finally, clone this repository by running this at the terminal:

```shell
git clone https://github.com/rtfeldman/intro-to-rust
cd intro-to-rust
```

You're all set!

## Next Steps

Here are some example applications to build out as next steps:

- Check out the `./lessons` directory for a Rust program that is a static site generator.  It built the companion site for this course.  It takes in markdown files and compiles them into HTML files.
- Check out the [Rust Learn](https://www.rust-lang.org/learn) page for more learning an other growth guides
  - Do the [Rustlings course](https://github.com/rust-lang/rustlings/)
- Check out other use cases and learn more at the [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/) page
- Build a simple web servers with a CRUD REST API, look into web server frameworks like [Actix](https://actix.rs/)
  - However, DYOR, because there has been controversy in how they were doing manual memory management within it, which was very un-Rust-like
- Learn how to build Web Assembly programs by compiling Rust into WASM

### Front End Developer Use Cases

- "The more things that you have on the screen, the more likely you'll want to reach for a performant language like Rust."
- Games are a great use case for using Rust because games need to be quick, snappy, performant, etc.
- Processing audio with Rust within the browser
- Simulations and visualizations in real time
