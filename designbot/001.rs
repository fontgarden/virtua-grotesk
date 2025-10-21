use designbot::prelude::*;

// Canvas dimensions and layout
const W: f64 = 2160.0;
const H: f64 = 2160.0;
const M: f64 = 120.0;  // Margin
const U: f64 = 64.0;   // Unit
const GRID_SIZE: usize = 10; // Number of grid cells (10x10)

fn main() {
    // Create canvas, renderer, and bg color
    let mut ctx = Canvas::new(W, H);
    let mut renderer = Renderer::new(W as u32, H as u32);
    ctx.background(Color::rgb(9, 9, 9));

    // Load the font
    renderer.load_font("fonts/VirtuaGrotesk-Regular.ttf").unwrap();

    // Calculate grid dimensions
    let grid_width = W - (M * 2.0);
    let grid_height = H - (M * 2.0);
    let cell_width = grid_width / GRID_SIZE as f64;
    let cell_height = grid_height / GRID_SIZE as f64;

    // Set colors
    let white = Color::white();
    let light_gray = Color::rgb(50, 50, 50);

    // Draw vertical lines
    ctx.stroke(white).stroke_width(2.0);
    for i in 0..=GRID_SIZE {
        let x = M + (i as f64 * cell_width);
        ctx.line(x, M, x, H - M);
    }

    // Draw horizontal lines
    for i in 0..=GRID_SIZE {
        let y = M + (i as f64 * cell_height);
        ctx.line(M, y, W - M, y);
    }

    // Set font
    ctx.font("Virtua Grotesk");
    ctx.fill(Color::rgb(255, 255, 255));

    // Title and version
    ctx.font_size(80.0);
    ctx.text("Virtua Grotesk Regular v0.1", M, H - M - 80.0);

    // Character sets
    let mut y_pos = H - M - (U * 4.0);

    // Uppercase letters - line 1
    ctx.font_size(256.0);
    ctx.text("ABCDEFGHIJ", M, y_pos);
    y_pos -= U * 5.0;

    // Uppercase letters - line 2
    ctx.text("KLMNOPQR", M, y_pos);
    y_pos -= U * 5.0;

    // Uppercase letters - line 3
    ctx.text("STUVWXYZ", M, y_pos);
    y_pos -= U * 5.5;

    // Numbers
    ctx.text("0123456789", M, y_pos);
    y_pos -= U * 5.5;

    // Lowercase letters - line 1
    ctx.text("abcdefghij", M, y_pos);
    y_pos -= U * 5.0;

    // Lowercase letters - line 2
    ctx.text("klmnopqr", M, y_pos);
    y_pos -= U * 5.0;

    // Lowercase letters - line 3
    ctx.text("stuvwxyz", M, y_pos);

    // Footer metadata
    ctx.font_size(80.0);
    ctx.text("Font.Garden", M, M + 240.0);

    // Arabic sample text
    ctx.text("أشهد يا إلهي بأنك خلقتني", M, M + 140.0);

    // License info
    ctx.font_size(60.0);
    ctx.text("Licensed under the SIL Open Font License v1.1", M, M + 40.0);

    // Render to PNG (output path handled by designbot CLI)
    renderer.render_to_png(&ctx, "output.png").unwrap();
}
