pub struct Raytracer {
    image: Vec<u8>,

    camera_pos: [f32; 3],
}

impl Raytracer {
    pub fn new() -> Self {
        Raytracer {
            image: Vec::new(),
            camera_pos: [0.0, 0.0, -5.0],
        }
    }
}

impl raytracer::Renderer for Raytracer {
    fn render(&mut self, width: u32, height: u32) -> &Vec<u8> {
        self.image.clear();

        let l = 1.0;

        for y in 0..height {
            for x in 0..width {
                let ax = self.camera_pos[0];
                let ay = self.camera_pos[1];
                let az = self.camera_pos[2];

                let bx = (x as f32 - width as f32 / 2.0) / height as f32;
                let by = (y as f32 - height as f32 / 2.0) / height as f32;
                let bz = l;

                let x0 = 0.0;
                let y0 = 0.0;
                let z0 = 5.0;
                let r = 2.0;

                let a = bx * bx + by * by + bz * bz;
                let b = 2.0 * (ax * bx + ay * by + az * bz);
                let c = ax * ax + ay * ay + az * az - r * r;
                let hit = b * b - 4.0 * a * c;

                let mut drawed = false;
                if hit >= 0.0 {
                    let root = hit.sqrt();
                    let hit1 = (-b - root) / (2.0 * a);
                    let hit2 = (-b + root) / (2.0 * a);
                    if hit1 >= 0.0 || hit2 >= 0.0 {
                        self.image.push(255);
                        self.image.push(0);
                        self.image.push(0);
                        self.image.push(255);
                        drawed = true;
                    }
                } 
                if !drawed {
                    self.image.push(255);
                    self.image.push(255);
                    self.image.push(0);
                    self.image.push(255);
                }
            }
        }
        &self.image
    }

    fn imgui_render(&mut self, ui: &imgui::Ui, width: u32, height: u32) -> bool {
        let mut render = false;
        let window = imgui::Window::new("Render");
        window
            .size([300.0, 100.0], imgui::Condition::FirstUseEver)
            .position([20.0, 20.0], imgui::Condition::FirstUseEver)
            .build(ui, || {
                ui.text(format!("Resolution: {} x {}", width, height));
                render = ui.button("Render");
                ui.input_float3("Camera Position", &mut self.camera_pos)
                    .build();
            });

        render
    }
}
