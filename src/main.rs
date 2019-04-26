#[macro_use]
extern crate chan;
extern crate rand;
use rand::Rng;

extern crate bear_lib_terminal;
use bear_lib_terminal::Color;
use bear_lib_terminal::geometry::Size;
use bear_lib_terminal::terminal::{self, config, Event, KeyCode};

use std::collections::HashMap;

use std::fs;

type Position = (i32, i32);
type TileMap = HashMap<Position, Tile>;

struct Tile
{
	x: i32,
	y: i32,
	alive: bool
}

fn update_tiles(tile_map: &mut TileMap)
{
	for x in 0..50
	{
		for y in 0..50
		{
			let tile: &mut Tile = tile_map.get_mut(&(x, y)).unwrap();

			let mut neighbors: Vec<i8> = vec![];
			let dirs: Vec<Position> = vec![
				(0,1),
				(0, -1),
				(-1, 0),
				(1, 0),
				(-1, -1),
				(1, -1),
				(1, 1),
				(-1, 1)
			];

			for dir in &dirs
			{
				match tile_map.get(&(x + dir.0, y + dir.1)) {
					Some(_) => neighbors.push(1),
					None => {}
				}
			}

			if true //tile.alive
			{

			}
			else
			{
				if neighbors.len()  == 3
				{
					//tile.alive = true;
				}
			}

		}
	}
}

fn ressurect(tile_map: &mut TileMap, pos: &Position)
{
	let tile: &mut Tile = tile_map.get_mut(pos).unwrap();

	tile.alive = true;
}

fn main() {
	terminal::open("Game Of Life", 60, 50);
	terminal::set(config::Window::empty());
  terminal::set(config::font::true_type(config::font::Origin::Root, "square.ttf", Size::new(0, 12)));
	terminal::set_foreground(Color::from_rgb(255, 0, 0));

	let mut current_map: TileMap = HashMap::new();

	let mut gen_number = 0;

	for x in 0..50
	{
		for y in 0..50
		{
			current_map.insert((x, y), Tile { x: x, y: y, alive: false });
		}
	}

	let living_cells: Vec<Position> = vec![
		(20, 20),
		(21, 20),
		(20, 21)
	];

	for pos in &living_cells
	{
		ressurect(&mut current_map, pos);
	}

	loop {
		let event = terminal::read_event();
		//terminal::print_xy(11, 9, &String::from("O"));

		let dirs: Vec<Position> = vec![
								(0,1),
								(0, -1),
								(-1, 0),
								(1, 0),
								(-1, -1),
								(1, -1),
								(1, 1),
								(-1, 1)
							];

		terminal::refresh();

		let tick = chan::tick_ms(500); // ten seconds

		chan_select! {
			tick.recv() => {
				for x in 0..50
		{
			for y in 0..50
			{
				let tile = current_map.get(&(x, y)).unwrap();

				if tile.alive
				{
					if false
					{
						let mut rng = rand::thread_rng();
						let r = rng.gen_range(0, 255);
						let b = rng.gen_range(0, 255);
						let g = rng.gen_range(0, 255);

						terminal::set_foreground(Color::from_rgb(r, b, g));
					}

					terminal::print_xy(x, y, &String::from("X"));
				}
				else {
					terminal::print_xy(x, y, &String::from(" "));		
				}
			}
		}
		terminal::set_foreground(Color::from_rgb(255, 255, 255));
		terminal::print_xy(50, 0, &String::from("          "));
		terminal::print_xy(50, 0, &String::from("Gen: "));
		terminal::print_xy(56, 0, &gen_number.to_string());
		terminal::set_foreground(Color::from_rgb(0, 255, 0));
		terminal::refresh();

				for x in 0..50
					{
						for y in 0..50
						{
							if !current_map.get(&(x, y)).unwrap().alive
							{
								let mut neighbors: Vec<i8> = vec![];
								let dirs: Vec<Position> = vec![
									(0,1),
									(0, -1),
									(-1, 0),
									(1, 0),
									(-1, -1),
									(1, -1),
									(1, 1),
									(-1, 1)
								];

								for dir in &dirs
								{
									match current_map.get(&(x + dir.0, y + dir.1)) {
										Some(t) => {
											if t.alive
											{
												neighbors.push(1);
											}
										},
										None => {}
									}
								}

								if x == 11 && y == 9
								{
									println!("{}, {}, {} neighbors, and is currently alive: {}", x, y, neighbors.len(), current_map.get_mut(&(x, y)).unwrap().alive);
								}

								if neighbors.len()  == 3
								{
									current_map.get_mut(&(x, y)).unwrap().alive = true;
								}
							}
						}
					}

					for x in 0..50
					{
						for y in 0..50
						{
							let mut neighbors: Vec<i8> = vec![];
							let dirs: Vec<Position> = vec![
								(0,1),
								(0, -1),
								(-1, 0),
								(1, 0),
								(-1, -1),
								(1, -1),
								(1, 1),
								(-1, 1)
							];

							for dir in &dirs
							{
								match current_map.get(&(x + dir.0, y + dir.1)) {
									Some(t) => {
										if t.alive
										{
											neighbors.push(1);
										}
									},
									None => {}
								}
							}

							if x == 11 && y == 9
							{
								println!("{}, {}, {} neighbors, and is currently alive: {}", x, y, neighbors.len(), current_map.get_mut(&(x, y)).unwrap().alive);
							}

							if neighbors.len() < 2
							{
								current_map.get_mut(&(x, y)).unwrap().alive = false;
							}
							else if neighbors.len() == 2 || neighbors.len() == 3
							{
								current_map.get_mut(&(x, y)).unwrap().alive = true;
							}
							else if neighbors.len() > 3
							{
								current_map.get_mut(&(x, y)).unwrap().alive = false;
							}
						}
					}

					match event {
						Some(e) => match e {
							Event::KeyPressed { key: KeyCode::Q, ctrl: _, shift: _ } => break,
							_ => {}
						}
						_ => (),
					}

					gen_number += 1;
					let mut data: String = String::from("");

					for x in 0..50
					{
						for y in 0..50
						{
							let tile = current_map.get(&(x, y)).unwrap();

							if tile.alive
							{
								data.push_str(&String::from("X"));
							}
							else
							{
								data.push_str(&String::from(" "));
							}
						}
					}

					let mut path: String = String::from("./gens/gen");
					path.push_str(&gen_number.to_string());
					path.push_str(&String::from(".txt"));

    			fs::write(path, data).expect("Unable to write file");
			}
		}
	}
	terminal::close();
}