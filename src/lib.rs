#[macro_export]
macro_rules! macro_task_first {
    ( $( $x:expr ),* ) => {
        (
            $(
                $x(),
            )*
        )
        }
}

#[allow(dead_code)]
fn foo() -> i32 {
    1
}

#[allow(dead_code)]
fn foo_even() -> i32 {
    1
}

#[allow(dead_code)]
fn bar_even() -> f32 {
    2.0
}

#[allow(dead_code)]
fn bar() -> f32 {
    2.0
}

#[allow(dead_code)]
fn baz() -> String {
    "Some".to_string()
}

#[allow(dead_code)]
fn baz_even() -> String {
    "Some".to_string()
}

#[test]
fn test_macro_task_first() {
    assert_eq!((1,), macro_task_first!(foo));
    assert_eq!(
        (1, 2.0, "Some".to_string()),
        macro_task_first!(foo, bar, baz)
    );
}

#[test]
fn test_macro_p_task_2() {
    let res = macro_p::fn_calls!("foo", "foo_even", "bar", "bar_even", "baz", "baz_even");
    assert_eq!((1, 2.0, "Some".to_string()), res);
}
