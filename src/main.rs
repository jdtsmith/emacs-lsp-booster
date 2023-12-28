use serde_json as json;

use emacs_lsp_booster::bytecode;


fn main() {
    let value: json::Value = json::from_reader(std::io::stdin()).unwrap();
    println!("{}", bytecode::generate_bytecode_repl(&value));
}
