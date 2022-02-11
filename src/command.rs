pub mod command {
  #[derive(PartialEq, Debug)]
  pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
  }
  impl Robot {
    pub fn new() -> Robot {
      Robot {
        x: 0,
        y: 0,
        dx: 0,
        dy: 1,
      }
    }
    fn move_forward(&mut self) {
      self.x += self.dx;
      self.y += self.dy;
    }

    fn set_direction(&mut self, d: (i32, i32)) {
      self.dx = d.0;
      self.dy = d.1;
    }

    fn get_direction(&self) -> (i32, i32) {
      (self.dx, self.dy)
    }
  }

  pub trait Command<T> {
    fn execute(&self, _: &mut T);
    fn undo(&self, _: &mut T);
  }

  pub struct Invoker<'a, T: 'a> {
    commands: Vec<Box<dyn Command<T> + 'a>>,
    target: &'a mut T,
    current_index: usize,
  }

  impl<'a, T> Invoker<'a, T> {
    pub fn new(t: &'a mut T) -> Self {
      Self {
        commands: Vec::new(),
        target: t,
        current_index: 0,
      }
    }

    pub fn target(&self) -> &T {
      self.target
    }
    pub fn append_command<U: Command<T> + 'a>(&mut self, c: U) {
      self.commands.push(Box::new(c))
    }

    pub fn execute_command(&mut self) {
      if self.commands.len() <= self.current_index {
        return;
      }

      let c = &*self.commands[self.current_index];
      let t = &mut *self.target;
      c.execute(t);
      self.current_index += 1;
    }
    pub fn execute_all_commands(&mut self) {
      for _ in self.current_index..self.commands.len() {
        self.execute_command();
      }
    }

    pub fn undo(&mut self) {
      if 0 == self.current_index {
        return;
      }
      self.current_index -= 1;
      let c = &*self.commands[self.current_index];
      let t = &mut *self.target;
      c.undo(t)
    }
  }

  pub struct CommandMoveForward;

  impl Command<Robot> for CommandMoveForward {
    fn execute(&self, r: &mut Robot) {
      r.move_forward()
    }
    fn undo(&self, r: &mut Robot) {
      let c1 = CommandTurnRight;
      c1.execute(r);
      c1.execute(r);
      self.execute(r);
      c1.execute(r);
      c1.execute(r);
    }
  }

  pub struct CommandTurnRight;
  impl Command<Robot> for CommandTurnRight {
    fn execute(&self, r: &mut Robot) {
      let (dx, dy) = r.get_direction();
      r.set_direction((dy, -dx));
    }

    fn undo(&self, r: &mut Robot) {
      let c = CommandTurnLeft;
      c.execute(r);
    }
  }

  pub struct CommandTurnLeft;

  impl Command<Robot> for CommandTurnLeft {
    fn execute(&self, r: &mut Robot) {
      let (dx, dy) = r.get_direction();
      r.set_direction((-dy, dx));
    }
    fn undo(&self, r: &mut Robot) {
      let c = CommandTurnRight;
      c.execute(r);
    }
  }
}
