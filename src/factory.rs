#[cfg(test)]
pub mod factory {
  trait ProductX {
    fn get_value(&self) -> String;
  }

  trait ProductY {
    fn get_value(&self) -> String;
  }
  trait AbstractFactory {
    fn create_product_x(&self) -> Box<dyn ProductX>;
    fn create_product_y(&self) -> Box<dyn ProductY>;
  }

  struct ConcreteProductX(String);
  impl ConcreteProductX {
    fn new(msg: String) -> Self {
      Self(msg + &"ProductX".to_string())
    }
  }

  impl ProductX for ConcreteProductX {
    fn get_value(&self) -> String {
      self.0.clone()
    }
  }

  struct ConcreteProductY(String);
  impl ConcreteProductY {
    fn new(msg: String) -> Self {
      Self(msg + &"ProductY".to_string())
    }
  }

  impl ProductY for ConcreteProductY {
    fn get_value(&self) -> String {
      self.0.clone()
    }
  }

  struct ConcreteFactoryA;
  impl AbstractFactory for ConcreteFactoryA {
    fn create_product_x(&self) -> Box<dyn ProductX> {
      Box::new(ConcreteProductX::new("FactoryA".to_string()))
    }
    fn create_product_y(&self) -> Box<dyn ProductY> {
      Box::new(ConcreteProductY::new("FactoryA".to_string()))
    }
  }

  struct ConcreteFactoryB;
  impl AbstractFactory for ConcreteFactoryB {
    fn create_product_x(&self) -> Box<dyn ProductX > {
      Box::new(ConcreteProductX::new("FactoryB".to_string())) as Box<dyn ProductX>
    }

    fn create_product_y(&self) -> Box<dyn ProductY> {
      Box::new(ConcreteProductY::new("FactoryB".to_string())) as Box<dyn ProductY>
    }
  }

  enum FactoryID {
    A,
    B,
  }
  fn cteate_factory<'a>(id: FactoryID) -> Box<dyn AbstractFactory> {
    match id {
      FactoryID::A => Box::new(ConcreteFactoryA),
      FactoryID::B => Box::new(ConcreteFactoryB),
    }
  }
  #[test]
  fn mm() {
    let factory_a = cteate_factory(FactoryID::A);
    let a_x = factory_a.create_product_x();
    let a_y = factory_a.create_product_y();
    println!("><><{}", a_x.get_value());
    println!("><><{}", a_y.get_value());

    let factory_b = cteate_factory(FactoryID::B);
    let b_x = factory_b.create_product_x();
    let b_y = factory_b.create_product_y();
    println!("><><{}", b_x.get_value());
    println!("><><{}", b_y.get_value());
  }

  struct FactoryStruct(String);
  trait FactoryM {
    fn gets(&self) -> String;
    fn sets(&mut self, b: String);
  }

  impl FactoryStruct {
    fn new() -> Self {
      Self(String::from("3"))
    }
  }
  impl FactoryM for FactoryStruct {
    fn gets(&self) -> String {
      self.0.clone()
    }
    fn sets(&mut self, b: String) {
      self.0 = b;
    }
  }
  

  struct MM;

  impl MM {
    fn create_factory() -> Box<dyn FactoryM> {
        Box::new(FactoryStruct::new())
    }
  }
  #[test]
  fn cm() {
    let mut m = MM::create_factory();
    
    m.sets(String::from("dsdsd"));
    let c = m.gets();
    println!("{:?}", c)
  }
}
