// Direct Rendering Manager
// libdrm
// https://www.kernel.org/doc/html/v4.11/gpu/drm-internals.html
// Fonction pour dessiner une ligne avec l'algorithme de Bresenham
// Affiche un point rouge au centre de l'écran avec le framebuffer (/dev/fb0)
// KMS
// OpenCL / pour le traitement sur le GPU
// apt-get install ocl-icd-opencl-dev

mod api_graphique; // Renommé en snake_case
use crate::api_graphique::buffer::frame_buffer_device::FrameBufferDevice; // Renommé en snake_case
use crate::api_graphique::forme_primitive::ligne::Ligne; // Renommé en snake_case
use crate::api_graphique::forme_primitive::pixel::Pixel; // Renommé en snake_case
use crate::api_graphique::forme_primitive::iforme::IForme; // Renommé en snake_case
use std::f32::consts::PI;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), std::io::Error> {
    // Crée le framebuffer
    let mut framebuffer = FrameBufferDevice::new(0)?;

    // Récupère les informations nécessaires
    let stride_pixels = framebuffer.stride_pixels;
    let height = framebuffer.height;
    let _xoffset = framebuffer.xoffset; // Préfixé avec un underscore
    let _yoffset = framebuffer.yoffset; // Préfixé avec un underscore

    // Efface le framebuffer avec du noir
    for pixel in framebuffer.iter_mut() {
        *pixel = 0x00000000;
    }

    // Animation de rotation
    let _pixel = Pixel; // Préfixé avec un underscore
    let mut angle: f32 = 0.0; // Ajout de l'annotation de type explicite
    let center_x = 150.0;
    let center_y = 150.0;
    let length = 150.0; // Longueur de la ligne

    // Variables pour stocker les coordonnées précédentes
    let mut prev_x1 = center_x - (length / 2.0) * (angle).cos();
    let mut prev_y1 = center_y - (length / 2.0) * (angle).sin();
    let mut prev_x2 = center_x + (length / 2.0) * (angle).cos();
    let mut prev_y2 = center_y + (length / 2.0) * (angle).sin();

    loop {
        // Efface uniquement la ligne précédente en dessinant avec une couleur de fond (noir)
        Ligne {
            x1: prev_x1 as i32,
            y1: prev_y1 as i32,
            x2: prev_x2 as i32,
            y2: prev_y2 as i32,
        }
        .dessiner(framebuffer.iter_mut(), stride_pixels, height, 0x00000000, 5); // Efface avec du noir

        // Calcule les nouvelles coordonnées avec la matrice de rotation
        let x1 = center_x - (length / 2.0) * (angle).cos();
        let y1 = center_y - (length / 2.0) * (angle).sin();
        let x2 = center_x + (length / 2.0) * (angle).cos();
        let y2 = center_y + (length / 2.0) * (angle).sin();

        // Dessine la nouvelle ligne avec rotation
        Ligne {
            x1: x1 as i32,
            y1: y1 as i32,
            x2: x2 as i32,
            y2: y2 as i32,
        }
        .dessiner(framebuffer.iter_mut(), stride_pixels, height, 0x00FFFFFF, 1);

        // Met à jour les coordonnées précédentes
        prev_x1 = x1;
        prev_y1 = y1;
        prev_x2 = x2;
        prev_y2 = y2;

        // Incrémente l'angle pour la prochaine rotation
        angle += 0.01;
        if angle > 2.0 * PI {
            angle -= 2.0 * PI;
        }

        // Pause pour ralentir l'animation
        sleep(Duration::from_millis(5));
    }
}