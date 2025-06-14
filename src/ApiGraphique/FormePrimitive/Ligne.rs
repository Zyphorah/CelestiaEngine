use crate::ApiGraphique::FormePrimitive::IForme::IForme;

pub struct Ligne {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

// DDA Line generation
impl IForme for Ligne {
    fn dessiner(
        &self,
        framebuffer: &mut [u32],
        width: usize,
        height: usize,
        couleur: u32,
    ) {
        let dx = (self.x2 - self.x1) as f32;
        let dy = (self.y2 - self.y1) as f32;
        let steps = dx.abs().max(dy.abs()) as usize;

        if steps == 0 {
            if self.x1 >= 0 && self.x1 < width as i32 && self.y1 >= 0 && self.y1 < height as i32 {
                let idx = (self.y1 as usize) * width + (self.x1 as usize);
                framebuffer[idx] = couleur;
            }
            return;
        }

        let x_inc = dx / steps as f32;
        let y_inc = dy / steps as f32;
        let mut x = self.x1 as f32;
        let mut y = self.y1 as f32;

        for _ in 0..=steps {
            let xi = x.round() as i32;
            let yi = y.round() as i32;

            if xi >= 0 && xi < width as i32 && yi >= 0 && yi < height as i32 {
                let idx = (yi as usize) * width + (xi as usize);
                framebuffer[idx] = couleur;
            }

            x += x_inc;
            y += y_inc;
        }
    }
}