> macro_rules!

> cargo install cargo-expand
> cargo expand

> https://doc.rust-lang.org/stable/std/

> rustfmt

> rustc hello_world.rs
> .pdb on windows

> cargo new hello_world_via_cargo
> cargo build
> cargo build --release
> cargo check

## repl

> cargo install irust
> cargo install evcxr_repl


> cargo install cargo-watch
> cargo fmt
> cargo fmt --check
> cargo fmt --check --all
>
>

```bash
diff=$(cargo fmt -- --check)
result=$?

if [[ $(result) -ne 0 ]]; then
cat << EOF
There are some code style issues, run 'cargo fmt' forst.
EOF
  exit 1
fi
exit 0
```

> chmod +x .git/hooks/pre-commit