<div align="center">

  Nixt
  ---
  Nixt is an interpreted lisp inspired programming language written in Rust

</div>

# Index

- [About](#about)
- [Examples](#examples)
- [Installation](#installation)
- [Build](#build)

# About

Nixt goal is to provide a fast and easy to learn scripting language that provides productivity due to its easy syntax and correct performances

# Examples

## Factorial

```lisp
(const factorial (func (n) {
  (let toret 1)
  (let i 2)
  (while (<= i n) {
    (set toret (* toret i))
    (set i (+ i 1))

  })
  (ret toret)
}))
```

## Ackermann function

```lisp
(let ackermann (func (m n) {
  (let toret 0)
  (if (= m 0)
    (set toret (+ n 1))
    (if (and (> m 0) (= n 0))
      (set toret (ackermann (- m 1) 1))
      (if (and (> m 0) (> n 0))
        (set toret (ackermann (- m 1) (ackermann m (- n 1))))
      )
    )
  )
  (ret toret)
}))
```

# Installation

## With werb

- Run `werb install nixt`
- Add `export NIXT_STD="/home/$USERNAME/.werb_bin/std/"` to your `~/.bashrc` (obviously replace $USERNAME with your username)

## Manually

- Run `wget https://github.com/Wafelack/nixt/releases/download/$VERSION/nixt.tar.gz` (replace $VERSION with the version tag you want to download (refer to the releases page))
- Run `mkdir ~/.nixt && tar -xzf nixt.tar.gz -C ~/.nixt/`
- Add `~/.nixt` to your path
- Add `export NIXT_STD="/home/$USERNAME/.nixt/std/"` to your `~/.bashrc` (obviously replace $USERNAME with your username)

# Build

```sh
git clone https://github.com/wafelack/nixt
cd nixt/
cargo test --release
cargo build --release
```
