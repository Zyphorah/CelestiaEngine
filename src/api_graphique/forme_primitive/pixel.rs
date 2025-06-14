pub struct Pixel;

impl Pixel {
    pub fn dessiner(
        &self,
        framebuffer: &mut [u32],
        stride_pixels: usize,
        height: usize,
        couleur: u32,
        x: i32,
        y: i32,
        xoffset: usize,
        yoffset: usize,
    ) {
        if x >= 0 && x < stride_pixels as i32 && y >= 0 && y < height as i32 {
            let idx = ((y + yoffset as i32) as usize) * stride_pixels + ((x + xoffset as i32) as usize);
            framebuffer[idx] = couleur;
        }
    }
}