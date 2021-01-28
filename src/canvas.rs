// Here, we define our own Canvas, Colour Picker and associated functions.

use image::{RgbImage, Rgb, DynamicImage};

use cursive::direction::Direction;
use cursive::event::{Event, EventResult, MouseButton, MouseEvent};
use cursive::theme::{Color, ColorStyle};
use cursive::Printer;
use cursive::Vec2;


// The big weak point of my implementation is the following.
// These should be properties of a View, but I don't know how to access them from inside the event handler of different objects.
static mut BRUSHCOLOR: Color = Color::Rgb(0,0,0);
static mut BRUSHBACKCOLOR: Color = Color::Rgb(0,0,0);


fn hsv_to_rbg(h: u16, s:f32, v:f32) -> (u8, u8, u8) {
    // How about some Colour Space conversion (HSV -> RGB)? I honestly don't understand this that well.
    // It would be smart to return a color, but I only figured out to return the enum Color itself. We know it's Color::Rgb though....

    let c = v * s;
    let m = v - c;
    let x = c * ((1 - ((h as i32 / 60) % 2) - 1).abs() as f32);

    // Maybe a match statement would have been better?
    if h < 60 {
        (((c + m) * 255.) as u8, ((x + m) * 255.) as u8,  (m * 255.) as u8)
    }

    else if h < 120 {
        (((x + m) * 255.) as u8, ((c + m) * 255.) as u8, (m * 255.) as u8)
    }

    else if h < 180 {
        ((m * 255.) as u8, ((c + m) * 255.) as u8, ((x + m) * 255.) as u8)
    }

    else if h < 240 {
        ((m * 255.) as u8, ((x + m) * 255.) as u8, ((c + m) * 255.) as u8)
    }

    else if h < 300 {
        (((x + m) * 255.) as u8, (m * 255.) as u8, ((c + m) * 255.) as u8)
    }

    else {
        (((c + m) * 255.) as u8, (m * 255.) as u8, ((x + m) * 255.) as u8)
    }
}

#[derive(Clone, Copy)]
pub struct Options {
    pub size: Vec2,
}

pub struct Board {
    pub size: Vec2,
    pub cells: Vec<Cell>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    pub color: Color,
    pub backcolor: Color,
    pub symbol: char
}

#[derive(Clone, Copy)]
pub enum Tool {
    Draw,
    Pipette,
    // Fill,  TODO

}

impl Board {
    pub fn new(size: Vec2) -> Self {
        let n_cells = size.x * size.y;

        let board = Board {
            size: size,
            cells: vec![Cell {color: Color::Rgb(255,255,255), backcolor: Color::Rgb(255,255,255), symbol: ' ' }; n_cells],
        };

        board
    }

    pub fn cell_id(&self, pos: Vec2) -> Option<usize> {
        if pos < self.size {
            Some(pos.x + pos.y * self.size.x)
        } else {
            None
        }
    }

    /*
    This is still from the the days where this was the minesweeper demo. I figure it might be useful to implement something like a fill tool.
    pub fn neighbours(&self, pos: Vec2) -> Vec<Vec2> {
        let pos_min = pos.saturating_sub((1, 1));
        let pos_max = (pos + (2, 2)).or_min(self.size);
        (pos_min.x..pos_max.x)        //img = img.thumbnail(200, 100);
            .flat_map(|x| (pos_min.y..pos_max.y).map(move |y| Vec2::new(x, y)))
            .filter(|&p| p != pos)
            .collect()
    }
    */
}

pub struct CanvasView {
    board: Board,

    overlay: Vec<Cell>,
    history_o: Vec<Vec<Cell>>,
    history_b: Vec<Board>,
    focused: Option<Vec2>,

    brushsymbol: char,
    tool: Tool
    //In an ideal world, this is where brush(back)color would be :^)
}

