// Code minimal pour ouvrir une fenêtre X11 en Rust
// Direct Rendering Manager
// libdrm

use std::ptr;
use x11::xlib;

fn main() {
   
    unsafe {
        let display = xlib::XOpenDisplay(ptr::null());
        if display.is_null() {
            eprintln!("Impossible d'ouvrir le display X11");
            return;
        }

        let screen = xlib::XDefaultScreen(display);
        let win = xlib::XCreateSimpleWindow(
            display,
            xlib::XRootWindow(display, screen),
            100, 100, 400, 300, 1,
            xlib::XBlackPixel(display, screen), // couleur du bord
            xlib::XBlackPixel(display, screen), // couleur de fond (noir)
        );

        // Demander à recevoir les événements d'exposition et de touche
        xlib::XSelectInput(
            display,
            win,
            xlib::ExposureMask | xlib::KeyPressMask,
        );

        xlib::XMapWindow(display, win);

        // Créer un GC
        let gc = xlib::XCreateGC(display, win, 0, std::ptr::null_mut());

        // Définir la couleur rouge
        let mut color: xlib::XColor = std::mem::zeroed();
        let colormap = xlib::XDefaultColormap(display, screen);
        let color_name = std::ffi::CString::new("red").unwrap();
        if xlib::XAllocNamedColor(display, colormap, color_name.as_ptr(), &mut color, &mut color) != 0 {
            xlib::XSetForeground(display, gc, color.pixel);
        }

        // Boucle d'événements : dessiner la ligne à chaque Expose
        let mut event: xlib::XEvent = std::mem::zeroed();
        loop {
            xlib::XNextEvent(display, &mut event);
            match event.get_type() {
                t if t == xlib::Expose => {
                    // Redessiner la ligne rouge horizontale au centre
                    xlib::XDrawLine(display, win, gc, 50, 150, 350, 150);
                }
                t if t == xlib::KeyPress => {
                    break;
                }
                _ => {}
            }
        }

        // Libérer le GC
        xlib::XFreeGC(display, gc);

        xlib::XDestroyWindow(display, win);
        xlib::XCloseDisplay(display);
    }
}
