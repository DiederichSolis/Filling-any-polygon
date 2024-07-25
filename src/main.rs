mod framebuffer;
mod bmp;
mod color;

use framebuffer::Framebuffer;
use color::Color;

fn main() {
    let width = 1000;
    let height = 1000;
    let background_color = Color::from_hex(0x000000).to_hex();
    
    let mut fb = Framebuffer::new(width, height, background_color);
    
    // Dibujar Polígono 1
    let poly1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), 
        (233, 330), (230, 360), (250, 380), (220, 385), 
        (205, 410), (193, 383)
    ];


    // poligono 2
let poly2 = [
    (321, 335), (288, 286), (339, 251), (374, 302)
];


let poligono_3 = vec![
        (377, 249), (411, 197), (436, 249),
    ];

    //poligono 4

    let poligono_4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180),
    ];

    
   
// Dibujar  polígonos

    fb.draw_polygon(&poly1, Color::from_hex(0xFFFFFF).to_hex(), Color::from_hex(0xFFFF00).to_hex());
    fb.draw_polygon(&poly2, Color::from_hex(0xFFFFFF).to_hex(), Color::from_hex(0x0000FF).to_hex());
    fb.draw_polygon(&poligono_3, Color::from_hex(0xFFFFFF).to_hex(), Color::from_hex(0xFF0000).to_hex());

    fb.draw_polygon(&poligono_4, Color::from_hex(0xFFFFFF).to_hex(), Color::from_hex(0x00FF00).to_hex());


    fb.render_buffer("out.bmp").unwrap();
    fb.display();
    

}
