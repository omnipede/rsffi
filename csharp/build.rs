fn main() {
    csbindgen::Builder::default()
        .input_extern_file("./src/lib.rs")
        .csharp_dll_name("csharp")
        .generate_csharp_file("./NativeMethods.g.cs")
        .unwrap();
}