impl CanvasView {
    pub fn new(size: Vec2) -> Self {
        let overlay = vec![Cell {color: Color::Rgb(255, 255, 255), backcolor: Color::Rgb(255, 255, 255), symbol: ' '} ; size.x * size.y];
        let history_o = vec![Cell {color: Color::Rgb(255, 255, 255), backcolor: Color::Rgb(255, 255, 255), symbol: ' '} ; size.x * size.y];
        let board = Board::new(size);
        let history_b = Board::new(size);
        CanvasView {
            board,
            overlay,
            history_o: vec![history_o], // The 250 means that we're keeping history for 250 steps.
            history_b: vec![history_b],
            focused: None,
            brushsymbol: ' ',
            tool: Tool::Draw
        }
    }

    pub fn changecolor(&mut self, pos: Vec2, color: Color, backcolor: Color) {
        if let Some(i) = self.board.cell_id(pos) {
            let symbol: char = self.brushsymbol;
            let new_cell = Cell {color, backcolor, symbol};

            self.overlay[i] = new_cell;
        }
    }

    pub fn changebrushsymbol(&mut self, symbol: char) {
        self.brushsymbol = symbol;
    }

    pub fn clear(&mut self) {
        self.overlay = vec![Cell {color: Color::Rgb(255, 255, 255), backcolor: Color::Rgb(255, 255, 255), symbol: ' '} ; self.board.size.x * self.board.size.y]
    }

    pub fn fill_all(&mut self) {
        // Applies the current brush to all cells of the canvas.
        self.add_history();
        unsafe {
            self.overlay  = vec![Cell {color: BRUSHCOLOR, backcolor: BRUSHBACKCOLOR, symbol: self.brushsymbol}; self.board.size.x * self.board.size.y]
        }
    }

    pub fn fliph(&mut self) {
        // Currently only flips the background colours.

        let rgbimg = self.toimage();
        let overlay_old = self.get_overlay();
        let image_flipped = DynamicImage::ImageRgb8(rgbimg).fliph();
        self.fromimage(image_flipped, Some(overlay_old));
    }
    pub fn flipv(&mut self) {
        // Currently only flips the background colours.

        let rgbimg = self.toimage();
        let overlay_old = self.get_overlay();
        let image_flipped = DynamicImage::ImageRgb8(rgbimg).flipv();
        self.fromimage(image_flipped, Some(overlay_old));
    }

    /*
    This version flips everything, not only the background, horizontally and vertically.

    pub fn flip(&mut self) {
        // Doesn't seem to work? I'm confused.
        let mut overlay_flipped = self.overlay.to_vec();

        overlay_flipped.reverse();
        self.overlay = overlay_flipped;
    }
    */

    pub fn grayscale(&mut self) {
        let rgbimg = self.toimage();
        let overlay_old = self.get_overlay();
        let greyimage = DynamicImage::ImageRgb8(rgbimg).grayscale();
        self.fromimage(greyimage, Some(overlay_old));
    }

    pub fn blur(&mut self, sigma: f32) {
        // Blurs background colours.

        let rgbimg = self.toimage();
        let overlay_old = self.get_overlay();
        let img_blurred = DynamicImage::ImageRgb8(rgbimg).blur(sigma);
        self.fromimage(img_blurred, Some(overlay_old));
    }

    pub fn rotate90(&mut self) {
        // TODO: Only handles the background, crashes if applied twice - probably due to fromimage()

        let rgbimg = self.toimage();
        let img_rot = DynamicImage::ImageRgb8(rgbimg).rotate90();
        self.fromimage(img_rot, None);
    }

    pub fn brighten(&mut self, value: i32) {
        //  Brightens or darkens the canvas by the given value.

        let rgbimg = self.toimage();
        let overlay_old = self.get_overlay();
        let img_bright = DynamicImage::ImageRgb8(rgbimg).brighten(value);
        self.fromimage(img_bright, Some(overlay_old));
    }

    pub fn adjust_contrast(&mut self, c: f32) {
        // Adjusts the background contrast by c.

        let rgbimg = self.toimage();
        let overlay_old = self.get_overlay();
        let img_contrast = DynamicImage::ImageRgb8(rgbimg).adjust_contrast(c);
        self.fromimage(img_contrast, Some(overlay_old));
    }

