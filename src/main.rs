use std::{ops::*};

use bevy::{prelude::*, window::close_on_esc};

const GRAVITATIONAL_CONSTANT: f64 = 1.0e-1f64;
fn main() {
    println!("Hello, world!");
    App::new().add_plugins(DefaultPlugins)
    .add_system(close_on_esc)
    .add_startup_system(setup)
    .add_system(step)
    .run();
}
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let Bodies = vec!(
        Body {
             pos: Vec2, 
             vel: , 
             mass:  }
    )

    commands.spawn(SpriteBundle {
        sprite: Sprite { color: (Color::rgb(1.0, 1.0, 1.0)), ..default()},
        transform: Transform { translation: (Vec3::new(0.0, 0.0, 0.0)), rotation: (Quat::from_axis_angle(Vec3::X, 0.0)), scale: (Vec3::splat(5.0)) },
        ..default()
    }).insert(Body::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite { color: (Color::rgb(1.0, 1.0, 1.0)), ..default()},
        transform: Transform { translation: (Vec3::new( body2_pos[0], body2_pos[1], 0.0)), rotation: (Quat::from_axis_angle(Vec3::X, 0.0)), scale: (Vec3::splat(5.0)) },
        ..default()
    }).insert(Body{
        pos: Vec2xf64 { x: body2_pos[0] as f64, y: body2_pos[1] as f64 },
        vel: Vec2xf64 { x:0.2f64, y: -1.3f64 },
        mass: 100.0f64,
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite { color: (Color::rgb(1.0, 1.0, 1.0)), ..default()},
        transform: Transform { translation: (Vec3::new(body3_pos[0], body3_pos[1], 0.0)), rotation: (Quat::from_axis_angle(Vec3::X, 0.0)), scale: (Vec3::splat(5.0)) },
        ..default()
    }).insert(Body{
        pos: Vec2xf64 { x: body3_pos[0] as f64, y: body3_pos[1] as f64},
        vel: Vec2xf64 { x:0.2f64, y: -0.3f64 },
        mass: 200.0f64,
    });

}

fn step(mut Bodies: Query<(&mut Body, &mut Transform)>) {
    let mut iter = Bodies.iter_combinations_mut();
    if let Some([(mut body1, mut transform1), (mut body2, mut transform2)]) = iter.fetch_next() {
        let force = ((body1.mass * body2.mass) / body1.pos.distance_squared(body2.pos)) * GRAVITATIONAL_CONSTANT;
        let mut direction = (body1.pos - body2.pos).normalize()*force;
        let mut mass = body2.mass.clone();

        body2.vel += direction/ mass; //force divide by mass
       
        mass = body1.mass.clone();
        body1.vel += direction.negative()/mass;        
    }
    for (mut body, mut transform) in Bodies.iter_mut() {
        body.pos.x += body.vel.x;
        body.pos.y += body.vel.y;
        transform.translation.x = body.pos.x as f32;
        transform.translation.y = body.pos.y as f32;
    }
}








#[derive(Clone, Copy, PartialEq)]
struct Vec2xf64 {
    pub x: f64,
    pub y: f64
}
#[allow(dead_code)]
impl Vec2xf64 {
    fn new(first: f64, second: f64) -> Self {
        Vec2xf64 { x: first, y: second }
    }
    fn default() -> Self {
        Vec2xf64 { x: 0.0f64, y: 0.0f64 }
    }
    pub fn distance_squared(self, rhs: Self) -> f64 {
        (self - rhs).length_squared()
    }
    fn splat(x: f64) -> Self {
        Vec2xf64 { x: (x), y: (x) }
    }
    pub fn distance(self, rhs: Self) -> f64 {
        (self - rhs).length()
    }
    pub fn length_recip(self) -> f64 {
        self.length().recip()
    }
    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }
    
    pub fn dot(self, rhs: Self) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }
    pub fn normalize(self) -> Self {
        #[allow(clippy::let_and_return)]
        let normalized = self.mul(self.length_recip());
        normalized
    }
    pub fn negative(self) -> Self{
        Vec2xf64 { x: -self.x, y: (-self.y) }
    }
}   

impl Default for Body {
    fn default() -> Self {
        Body { pos: Vec2xf64::default(), vel: Vec2xf64 {
            x: 0.0f64,
            y: 0.3f64,
        }, mass: 600.0}
    }
}

#[derive(Component)]
struct Body {
    pub pos: Vec2xf64,
    pub vel: Vec2xf64,
    pub mass: f64
}
impl Body {
    fn distance(&self, rhs: &Body) -> f64 {
        self.pos.distance(rhs.pos)
    }
}
impl Add for Vec2xf64 {
    type Output = Vec2xf64;

    fn add(self, other: Vec2xf64) -> Vec2xf64 {
        Vec2xf64 { x: (self.x + other.x), y: (self.y + other.y) }
    }
}
impl Mul<f64> for Vec2xf64 {
    type Output =  Self;
    #[inline]
    fn mul(self, rhs: f64) -> Vec2xf64 {
        Vec2xf64 {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}
impl Mul<Vec2xf64> for Vec2xf64 {
    type Output =  Self;
    #[inline]
    fn mul(self, rhs: Vec2xf64) -> Vec2xf64 {
        Vec2xf64 {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}
impl Sub<Vec2xf64> for Vec2xf64 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}
impl Div<f64> for Vec2xf64 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}
impl AddAssign for Vec2xf64 {

    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}
impl AddAssign<Vec2xf64> for Vec3 {

    fn add_assign(&mut self, rhs: Vec2xf64) {
        *self = Self {
            x: self.x + (rhs.x as f32),
            y: self.y + (rhs.y as f32),
            z: self.z
        }
    } 
}
