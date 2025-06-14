use crate::ApiGraphique::FormePrimitive::IForme::IForme;

pub struct Ligne {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl IForme for Ligne {
    fn dessiner(&self, framebuffer: &mut [u32], width: usize, height: usize, couleur: u32) {
        let dx = (self.x2 - self.x1) as f32;
        let dy = (self.y2 - self.y1) as f32;
        let steps = dx.abs().max(dy.abs()) as usize;
        let x_inc = dx / steps as f32;
        let y_inc = dy / steps as f32;
        let mut x = self.x1 as f32;
        let mut y = self.y1 as f32;

        for _ in 0..=steps {
            let xi = x.round() as i32;
            let yi = y.round() as i32;
            if xi >= 0 && (xi as usize) < width && yi >= 0 && (yi as usize) < height {
                framebuffer[(yi as usize) * width + (xi as usize)] = couleur;
            }
            x += x_inc;
            y += y_inc;
        }
    }
}