    pub fn huerotate(&mut self, value: i32) {
        // Rotates the background hues by value.
        // TODO: So this is weird, image's huerotate also darkens the colours. I added a self.brighten to counteract.

        self.brighten(1);
        let rgbimg = self.toimage();
        let overlay_old = self.get_overlay();
        let img_hue = DynamicImage::ImageRgb8(rgbimg).huerotate(value);
        self.fromimage(img_hue, Some(overlay_old));
    }

    pub fn toimage(&mut self) -> RgbImage {
        // a default (black) image containing Rgb values
        let mut image = RgbImage::new(self.board.size.x as u32, (2 * self.board.size.y) as u32);

        for (i, cell) in self.overlay.iter().enumerate() {
            let x = (i % self.board.size.x) as u32;
            let y = (i / self.board.size.x) as u32;

            let col = cell.backcolor;
            match col {
                Color::Rgb(r, g, b) => {
                    image.put_pixel(x as u32, (2 * y) as u32, Rgb([r,g,b]));
                    image.put_pixel(x as u32, (2 * y + 1) as u32, Rgb([r,g,b]));
                }

                _ => continue
            }
        }

        image
    }

    pub fn fromimage(&mut self, img: DynamicImage, overlay_old: Option<Vec<Cell>>) {
        self.add_history();
        // Loads an image into the self.board

        // The Option for passing an old overlay is there for internal processing, it allows us to skip resizing.
        // We try not to make the canvas larger than 100 x 50 cells, so images above that will be scaled down.
        // Amazingly, however, it doesn't crash on images even as large as 1920x1080, it's just super slow.

        let mut rgbimg = img.into_rgb8();
        let (img_w, img_h) = rgbimg.dimensions() as (u32, u32);
        self.board = Board::new( Vec2::new(img_w as usize, (img_h / 2) as usize));
        let mut overlay_new: Vec<Cell>;

        match overlay_old {
            Some(o) => overlay_new = o,

            None => {if img_w > 100 || img_h > 100 {
                         rgbimg = DynamicImage::ImageRgb8(rgbimg).thumbnail(100, 50).into_rgb8();
                         let (img_w, img_h) = rgbimg.dimensions() as (u32, u32);
                         self.board = Board::new( Vec2::new(img_w as usize, (img_h / 2) as usize));
                     }

                     self.clear();  //For quickly resizing the overlay

                     overlay_new = vec![Cell {color: Color::Rgb(255, 255, 255), backcolor: Color::Rgb(255, 255, 255), symbol: ' '}; self.board.size.x * self.board.size.y];
                 },
        }


        for (i, _cell) in self.overlay.iter().enumerate() {
            let x = (i % self.board.size.x) as u32;
            let y = (i / self.board.size.x) as u32;

            // Only every second line is parsed into the canvas to conserve image aspect ratio.
            let rgb = rgbimg.get_pixel(x, 2*y);
            overlay_new[i].backcolor = Color::Rgb(rgb[0], rgb[1], rgb[2]);
        }

        self.overlay = overlay_new;
    }


    pub fn totext(&mut self) -> String {
        let mut text: String = String::from("");

        for (i, cell) in self.overlay.iter().enumerate() {
            let x = i % self.board.size.x;

            if x != self.board.size.x - 1 {
                text.push(cell.symbol);
            }

            else {
                text.push(cell.symbol);
                text.push('\n');
            }
        }

        text
    }

    pub fn tofile (&mut self) -> String {
        let mut text: String = String::from("");
        match self.overlay[0].backcolor {
            Color::Rgb(_r, _g, _b) => {text = text + "RGB\n";}, // We might want an Alpha Channel in the future, so we better tell everyone what this is.
            _ => {},
        }

        text = text + &format!("{:0>4}x{:0>4}\n\t", self.board.size.x, self.board.size.y)[..]; // This formatting assumes maximum dimenstions of 9999x9999

        for (_i, cell) in self.overlay.iter().enumerate() {
            let back_col = cell.backcolor;
            let col = cell.color;
            match (col, back_col) {
                (Color::Rgb(r, g, b), Color::Rgb(r2, g2, b2)) => {
                    text = text + &format!("{:0>3}{:0>3}{:0>3}|{:0>3}{:0>3}{:0>3}|{}\t", r,g,b,r2,g2,b2,cell.symbol)[..];
                }
                _ => continue
            }
        }
        text
    }

