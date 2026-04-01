fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("../assets/Logo.ico");
    res.compile().expect("failed to compile Windows Resource");
}
