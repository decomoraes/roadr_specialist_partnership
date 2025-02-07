// pub fn size(size: f64) -> String {
//     format!("{}px", size * 16)
// }

pub fn size(size: f64) -> &'static str {
    Box::leak(format!("{}px", size * 16.0).into_boxed_str())
}
