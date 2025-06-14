pub trait IForme {
    fn dessiner(&self, framebuffer: &mut [u32], width: usize, height: usize, couleur: u32);
}
