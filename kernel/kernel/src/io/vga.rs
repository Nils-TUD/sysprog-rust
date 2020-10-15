use crate::util::StaticCell;
use rlibc;

const BUFFER: *mut u8 = 0xB8000 as *mut u8;
const COLS: usize = 80;
const ROWS: usize = 25;

/// The global VGA instance remembers the cursor position on the screen
pub static VGA_INST: StaticCell<VGA> = StaticCell::new(VGA::new());

#[derive(Default)]
pub struct VGA {
    col: usize,
    row: usize,
}

/// Inits the VGA screen (clears it)
pub fn init() {
    unsafe {
        rlibc::memset(BUFFER, 0, ROWS * COLS * 2);
    }
}

impl VGA {
    const fn new() -> Self {
        Self { col: 0, row: 0 }
    }

    /// Writes the given character to the current screen position, potentially moving lines up if
    /// required.
    pub fn write_char(&mut self, b: u8) {
        // explicit newline or end of line?
        if self.col >= COLS || b == b'\n' {
            self.row += 1;
            self.col = 0;
        }
        // carriage return?
        else if b == b'\r' {
            self.col = 0;
        }

        // last line?
        if self.row >= ROWS {
            // copy all chars one line back
            unsafe {
                rlibc::memmove(
                    BUFFER,
                    BUFFER.offset((COLS * 2) as isize),
                    (ROWS - 1) * COLS * 2,
                );
                rlibc::memset(BUFFER.offset(((ROWS - 1) * COLS * 2) as isize), 0, COLS * 2);
            }
            self.row -= 1;
        }

        // tabs
        if b == b'\t' {
            self.write_char(b' ');
            self.write_char(b' ');
        }
        // whitespace and printable chars
        else if b == b' ' || b.is_ascii_graphic() {
            let off = self.row * COLS * 2 + self.col * 2;
            unsafe {
                *BUFFER.offset(off as isize) = b;
                *BUFFER.offset(off as isize + 1) = 0xb;
            }
            self.col += 1;
        }
    }
}
