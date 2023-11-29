# build

* clippy
* test
* `README.md`
* `pandoc.docx`
* `pipeline.png`

```
cargo build --release
```

# `README.md`

* `t/README.md`
* `input.md`
* `Cargo.toml`
* `CHANGELOG.md`
* `src/**/*.rs`

```
cargo build --release
kapow -R {0} >{target}
```

# clippy

```
cargo clippy -- -D clippy::all
```

# test

```
cargo test
```

# check

```
cargo outdated --exit-code 1
cargo audit
```

# update

```
cargo upgrade --incompatible
cargo update
```

# install

* `README.md`

```
cargo install --path .
```

# uninstall

```
cargo uninstall $(toml get -r Cargo.toml package.name)
```

# install-deps

```
cargo install cargo-audit cargo-edit cargo-outdated cocomo kapow tokei toml-cli
```

# clean

```
cargo clean
```

# cocomo

```bash -eo pipefail
tokei; echo
cocomo -o sloccount
cocomo
```

# full

* update
* check
* build
* install

# `kapow.md`

* `input.md`

```
kapow {0} >{target}
```

# `stylin.md`

* `kapow.md`
* `README.md`

```
./target/release/stylin {0} >{target}
```

# `reference.docx`

```
pandoc -o {target} --print-default-data-file reference.docx
```

# `pandoc.docx`

* `stylin.md`
* `reference.docx`

```
pandoc --reference-doc=reference.docx {0} -o {target}
```

# `pipeline.png`

* `pipeline.dot`

```
dot -Tpng {0} >{target}
```

