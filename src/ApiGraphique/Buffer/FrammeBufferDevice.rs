use std::fs::OpenOptions;
use std::io; 
use std::os::unix::io::AsRawFd; 
use std::ptr; 

use libc::{mmap, munmap, off_t, MAP_SHARED, PROT_READ, PROT_WRITE}; 


pub struct FrameBufferDevice 
{
    width: usize, 
    height: usize,
    stride: usize, 
    fb_offset: off_t, 
    fb_ptr: *mut libc::c_void, 
    fb_size: usize,           
}

impl FrameBufferDevice 
{
    pub fn new(width: usize, height: usize, fb_offset: off_t) -> io::Result<Self>
    {
        let stride = width * 4; 
        let fb_size = stride * height; 

        // Ouvre le framebuffer classique (nécessite /dev/fb0)
        let fb = OpenOptions::new()
            .read(true) 
            .write(true) 
            .open("/dev/fb0")?; 
        let fd = fb.as_raw_fd(); // Récupère le descripteur de fichier

        // Mappe le framebuffer en mémoire
        let fb_ptr = unsafe {
            mmap(
                ptr::null_mut(), // Laisse le système choisir l'adresse
                fb_size, // Taille à mapper
                PROT_READ | PROT_WRITE, // Droits de lecture et écriture
                MAP_SHARED, // Mapping partagé
                fd, // Descripteur de fichier
                fb_offset, // Offset dans le fichier
            )
        };
        if fb_ptr == libc::MAP_FAILED {
            eprintln!("Erreur mmap"); // Affiche une erreur si le mapping échoue
            return Err(io::Error::last_os_error()); // Retourne l'erreur
        }

        Ok(FrameBufferDevice {
            width,
            height,
            stride,
            fb_offset,
            fb_ptr,
            fb_size,
        })
    }

    /// Permet d'itérer mutablement sur les pixels du framebuffer (XRGB8888)
    pub fn iter_mut(&mut self) -> &mut [u32] {
        unsafe {
            std::slice::from_raw_parts_mut(self.fb_ptr as *mut u32, self.width * self.height)
        }
    }
}

impl Drop for FrameBufferDevice 
{
    fn drop(&mut self) 
    {
        unsafe
        {
            munmap(self.fb_ptr, self.fb_size); 
        }
    }
}