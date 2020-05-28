mod filesystem;



struct ProgramArgs {
    input_file: Option<String>,
    output_file: Option<String>,
    in_meta: Option<String>,
    out_meta: String
}

fn main() -> std::io::Result<()> {

    println!("Hello, world!");
    Ok(())
}
