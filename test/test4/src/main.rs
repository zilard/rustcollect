
struct Vec2 {
    x: f64,
    y: f64,
}



fn main() {

    let v1 = Vec2 { x: 1.0, y: 3.0 };
    let v2 = Vec2 { y: 2.0, x: 4.0 };

    println!("| v1: {0} | v2: {1} |", v1.x, v2.x);


    let v3 = Vec2 {
        x: 14.0,
        ..v2
    };

    println!("| v3: {} |", v3.x);

 
    let v4 = Vec2 { ..v3 };

    println!("| v4: {} |", v4.x);


 
    let v = Vec2 { x: 3.0, y: 6.0 };
    let Vec2 { x, y } = v;

    println!(" x: {0}  |   y: {1} ", x, y);


    let Vec2 { x, .. } = v;
    println!(" x: {} ", x);


}

