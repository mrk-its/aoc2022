#![no_std]
#![no_main]

#[mos_test::tests]
mod tests {
    use ufmt_stdio::*;
    use utils::*;

    #[test]
    fn foo() {
        println!("oki!");
    }
}
