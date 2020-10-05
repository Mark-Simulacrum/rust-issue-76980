fn main() {
    cc::Build::new()
        .file("src/foo.cpp")
		.cpp(true)
        .compile("foo");
}
