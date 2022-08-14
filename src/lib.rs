extern "C" {
    pub fn fillPixel(x: i32, y: i32, r: u8, g: u8, b: u8, a: u8);
    pub fn fillCircle(x: i32, y: i32, radius: u32, r: u8, g: u8, b: u8, a: u8);
    pub fn rand() -> f64;
    pub fn alert(x: u32);
}

fn fill_circle(x: i32, y: i32, radius: u32, r: u8, g: u8, b: u8, a: u8) {
    unsafe {
        fillCircle(x, y, radius, r, g, b, a);
    }
}

fn fill_pixel(x: i32, y: i32, r: u8, g: u8, b: u8, a: u8) {
    unsafe {
        fillPixel(x, y, r, g, b, a);
    }
}

#[derive(Clone)]
pub struct Point {
    x: i32,
    y: i32,
}
pub struct Game {
    width: u32,
    height: u32,
    no_of_seeds: u32,
    seeds: Option<Vec<Point>>,
}

impl Game {
    fn reset(&mut self) {
        self.generate_seeds();
    }

    fn generate_seeds(&mut self) {
        let mut seeds = vec![Point { x: 0, y: 0 }; self.no_of_seeds as usize];
        for i in 0..self.no_of_seeds {
            let x: u32 = (unsafe { rand() } * 100000 as f64) as u32 % self.width;
            let y: u32 = (unsafe { rand() } * 100000 as f64) as u32 % self.height;
            seeds[i as usize] = Point {
                x: x as i32,
                y: y as i32,
            };
        }

        self.seeds = Some(seeds);
    }

    fn render(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut point = &self.seeds.as_ref().expect("test")[0];

                for i in 1..self.no_of_seeds {
                    let curr_point = &self.seeds.as_ref().expect("test")[i as usize];
                    if sqrt_distance(x as i32, y as i32, point.x, point.y)
                        > sqrt_distance(x as i32, y as i32, curr_point.x, curr_point.y)
                    {
                        point = curr_point;
                    }
                }
                fill_pixel(
                    x as i32,
                    y as i32,
                    (point.x % 255) as u8,
                    (point.y % 255) as u8,
                    ((point.y << 8) % 255) as u8,
                    255,
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn init(width: u32, height: u32, no_of_seeds: u32) -> *mut Game {
    let mut game: Game = Game {
        height: height,
        width: width,
        no_of_seeds: no_of_seeds,
        seeds: None,
    };

    game.generate_seeds();
    return Box::into_raw(Box::new(game));
}

#[no_mangle]
pub extern "C" fn reset(game: *mut Game) {
    let game = unsafe { &mut *game };
    game.reset();
}

fn sqrt_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    return (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);
}

#[no_mangle]
pub extern "C" fn draw(game: *mut Game) {
    let game = unsafe { &mut *game };
    game.render();
    for i in 0..game.no_of_seeds as usize {
        fill_circle(
            game.seeds.as_ref().expect("Failed").get(i).unwrap().x,
            game.seeds.as_ref().expect("fail").get(i).unwrap().y,
            5,
            250,
            0,
            0,
            0255,
        );
    }
}
