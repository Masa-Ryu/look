
pub mod look {
    use colored::{Colorize, ColoredString};

    // pub fn look<T: Colored>(write: T) {
        // print!("{}", write.colored());
    // }

    // pub fn lookln<T: Colored>(write: T) {
        // println!("{}", write.colored());
    // }
    pub fn look(write: Box<dyn Colored>) {
        print!("{}", write.colored());
    }

    pub fn lookln(write: Box<dyn Colored>) {
        println!("{}", write.colored());
    }
    trait Colored {
        fn colored(&self) -> ColoredString;
    }

    impl Colored for i8 {
        fn colored(&self) -> ColoredString {
            format!("{}", self).green()
        }
    }

    impl Colored for i16 {
        fn colored(&self) -> ColoredString {
            format!("{}", self).green()
        }
    }

    impl Colored for i32 {
        fn colored(&self) -> ColoredString {
            format!("{}", self).green()
        }
    }

    impl Colored for i64 {
        fn colored(&self) -> ColoredString {
            format!("{}", self).green()
        }
    }

    impl Colored for i128 {
        fn colored(&self) -> ColoredString {
            format!("{}", self).green()
        }
    }

    impl Colored for u8 {
        fn colored(&self) -> ColoredString {
            format!("{}", self).on_green()
        }
    }

    impl Colored for u16 {
        fn colored(&self) -> ColoredString {
            format!("{}", self).on_green()
        }
    }

    impl Colored for u32 {
        fn colored(&self) -> ColoredString {
            format!("{}", self).on_green()
        }
    }

    impl Colored for u64 {
        fn colored(&self) -> ColoredString {
            format!("{}", self).on_green()
        }
    }

    impl Colored for u128 {
        fn colored(&self) -> ColoredString {
            format!("{}", self).on_green()
        }
    }

    impl Colored for f32 {
        fn colored(&self) -> ColoredString {
            format!("{}", self).blue()
        }
    }

    impl Colored for f64 {
        fn colored(&self) -> ColoredString {
            format!("{}", self).blue()
        }
    }

    impl Colored for bool {
        fn colored(&self) -> ColoredString {
            format!("{}", self).yellow()
        }
    }

    impl Colored for char {
        fn colored(&self) -> ColoredString {
            format!("{}", self).red()
        }
    }

    impl Colored for &'static str {
        fn colored(&self) -> ColoredString {
            format!("{}", self).cyan()
        }
    }

    impl Colored for String {
        fn colored(&self) -> ColoredString {
            format!("{}", self).cyan()
        }
    }

    impl Colored for (i32, f64, char) {
        fn colored(&self) -> ColoredString {
            format!("{:?}", self).purple()
        }
    }


}




