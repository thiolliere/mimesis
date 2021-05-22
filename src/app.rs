const BUTTON_Y: f32 = 0.5;
const BUTTON_RADIUS: f32 = 0.2;

const COLOR_ACTIVE: [f32; 4] = [1., 1., 0., 1.];
const COLOR: [f32; 4] = [0.4, 0.4, 0., 1.];

const PROGRESS: [f32; 4] = [1., 0., 0., 1.];
const PROGRESS_BACKGROUND: [f32; 4] = [0.4, 0., 0., 1.];

const BLACK : [f32; 4] = [0., 0., 0., 1.];

const GAME_TIME: f64 = 5.;

const END_TIMER: f64 = 3.;
const END_BLINKING_TIMER: f64 = 0.1;

pub struct App {
    up_touch_id: Option<u64>,
    down_touch_id: Option<u64>,
    state: State,
}

pub enum State {
    Start,
    Game {
        up_score: f32,
        down_score: f32,
    },
    End {
        up_winner: bool,
        timer: f64,
        blinking_timer: f64,
        blink: bool,
    },
}

impl App {
    pub fn new() -> App {
        js! {
            tgl.draw_start = function(up, down) {
                this.context.clearRect(0, 0, this.canvas.width, this.canvas.height);
                if up {
                } else {
                }
                if down {
                } else {
                }
            };
        }
        App {
            up_touch_id: None,
            down_touch_id: None,
            state: State::Start,
        }
    }
    pub fn update(&mut self, dt: f64) {
        self.state = match self.state {
            State::Start => {
                if self.up_touch_id.is_none() == self.down_touch_id.is_none() {
                    State::Game {
                        up_score: 0.0,
                        down_score: 0.0,
                    }
                } else {
                    State::Start
                }
            },
            State::Game { mut up_score, mut down_score } => {
                if self.up_touch_id.is_none() == self.down_touch_id.is_none() {
                    down_score += (dt/GAME_TIME) as f32;
                } else {
                    up_score += (dt/GAME_TIME) as f32;
                }

                if down_score >= 1. || up_score >= 1. {
                    State::End {
                        up_winner: up_score >= 1.,
                        blink: false,
                        blinking_timer: END_BLINKING_TIMER,
                        timer: END_TIMER,
                    }
                } else {
                    State::Game {
                        up_score,
                        down_score,
                    }
                }
            }
            State::End { mut timer, mut blinking_timer, mut blink, up_winner } => {
                timer -= dt;
                blinking_timer -= dt;
                if blinking_timer <= 0. {
                    blinking_timer = END_BLINKING_TIMER;
                    blink = !blink;
                }

                if timer <= 0. {
                    State::Start
                } else {
                    State::End {
                        timer,
                        blinking_timer,
                        blink,
                        up_winner,
                    }
                }
            }
        };
    }
    pub fn draw(&self) {
        let up_color = if self.up_touch_id.is_some() {
            COLOR_ACTIVE
        } else {
            COLOR
        };

        let down_color = if self.down_touch_id.is_some() {
            COLOR_ACTIVE
        } else {
            COLOR
        };
    }
    pub fn touch(&mut self, touch: ::winit::Touch) {
        use ::winit::TouchPhase::*;
        match touch.phase {
            Started => {
                if touch.location.1 >= 1./3. {
                    self.up_touch_id = Some(touch.id);
                } else if touch.location.1 <= -1./3. {
                    self.down_touch_id = Some(touch.id);
                }
            },
            Ended | Cancelled => {
                if self.up_touch_id == Some(touch.id) {
                    self.up_touch_id = None;
                }
                if self.down_touch_id == Some(touch.id) {
                    self.down_touch_id = None;
                }
            },
            _ => (),
        }
    }
}
