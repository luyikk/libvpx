use cc;
fn main() {
    std::env::remove_var("path");
    std::env::set_var("path","C:/Program Files/Unity/Hub/Editor/2020.2.6f1c1/Editor/Data/PlaybackEngines/AndroidPlayer/NDK/toolchains/llvm/prebuilt/windows-x86_64/bin");
    let mut c = cc::Build::new();
    c.cpp(true);
    c.warnings(false);
    c.flag("-fno-rtti");
    c.flag("-std=gnu++11");
    c.flag("-fno-exceptions");
    c.include("include");
    c.includes(&["include", "include/libyuv"]);
    for f in std::fs::read_dir("source").unwrap().filter_map(|x|x.ok()) {
        c.file(f.path());
    }
    c.opt_level(2);
    c.compile("libyuv.a");
}
