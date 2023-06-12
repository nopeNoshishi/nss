#[allow(dead_code)]
fn padding(size: usize) -> usize {
    // calclate padding size
    let floor = (size - 2) / 8;
    let target = (floor + 1) * 8 + 2;
    let padding = target - size;

    padding
}