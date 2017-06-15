
#[macro_use]
extern crate rusty_json_learning;
use rusty_json_learning::Json;

fn main() {
    let js = json!({
        "Yukari" => 1,
        "Maki" => 2,
        "Kotonoha" => {
            "Akane" => 3,
            "Aoi" => 4
        },
        "Others" => [
            5,
            6
        ]
    });
    println!("{}", js);
}
