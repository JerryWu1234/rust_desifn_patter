mod adapter;
mod bridge;
mod builder;
mod command;
mod factory;
use crate::adapter::adapter::function_adapter;
use crate::bridge::bridge::{Adapter, ObjectX, ObjectY};
use crate::builder::builder::{AliceBuilder, Director};
use crate::command::command::{
    CommandMoveForward, CommandTurnLeft, CommandTurnRight, Invoker, Robot,
};

fn main() {
    function_adapter(3);
    // bridge patter
    println!("-_______________________bridge");
    let obj_x = ObjectX { a: 10, b: 110 };
    let obj_y = ObjectY { m: 10, n: 110 };
    println!(
        "bridge object x param1>>>{},bridge param2>>>>{}",
        obj_x.set_b(),
        obj_x.get_a()
    );
    println!(
        "bridge  object y param1>>>>{},bridge param2>>>>{}",
        obj_y.set_b(),
        obj_y.get_a()
    );
    println!("-_______________________builder");
    let alice_builder = Box::new(AliceBuilder::new());
    let mut director = Director::new(alice_builder);
    let m = director.build();
    println!("builder->>>>{:?}", m);
    println!("-_______________________command");
    let mut r = Robot::new();
    let mut invoker = Invoker::new(&mut r);
    assert_eq!(
        *invoker.target(),
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy: 1,
        }
    );

    invoker.append_command(CommandTurnRight);
    invoker.append_command(CommandTurnLeft);
    invoker.append_command(CommandMoveForward);
    invoker.execute_all_commands();

    assert_eq!(
        *invoker.target(),
        Robot {
            x: 0,
            y: 1,
            dx: 0,
            dy: 1,
        }
    );

    invoker.undo();
    assert_eq!(
        *invoker.target(),
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy: 1,
        }
    );

    invoker.undo();
    assert_eq!(
        *invoker.target(),
        Robot {
            x: 0,
            y: 0,
            dx: 1,
            dy: 0,
        }
    );
}
