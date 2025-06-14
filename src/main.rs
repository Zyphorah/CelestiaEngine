mod ApiGraphique;
use crate::ApiGraphique::Buffer::FrammeBufferDevice::FrameBufferDevice;
use crate::ApiGraphique::FormePrimitive::Ligne::Ligne;
use crate::ApiGraphique::FormePrimitive::IForme::IForme; // Import du trait IForme

fn main() {
    // Crée le framebuffer
    let mut framebuffer = FrameBufferDevice::new(0).expect("Erreur lors de la création du framebuffer");

    // Affiche le stride (par exemple, framebuffer.stride_pixels)
    println!("Stride : {:?}", framebuffer.stride_pixels);

    // Emprunte les champs nécessaires avant l'appel à iter_mut
    let stride_pixels = framebuffer.stride_pixels;
    let height = framebuffer.height;

    Ligne {
         x1: 100,
         y1: 100,
         x2: 0,
         y2: 0
    }.dessiner(
        framebuffer.iter_mut(),
        stride_pixels,
        height,
        0xFF0000, // Couleur rouge
        1, // Épaisseur de 1 pixel
    );
}