#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet, LinkedList, BinaryHeap};
    use std::rc::Rc;
    use std::sync::Arc;
    use std::ptr::NonNull;
    use super::*;

    // primitive types
    fn setup_int8() -> i8 {
        42
    }

    fn setup_int16() -> i16 {
        42
    }

    fn setup_int32() -> i32 {
        42
    }

    fn setup_int64() -> i64 {
        42
    }

    fn setup_int128() -> i128 {
        42
    }

    fn setup_uint8() -> u8 {
        42
    }

    fn setup_uint16() -> u16 {
        42
    }

    fn setup_uint32() -> u32 {
        42
    }

    fn setup_uint64() -> u64 {
        42
    }

    fn setup_uint128() -> u128 {
        42
    }

    fn setup_float32() -> f32 {
        3.14
    }

    fn setup_float64() -> f64 {
        3.14
    }

    fn setup_boolean() -> bool {
        true
    }

    fn setup_character() -> char {
        'a'
    }

    // text types
    fn setup_str_slice() -> &'static str {
        "Hello, world!"
    }

    fn setup_string() -> String {
        String::from("Hello, world!")
    }

    // composite types
    fn setup_tuple() -> (i32, f64, char) {
        (42, 3.14, 'a')
    }

    fn setup_array() -> [i32; 5] {
        [1, 2, 3, 4, 5]
    }

    // collection types
    fn setup_vector() -> Vec<i32> {
        vec![1, 2, 3, 4, 5]
    }

    fn setup_slice() -> &'static[i32] {
        &[1, 2, 3, 4]
    }

    fn setup_hash_map() -> HashMap<String, i32> {
        let mut hash_map: HashMap<String, i32> = HashMap::new();
        hash_map.insert("key".to_string(), 42);
        hash_map
    }

    fn setup_hash_set() -> HashSet<i32> {
        let mut hash_set: HashSet<i32> = HashSet::new();
        hash_set.insert(42);
        hash_set
    }

    fn setup_linked_list() -> LinkedList<i32> {
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.push_back(42);
        linked_list
    }

    fn setup_binary_heap() -> BinaryHeap<i32> {
        let mut binary_heap: BinaryHeap<i32> = BinaryHeap::new();
        binary_heap.push(42);
        binary_heap
    }

    // other types
    fn setup_optional() -> Option<i32> {
        Some(42)
    }

    fn setup_result() -> Result<i32, String> {
        Ok(42)
    }

    fn setup_closure() -> Box<dyn Fn()> {
        Box::new(|| println!("I'm a closure."))
    }

    fn setup_fn_pointer() -> fn() {
        || println!("I'm a function pointer.")
    }

    // User-defined types
    #[derive(Debug)]
    struct MyStruct {
        _field: i32,
        _bool_field: bool,
    }
    fn setup_my_struct() -> MyStruct {
        MyStruct { _field: 42, _bool_field: true }
    }

    #[derive(Debug)]
        enum MyEnum {
            _Variant(i32),
            _Boolean(bool)
        }
    fn setup_my_enum() -> MyEnum {
        MyEnum::_Variant(42)
    }

    // Smart pointers and pointer types
    fn setup_box_type() -> Box<i32> {
        Box::new(42)
    }

    fn setup_rc_type() -> Rc<i32> {
        Rc::new(42)
    }

    fn setup_arc_type() -> Arc<i32> {
        Arc::new(42)
    }

    fn setup_non_null_type() -> Option<NonNull<i32>> {
        NonNull::new(Box::into_raw(Box::new(42)))
    }
    #[test]
    fn test_normal() {
        println!("{}", "-".repeat(80));
        println!("They are normal prints.");

        println!("int8: {:?}", setup_int8());
        println!("int16: {:?}", setup_int16());
        println!("int32: {:?}", setup_int32());
        println!("int64: {:?}", setup_int64());
        println!("int128: {:?}", setup_int128());
        println!("uint8: {:?}", setup_uint8());
        println!("uint16: {:?}", setup_uint16());
        println!("uint32: {:?}", setup_uint32());
        println!("uint64: {:?}", setup_uint64());
        println!("uint128: {:?}", setup_uint128());
        println!("float32: {:?}", setup_float32());
        println!("float64: {:?}", setup_float64());
        println!("boolean: {:?}", setup_boolean());
        println!("character: {:?}", setup_character());
        println!("str_slice: {:?}", setup_str_slice());
        println!("string: {:?}", setup_string());
        println!("tuple: {:?}", setup_tuple());
        println!("array: {:?}", setup_array());
        println!("vector: {:?}", setup_vector());
        println!("slice: {:?}", setup_slice());
        println!("hash_map: {:?}", setup_hash_map());
        println!("hash_set: {:?}", setup_hash_set());
        println!("linked_list: {:?}", setup_linked_list());
        println!("binary_heap: {:?}", setup_binary_heap());
        println!("optional: {:?}", setup_optional());
        println!("result: {:?}", setup_result());
        setup_closure();
        println!("fn_pointer: {:?}", setup_fn_pointer());
        println!("my_struct: {:?}", setup_my_struct());
        println!("my_enum: {:?}", setup_my_enum());
        println!("box_type: {:?}", setup_box_type());
        println!("rc_type: {:?}", setup_rc_type());
        println!("arc_type: {:?}", setup_arc_type());
        println!("non_null_type: {:?}", setup_non_null_type());


    }

    #[test]
    fn test_look() {
        println!("{}", "-".repeat(80));
        println!("Uses look");

        print(setup_int8());
        println!("");
        print(setup_int16());
        println!("");
        print(setup_int32());
        println!("");
        print(setup_int64());
        println!("");
        print(setup_int128());
        println!("");
        print(setup_uint8());
        println!("");
        print(setup_uint16());
        println!("");
        print(setup_uint32());
        println!("");
        print(setup_uint64());
        println!("");
        print(setup_uint128());
        println!("");
        print(setup_float32());
        println!("");
        print(setup_float64());
        println!("");
        print(setup_boolean());
        println!("");
        print(setup_character());
        println!("");
        // look(setup_tuple());
        // println!("");
        // look(setup_array());
        // println!("");
        // look(setup_str_slice());
        // println!("");
        // look(setup_string());
        // println!("");
        // look(setup_vector());
        // println!("");
        // look(setup_slice());
        // println!("");
        // look(setup_optional());
        // println!("");
        // look(setup_result());
        // println!("");
        // look(setup_hash_map());
        // println!("");
        // look(setup_hash_set());
        // println!("");
        // look(setup_linked_list());
        // println!("");
        // look(setup_binary_heap());
        // println!("");
        // look(setup_my_struct());
        // println!("");
        // look(setup_my_enum());
        // println!("");
        // look(setup_box_type());
        // println!("");
        // look(setup_rc_type());
        // println!("");
        // look(setup_arc_type());
        // println!("");
        // look(setup_non_null_type());
        // println!("");
    }

    #[test]
    fn test_lookln() {
        println!("{}", "-".repeat(80));
        println!("Uses lookln");

        println(setup_int8());
        println(setup_int16());
        println(setup_int32());
        println(setup_int64());
        println(setup_int128());
        println(setup_uint8());
        println(setup_uint16());
        println(setup_uint32());
        println(setup_uint64());
        println(setup_uint128());
        println(setup_float32());
        println(setup_float64());
        println(setup_boolean());
        println(setup_character());
        // lookln(setup_tuple());
        // lookln(setup_array());
        println(setup_str_slice());
        println(setup_string());
        // lookln(setup_vector());
        // lookln(setup_slice());
        // lookln(setup_optional());
        // lookln(setup_result());
        // lookln(setup_hash_map());
        // lookln(setup_hash_set());
        // lookln(setup_linked_list());
        // lookln(setup_binary_heap());
        // lookln(setup_my_struct());
        // lookln(setup_my_enum());
        // lookln(setup_box_type());
        // lookln(setup_rc_type());
        // lookln(setup_arc_type());
        // lookln(setup_non_null_type());
    }


}