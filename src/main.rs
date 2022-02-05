use std::thread;
use std::time;

use std::io;
use std::io::Write;

use termion;
use termion::color;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

type MillId = u32;

struct IdGenerator {
    id: MillId
}

impl IdGenerator {
    fn new() -> Self {
        Self {
            id: 1
        }
    }

    fn gimme(&mut self) -> MillId {
        self.id = self.id + 1;
        return self.id;
    }
}

// use termion::raw::RawTerminal;
// use termion::terminal_size;
// fn new(stdout: &RawTerminal<Stdout>) -> Self {
// println!("Size is {:?}", terminal_size().unwrap());

struct TerminalDisplay {
    view_center_x: i32,
    view_center_y: i32,
    view_width: i32,
    view_height: i32,
}

impl TerminalDisplay {
    fn new() -> Self {
        Self {
            view_center_x: 10,
            view_center_y: 5,
            view_width: 40,
            view_height: 20,
        }
        // TODO: add support for current center of viewport
    }

    fn dump_world(&self, world: &World) -> () {
        let p = &world.physics;
        println!("{}Playmill{} - {}A Simple Factory Simulator in Rust{}\r",
            color::Fg(color::LightRed),
            color::Fg(color::Reset),
            color::Fg(color::Cyan),
            color::Fg(color::Reset)
        );
        println!("\r");
        print!("[tick {:0>6} {}ms]", p.tick_count, p.tick_post_sleep_ms);
        print!(" [position {},{} | view {}x{}]", self.view_center_x, self.view_center_y, self.view_width, self.view_height);
        println!(" [world {}..{}/{}..{}]\r", p.min_x, p.max_x, p.min_y, p.max_y);
        println!("Controls: space to pause, q to exit\r");
        println!("\r");

        for building in world.buildings.iter() {
            println!("Building id={:?} display_character={:?}\r", building.id, building.display_character);
        }
    }
}

struct Building {
    id: MillId,
    display_character: char,
}

impl Building {
    fn new(id: MillId) -> Self {
        Self {
            id: id,
            display_character: '@',
        }
    }
}

struct Physics {
    tick_count: u128,
    ticks_max: u32,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    tick_post_sleep_ms: u64,
}

impl Physics {
    fn new() -> Self {
        Self {
            tick_count: 0,
            ticks_max: 10000,
            tick_post_sleep_ms: 150,
            min_x: -100,
            max_x: 100,
            min_y: -100,
            max_y: 100
        }
    }

    fn tick_physics(&mut self) -> () {
        self.tick_count = self.tick_count + 1;
    }

    fn tick_sleep(&self) -> () {
        thread::sleep(time::Duration::from_millis(self.tick_post_sleep_ms));
    }

}

struct World {
    idgen: IdGenerator,
    physics: Physics,
    buildings: Vec<Building>
}

impl World {
    fn new() -> Self {
        Self {
            idgen: IdGenerator::new(),
            physics: Physics::new(),
            buildings: Vec::new()
        }
    }

    fn add_building(&mut self, building: Building) {
        self.buildings.push(building);
    }

    fn new_building(&mut self) -> Building {
        let bid = self.idgen.gimme();
        return Building::new(bid);
    }

    fn tick_world(&mut self) -> () {
        self.physics.tick_physics();
    }

    fn tick_sleep(&mut self) -> () {
        self.physics.tick_sleep();
    }
}

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();

    let mut display = TerminalDisplay::new();
    let mut world = World::new();
    let building = world.new_building();
    world.add_building(building);
    let building = Building::new(99);
    world.add_building(building);

    // for each loop:
    // clear the screen
    // exit automatically after world.ticks_max ticks
    // exit if user presses the "q" key
    // pause/unpause if user presses the space key
    // display the current world

    let mut loops = 0;
    let mut running = true;
    loop {
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Hide,
            termion::cursor::Goto(1,1)
        )
        .unwrap();
        stdout.lock().flush().unwrap();

        display.dump_world(&world);
        if running {
            world.tick_world();
        }
        let input = stdin.next();

        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Char('q') => break,
                termion::event::Key::Char(' ') => {
                    running = !running;
                },
                _ => {
                }
            }
        }
        world.tick_sleep();
        display = display;
        if running {
            loops = loops + 1;
        }
        if loops >= world.physics.ticks_max {
            break;
        }
    }
}
