mod find_point;

fn main() {
    let px: i32 = 3;
    let py: i32 = 0;
    let qx: i32 = 2;
    let qy: i32 = 0;

    let result = find_point::find_point(px, py, qx, qy);

    for x in result {
        println!("{x}");
    }
}
