# p001_modules

I have the impression that rust use `mod.rs`^1 under directory to handle the module, but it looks like different now.

## Gained knowledge
+ module is file.
+ The directory with the same name as one module (`*.rs`) contains the sub modules. See `module1.rs` and `module1/`.
+ `mod` used to specify the modules used in current module.
+ `pub mod` can expose the used module to upper level (see `sub_module1`); otherwise, `mod` without `pub` means the module is used locally (see `sub_module2`).
+ `use` import the crate used in this module, so it can have multiple level and separated by `::`. Moreover, the last one should be crate name.
+ A function can be imported by `use`. See `use_sub1_in_mod2` in `main.rs`.
+ The root of project to import absolute crate is `crate::`. See `use_sub1_in_mod2` in `module2.rs`.

## References
1. [Rust Modules tree (maybe old)](https://riptutorial.com/rust/example/8362/modules-tree)
2. [Explaining Rust’s Modules](https://betterprogramming.pub/explaining-rusts-modules-420d38eed6c5)
3. [(Modules) Tracking issue for the mod.rs changes #53125](https://github.com/rust-lang/rust/issues/53125)