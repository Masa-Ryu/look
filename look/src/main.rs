use std::collections::{HashMap, HashSet, LinkedList, BinaryHeap};
use std::rc::Rc;
use std::sync::Arc;
use std::ptr::NonNull;

// use colored::*;

fn main() {

    // primitive types
    let int8: i8 = 42;
    let int16: i16 = 42;
    let int32: i32 = 42;
    let int64: i64 = 42;
    let int128: i128 = 42;

    let uint8: u8 = 42;
    let uint16: u16 = 42;
    let uint32: u32 = 42;
    let uint64: u64 = 42;
    let uint128: u128 = 42;

    let float32: f32 = 3.14;
    let float64: f64 = 3.14;

    let boolean: bool = true;
    let character: char = 'a';

    // composite types
    let tuple: (i32, f64, char) = (42, 3.14, 'a');
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    // text types
    let str_slice: &str = "Hello, world!";
    let string: String = String::from("Hello, world!");

    // collection types
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    let slice: &[i32] = &vector[..];

    let mut hash_map: HashMap<String, i32> = HashMap::new();
    hash_map.insert("key".to_string(), 42);

    let mut hash_set: HashSet<i32> = HashSet::new();
    hash_set.insert(42);

    let mut linked_list: LinkedList<i32> = LinkedList::new();
    linked_list.push_back(42);

    let mut binary_heap: BinaryHeap<i32> = BinaryHeap::new();
    binary_heap.push(42);

    // other types
    let optional: Option<i32> = Some(42);
    let result: Result<i32, String> = Ok(42);
    let _closure: Box<dyn Fn()> = Box::new(|| println!("I'm a closure."));
    let _fn_pointer: fn() = || println!("I'm a function pointer.");

    // User-defined types
    #[derive(Debug)]
    struct MyStruct {
        _field: i32,
        _bool_field: bool,
    }
    let my_struct = MyStruct { _field: 42, _bool_field: true };

    #[derive(Debug)]
    enum MyEnum {
        _Variant(i32),
        _Boolean(bool)
    }
    let my_enum = MyEnum::_Variant(42);

    // Smart pointers and pointer types
    let box_type: Box<i32> = Box::new(42);
    let rc_type: Rc<i32> = Rc::new(42);
    let arc_type: Arc<i32> = Arc::new(42);
    let non_null_type: Option<NonNull<i32>> = NonNull::new(Box::into_raw(Box::new(42)));


    // すべての変数を出力
    println!("int8: {:?}", int8);
    println!("int16: {:?}", int16);
    println!("int32: {:?}", int32);
    println!("int64: {:?}", int64);
    println!("int128: {:?}", int128);
    println!("uint8: {:?}", uint8);
    println!("uint16: {:?}", uint16);
    println!("uint32: {:?}", uint32);
    println!("uint64: {:?}", uint64);
    println!("uint128: {:?}", uint128);
    println!("float32: {:?}", float32);
    println!("float64: {:?}", float64);
    println!("boolean: {:?}", boolean);
    println!("character: {:?}", character);
    println!("tuple: {:?}", tuple);
    println!("array: {:?}", array);
    println!("str_slice: {:?}", str_slice);
    println!("string: {:?}", string);
    println!("vector: {:?}", vector);
    println!("slice: {:?}", slice);
    println!("optional: {:?}", optional);
    println!("result: {:?}", result);
    println!("hash_map: {:?}", hash_map);
    println!("hash_set: {:?}", hash_set);
    println!("linked_list: {:?}", linked_list);
    println!("binary_heap: {:?}", binary_heap);
    println!("my_struct: {:?}", my_struct);
    println!("my_enum: {:?}", my_enum);
    println!("box_type: {:?}", box_type);
    println!("rc_type: {:?}", rc_type);
    println!("arc_type: {:?}", arc_type);
    println!("non_null_type: {:?}", non_null_type);

}

