fn main() {
    let src = "../progenitor/sample_openapi/openai-openapi.yaml";
    println!("cargo:rerun-if-changed={}", src);
    let file = std::fs::File::open(src).unwrap();
    let spec = serde_yaml::from_reader(file).unwrap();
    let mut generator = progenitor::Generator::default();

    let tokens = generator.httpmock(&spec, "sdk").unwrap();
    let ast = syn::parse2(tokens).unwrap();
    let content = prettyplease::unparse(&ast);

    let mut out_file = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("httpmock_codegen.rs");

    std::fs::write(out_file, content).unwrap();
}
