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
use crate::ApiGraphique::FormePrimitive::Ligne::Ligne; // Import de Ligne
use crate::ApiGraphique::FormePrimitive::Pixel::Pixel;
use crate::ApiGraphique::FormePrimitive::IForme::IForme; // Import du trait IForme
use std::io::Result;

fn main() -> Result<()> {
    // Crée le framebuffer
    let mut framebuffer = FrameBufferDevice::new(0)?;

    // Récupère les informations nécessaires
    let stride_pixels = framebuffer.stride_pixels;
    let height = framebuffer.height;
    let xoffset = framebuffer.xoffset;
    let yoffset = framebuffer.yoffset;

    // Affiche les informations récupérées
    println!(
        "Framebuffer info: width: {}, height: {}, stride_pixels: {}, xoffset: {}, yoffset: {}",
        framebuffer.width, framebuffer.height, framebuffer.stride_pixels, framebuffer.xoffset, framebuffer.yoffset
    );
    for pixel in framebuffer.iter_mut() {
        *pixel = 0x00000000; // Efface le framebuffer avec du noir
    }
    // Test de pixels simples pour diagnostiquer le problème
    let pixel = Pixel;
    pixel.dessiner(framebuffer.iter_mut(), stride_pixels, height, 0x00FF0000, 0, 0, xoffset, yoffset); // Rouge coin haut gauche
    pixel.dessiner(framebuffer.iter_mut(), stride_pixels, height, 0x0000FF00, 1, 0, xoffset, yoffset); // Vert à droite du rouge
    pixel.dessiner(framebuffer.iter_mut(), stride_pixels, height, 0x000000FF, 0, 1, xoffset, yoffset); // Bleu en dessous du rouge

    // Dessine 20 lignes empilées l'une sur l'autre
    for i in 0..200 {
        pixel.dessiner(framebuffer.iter_mut(), stride_pixels + 10, height, 0x00FF0000, 100, i, xoffset, yoffset);
    }

    // Dessine une ligne verticale avec Ligne
    Ligne {
        x1: 200,
        y1: 200,
        x2: 700,
        y2: 700,
    }
    .dessiner(framebuffer.iter_mut(), stride_pixels+10, height, 0x00FF0000);

    Ok(())
}