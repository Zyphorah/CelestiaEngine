// Direct Rendering Manager
// libdrm
// https://www.kernel.org/doc/html/v4.11/gpu/drm-internals.html
// Fonction pour dessiner une ligne avec l'algorithme de Bresenham
// Affiche un point rouge au centre de l'écran avec le framebuffer (/dev/fb0)
// KMS
// OpenCL / pour le traitement sur le GPU
// apt-get install ocl-icd-opencl-dev

mod ApiGraphique;
use crate::ApiGraphique::Buffer::FrammeBufferDevice::FrameBufferDevice;

use std::io::Result;

fn main() -> Result<()> {
    let mut framebuffer = FrameBufferDevice::new(1366, 768, 0)?; // Crée un nouveau framebuffer avec une largeur de 800 pixels et une hauteur de 600 pixels

    
    // Remplit tout l'écran en blanc (XRGB8888)
    for pixel in framebuffer.iter_mut() {
        *pixel = 0x00FFFFFF;
    }
    
    Ok(())
}