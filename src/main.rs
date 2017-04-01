const n: usize = 4;
fn main() {
    let mut m: [[f32; n + 1]; n] = [
        [1.0, 1.0, 1.0, 1.0, 10.0],
        [2.0, 1.0, 2.0, 1.0, 14.0],
        [1.0, 2.0, 3.0, -4.0, -2.0],
        [1.0, -1.0, -1.0, 1.0, 0.0],
    ];
    for i in 0..n {
        let pivot = m[i][i];
        for j in 0..(n + 1) {
            m[i][j] = (1.0 / pivot) * m[i][j];
        }
        for k in (i + 1)..n {
            let c = m[k][i];
            for l in i..(n + 1) {
                m[k][l] -= c * m[i][l];
            }
        }
    }
    println!("{:?}", m);
}
