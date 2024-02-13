#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64),
}


impl FileSize{
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
            FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
            FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
            FileSize::Terabytes(tb) => format!("{:.2} TB", tb),
        }
    }

    fn as_bytes(&self) -> u64 {
        match self {
            FileSize::Bytes(bytes) => *bytes,
            FileSize::Kilobytes(kb) => (*kb * 1000.0) as u64,
            FileSize::Megabytes(mb) => (*mb * 1_000_000.0) as u64,
            FileSize::Gigabytes(gb) => (*gb * 1_000_000_000.0) as u64,
            FileSize::Terabytes(tb) => (*tb * 1_000_000_000_000.0) as u64,
        }
    }

    fn match_to_units(&self) -> FileSize {
        let in_bytes = self.as_bytes();

        match in_bytes {
            0..=999 => FileSize::Bytes(in_bytes),
            1000..=999_999 => FileSize::Kilobytes(in_bytes as f64 / 1000.0),
            1_000_000..=999_999_999 => FileSize::Megabytes(in_bytes as f64 / 1_000_000.0),
            1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(in_bytes as f64 / 1_000_000_000.0),
            _ => FileSize::Terabytes(in_bytes as f64 / 1_000_000_000_000.0),
        }
    }

    fn largest_unit(&self) -> FileSize {
        let tmp_filesize = self.match_to_units();

        match tmp_filesize {
            FileSize::Bytes(_) => FileSize::Bytes(self.as_bytes()),
            FileSize::Kilobytes(_) => FileSize::Kilobytes(self.as_bytes() as f64 / 1000.0),
            FileSize::Megabytes(_) => FileSize::Megabytes(self.as_bytes() as f64 / 1_000_000.0),
            FileSize::Gigabytes(_) => FileSize::Gigabytes(self.as_bytes() as f64 / 1_000_000_000.0),
            FileSize::Terabytes(_) => FileSize::Terabytes(self.as_bytes() as f64 / 1_000_000_000_000.0),
        }
    }

}

fn main() {
    let size = 2500;
    let byte_tmp = FileSize::Bytes(size);
    println!("{:?}", byte_tmp.largest_unit());
}
