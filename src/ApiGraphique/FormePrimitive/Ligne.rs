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
        epaisseur: usize, // Ajout de l'argument pour l'épaisseur
    ) {
        let dx = (self.x2 - self.x1) as f32;
        let dy = (self.y2 - self.y1) as f32;
        let steps = dx.abs().max(dy.abs()) as usize;

        if steps == 0 {
            for ex in 0..epaisseur {
                for ey in 0..epaisseur {
                    let xi = self.x1 + ex as i32;
                    let yi = self.y1 + ey as i32;
                    if xi >= 0 && xi < width as i32 && yi >= 0 && yi < height as i32 {
                        let idx = (yi as usize) * width + (xi as usize);
                        framebuffer[idx] = couleur;
                    }
                }
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

            for ex in 0..epaisseur {
                for ey in 0..epaisseur {
                    let xi_offset = xi + ex as i32;
                    let yi_offset = yi + ey as i32;

                    if xi_offset >= 0 && xi_offset < width as i32 && yi_offset >= 0 && yi_offset < height as i32 {
                        let idx = (yi_offset as usize) * width + (xi_offset as usize);
                        framebuffer[idx] = couleur;
                    }
                }
            }

            x += x_inc;
            y += y_inc;
        }
    }
}