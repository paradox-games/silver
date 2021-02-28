//!  Silver3d web render engine:
//! camera module with Camera struct
//! and implementations

use nalgebra::{Isometry3, Perspective3, Point3, Vector3};
use std::f32::consts::PI;


pub struct Camera {
    projection: Perspective3<f32>,
    horizontal_radians: f32,
    vertical_radians: f32,
    orbit_radius: f32,
    fovy: f32,
}

impl Camera {
    pub fn init(fovy: f32) -> Camera {
        return Camera {
            projection: Perspective3::new(fovy, 1.0, 0.1, 50.0),
            left_right_radians: 45.0f32.to_radians(),
            up_down_radians: 80.0f32.to_radians(),
            orbit_radius: 15.,
            fovy: fovy,
        };
    }

    pub fn view(&self) -> [f32; 16] {
        let eye = self.get_eye_pos();
        let target = Point3::new(0.0, 0.0, 0.0);

        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());
        let view = view.to_homogeneous();

        let mut view_array = [0.; 16];
        view_array.copy_from_slice(view.as_slice());

        return view_array;
    }

    pub fn view_flipped(&self, axis: char) -> [f32; 16] {
        let eye = self.get_eye_pos();

        match axis {
            'x' => { eye.x = -eye.x; },
            'y' => { eye.y = -eye.y; },
            'z' => { eye.z = -eye.z; }
        }

        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());
        let view = view.to_homogeneous();

        let mut view_array = [0.; 16];
        view_array.copy_from_slice(view.as_slice());

        return view_array;
    }

    pub fn get_eye_pos(&self) -> Point3<f32> {
        let yaw = self.horizontal_radians;
        let pitch = self.vertical_radians;

        let eye_x = self.orbit_radius * yaw.sin() * pitch.cos();
        let eye_y = self.orbit_radius * pitch.sin();
        let eye_z = self.orbit_radius * yaw.cos() * pitch.cos();

        return Point3::new(eye_x, eye_y, eye_z);
    }

    pub fn set_fovy(&self, fovy) {
        self.fovy = fovy;
    }

    pub fn projection(&self) -> [f32; 16] {
        let mut perspective_array = [0.; 16];
        perspective_array.copy_from_slice(self.projection.as_matrix().as_slice());

        return perspective_array;
    }

    pub fn orbit_horizontal(&mut self, delta: f32) {
        self.horizontal_radians += delta;
    }

    pub fn orbit_vertical(&mut self, delta: f32) {
        self.vertical_radians += delta;
        // 0.1 <= radians <= PI / 2.1

        if self.vertical_radians - (PI / 2.1) > 0. {
            self.vertical_radians = PI / 2.1;
        }

        if self.vertical_radians - 0.1 < 0. {
            self.vertical_radians = 0.1;
        }
    }

    pub fn zoom(&mut self, zoom: f32) {
        self.orbit_radius += zoom;
        // max orbit radius is 30 and min is 5

        if self.orbit_radius > 30. {
            self.orbit_radius = 30.;
        } else if self.orbit_radius < 5. {
            self.orbit_radius = 5.;
        }
    }

    pub fn recalc_projection(&self) {
        self.projection = Perspective3::new(fovy, 1.0, 0.1, 50.0);
    }
}