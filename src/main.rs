mod section;

fn main() {
    println!("section1---------");
    section::section_1::hello_world();
    section::section_1::print_array();
    println!("-----------------");

    println!("section2---------");
    section::section_2::compile_error();
    println!("-----------------");

    println!("section3---------");
    section::section_3::imutable_valiable();
    section::section_3::mutable_valiable();
    section::section_3::constant_valiable();
    section::section_3::static_valiable();
    println!("-----------------");

    println!("section4---------");
    section::section_4::interger();
    section::section_4::float();
    section::section_4::bool();
    section::section_4::array();
    section::section_4::slice();
    section::section_4::mutable_slice();
    section::section_4::tuple();
    section::section_4::string();
    section::section_4::string_to_bytearray();
    println!("-----------------");

    println!("section5---------");
    section::section_5::ifelse();
    section::section_5::ifelse_is_expression();
    section::section_5::rust_loop();
    section::section_5::rust_while();
    section::section_5::rust_for();
    println!("-----------------");
}
