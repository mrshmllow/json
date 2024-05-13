use lalrpop_util::lalrpop_mod;

mod ast;

lalrpop_mod!(pub json);

fn main() {
    let string = "{\"value\": \"hi\",\"value\": [\"hi\"],\"value\": 1,\"value\": -1,\"value\": 1.001,\"value\": null}";
    println!("Parsing: {string:?}");
    println!(
        "{:?}",
        json::ValueParser::new()
            .parse(
                "
            {
                \"value\": \"hi\",
                \"value\": [\"hi\"],
                \"value\": 1,
                \"value\": -1,
                \"value\": 1.001,
                \"value\": null
            }
            "
            )
            .unwrap()
    );
}
