use bevy::{prelude::*, window::close_on_esc};


const GRAVITATIONAL_CONSTANT: f64 = 1e4;
fn main() {
    println!("Hello, world!");
    App::new().add_plugins(DefaultPlugins)
    .add_system(close_on_esc)
    .add_startup_system(setup)
    .run();
}
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite { color: (Color::rgb(1.0, 1.0, 1.0)), ..default()},
        transform: Transform { translation: (Vec3::new(0.0, 0.0, 0.0)), rotation: (Quat::from_axis_angle(Vec3::X, 0.0)), scale: (Vec3::splat(5.0)) },
        ..default()
    }).insert(Body::default());

}
struct Vec2xf64 {
    pub x: f64,
    pub y: f64
}
impl Vec2xf64 {
    fn new(first: f64, second: f64) -> Self {
        Vec2xf64 { x: first, y: second }
    }
    fn default() -> Self {
        Vec2xf64 { x: 0.0f64, y: 0.0f64 }
    }
    fn distance(&self, rhs: Vec2xf64) -> f64 {
        f64::powi(self.x - rhs.x, 2) + f64::powi(self.y - rhs.y, 2)
    }
}

impl Default for Body {
    fn default() -> Self {
        Body { pos: Vec2xf64::default(), vel: Vec2xf64::default(), mass: 0.0}
    }
}

#[derive(Component)]
struct Body {
    pub pos: Vec2xf64,
    pub vel: Vec2xf64,
    pub mass: f64
}
impl Body {
    fn distance(&self, rhs: Body) -> f64 {
        self.pos.distance(rhs.pos)
    }
}

fn step(mut Bodies: Query<(&mut Body, &mut Transform)>) {
    let mut iter = Bodies.iter_combinations_mut();
    if let Some([(body1, transform1), (body2, transform2)]) = iter.fetch_next() {
        let force = (body1.mass * body2.mass) / f64::powi(body1.distance(body2), 2);
        body1.vel
    }
}
