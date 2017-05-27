use glium::glutin;
use graphics;

const UP_COLOR_ACTIVE: [f32; 4] = [1., 0., 0., 1.];
const DOWN_COLOR_ACTIVE: [f32; 4] = [1., 0., 0., 1.];

const UP_COLOR: [f32; 4] = [1., 1., 0., 1.];
const DOWN_COLOR: [f32; 4] = [1., 1., 0., 1.];

const PROGRESS: [f32; 4] = [0., 1., 0., 1.];
const PROGRESS_BACKGROUND: [f32; 4] = [1., 0., 0., 1.];

const TIME: f64 = 10.;

const END_TIMER: f64 = 5.;
const END_BLINKING_TIMER: f64 = 0.1;


enum State {
    Running,
    End {
        timer: f64,
        blinking_timer: f64,
        blinking: bool
    }
}

pub struct App {
    up_touch_id: Option<u64>,
    down_touch_id: Option<u64>,
    up_score: f32,
    down_score: f32,
    state: State,
}

impl App {
    pub fn new() -> App {
        App {
            up_touch_id: None,
            down_touch_id: None,
            up_score: 0.,
            down_score: 0.,
            state: State::Running,
        }
    }
    pub fn update(&mut self, dt: f64) {
        self.state = match self.state {
            State::Running => {
                if self.up_touch_id.is_none() == self.down_touch_id.is_none() {
                    self.down_score += (dt/TIME) as f32;
                } else {
                    self.up_score += (dt/TIME) as f32;
                }

                if self.down_score >= 1. || self.up_score >= 1. {
                    State::End {
                        timer: END_TIMER,
                        blinking_timer: END_BLINKING_TIMER,
                        blinking: false,
                    }
                } else {
                    State::Running
                }
            }
            State::End { mut timer, mut blinking_timer, mut blinking } => {
                timer -= dt;
                blinking_timer -= dt;
                if blinking_timer <= 0. {
                    blinking_timer = END_BLINKING_TIMER;
                    blinking = !blinking;
                }

                if timer <= 0. {
                    self.up_score = 0.;
                    self.down_score = 0.;
                    State::Running
                } else {
                    State::End {
                        timer: timer,
                        blinking_timer: blinking_timer,
                        blinking: blinking,
                    }
                }
            }
        };
    }
    pub fn draw(&mut self, frame: &mut graphics::Frame) {
        let up_color = if self.up_touch_id.is_some() {
            UP_COLOR_ACTIVE
        } else {
            UP_COLOR
        };

        frame.draw_rectangle(0., 2./3., 2., 2./3., up_color);

        let down_color = if self.down_touch_id.is_some() {
            DOWN_COLOR_ACTIVE
        } else {
            DOWN_COLOR
        };

        frame.draw_rectangle(0., -2./3., 2., 2./3., down_color);

        frame.draw_rectangle(0., 0., 2., 2./3., PROGRESS_BACKGROUND);

        match self.state {
            State::Running | State::End { timer: _, blinking_timer: _, blinking: true} => {
                frame.draw_rectangle(0., 1./6., self.up_score * 2., 1./3., PROGRESS);
                frame.draw_rectangle(0., -1./6., self.down_score * 2., 1./3., PROGRESS);
            },
            State::End { timer: _, blinking_timer: _, blinking: false } => {
                if self.up_score > self.down_score {
                    frame.draw_rectangle(0., -1./6., self.down_score * 2., 1./3., PROGRESS);
                } else {
                    frame.draw_rectangle(0., 1./6., self.up_score * 2., 1./3., PROGRESS);
                }
            }
        }
    }
    pub fn touch(&mut self, touch: glutin::Touch) {
        use glutin::TouchPhase::*;
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
