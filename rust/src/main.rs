#[wasmtime_rust::wasmtime]
trait WasmMarkdown {
    fn render(&mut self, input: &str) -> String;
}

fn main() -> Result<(), failure::Error> {
    let mut markdown = WasmMarkdown::load_file("markdown.wasm")?;
    println!("{}", markdown.render("# Hello, Rust!"));
    Ok(())
}
