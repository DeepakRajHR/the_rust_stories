fn main() {
    // The variables can't change the type just by using `mut`
    let _var = 10;
    let _var = "str".to_string();

    /* The following is not legit because
     * let mut _var = 10;
     * _var = "str".to_string();
     */

    // Nothing conclusive from the following code
    let mut _var = 10;
    // We can shadow mutable variables and make them immutable
    let _var = 40;

    let _immutable_var = 10;
    // We shadow immutable variables and make them mutable
    let mut _immutable_var = 30;

    let _str_var: String = String::new();
    let mut _str_var: String = "bcd".to_string();
}
