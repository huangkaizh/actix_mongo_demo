use json::JsonValue;

pub fn ok_json(json_value: JsonValue) -> String {
    let obj = object!{
        "msg" => "success",
        "code" => 0,
        "data" => json_value
    };
    return json::stringify(obj)
}