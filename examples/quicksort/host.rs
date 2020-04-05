#[link(name = "roc_qs_main")]
extern "C" {
    #[allow(improper_ctypes)]
    #[link_name = "$Test.main"]
    fn list_from_roc() -> Box<[i64]>;
}

pub fn main() {
    let list = unsafe { list_from_roc() };

    println!("Roc says: {:?}", list);
}
