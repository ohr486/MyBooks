fn main() {
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";

    let ans = rpn(exp);

    debug_assert_eq!("26.284", format!("{:.4}", ans));

    println!("{} = {:.4}", exp, ans);
}
