use std::f64::consts::PI;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: usize,
    verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Calculation {
    Perimeter,
    Area,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "shape")]
enum Shape {
    Circle { radius: f64 },
    Rectangle { length: f64, width: f64 },
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    calculation: Calculation,
    #[serde(flatten)]
    shape: Shape,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    result: f64,
}

fn calculate_handler(req: Request) -> Response {
    let result = match (req.calculation, req.shape) {
        (Calculation::Perimeter, Shape::Circle { radius }) => PI * 2.0 * radius,
        (Calculation::Perimeter, Shape::Rectangle { length, width }) => 2.0 * (length + width),
        (Calculation::Area, Shape::Circle { radius }) => PI * radius * radius,
        (Calculation::Area, Shape::Rectangle { length, width }) => length * width,
    };

    Response { result }
}

fn main() {
    let json = r#"
        {
            "name": "zhongren",
            "age": 20,
            "verified": false
        }
        "#;
    let person: Person = serde_json::from_str(json).expect("Wrong json structure");
    println!("json: {:?}", person);

    let cal = r#"
        {
                  "calculation": "perimeter",
                  "shape": "circle",
                  "radius": 2.3
                }
        "#;
    let request: Request = serde_json::from_str(cal).unwrap();
    let res: Response = calculate_handler(request);
    println!("area: {:?}", res)
}
