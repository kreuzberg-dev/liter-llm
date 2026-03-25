use ext_php_rs::prelude::*;

/// Returns the version of the liter-lm library.
#[php_function]
pub fn liter_lm_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(liter_lm_version))
}
