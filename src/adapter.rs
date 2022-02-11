pub mod adapter {
  fn function(a: u32) {
    println!("hello world{}", a)
  }
  pub fn function_adapter(a: u32) {
    function(a);
    println!("hello world2")
  }
}
