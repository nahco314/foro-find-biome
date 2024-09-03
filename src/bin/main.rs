use onefmt_find_biome::main_with_json;
use serde_json::json;

fn main() {
    let in_ = json!({
        "current-dir": "./"
    });

    let res = main_with_json(in_);

    println!("{}", res);
}
