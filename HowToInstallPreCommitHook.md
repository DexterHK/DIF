Use this website as a reference if you ever get lost: https://rodneylab.com/rust-ci-tooling/


## Installing Tools:

Let's start with clippy and rustfmt.
You can install both of them using Rustup.
Just type the following:
```
rustup component add clippy rustfmt
```
To install our Changelog generator.
1. You should have Python installed.
2. Pip installed.
3. Type the following:
```
pip install Commitizen pre-commit
```
## Installing Pre-commit hook
pre-commit install # install pre-commit in new project
```
pre-commit install # install pre-commit in new project
pre-commit run --all-files # one-off check all files
```


Test the following tools to see if they work manually before having them executed in a pre-commit hook:

* Check Rust code for errors (comes with RustUp no need to install manually)
 
```
cargo check
```
* Format Rust code
```
cargo fmt
```
* Lint Rust code
```
cargo clippy
```

* commitlint

```
cz init # initialise new project
cz c # commit
cz c --retry # retry failed commit with same parameters
cz bump # bump SemVer version
cz changelog # generate CHANGELOG.md
```


The following File is already pushed, so you don't have to, just pull the github repo:

```
repos:
  - hooks:
      - id: commitizen
        stages:
          - commit-msg
    repo: https://github.com/commitizen-tools/commitizen
    rev: v2.24.0
  - hooks:
      - id: fmt
      - id: cargo-check
    repo: https://github.com/doublify/pre-commit-rust
    rev: v1.0 
```

Now try to add anything and commit using 
``` cz c ``` 

and then pushhh
