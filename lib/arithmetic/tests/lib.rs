extern crate arithmetic;

use arithmetic::ArithmeticCode;

#[test]
fn foo() {
    let data = b"A_DEAD_DAD_CEDED_A_BAD_BABE_A_BEADED_ABACA_BED";
    // let data = b"AAAAAAAAAAAAAAABBBBBBBCCCCCCDDDDDDEEEEE";
    let code = ArithmeticCode::from_data(data);
    let comp = code.transform(data);
    println!("{:?} -> {:?}", (String::from_utf8(data.to_vec())), comp);
    println!("{:?}", String::from_utf8(code.recover(comp.unwrap()).unwrap()));
}
