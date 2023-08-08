pub fn approximately_eq(actual: f32, expected: f32, boundary: f32) -> bool {
    if expected + boundary > actual && expected - boundary < actual {
        return true;
    }
    false
}