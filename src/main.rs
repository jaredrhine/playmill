use std::{thread, time};

use std::io;
use std::io::{Stdout, Write};

use termion;
use termion::color;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

// use termion::terminal_size;
// println!("Size is {:?}", terminal_size().unwrap());

type MillId = usize;
type Coord = (isize, isize);

struct IdGenerator {
    id: MillId
}

impl IdGenerator {
    fn new() -> Self {
        Self {
            id: 0
        }
    }

    fn gimme(&mut self) -> MillId {
        self.id = self.id + 1;
        return self.id;
    }
}

struct TerminalDisplay {
    view_center_x: isize,
    view_center_y: isize,
    view_width: usize,
    view_height: usize,
    log_lines_displayed: usize,
}

impl TerminalDisplay {
    fn new() -> Self {
        Self {
            view_center_x: 10,
            view_center_y: 5,
            view_width: 68,
            view_height: 30,
            log_lines_displayed: 12,
        }
    }

    fn dump_world(&self, world: &World, stdout: &mut RawTerminal<Stdout>) -> () {
        let p = &world.physics;

        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Hide,
            termion::cursor::Goto(1,1)
        )
        .unwrap();
        stdout.lock().flush().unwrap();

        // Render the header
        println!("{}Playmill{} - {}A Simple Factory Simulator in Rust{}\r",
            color::Fg(color::LightRed),
            color::Fg(color::Reset),
            color::Fg(color::Cyan),
            color::Fg(color::Reset)
        );
        println!("\r");

        // Render the info readout
        print!("[tick {:0>6} {}ms]", p.tick_count, p.tick_post_sleep_ms);
        print!(" [position {},{} | view {}x{}]", self.view_center_x, self.view_center_y, self.view_width, self.view_height);
        println!(" [world {}..{}/{}..{}]\r", p.min_x, p.max_x, p.min_y, p.max_y);

        // Render the controls
        println!("Controls: space to pause, q to exit\r");
        println!("\r");

        // Render an outer box
        // TODO: replace whenever
        for _ in 1..(self.view_width+2) { print!("-"); }
        println!("\r");
        for _ in 1..(self.view_height) {
            print!("|");
            for _ in 1..(self.view_width) { print!(" "); }
            println!("|\r");
        }
        for _ in 1..(self.view_width+2) { print!("-"); }
        println!("\r");

        // Calculate box offsets for later
        let _viewport_offset_x = 1;
        let viewport_offset_y = 5 + 2 + 1;

        // TODO: Incorporate viewport and offsets into calculations

        // Render each of the buildings
        for building in world.buildings.iter() {
            let (x, y) = building.coord;
            let token = building.display_character();
            write!(stdout, "{}{}", termion::cursor::Goto(x as u16, y as u16), token).unwrap();
        }

        // Render the building debug bar
        let debug_row = (viewport_offset_y + self.view_height) as u16;
        write!(stdout, "{}", termion::cursor::Goto(1,debug_row)).unwrap();
        for line in world.log.iter().rev().take(self.log_lines_displayed) {
            println!("{}\r", line);
        }

        stdout.lock().flush().unwrap();
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum BuildingLabel {
    Conveyor,
    Creator,
}

type ResourceCount = usize;

#[derive(Debug)]
struct Building {
    id: MillId,
    label: BuildingLabel,
    coord: Coord,
    contains_count: ResourceCount,
    contains_max: ResourceCount,
}

// For now, buildings have one x/y coordinate and take up all of that cell and only that cell

fn building_character_default(label: BuildingLabel) -> char {
    return match label {
        BuildingLabel::Conveyor => ':',
        BuildingLabel::Creator => '-',
    }
}

impl Building {
    fn new(id: MillId, label: BuildingLabel, coord: Coord, count: ResourceCount) -> Self {
        Self {
            id: id,
            label: label,
            coord: coord,
            contains_count: count,
            contains_max: 8,
        }
    }

