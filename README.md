<h1 align="center">Welcome to nbcat 👋</h1>
<p>
  <img alt="Version" src="https://img.shields.io/crates/v/nbcat.svg" />
  <a href="https://github.com/moisutsu/nbcat/blob/master/LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
  <a href="https://twitter.com/moisutsu" target="_blank">
    <img alt="Twitter: moisutsu" src="https://img.shields.io/twitter/follow/moisutsu.svg?style=social" />
  </a>
</p>

> A command-line tool that displays .ipynb file like `cat` command.

## Install

Homebrew

```sh
brew install moisutsu/tap/nbcat
```

Cargo

```sh
cargo install nbcat
```

## Usage

You can view the .ipynb file on the terminal by simply passing the path, as in the `cat` command.

```sh
nbcat <file-name>
```

### Features

- Display notebooks containing images
- Option to ignore cell output (`--ignore-output`)

![Display notebook include graph](image/display_notebook_include_graph.png)

## Author

👤 **moisutsu**

* Twitter: [@moisutsu](https://twitter.com/moisutsu)
* Github: [@moisutsu](https://github.com/moisutsu)

## Show your support

Give a ⭐️ if this project helped you!

***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_
