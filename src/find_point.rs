pub fn find_point(px: i32, py: i32, qx: i32, qy: i32) -> Vec<i32> {
    let new_px: i32 = px + (qx - px) * 2;
    let new_py: i32 = py + (qy - py) * 2;
    let v: Vec<i32> = vec![new_px, new_py];
    v
}
