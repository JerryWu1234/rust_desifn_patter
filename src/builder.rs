pub mod builder {
  #[derive(Clone, Debug)]
  pub struct Person {
    name: String,
    age: u32,
    job: Option<String>,
  }
  impl Person {
    fn new() -> Person {
      Person {
        name: Default::default(),
        age: Default::default(),
        job: Default::default(),
      }
    }
    fn set_name(&mut self, name: String) {
      self.name = name
    }
    fn set_age(&mut self, age: u32) {
      self.age = age
    }
    fn set_job(&mut self, job: Option<String>) {
      self.job = job
    }
  }

  pub trait Builder {
    fn build_name(&mut self);
    fn build_age(&mut self);
    fn build_job(&mut self);
    fn build(&mut self) -> Person;
  }

  pub struct AliceBuilder {
    obj: Person,
  }

  impl AliceBuilder {
    pub fn new() -> AliceBuilder {
      AliceBuilder { obj: Person::new() }
    }
  }
  impl Builder for AliceBuilder {
    fn build_name(&mut self) {
      self.obj.set_name("Alice".to_string())
    }

    fn build_age(&mut self) {
      self.obj.set_age(12)
    }

    fn build_job(&mut self) {
      self.obj.set_job(Some("student".to_string()))
    }

    fn build(&mut self) -> Person {
      self.obj.clone()
    }
  }

  pub struct Director {
    builder: Box<dyn Builder>,
  }

  impl Director {
    pub fn new(b: Box<dyn Builder>) -> Director {
      Director { builder: b }
    }
    pub fn build(&mut self) -> Person {
      self.builder.build_name();
      self.builder.build_age();
      self.builder.build_job();
      self.builder.build()
    }
  }
}
