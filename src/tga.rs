use std::{
    backtrace::BacktraceStatus,
    fmt,
    fs::File,
    io::{self, Write},
    io::{BufRead, BufReader, BufWriter, Read},
    time::Instant,
};

use bytemuck::{bytes_of, bytes_of_mut};
use bytemuck_derive::{Pod, Zeroable};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TGAColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl TGAColor {
    pub const WHITE: TGAColor = TGAColor {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const BLACK: TGAColor = TGAColor {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const RED: TGAColor = TGAColor {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: TGAColor = TGAColor {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: TGAColor = TGAColor {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const YELLOW: TGAColor = TGAColor {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const PURPLE: TGAColor = TGAColor {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const CLEAR: TGAColor = TGAColor {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub fn raw(&self) -> [u8; 4] {
        [self.b, self.g, self.r, self.a]
    }
    pub fn get_color(&self, intensity: f32) -> TGAColor {
        TGAColor {
            r: (self.r as f32 * intensity) as u8,
            g: (self.g as f32 * intensity) as u8,
            b: (self.b as f32 * intensity) as u8,
            a: self.a,
        }
    }
}

impl fmt::Display for TGAColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

#[derive(Clone, Copy)]
pub enum TGAFormat {
    GRAYSCALE = 1,
    RGB = 3,
    RGBA = 4,
}

#[derive(Default, Clone, Copy, Pod, Zeroable)]
#[repr(C, packed)]
struct TGAHeader {
    id_length: u8,
    color_map_type: u8,
    data_type_code: u8,
    color_map_origin: u16,
    color_map_length: u16,
    color_map_depth: u8,
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    bits_per_pixel: u8,
    image_description: u8,
}

pub struct TGAImage {
    width: usize,
    height: usize,
    format: TGAFormat,
    data: Vec<u8>,
}

impl TGAImage {
    pub fn new(width: usize, height: usize, format: TGAFormat) -> Self {
        let bytespp = format as usize;
        Self {
            width,
            height,
            format,
            data: vec![0; width * height * bytespp],
        }
    }

    pub fn bytespp(&self) -> usize {
        self.format as usize
    }

    pub fn read_tga_file(&mut self, filename: &str) -> io::Result<()> {
        let mut r = BufReader::new(File::open(filename)?);
        let mut header = TGAHeader::default();
        let mut u8buff: u8 = 0;

        r.read(bytes_of_mut(&mut u8buff))?;
        header.id_length = u8buff;
        r.read(bytes_of_mut(&mut u8buff))?;
        header.color_map_type = u8buff;
        r.read(bytes_of_mut(&mut u8buff))?;
        header.data_type_code = u8buff;
        let mut u16buf: u16 = 0;
        r.read(bytes_of_mut(&mut u16buf))?;
        header.color_map_origin = u16buf;
        r.read(bytes_of_mut(&mut u16buf))?;
        header.color_map_length = u16buf;
        r.read(bytes_of_mut(&mut u8buff))?;
        header.color_map_depth = u8buff;
        r.read(bytes_of_mut(&mut u16buf))?;
        header.x_origin = u16buf;
        r.read(bytes_of_mut(&mut u16buf))?;
        header.y_origin = u16buf;
        r.read(bytes_of_mut(&mut u16buf))?;
        header.width = u16buf;
        r.read(bytes_of_mut(&mut u16buf))?;
        header.height = u16buf;
        r.read(bytes_of_mut(&mut u8buff))?;
        header.bits_per_pixel = u8buff;
        r.read(bytes_of_mut(&mut u8buff))?;
        header.image_description = u8buff;

        self.width = header.width as usize;
        self.height = header.height as usize;
        let bytespp = header.bits_per_pixel >> 3;
        let mut badformat = false;
        if bytespp == 1 {
            self.format = TGAFormat::GRAYSCALE;
        } else if bytespp == 3 {
            self.format = TGAFormat::RGB;
        } else if bytespp == 4 {
            self.format = TGAFormat::RGBA;
        } else {
            badformat = true;
        }
        if self.width <= 0 || self.height <= 0 || badformat {
            return Err(io::Error::new(io::ErrorKind::Other, "Bad format"));
        }
        let bytespp = self.bytespp();
        let nbytes = bytespp * self.width * self.height;
        self.data = vec![0; nbytes];
        if header.data_type_code == 3 || header.data_type_code == 2 {
            r.read(&mut self.data)?;
        } else if header.data_type_code == 10 || header.data_type_code == 11 {
            self.load_rle_data(&mut r)?;
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "Bad data type"));
        }
        if !((header.image_description & 0x20) != 0) {
            self.flip_vertically();
        }
        if (header.image_description & 0x10) != 0 {
            self.flip_horizontally();
        }

        Ok(())
    }

    fn load_rle_data(&mut self, file: &mut BufReader<File>) -> io::Result<()> {
        let pixel_count = self.width * self.height;
        let mut current_pixel: usize = 0;
        let mut current_byte: usize = 0;
        // let color_buffer = TGAColor::CLEAR;
        let bytespp = self.bytespp();
        let mut color_buff: Vec<u8> = vec![0; bytespp];
        while current_pixel < pixel_count {
            let mut chunk_header: u8 = 0;
            file.read(bytes_of_mut(&mut chunk_header))?;
            if chunk_header < 128 {
                chunk_header += 1;
                for _i in 0..chunk_header {
                    file.read(&mut color_buff)?;
                    for t in 0..bytespp {
                        self.data[current_byte] = color_buff[t];
                        current_byte += 1;
                    }
                    current_pixel += 1;
                    if current_pixel > pixel_count {
                        return Err(io::Error::new(io::ErrorKind::Other, "Too many pixels read"));
                    }
                }
            } else {
                chunk_header -= 127;
                file.read(&mut color_buff)?;
                for _i in 0..chunk_header {
                    for t in 0..bytespp {
                        self.data[current_byte] = color_buff[t];
                        current_byte += 1;
                    }
                    current_pixel += 1;
                    if current_pixel > pixel_count {
                        return Err(io::Error::new(io::ErrorKind::Other, "Too many pixels read"));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn write_tga_file(&self, filename: &str, rle: bool) -> io::Result<()> {
        let mut o = BufWriter::new(File::create(filename)?);

        let developer_area_ref: [u8; 4] = [0; 4];
        let extension_area_ref: [u8; 4] = [0; 4];
        let footer: [char; 18] = [
            'T', 'R', 'U', 'E', 'V', 'I', 'S', 'I', 'O', 'N', '-', 'X', 'F', 'I', 'L', 'E', '.',
            '\0',
        ];

        let mut header = TGAHeader::default();
        header.bits_per_pixel = (self.bytespp() << 3) as u8;
        header.width = self.width as u16;
        header.height = self.height as u16;
        header.data_type_code = if let TGAFormat::GRAYSCALE = self.format {
            if rle {
                11
            } else {
                3
            }
        } else {
            if rle {
                10
            } else {
                2
            }
        };
        header.image_description = 0x20; // top-left origin
        if let Err(error) = o.write_all(bytes_of(&mut header)) {
            return Err(error);
        }

        if !rle {
            if let Err(error) = o.write_all(&self.data) {
                return Err(error);
            }
        } else {
            if let Err(error) = self.unload_rle_data(&mut o) {
                return Err(error);
            }
        }
        if let Err(error) = o.write_all(&developer_area_ref) {
            return Err(error);
        }
        if let Err(error) = o.write_all(&extension_area_ref) {
            return Err(error);
        }
        if let Err(error) = o.write_all(&footer.map(|c| c as u8)) {
            return Err(error);
        }

        Ok(())
    }

    fn unload_rle_data(&self, file: &mut BufWriter<File>) -> io::Result<()> {
        let max_chunk_length: usize = 128;
        let npixels = self.width * self.height;
        let mut curpix = 0;
        while curpix < npixels {
            let bytespp = self.bytespp();
            let chunkstart = curpix * bytespp;
            let mut curbyte = curpix * bytespp;
            let mut run_length = 1;
            let mut raw = true;
            while (curpix + run_length) < npixels && run_length < max_chunk_length {
                let mut succ_eq = true;
                let mut t = 0;
                while succ_eq && t < bytespp {
                    succ_eq = self.data[curbyte + t] == self.data[curbyte + t + bytespp];
                    t += 1;
                }
                curbyte += bytespp;
                if run_length == 1 {
                    raw = !succ_eq;
                }
                if raw && succ_eq {
                    run_length -= 1;
                    break;
                }
                if !raw && !succ_eq {
                    break;
                }
                run_length += 1;
            }
            curpix += run_length;
            let v = if raw {
                run_length - 1
            } else {
                run_length + 127
            } as u8;
            if let Err(err) = file.write_all(&[v]) {
                println!("{}", err);
                return Err(err);
            }
            let len = if raw { run_length * bytespp } else { bytespp };
            if let Err(err) = file.write_all(&self.data[chunkstart..(chunkstart + len)]) {
                println!("{}", err);
                return Err(err);
            }
        }

        Ok(())
    }

    pub fn flip_horizontally(&mut self) {
        let half = self.width >> 1;
        for i in 0..half {
            for j in 0..self.height {
                let c1 = self.get(i, j);
                let c2 = self.get(self.width - 1 - i, j);
                self.set(i, j, c2);
                self.set(self.width - 1 - i, j, c1);
            }
        }
    }

    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let format = self.format;
        let off1 = self.offset(x1, y1);
        let off2 = self.offset(x2, y2);
        let bytespp = self.bytespp();
        match format {
            TGAFormat::GRAYSCALE => self.data.swap(off1, off2),
            TGAFormat::RGB | TGAFormat::RGBA => {
                for i in 0..bytespp {
                    self.data.swap(off1 + i, off2 + i)
                }
            }
        }
    }

    pub fn flip_vertically(&mut self) {
        let half = self.height >> 1;
        for i in 0..self.width {
            for j in 0..half {
                self.swap(i, j, i, self.height - 1 - j);
            }
        }
    }

    pub fn scale(&mut self, w: usize, h: usize) {
        let bytespp = self.bytespp();
        let mut tdata: Vec<u8> = vec![0; w * h * bytespp];
        let mut nscanline: usize = 0;
        let mut oscanline: usize = 0;
        let mut erry: i64 = 0;
        let nlinebytes = w * bytespp;
        let olinebytes = self.width * bytespp;
        for _j in 0..self.height {
            let mut errx = self.width as i64 - w as i64;
            let mut nx = -(bytespp as i64);
            let mut ox = -(bytespp as i64);
            for _i in 0..self.width {
                ox += bytespp as i64;
                errx += w as i64;
                while errx >= self.width as i64 {
                    errx -= self.width as i64;
                    nx += bytespp as i64;
                    let toff = nscanline + nx as usize;
                    let ooff = oscanline + ox as usize;
                    tdata[toff..toff + bytespp].copy_from_slice(&self.data[ooff..ooff + bytespp]);
                }
            }
            erry += h as i64;
            oscanline += olinebytes;
            while erry >= self.height as i64 {
                if erry >= (self.height << 1) as i64 {
                    let toff = nscanline + nlinebytes;
                    let mut tbuff: Vec<u8> = vec![0; nlinebytes];
                    tbuff.copy_from_slice(&tdata[nscanline..nscanline + nlinebytes]);
                    tdata[toff..toff + nlinebytes].copy_from_slice(&tbuff);
                }
                erry -= self.height as i64;
                nscanline += nlinebytes;
            }
        }
        self.data = tdata;
        self.width = w;
        self.height = h;
    }

    fn offset(&self, x: usize, y: usize) -> usize {
        (x + y * self.width) * self.bytespp()
    }

    pub fn get(&self, x: usize, y: usize) -> TGAColor {
        if x >= self.width || y >= self.height {
            return TGAColor::CLEAR;
        }
        let offset = self.offset(x, y);
        match self.format {
            TGAFormat::GRAYSCALE => TGAColor {
                r: 0,
                g: 0,
                b: self.data[offset],
                a: 0,
            },
            TGAFormat::RGB => TGAColor {
                r: self.data[offset + 2],
                g: self.data[offset + 1],
                b: self.data[offset],
                a: 255,
            },
            TGAFormat::RGBA => TGAColor {
                r: self.data[offset + 2],
                g: self.data[offset + 1],
                b: self.data[offset],
                a: self.data[offset + 3],
            },
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: TGAColor) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        let bytespp = self.bytespp();
        let offset = (x + y * self.width) * bytespp;
        self.data[offset..offset + bytespp].copy_from_slice(&color.raw()[0..bytespp]);

        return true;
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn clear(&mut self) {
        self.data = vec![0; self.width * self.height * self.bytespp()];
    }
}
