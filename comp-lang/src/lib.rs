#[macro_use]
extern crate descr_common;
mod lang;

pub fn parse() {
    let buf = descr_common::util::load_file("../src/comps/counter.comp");
    let res = lang::parsers::start(&buf[..]);
    println!("\n= Result ===========================");
    println!("{:#?}", res);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
