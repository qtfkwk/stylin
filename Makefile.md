# all

* clippy
* test
* build
* doc

# check

* outdated
* audit

# update

* update-toml
* update-lock

# run

* `target/release/{dirname}`

```
target/release/{dirname}
```

# clippy

* `Cargo.lock`
* `Cargo.toml`
* `**/*.rs`

```
cargo clippy -- -D clippy::all
```

# test

* `Cargo.lock`
* `Cargo.toml`
* `**/*.rs`

```
cargo test
```

# build

* `target/release/{dirname}`

# `target/release/{dirname}`

* `Cargo.lock`
* `Cargo.toml`
* `**/*.rs`
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
* `**/*.rs`
* `stylin.md`

```
cargo build --release
kapow -R {0} >{target}
```

# doc

```
cargo doc
```

# outdated

```
cargo outdated --exit-code=1
```

# audit

```
cargo audit
```

# update-toml

```
cargo upgrade -i
```

# update-lock

```
cargo update
```

# install

* `README.md`

```
cargo install --path .
```

# uninstall

```
cargo uninstall {dirname}
```

# install-deps

```
cargo install cargo-audit cargo-edit cargo-outdated cocomo dtg kapow tokei toml-cli
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

# commit

```bash
set -xeo pipefail
V=$(toml get -r Cargo.toml package.version)
git commit -m "$V"
git tag -a "$V" -m "$V"
```

# publish

```
cargo publish
git push
git push --tags
```

# full

* update
* check
* all
* install

# `kapow.md`

* `input.md`

```
kapow {0} >{target}
```

# `stylin.md`

* `kapow.md`

```
./target/release/stylin {0} >{target}
```

# `reference.docx`

```
pandoc -o {target} --print-default-data-file {target}
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

