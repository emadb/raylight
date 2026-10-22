const SHEET: &[u8] = include_bytes!("../wolfenstein_textures.png");

pub const TEX_SIZE: usize = 64;
const COLS: usize = 10;
const ROWS: usize = 11;

pub struct Tile {
    pub index: usize,
    pub pixels: Vec<u32>,
}

pub struct Textures {
    images: Vec<Tile>,
}

struct Sheet {
    width: usize,
    pixels: Vec<u32>,
}

fn decode(bytes: &[u8]) -> Sheet {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND);

    let mut reader = decoder.read_info().expect("invalid texture sheet");
    let mut buf = vec![
        0;
        reader
            .output_buffer_size()
            .expect("texture sheet too large")
    ];
    let info = reader
        .next_frame(&mut buf)
        .expect("cannot decode texture sheet");

    let channels = info.color_type.samples();
    let pixels = buf[..info.buffer_size()]
        .chunks_exact(channels)
        .map(|p| u32::from_be_bytes([0, p[0], p[1], p[2]]))
        .collect();

    Sheet {
        width: info.width as usize,
        pixels,
    }
}

fn tile(sheet: &Sheet, index: usize) -> Tile {
    let x0 = (index % COLS) * TEX_SIZE;
    let y0 = (index / COLS) * TEX_SIZE;

    let pixels = (y0..y0 + TEX_SIZE)
        .flat_map(|y| {
            let start = y * sheet.width + x0;
            sheet.pixels[start..start + TEX_SIZE].iter().copied()
        })
        .collect();

    Tile { index, pixels }
}

impl Textures {
    pub fn load_textures() -> Self {
        let sheet = decode(SHEET);
        let images = (0..COLS * ROWS).map(|i| tile(&sheet, i)).collect();

        Self { images }
    }

    pub fn get(&self, index: usize) -> &Tile {
        self.images.get(index).unwrap()
    }

    pub fn len(&self) -> usize {
        self.images.len()
    }

    pub fn is_empty(&self) -> bool {
        self.images.is_empty()
    }
}
