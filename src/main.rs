use nannou::prelude::*;
use rand::Rng;
use std::time::{Duration, SystemTime};

static INTRO_TEXT: &str = "Test your reflexes

You'll be presented a blank screen that will change color 3 to 5 times. 

Every time the color changes, press space bar as quickly as you can.

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
            colors: vec![
                ALICEBLUE, ANTIQUEWHITE, AQUA, AQUAMARINE, AZURE, BEIGE, BISQUE, BLACK,
                BLANCHEDALMOND, BLUE, BLUEVIOLET, BROWN, BURLYWOOD, CADETBLUE, CHARTREUSE,
                CHOCOLATE, CORAL, CORNFLOWERBLUE, CORNSILK, CRIMSON, CYAN, DARKBLUE, DARKCYAN,
                DARKGOLDENROD, DARKGRAY, DARKGREEN, DARKGREY, DARKKHAKI, DARKMAGENTA,
                DARKOLIVEGREEN, DARKORANGE, DARKORCHID, DARKRED, DARKSALMON, DARKSEAGREEN,
                DARKSLATEBLUE, DARKSLATEGRAY, DARKSLATEGREY, DARKTURQUOISE, DARKVIOLET, DEEPPINK,
                DEEPSKYBLUE, DIMGRAY, DIMGREY, DODGERBLUE, FIREBRICK, FLORALWHITE, FORESTGREEN,
                FUCHSIA, GAINSBORO, GHOSTWHITE, GOLD, GOLDENROD, GRAY, GREEN, GREENYELLOW, GREY,
                HONEYDEW, HOTPINK, INDIANRED, INDIGO, IVORY, KHAKI, LAVENDER, LAVENDERBLUSH,
                LAWNGREEN, LEMONCHIFFON, LIGHTBLUE, LIGHTCORAL, LIGHTCYAN, LIGHTGOLDENRODYELLOW,
                LIGHTGRAY, LIGHTGREEN, LIGHTGREY, LIGHTPINK, LIGHTSALMON, LIGHTSEAGREEN,
                LIGHTSKYBLUE, LIGHTSLATEGRAY, LIGHTSLATEGREY, LIGHTSTEELBLUE, LIGHTYELLOW, LIME,
                LIMEGREEN, LINEN, MAGENTA, MAROON, MEDIUMAQUAMARINE, MEDIUMBLUE, MEDIUMORCHID,
                MEDIUMPURPLE, MEDIUMSEAGREEN, MEDIUMSLATEBLUE, MEDIUMSPRINGGREEN, MEDIUMTURQUOISE,
                MEDIUMVIOLETRED, MIDNIGHTBLUE, MINTCREAM, MISTYROSE, MOCCASIN, NAVAJOWHITE, NAVY,
                OLDLACE, OLIVE, OLIVEDRAB, ORANGE, ORANGERED, ORCHID, PALEGOLDENROD, PALEGREEN,
                PALETURQUOISE, PALEVIOLETRED, PAPAYAWHIP, PEACHPUFF, PERU, PINK, PLUM, POWDERBLUE,
                PURPLE, REBECCAPURPLE, RED, ROSYBROWN, ROYALBLUE, SADDLEBROWN, SALMON, SANDYBROWN,
                SEAGREEN, SEASHELL, SIENNA, SILVER, SKYBLUE, SLATEBLUE, SLATEGRAY, SLATEGREY,
                SNOW, SPRINGGREEN, STEELBLUE, TAN, TEAL, THISTLE, TOMATO, TURQUOISE, VIOLET,
                WHEAT, WHITE, WHITESMOKE, YELLOW, YELLOWGREEN,
            ],
            index: 7, // start with BLACK
        }
    }
    fn get(&self) -> &Rgb<u8> {
        &self.colors[self.index]
    }
    // Returning `self` allow chained-calls: `.next().next()...`
    fn next(&mut self) -> &mut Self {
		loop {
			let new_index = rand::thread_rng().gen_range(0..self.colors.len());
			// make sure not to repeat the color
			if new_index != self.index {
				self.index = new_index;
				break;
			}
		}
        self
    }
}

struct Model {
    state: State,
    color: Color,
    timer: SystemTime,
	rnd_duration: Duration,
    reflex_millis: Vec<u128>,
    iters: u32,
    i: u32,
}
impl Model {
    fn new() -> Self {
        Self {
            state: State::Intro,
            color: Color::new(),
            timer: SystemTime::now(),
			rnd_duration: Duration::ZERO,
            reflex_millis: vec![],
            iters: rand::thread_rng().gen_range(3..6),
            i: 0,
        }
    }
    fn reset(&mut self) {
        self.state = State::Intro;
        self.reflex_millis.clear();
        self.iters = rand::thread_rng().gen_range(3..6);
        self.i = 0;
    }
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(500, 300)
        .title("What's your reflexes?")
        .view(view)
        .event(event)
        .build()
        .unwrap();
    Model::new()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    match model.state {
        State::Intro => {}
        State::IntroFade => {
            model.state = State::Waiting;
        }
        State::Waiting => {
			if model.rnd_duration == Duration::ZERO {
				model.timer = SystemTime::now();
				model.rnd_duration = 
					Duration::from_millis(rand::thread_rng().gen_range(500..3000));
			} else {
				if SystemTime::now() > model.timer + model.rnd_duration {
					model.timer = SystemTime::now();
					model.rnd_duration = Duration::ZERO;
					model.color.next();
					model.state = State::Running;
				}
			}
        }
        State::Running => {}
        State::Stats => {}
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(Key::Space) => match model.state {
            State::Intro => {
                model.state = State::IntroFade;
            }
            State::Running => {
                let et = model.timer.elapsed().unwrap().as_millis();
                if et < 100 {
                    println!("Too quick - are you a cat?");
                    model.reset();
                    return;
                }
                model.reflex_millis.push(et);
                model.i += 1;
                if model.i < model.iters {
                    model.state = State::Waiting;
                } else {
                    println!("Reaction times: {:?}", model.reflex_millis);
                    model.state = State::Stats
                }
            }
            _ => (),
        },
        KeyPressed(Key::R) => model.reset(),
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();

    match model.state {
        State::Intro | State::Stats => {
            let text = match model.state {
                State::Intro => format!("{INTRO_TEXT}"),
                State::Stats => format!(
                    "Your mean reflex is {} ms\nYour fastest reaction was {} ms\n\nR to restart.",
                    model.reflex_millis.iter().sum::<u128>() / model.reflex_millis.len() as u128,
                    model.reflex_millis.iter().min().unwrap()
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
        }
        State::IntroFade | State::Waiting | State::Running => {
            frame.clear(*model.color.get());
        }
    }
}
