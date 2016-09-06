fn main() {
    extern crate diesel_codegen_syntex as diesel_codegen;
    extern crate syntex;
    //extern crate dotenv_codegen;

    use std::env;
    use std::path::Path;

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut registry = syntex::Registry::new();
    diesel_codegen::register(&mut registry);
    //dotenv_codegen::register(&mut registry);

    let src = Path::new("src/model.in.rs");
    let dst = Path::new(&out_dir).join("model.rs");

    registry.expand("", &src, &dst).unwrap();
}