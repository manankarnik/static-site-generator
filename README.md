<h1 align="center">Static Site Generator</h1>

<div align="center">

![CodeSize](https://img.shields.io/github/languages/code-size/manankarnik/static-site-generator?style=for-the-badge)
[![License](https://img.shields.io/github/license/manankarnik/static-site-generator?label=License&style=for-the-badge)](LICENSE)
![LastCommit](https://img.shields.io/github/last-commit/manankarnik/static-site-generator?style=for-the-badge)

</div>

Static Site Generator using Yew and Markdown files

Example Blog: https://cosmos-blog.netlify.app/

## Pre-requisites

- [Rust](https://www.rust-lang.org/)
- [Tailwind CLI](https://tailwindcss.com/docs/installation)

## Quick Start

### Clone Repo

```sh
git clone https://github.com/manankarnik/static-site-generator.git
```

### `cd` to Project

```sh
cd static-site-generator
```

### Add `wasm` Target

```sh
rustup target add wasm32-unknown-unknown
```

### Install `trunk`

```sh
cargo install --locked trunk
```

### Serve

```sh
trunk serve
```
