#[cfg(test)]

pub mod observer {
  trait Observer {
    fn update(&self);
  }

  trait Observable<'a, T: Observer> {
    fn add_observer(&mut self, observer: &'a T);
    fn delete_observer(&mut self, observer: &'a T);
    fn notify_observers(&self);
  }

  struct Display {
    name: String,
  }

  struct Weather<'a, T> {
    temperature: f64,
    observers: Vec<&'a T>,
  }

  impl<'a> Weather<'a, Display> {
    fn set_temperature(&mut self, temperature: f64) {
      self.temperature = temperature;
      // self.not
    }
  }

  impl Observer for Display {
    fn update(&self) {
      println!("Display {} updated!", self.name);
    }
  }

  impl Display {
    fn new(name: String) -> Display {
      Display { name: name }
    }
  }

  impl std::cmp::PartialEq for Display {
    fn eq(&self, other: &Display) -> bool {
      self.name == other.name
    }
  }

  impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "Display {}", self.name)
    }
  }

  impl<'a, T: Observer + PartialEq + std::fmt::Display> Observable<'a, T> for Weather<'a, T> {
    fn add_observer(&mut self, observer: &'a T) {
      println!("add_observer({});", observer);
      self.observers.push(observer);
    }
    fn delete_observer(&mut self, observer: &'a T) {
      let mut index = 0;
      let mut found = false;
      for &obs in self.observers.iter() {
        if obs == observer {
          println!("delete_observer({});", observer);
          found = true;
          break;
        }
        index += 1;
      }
      if found {
        self.observers.remove(index);
      }
    }
    fn notify_observers(&self) {
      for &observer in self.observers.iter() {
        observer.update();
      }
    }
  }

  #[test]
  fn execute1() {
    let display = Display::new("Desktop".to_string());
    let display2 = Display::new("Desktop2".to_string());
    let mut weather = Weather {
      temperature: 19.0,
      observers: Vec::new(),
    };
    weather.add_observer(&display);
    weather.add_observer(&display2);
    weather.set_temperature(20.0);
    weather.delete_observer(&display2);
    weather.set_temperature(21.0);
    weather.notify_observers()
    // Observer::on('one')
    // Observer::on('two')

    // Observer::notify()
  }
  trait Observeable2<T> {
    fn register(&mut self, observer: Box<dyn Observer2<Item = T>>);
  }

  trait Observer2 {
    type Item;
    fn notify(&self, val: &Self::Item);
  }

  struct EventCounter {
    counter: u32,
    observers: Vec<Box<dyn Observer2<Item = u32>>>,
  }

  impl EventCounter {
    fn new() -> Self {
      Self {
        counter: 0u32,
        observers: Vec::new(),
      }
    }

    fn run(&mut self) {
      loop {
        use std::thread;
        use std::time::Duration;
        thread::sleep(Duration::from_millis(self.get_rand_duration()));
        self.counter += 1;
        if self.counter % 2 == 0 {
          for observer in self.observers.iter() {
            observer.notify(&self.counter);
          }
        }
      }
    }

    fn get_rand_duration(&self) -> u64 {
      if cfg!(target_os = "windows") {
        600u64
      } else {
        use std::process::Command;
        use std::str::FromStr;
        let rand_cmd = Command::new("sh")
          .arg("-c")
          .arg("echo $(( $RANDOM%1000 + 1 ))")
          .output()
          .expect("failed to get OS RNG");
        u64::from_str(&String::from_utf8(rand_cmd.stdout).unwrap().trim()).unwrap()
      }
    }
  }

  impl Observeable2<u32> for EventCounter {
    fn register(&mut self, observer: Box<dyn Observer2<Item = u32>>) {
      self.observers.push(observer)
    }
  }

  struct EvenObserver {
    name: String,
  }

  impl EvenObserver {
    fn new(name: String) -> Self {
      Self { name }
    }
    fn name(&self) -> &str {
      &self.name
    }
  }

  impl Observer2 for EvenObserver {
    type Item = u32;
    fn notify(&self, val: &Self::Item) {
      println!("{} got {}", self.name(), val)
    }
  }

  #[test]
  fn execute2() {
    let mut foo = EventCounter::new();
    let (bar, baz, quux) = (
      Box::new(EvenObserver::new("bar".to_string())),
      Box::new(EvenObserver::new("baz".to_string())),
      Box::new(EvenObserver::new("quux".to_string())),
    );
    foo.register(bar);
    foo.register(baz);
    foo.register(quux);
    foo.run();
  }

  trait Subject3<T: Clone> {
    fn notify_observers(&self, a: &T);
    fn register_observer(&mut self, a: Box<dyn Observer3<T>>) -> usize;
    fn unregister_observer(&mut self, a: usize);
  }

  trait Observer3<T: Clone> {
    fn no_notify(&self, a: &T);
  }

  #[derive(Debug, Clone)]
  struct EventObject3(usize);

  struct SubjectX {
    observers: Vec<(bool, Box<dyn Observer3<EventObject3>>)>,
  }

  impl SubjectX {
    fn new() -> SubjectX {
      SubjectX {
        observers: Vec::new(),
      }
    }
  }

  impl Subject3<EventObject3> for SubjectX {
    fn notify_observers(&self, a: &EventObject3) {
      for observer in self.observers.iter() {
        if observer.0 {
          observer.1.no_notify(a)
        }
      }
    }

    fn register_observer(&mut self, a: Box<dyn Observer3<EventObject3>>) -> usize {
      self.observers.push((true, a));
      self.observers.len() - 1
    }

    fn unregister_observer(&mut self, a: usize) {
      self.observers[a].0 = false
    }
  }

  struct ObserverX3(usize);
  impl Observer3<EventObject3> for ObserverX3 {
    fn no_notify(&self, a: &EventObject3) {
      println!("ObserverX {} Get {:?}", self.0, a)
    }
  }

  #[test]
  fn execute3() {
    let mut subject = SubjectX::new();
    subject.register_observer(Box::new(ObserverX3(1)));
    subject.register_observer(Box::new(ObserverX3(2)));
    subject.register_observer(Box::new(ObserverX3(3)));

    subject.notify_observers(&EventObject3(100));
    subject.notify_observers(&EventObject3(20));
  }

  trait WatchTrait {
    fn update(&self);
  }

  struct Watch {
    name: String,
  }

  impl Watch {
    fn new(name: String) -> Self {
      Self { name }
    }
  }

  impl WatchTrait for Watch {
    fn update(&self) {
      println!(">>>>>>>>>>>>>>>>>{}", self.name)
    }
  }

  trait Observer4T {
    fn emit(&mut self, name: String, callback: Box<dyn WatchTrait>);
    fn on(&mut self, name: String);
  }

  struct Observer4 {
    list: Vec<(String, Vec<Box<dyn WatchTrait>>)>,
  }

  impl Observer4 {
    fn new() -> Self {
      Self { list: Vec::new() }
    }
  }
  impl Observer4T for Observer4 {
    fn emit(&mut self, name: String, callback: Box<dyn WatchTrait>) {
      let item = self.list.iter_mut().find(|item| item.0 == name);

      if let Some(t) = item {
        t.1.push(callback)
      } else {
        let mut k = Vec::new();
        k.push(callback);
        self.list.push((name, k));
      }
      // for item in self.list.iter_mut() {
      //   if item.0 == name {
      //     item.1.push(callback);
      //     return;
      //     // index = i as i32
      //   }
      // }

      // let mut k = Vec::new();
      // k.push(callback);
      // let l = (name, k);
      // self.list.push(l);
    }
    fn on(&mut self, name: String) {
      for item in self.list.iter() {
        if item.0 == name {
          for t in item.1.iter() {
            t.update()
          }
        }
      }
    }
  }

  #[test]
  fn execute4() {
    let mut o = Observer4::new();
    o.emit("noe".to_string(), Box::new(Watch::new("name".to_string())));
    o.emit("noe".to_string(), Box::new(Watch::new("3232".to_string())));
    o.on("noe".to_string())
  }
}
