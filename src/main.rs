mod framebuffer;
mod line;
mod bmp;

use framebuffer::Framebuffer;
use line::Line;

fn main() -> std::io::Result<()> {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, 0x000000); // Fondo negro

    // Limpiar el framebuffer con un fondo negro
    framebuffer.clear();

    // Definir los puntos del Polígono 1
    let polygon1_points = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360), 
        (250, 380), (220, 385), (205, 410), (193, 383)
    ];

    // Dibujar el Polígono 1 (amarillo con orilla blanca)
    framebuffer.draw_polygon(
        &polygon1_points, 
        0xFFFFFF, // Color de la orilla (blanco)
        0xFFFF00  // Color de relleno (amarillo)
    );

    // Guardar el framebuffer como un archivo BMP
    framebuffer.render_buffer("out.bmp")?;

    println!("Framebuffer rendered to out.bmp");

    // Mostrar la imagen en una ventana
    framebuffer.display();

    Ok(())
}
