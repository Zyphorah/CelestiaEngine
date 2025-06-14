use libc::{ioctl, mmap, off_t, c_void, MAP_SHARED, PROT_READ, PROT_WRITE};
use std::fs::OpenOptions;
use std::io;
use std::mem;
use std::os::unix::io::AsRawFd;
use std::ptr;

// Définition de la structure FbVarScreeninfo au niveau du module
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FbVarScreeninfo {
    pub xres: u32,
    pub yres: u32,
    pub xres_virtual: u32,
    pub yres_virtual: u32,
    pub xoffset: u32,
    pub yoffset: u32,
    pub bits_per_pixel: u32,
    _padding: [u8; 200], // pour compléter la structure
}

const FBIOGET_VSCREENINFO: u64 = 0x4600;

pub struct FrameBufferDevice {
    pub width: usize,         // largeur visible en pixels
    pub height: usize,        // hauteur visible en pixels
    pub stride_pixels: usize, // stride réel en pixels
    pub xoffset: usize,       // offset horizontal
    pub yoffset: usize,       // offset vertical
    fb_offset: off_t,
    fb_ptr: *mut libc::c_void,
    fb_size: usize,
}

impl FrameBufferDevice {
    pub fn new(fb_offset: off_t) -> io::Result<Self> {
        // Ouvre /dev/fb0 pour récupérer le descripteur de fichier
        let fb = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/fb0")?;
        let fd = fb.as_raw_fd();

        // Récupère les informations d'écran
        let (stride_pixels, width, height, xoffset, yoffset) = FrameBufferDevice::get_screen_info(fd);

        // Calcule la taille totale du framebuffer en mémoire
        let fb_size = stride_pixels * 4 * height; // stride_pixels * 4 (octets par pixel) * hauteur
        println!("Framebuffer size (bytes): {}", fb_size); // Affiche la taille mappée

        // Mappe le framebuffer en mémoire
        let fb_ptr = unsafe {
            mmap(
                ptr::null_mut(),
                fb_size,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                fd,
                fb_offset,
            )
        };

        if fb_ptr == libc::MAP_FAILED {
            eprintln!("Erreur mmap");
            return Err(io::Error::last_os_error());
        }

        Ok(FrameBufferDevice {
            width,
            height,
            stride_pixels,
            xoffset,
            yoffset,
            fb_offset,
            fb_ptr,
            fb_size,
        })
    }

    /// Renvoie un slice mutable des pixels visibles (en tenant compte du stride réel)
    pub fn iter_mut(&mut self) -> &mut [u32] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.fb_ptr as *mut u32,
                self.stride_pixels * self.height,
            )
        }
    }

    /// Récupère les informations d'écran via FBIOGET_VSCREENINFO
    pub fn get_screen_info(fd: i32) -> (usize, usize, usize, usize, usize) {
        let mut vinfo: FbVarScreeninfo = unsafe { mem::zeroed() };

        let ret = unsafe {
            ioctl(
                fd,
                FBIOGET_VSCREENINFO,
                &mut vinfo as *mut FbVarScreeninfo as *mut c_void,
            )
        };

        if ret < 0 {
            panic!("Erreur ioctl pour FBIOGET_VSCREENINFO");
        }

        let bpp = vinfo.bits_per_pixel as usize;
        let stride = vinfo.xres_virtual as usize * (bpp / 8);

        (
            stride / 4,               // stride en pixels
            vinfo.xres as usize,      // largeur visible
            vinfo.yres as usize,      // hauteur visible
            vinfo.xoffset as usize,   // offset horizontal
            vinfo.yoffset as usize,   // offset vertical
        )
    }
}



