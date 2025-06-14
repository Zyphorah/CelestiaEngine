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
use crate::ApiGraphique::FormePrimitive::IForme::IForme;
use crate::ApiGraphique::FormePrimitive::Ligne::Ligne;

use std::io::Result;

fn main() -> Result<()> {
    let mut framebuffer = FrameBufferDevice::new(1366, 768, 0)?;

    // Remplit tout l'écran en blanc (XRGB8888)
    for pixel in framebuffer.iter_mut() {
        *pixel = 0x00FFFFFF;
    }

    // Dessine une ligne rouge du coin haut gauche au coin bas droit
    let ligne = Ligne { x1: 0, y1: 0, x2: 600, y2: 600 };
    ligne.dessiner(framebuffer.iter_mut(), 1366, 768, 0x00FF0000);

    Ok(())
}