    pub fn fromfile (&mut self, text: String) {
        let lines: Vec<&str> = text.lines().collect();

        if lines[0] == "RGB" {
            let (width, height) = (&lines[1][0..4], &lines[1][5..9]);

            self.board = Board::new( Vec2::new(width.parse::<usize>().unwrap(), height.parse::<usize>().unwrap()));
            self.clear();
            let mut overlay_new = vec![Cell {color: Color::Rgb(255, 255, 255), backcolor: Color::Rgb(255, 255, 255), symbol: ' '} ; self.board.size.x * self.board.size.y];

            let cs: Vec<&str> = lines[2].split('\t').collect();

            for (i, _cell) in self.overlay.iter().enumerate() {
                let c: Vec<char> = cs[i+1].to_string().chars().collect();
                // the following is written so badly it's almost comical, sorry. Iterating over unicode strings is so hard :(
                let (r, g, b) = (c[0..3].iter().collect::<String>().parse::<u8>().unwrap(), c[3..6].iter().collect::<String>().parse::<u8>().unwrap(), c[6..9].iter().collect::<String>().parse::<u8>().unwrap());
                let (r2,g2,b2) = (c[10..13].iter().collect::<String>().parse::<u8>().unwrap(), c[13..16].iter().collect::<String>().parse::<u8>().unwrap(), c[16..19].iter().collect::<String>().parse::<u8>().unwrap());
                let s = c[20].to_string().parse::<char>().unwrap_or(' ');



                overlay_new[i] = Cell {color: Color::Rgb(r, g, b), backcolor: Color::Rgb(r2,g2,b2), symbol: s};
            }
            self.overlay = overlay_new;

        }
    }

    pub fn back(&mut self) {
        let overlay_past = self.history_o.pop();

        match overlay_past {
            Some(o) => {self.overlay = o;
                        self.board = self.history_b.pop().unwrap()},
            None => {}
        }
    }

    pub fn add_history(&mut self) {
        if self.history_o.len() > 250 { // We could decide on a different history length, but this works for me.
            self.history_o.remove(0);
            self.history_b.remove(0);
        }

        let overlay = self.get_overlay();
        let board = self.get_board();
        self.history_o.push(overlay);
        self.history_b.push(board);
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.tool = tool;
    }

    fn get_cell(&self, mouse_pos: Vec2, offset: Vec2) -> Option<Vec2> {
        mouse_pos
            .checked_sub(offset)
            .map(|pos| pos.map_x(|x| x))
            .and_then(|pos| {
                if pos.fits_in(self.board.size) {
                    Some(pos)
                } else {
                    None
                }
            })
    }
    pub fn get_width(&mut self) -> u32 {
        self.board.size.x as u32
    }
    pub fn get_height(&mut self) -> u32 {
        self.board.size.y as u32
    }
    pub fn get_overlay(&mut self) -> Vec<Cell> {
        let mut copy = vec![Cell {color: Color::Rgb(255, 255, 255), backcolor: Color::Rgb(255, 255, 255), symbol: ' '} ; self.board.size.x * self.board.size.y];
        for (i, cell) in self.overlay.iter().enumerate() {copy[i] = *cell}
        copy
    }
    pub fn get_overlay_len(&mut self) -> u32 {
        self.overlay.len() as u32
    }
    pub fn get_board(&mut self) -> Board {
        Board::new(self.board.size)
    }
}

