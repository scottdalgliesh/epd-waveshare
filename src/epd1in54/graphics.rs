use epd1in54::{DEFAULT_BACKGROUND_COLOR, WIDTH, HEIGHT};
use graphics::DisplayDimension;

pub struct DisplayEink1in54BlackWhite {
    width: u32,
    height: u32,
    pub buffer: [u8; WIDTH as usize * HEIGHT as usize / 8],
}



impl Default for DisplayEink1in54BlackWhite {
    fn default() -> Self {
        DisplayEink1in54BlackWhite {
            width: WIDTH,
            height: HEIGHT,
            buffer: [
                DEFAULT_BACKGROUND_COLOR.get_byte_value();
                WIDTH as usize * HEIGHT as usize / 8                
            ]
        }
    }
}

impl DisplayDimension for DisplayEink1in54BlackWhite {
    fn buffer(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use graphics::{DisplayRotation, Display};
    use embedded_graphics::coord::Coord;
    use embedded_graphics::primitives::Line;
    use color::Color;
    use embedded_graphics::prelude::*;

    // test buffer length
    #[test]
    fn graphics_size() {
        let mut display1in54 = DisplayEink1in54BlackWhite::default();
        let display = Display::new(WIDTH, HEIGHT, &mut display1in54.buffer);
        assert_eq!(display.buffer().len(), 5000);
    }
    
    // test default background color on all bytes
    #[test]
    fn graphics_default() {
        let mut display1in54 = DisplayEink1in54BlackWhite::default();
        let display = Display::new(WIDTH, HEIGHT, &mut display1in54.buffer);
        for &byte in display.buffer() {
            assert_eq!(byte, DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
    }

    #[test]
    fn graphics_rotation_0() {
        let mut display1in54 = DisplayEink1in54BlackWhite::default();
        let mut display = Display::new(WIDTH, HEIGHT, &mut display1in54.buffer);
        display.draw(
            Line::new(Coord::new(0, 0), Coord::new(7, 0))
                .with_stroke(Some(Color::Black))
                .into_iter(),
        );
        
        let buffer = display.buffer();

        assert_eq!(buffer[0], Color::Black.get_byte_value());

        for &byte in buffer.iter().skip(1) {
            assert_eq!(byte, DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
    }

    #[test]
    fn graphics_rotation_90() {
        let mut display1in54 = DisplayEink1in54BlackWhite::default();
        let mut display = Display::new(WIDTH, HEIGHT, &mut display1in54.buffer);
        display.set_rotation(DisplayRotation::Rotate90);
        display.draw(
            Line::new(Coord::new(0, 192), Coord::new(0, 199))
                .with_stroke(Some(Color::Black))
                .into_iter(),
        );
        
        let buffer = display.buffer();

        assert_eq!(buffer[0], Color::Black.get_byte_value());

        for &byte in buffer.iter().skip(1) {
            assert_eq!(byte, DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
    }

    #[test]
    fn graphics_rotation_180() {
        let mut display1in54 = DisplayEink1in54BlackWhite::default();
        let mut display = Display::new(WIDTH, HEIGHT, &mut display1in54.buffer);
        display.set_rotation(DisplayRotation::Rotate180);
        display.draw(
            Line::new(Coord::new(192, 199), Coord::new(199, 199))
                .with_stroke(Some(Color::Black))
                .into_iter(),
        );
        
        let buffer = display.buffer();

        extern crate std;
        std::println!("{:?}", buffer);

        assert_eq!(buffer[0], Color::Black.get_byte_value());

        for &byte in buffer.iter().skip(1) {
            assert_eq!(byte, DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
        
    }

    #[test]
    fn graphics_rotation_270() {
        let mut display1in54 = DisplayEink1in54BlackWhite::default();
        let mut display = Display::new(WIDTH, HEIGHT, &mut display1in54.buffer);
        display.set_rotation(DisplayRotation::Rotate270);
        display.draw(
            Line::new(Coord::new(199, 0), Coord::new(199, 7))
                .with_stroke(Some(Color::Black))
                .into_iter(),
        );
        
        let buffer = display.buffer();

        extern crate std;
        std::println!("{:?}", buffer);

        assert_eq!(buffer[0], Color::Black.get_byte_value());

        for &byte in buffer.iter().skip(1) {
            assert_eq!(byte, DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
        
    }
}