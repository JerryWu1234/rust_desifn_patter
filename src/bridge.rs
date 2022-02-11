pub mod bridge {
  pub trait Adapter {
    fn get_a(&self) -> usize;
    fn set_b(&self) -> usize;
  }
  pub struct ObjectX {
    pub a: usize,
    pub b: usize,
  }

  impl Adapter for ObjectX {
    fn get_a(&self) -> usize {
      self.a
    }
    fn set_b(&self) -> usize {
      self.b
    }
  }

  pub struct ObjectY {
    pub m: u32,
    pub n: u32,
  }

  impl Adapter for ObjectY {
    fn get_a(&self) -> usize {
      self.m as usize
    }
    fn set_b(&self) -> usize {
      self.n as usize
    }
  }
  
}