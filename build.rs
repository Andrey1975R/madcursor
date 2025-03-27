fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/bat.ico")
           .set("ProductName", "Mad cursor")
           .set("FileDescription", "just mad cursor");
        res.compile().unwrap();
    }
}