use std::{io::{Write, Read, Seek}, mem::transmute};

type BYTE = u8;
type DWORD = u32;
type LONG = i32;
type WORD = u16;

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct BITMAPFILEHEADER {
    pub bf_type: WORD,
    pub bf_size: DWORD,
    pub bf_reserved1: WORD,
    pub bf_reserved2: WORD,
    pub bf_off_bits: DWORD,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct BITMAPINFOHEADER {
    pub bi_size: DWORD,
    pub bi_width: LONG,
    pub bi_height: LONG,
    pub bi_planes: WORD,
    pub bi_bit_count: WORD,
    pub bi_compression: DWORD,
    pub bi_size_image: DWORD,
    pub bi_x_pels_per_meter: LONG,
    pub bi_y_pels_per_meter: LONG,
    pub bi_clr_used: DWORD,
    pub bi_clr_important: DWORD,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct RGBTRIPLE {
    pub rgbt_blue: BYTE,
    pub rgbt_green: BYTE,
    pub rgbt_red: BYTE,
}

#[derive(Debug)]
#[repr(C)]
pub struct BmpImg {
    pub infoheader: BITMAPINFOHEADER,
    pub fileheader: BITMAPFILEHEADER,
    pub pixels: Vec<RGBTRIPLE>,
}

pub trait ToBytes {
    /// Turns the struct into bytes
    fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for BITMAPFILEHEADER {
    fn to_bytes(&self) -> Vec<u8> {
        unsafe { transmute::<BITMAPFILEHEADER, [u8; 14]>(*self).to_vec() }
    }
}

impl ToBytes for BITMAPINFOHEADER {
    fn to_bytes(&self) -> Vec<u8> {
        unsafe { transmute::<BITMAPINFOHEADER, [u8; 40]>(*self).to_vec() }
    }
}

impl ToBytes for RGBTRIPLE {
    fn to_bytes(&self) -> Vec<u8> {
        unsafe { transmute::<RGBTRIPLE, [u8; 3]>(*self).to_vec() }
    }
}

impl ToBytes for BmpImg {
    fn to_bytes(&self) -> Vec<u8> {
        let mut r_vec = vec![self.fileheader.to_bytes(), self.infoheader.to_bytes()].concat();
        let mut p_vec: Vec<u8> = vec![];

        for rgb in &self.pixels {
            p_vec.extend(rgb.to_bytes())
        }

        r_vec.extend(p_vec);

        r_vec
    }
}

impl BmpImg {
    /// Fills the whole image with the desired RGB values
    pub fn fill_image(&mut self, r: BYTE, g: BYTE, b: BYTE) {
        for rgb in &mut self.pixels {
            rgb.rgbt_blue = b;
            rgb.rgbt_green = g;
            rgb.rgbt_red = r;
        }
    }

    /// Creates a new [`BmpImg`].
    pub fn new(height: i32, width: i32) -> Self {
        Self {
            infoheader: BITMAPINFOHEADER {
                bi_size: 40,
                bi_width: width,
                bi_height: height,
                bi_planes: 1,
                bi_bit_count: 24,
                bi_compression: 0,
                bi_size_image: (height * width + 54) as u32,
                bi_x_pels_per_meter: 30,
                bi_y_pels_per_meter: 30,
                bi_clr_used: 0,
                bi_clr_important: 0,
            },
            fileheader: BITMAPFILEHEADER {
                bf_type: 0x4D42,
                bf_size: (width * height + 54) as u32,
                bf_reserved1: 0,
                bf_reserved2: 0,
                bf_off_bits: 54,
            },
            pixels: vec![
                RGBTRIPLE {
                    rgbt_blue: 0,
                    rgbt_green: 0,
                    rgbt_red: 0
                };
                (width * height) as usize
            ],
        }
    }

    /// Writes all the structs to a file pointed to by the [`path`] argument
    ///
    /// # Panics
    ///
    /// Panics if creating the file fails or if the structs cant't be written to the file
    pub fn write_to_file(&self, path: &str) {
        let path = std::path::Path::new(path);

        let mut fd = match std::fs::File::create(path) {
            Ok(f) => {f},
            Err(e) => {panic!("Oops -> {e}")},
        };

        match fd.write(&self.to_bytes()) {
            Ok(_) => {},
            Err(e) => {panic!("Oops -> {e}")},
        };
    }

    pub fn from_file(path: &str) -> BmpImg {
        let path = std::path::Path::new(path);

        let mut fd = match std::fs::File::options().read(true).write(true).open(path) {
            Ok(f) => {f},
            Err(e) => {panic!("Oops -> {e}")},
        };

        let mut file_header: [u8; 14] = [0u8; 14];
        fd.read(&mut file_header).expect("Failed reading file header!");

        let mut info_header: [u8; 40] = [0u8; 40];
        fd.seek(std::io::SeekFrom::Start(14)).expect("Failed positioning the file cursor into offset 14 (INFOHEADER)");
        fd.read(&mut info_header).expect("Failed reading info header");

        let mut rgb_vec: Vec<u8> = vec![];
        fd.seek(std::io::SeekFrom::Start(54)).expect("Failed positioning the file cursor into offset 54 (start of RGB info)");
        fd.read_to_end(&mut rgb_vec).expect("Failed reading RGB");

        BmpImg { 
            infoheader: unsafe {transmute::<[u8; 40], BITMAPINFOHEADER>(info_header)}, 
            fileheader: unsafe {transmute::<[u8; 14], BITMAPFILEHEADER>(file_header)}, 
            pixels: unsafe {transmute::<Vec<u8>, Vec<RGBTRIPLE>>(rgb_vec)}, 
        }
    }
}