    fn display_character(&self) -> char {
        if self.label == BuildingLabel::Creator {
            return match self.contains_count {
                n @ 0 ..= 8 => (48 + n as u8) as char,
                _ => '9'
            }
        }
        if self.contains_count > 0 {
            return '@';
        }
        return building_character_default(self.label);
    }
}

struct Physics {
    tick_count: u128,
    ticks_max: usize,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    tick_post_sleep_ms: u64,
}

impl Physics {
    fn new() -> Self {
        Self {
            tick_count: 0,
            ticks_max: 2000,
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

    // TODO: incorporate before/after snapshot
}

struct World {
    idgen: IdGenerator,
    physics: Physics,
    buildings: Vec<Building>,
    log: Vec<String>,
}

impl World {
    fn new() -> Self {
        let mut me = Self {
            idgen: IdGenerator::new(),
            physics: Physics::new(),
            buildings: Vec::new(),
            log: Vec::new(),
        };
        me.add_log_line("World initialized");
        return me;
    }

    fn add_new_building(&mut self, label: BuildingLabel, coord: Coord, count: ResourceCount) -> MillId {
        let bid = self.idgen.gimme();
        let log_line = format!("Added building #{} {:?} \"{}\" {:?} w/{} item{}", bid, label, building_character_default(label), coord, count, if count == 1 { ' ' } else { 's' });
        let building = Building::new(bid, label, coord, count);
        self.buildings.push(building);
        self.add_log_line(&log_line);
        return bid;
    }

    fn tick_world(&mut self) -> () {
        self.physics.tick_physics();
        if self.physics.tick_count % 50 == 0 {
            let log_line = format!("Reached tick {}", self.physics.tick_count);
            self.add_log_line(&log_line);
        }

        // TODO: move or modularize
        for i in 0..self.buildings.len() {
            let building = &self.buildings[i];
            let label = building.label;
            if label == BuildingLabel::Creator {
                if self.physics.tick_count % 25 == 0 {
                    let building = &mut self.buildings[i];
                    if building.contains_count < building.contains_max {
                        building.contains_count += 1;
                        let log_line = format!("Incremented resource in building #{} to {}", building.id, building.contains_count);
                        self.add_log_line(&log_line);
                    } else {
                        let log_line = format!("Skipped adding resource in building #{} to {} since already at max {}", building.id, building.contains_count, building.contains_max);
                        self.add_log_line(&log_line);
                    }
                }
            }
        }
    }

    fn tick_sleep(&mut self) -> () {
        self.physics.tick_sleep();
    }

    fn add_log_line(&mut self, line: &str) -> () {
        self.log.push(line.to_string());
    }
}

fn scenario_4x_conveyor(world: &mut World) -> () {
    let _bid1 = world.add_new_building(BuildingLabel::Conveyor, (18,12), 0);
    let _bid2 = world.add_new_building(BuildingLabel::Conveyor, (19,12), 1);
    // TODO: connect bid1 to bid2
    world.add_new_building(BuildingLabel::Conveyor, (20,12), 0);
    world.add_new_building(BuildingLabel::Conveyor, (21,12), 0);
    world.add_new_building(BuildingLabel::Conveyor, (21,11), 0);
    world.add_new_building(BuildingLabel::Conveyor, (21,10), 0);
    world.add_new_building(BuildingLabel::Creator, (25,20), 0);
    world.add_new_building(BuildingLabel::Conveyor, (25,21), 0);
    world.add_new_building(BuildingLabel::Conveyor, (25,22), 0);
    world.add_new_building(BuildingLabel::Conveyor, (25,23), 0);
}

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();

    let mut display = TerminalDisplay::new();
    let mut world = World::new();

    scenario_4x_conveyor(&mut world);

    // for each loop:
    // tick: evolve the world
    // clear the screen
    // display the current world
    // sleep before next tick
    // exit automatically after world.ticks_max ticks
    // exit if user presses the "q" key
    // pause/unpause if user presses the space key

    let mut loops = 0;
    let mut running = true;
    loop {
        if running {
            loops = loops + 1;
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

        display.dump_world(&world, &mut stdout);
        display = display;

        world.tick_sleep();

        if loops >= world.physics.ticks_max {
            break;
        }
    }
}
