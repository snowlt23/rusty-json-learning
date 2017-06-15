
#[macro_use]
extern crate rusty_json_learning;
#[macro_use]
extern crate rusty_json_serialize_macro;

use rusty_json_learning::Json;

#[derive(JsonSerialize)]
struct Voiceroid {
    name: String,
    since: i32
}

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
    let vr = Voiceroid {
        name: "Yuduki Yukari".to_string(),
        since: 2011
    };
    println!("{}", js);
    println!("{}", vr.serialize());
}
