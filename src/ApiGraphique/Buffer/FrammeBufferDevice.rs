use libc::{ioctl, mmap, munmap, off_t, c_void, MAP_SHARED, PROT_READ, PROT_WRITE};
use std::fs::{OpenOptions, File};
use std::io;
use std::mem;
use std::os::unix::io::AsRawFd;
use std::ptr;

const FBIOGET_VSCREENINFO: u64 = 0x4600;
const FBIOGET_FSCREENINFO: u64 = 0x4602;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct FbVarScreeninfo {
    xres: u32,
    yres: u32,
    xres_virtual: u32,
    yres_virtual: u32,
    xoffset: u32,
    yoffset: u32,
    bits_per_pixel: u32,
    _pad: [u8; 200],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct FbFixScreeninfo {
    _id: [u8; 16],
    _smem_start: u64,
    _smem_len: u32,
    _type: u32,
    _type_aux: u32,
    _visual: u32,
    _xpanstep: u16,
    _ypanstep: u16,
    _ywrapstep: u16,
    line_length: u32,
    _pad: [u8; 152],
}

pub struct FrameBufferDevice {
    pub width: usize,
    pub height: usize,
    pub stride_pixels: usize,
    pub xoffset: usize,
    pub yoffset: usize,
    fb_ptr: *mut c_void,
    fb_size: usize,
    _fb_file: File,
}

impl FrameBufferDevice {
    /// Crée une nouvelle instance de FrameBufferDevice
    pub fn new(fb_offset: off_t) -> io::Result<Self> {
        let fb_file = OpenOptions::new().read(true).write(true).open("/dev/fb0")?;
        let fd = fb_file.as_raw_fd();

        // Récupère les informations d'écran
        let vinfo = Self::get_var_screen_info(fd)?;
        let finfo = Self::get_fix_screen_info(fd)?;

        // Calcule les dimensions et le stride
        let stride_pixels = finfo.line_length as usize / 4;
        let width = vinfo.xres as usize;
        let height = vinfo.yres as usize;
        let xoffset = vinfo.xoffset as usize;
        let yoffset = vinfo.yoffset as usize;

        // Calcule la taille totale du framebuffer
        let fb_size = finfo.line_length as usize * height;

        // Mappe le framebuffer en mémoire
        let fb_ptr = Self::map_framebuffer(fd, fb_size, fb_offset)?;

        Ok(FrameBufferDevice {
            width,
            height,
            stride_pixels,
            xoffset,
            yoffset,
            fb_ptr,
            fb_size,
            _fb_file: fb_file,
        })
    }

    /// Récupère les informations variables d'écran (FbVarScreeninfo)
    fn get_var_screen_info(fd: i32) -> io::Result<FbVarScreeninfo> {
        let mut vinfo: FbVarScreeninfo = unsafe { mem::zeroed() };
        let ret = unsafe { ioctl(fd, FBIOGET_VSCREENINFO, &mut vinfo as *mut _ as *mut c_void) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(vinfo)
    }

    /// Récupère les informations fixes d'écran (FbFixScreeninfo)
    fn get_fix_screen_info(fd: i32) -> io::Result<FbFixScreeninfo> {
        let mut finfo: FbFixScreeninfo = unsafe { mem::zeroed() };
        let ret = unsafe { ioctl(fd, FBIOGET_FSCREENINFO, &mut finfo as *mut _ as *mut c_void) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(finfo)
    }

    /// Mappe le framebuffer en mémoire
    fn map_framebuffer(fd: i32, fb_size: usize, fb_offset: off_t) -> io::Result<*mut c_void> {
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
            return Err(io::Error::last_os_error());
        }
        Ok(fb_ptr)
    }

    /// Renvoie un slice mutable des pixels visibles
    pub fn iter_mut(&mut self) -> &mut [u32] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.fb_ptr as *mut u32,
                self.stride_pixels * self.height,
            )
        }
    }

    /// Retourne un pointeur mutable vers la mémoire mappée
    pub fn fb_ptr_mut(&mut self) -> *mut c_void {
        self.fb_ptr
    }
}

impl Drop for FrameBufferDevice {
    fn drop(&mut self) {
        // Libère la mémoire mappée
        if !self.fb_ptr.is_null() {
            unsafe {
                if munmap(self.fb_ptr, self.fb_size) != 0 {
                    eprintln!("Erreur lors de la libération de la mémoire mappée");
                }
            }
        }
    }
}