impl cursive::view::View for CanvasView {
    fn draw(&self, printer: &Printer) {
        for (i, cell) in self.overlay.iter().enumerate() {
            let x = i % self.board.size.x;
            let y = i / self.board.size.x;

            let text = cell.symbol;
            let backcolor = cell.backcolor;
            let color = cell.color;

            printer.with_color(
                ColorStyle::new(color, backcolor),
                |printer| printer.print((x, y), &text.to_string()),
            );
        }
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Press(_btn),
            } => {
                match self.tool {
                    Tool::Pipette => {
                        if let Some(pos) = self.get_cell(position, offset) {
                            let i = pos.x + self.get_width() as usize * pos.y;
                            let cell = self.overlay[i];
                            unsafe {
                                BRUSHCOLOR = cell.color;
                                BRUSHBACKCOLOR = cell.backcolor;
                            }

                            self.brushsymbol = cell.symbol;
                            self.tool = Tool::Draw;
                    }},

                    _ => {}
                }
            },

            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Hold(_btn),
            } => {
                // Get cell for position
                if let Some(pos) = self.get_cell(position, offset) {
                    self.focused = Some(pos);
                    unsafe {
                        self.changecolor(pos, BRUSHCOLOR, BRUSHBACKCOLOR);
                    }

                    return EventResult::Consumed(None);
                }
            },

            Event::Mouse {
                event: MouseEvent::Release(_btn), ..
            } => {
                self.add_history();
            }
            _ => (),
        }

        EventResult::Ignored
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.board.size.map_x(|x| x)
    }
}

pub struct PickView {
    // We'd like to use a painter's colour picker here, so we need to think in Hue, Brightness and Saturation.

    // Hue is a number from 0 to 360, maybe imagine degrees around the colour wheel.
    // Width and Height are the width and height on the screen in cells the picker takes up.

    hue: u16,
    pub width:u8,
    pub height: u8,
}

impl PickView {
    pub fn new(width: u8, height: u8) -> Self {
        let hue = 0 as u16;
        PickView {hue, width, height}
    }


    fn get_cell(&self, mouse_pos: Vec2, offset: Vec2) -> Option<Vec2> {
        mouse_pos
            .checked_sub(offset)
            .map(|pos| pos.map_x(|x| x))
            .and_then(|pos| {
                if pos.fits_in(Vec2::new(self.width.into(), self.height.into())) {
                    Some(pos)
                } else {
                    None
                }
            })
    }

    pub fn set_hue (&mut self, h: u16) {
        self.hue = h;
    }
}

impl cursive::view::View for PickView {
    //  Here we implement Cursive's methods for our Colour Picker.

    fn draw(&self, printer: &Printer) {
        let rect = self.width * self.height;

        for n in 0..rect {
            let x = n % self.width;
            let y = n / self.width;

            if x > y {

            }

            let value = 1. - (y as f32 / self.height as f32); // "darkness"
            let saturation = x as f32 / self.width as f32;

            let rgb = hsv_to_rbg(self.hue, saturation, value);

            printer.with_color(
                ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(rgb.0, rgb.1, rgb.2)),
                |printer| printer.print((x, y), " "),
            );
        }
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Press(btn),
            } => {  // TODO: I'd like to avoid doubling the definition of rgb, but I don't know how to get pos.x / pos.y in this line...
                    match btn {
                        MouseButton::Left => {
                            if let Some(pos) = self.get_cell(position, offset) {
                                unsafe {
                                    let value = 1. - (pos.y as f32 / self.height as f32);
                                    let saturation = pos.x as f32 / self.width as f32;
                                    let rgb = hsv_to_rbg(self.hue, saturation, value);

                                    BRUSHBACKCOLOR = Color::Rgb(rgb.0, rgb.1, rgb.2);
                                }

                                return EventResult::Consumed(None);
                            }
                        }
                        MouseButton::Right => {
                            if let Some(pos) = self.get_cell(position, offset) {
                                unsafe {
                                    let value = 1. - (pos.y as f32 / self.height as f32);
                                    let saturation = pos.x as f32 / self.width as f32;
                                    let rgb = hsv_to_rbg(self.hue, saturation, value);

                                    BRUSHCOLOR = Color::Rgb(rgb.0, rgb.1, rgb.2);
                                }

                                return EventResult::Consumed(None);
                            }
                        }
                        _ => (),
                    }
            }

            _ => (),
        }

        EventResult::Ignored
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        let vector = Vec2::new(self.width.into(), self.height.into());
        vector.map_x(|x| x)
    }
}
