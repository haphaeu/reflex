use nannou::prelude::*;
use std::time::{SystemTime, Duration};
use std::thread::sleep;


static INTRO_TEXT: &str = "Test your reflexes

You'll be presented a blank screen that will change color 
a couple of times. 

Every time the color changes, press space abr as quickly as you can.

Press space bar to start.
";

#[derive(PartialEq)]
enum State {
	Intro,
	IntroFade,
	Waiting,
	Running,
	Stats,
}

struct Color {
	colors: Vec<Rgb<u8>>,
	index: usize,
}

impl Color {
	fn new() -> Self {
		Self {
			colors: vec![BLACK, WHITE, CORNFLOWERBLUE],
			index: 0,
		}
	}
	fn get(&self) -> &Rgb<u8> {
        &self.colors[self.index]
    }
	// Returning `self` allow chained-calls: `.next().next()...`
    fn next(&mut self) -> &mut Self {
		self.index = if self.index == self.colors.len() - 1 { 0 } else { self.index + 1 };
		self
    }
}

struct Model {
	state: State,
	color: Color,
	timer: SystemTime,
	millis: Vec<u128>,
	iters: u32,
	i: u32,
}
impl Model {
	fn new() -> Self {
		Self {
			state: State::Intro,
			color: Color::new(),
			timer: SystemTime::now(),
			millis: vec![],
			iters: 5,
			i: 0,
		}
	}
}

fn main() {
    nannou::app(model)
		.update(update)
		.loop_mode(LoopMode::Wait)
		.run();
}

fn model(app: &App) -> Model {
    // Create a new window! Store the ID so we can refer to it later.
    let _window = app
        .new_window()
        .size(400, 200)
        .title("What's your reflexes?")
        .view(view) 
        .event(event) 
        .build()
        .unwrap();
    Model::new()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
	print!("update called");
	match model.state {
		State::Intro => {
			println!(" - intro ");
		},
		State::IntroFade => {
			model.state = State::Waiting;
		},
		State::Waiting => {
			println!(" - waiting");
			sleep(Duration::new(2, 0));
			model.color.next();
			model.state = State::Running;
			model.timer = SystemTime::now();
		},
		State::Running => {
			println!(" - running");
		},
		State::Stats => {
			println!(" - stats");
		},
	}
}

// Handle events related to the window and update the model if necessary
fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
		KeyPressed(Key::Space) => {
			print!("Pressed space key - ");
			match model.state {
				State::Intro => {
					model.state = State::IntroFade;
					println!("changed to IntroFade state");
				},
				State::Running => {
					let et = model.timer.elapsed().unwrap().as_millis();
					println!("Reaction time {et} ms");
					model.millis.push(et);
					model.i += 1;
					if model.i < model.iters {
						model.state = State::Waiting;
					} else {
						model.state = State::Stats
					}
				},
				_ => (),
			}
		},
		_ => (),
	}
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
	frame.clear(BLACK);
	
	let draw = app.draw();
	
	match model.state {
		State::Intro | State::Stats => {
			let text = match model.state {
					State::Intro => format!("{INTRO_TEXT}"),
					State::Stats => format!(
						"Your mean reflex is {} ms\nYour fastest reaction was {} ms",
						model.millis.iter().sum::<u128>() / model.millis.len() as u128,
						model.millis.iter().min().unwrap()
					),
					_ => format!(""),
			};
			let winp = app.window_rect().pad(20.0);
			let text_area = geom::Rect::from_wh(winp.wh()).top_left_of(winp);
			draw.text(&text)
				.xy(text_area.xy())
				.wh(text_area.wh())
				.align_text_bottom()
				.left_justify()
				.color(RED);
			draw.to_frame(app, &frame).unwrap();
		},
		State::IntroFade | State::Waiting | State::Running => {
			println!("view - frame cleared");
			frame.clear(*model.color.get());
		},
	}
}