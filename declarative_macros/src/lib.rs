// A simplified version of the vec! macro to serve as an example.
// 'macro_rules!' is planned to be replaced with something "better" in the future.
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}