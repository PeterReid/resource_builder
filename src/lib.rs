use std::fs::File;
use std::io::{Read, Write};

fn go(input: &[u8], name: &str) -> String {
    let mut result = String::with_capacity(100 + input.len()*6);
    result.push_str( &format!("pub const {}: [u8; {}] = [\n", name, input.len()) );

    for line_chunk in input.chunks(40) {
        result.push_str("    ");
        for byte in line_chunk {
            result.push_str( &format!("0x{:02x},", byte) );
        }
        result.push_str("\n");
    }
    result.push_str("];\n");

    result
}

pub fn build(resource_path: &str, code_path: &str, name: &str) {
    let mut resource = File::open(resource_path).unwrap_or_else(|e| {
        panic!("Failed to open resource file from {}\n    {:?}", resource_path, e);
    });

    let mut resource_contents = Vec::new();
    resource.read_to_end(&mut resource_contents).unwrap_or_else(|e| {
        panic!("Failed to read resource file from {}\n    {:?}", resource_path, e);
    });

    let mut code_file: File = File::create(code_path).unwrap_or_else(|e| {
        panic!("Failed to create code file from {}\n    {:?}", code_path, e);
    });

    let code = go(&resource_contents, name);

    code_file.write_all(code.as_bytes()).unwrap_or_else(|e| {
        panic!("Failed to write to code file from {}\n    {:?}", code_path, e);
    });
}

#[test]
fn it_works() {
    println!("{}", go(&[1,2,3], "FOO"));
    assert!(false);
}
