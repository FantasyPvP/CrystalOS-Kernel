use lazy_static::lazy_static;
use spin::Mutex;


lazy_static! {
    pub static ref FILESYSTEM: Mutex<Directory> = Mutex::new(Directory::new());
}

pub type Directory = Vec<Box<File>>;

pub struct File<T> {
    pub name: String,
    pub data: T,
}
impl<T> File<T> {
    pub fn new(name: String, data: T) -> Self {
        Self { name, data }
    }
}

fn mkfs() {
    FILESYSTEM.lock().push(File)
}
 

/* 
lazy_static! {
    pub static ref FILESYSTEM: Mutex<Filesystem> = Mutex::new(Filesystem::new());
}

enum FsError {
    FileNotFound,
    AlreadyExists,
    InvalidPath,
}

pub type Path = String;

impl Path {
    pub fn new(path: &str) -> Result<Path, FsError> {
        if path.is_empty() {
            return Err(FsError::FileNotFound);
        }
        Ok(path.to_string())
    }
    pub fn dirs(&self) -> Vec<String> {
        self.split('/').map(|s| s.to_string()).collect()
    }
}


pub struct Filesystem {
    pub root: Vec<File>
}



pub type Directory = Vec<File>;

impl Directory {
    pub fn new(name: String, files: Vec<File>) -> Self {
        Self { name, files }
    }
    pub fn size(&self) -> usize {
        self.files.len()
    }
    pub fn mkdir(&mut self, name: &str, location: Path) -> Result<(), FsError> {
        if self.exists(location.as_str()) {
            return Err(FsError::AlreadyExists);
        }
        self.files.push(File::new(name, FileType::Directory(Box::new(Directory::new()))))
    }
}


pub enum FileType {
    Directory(Box<Directory>),
    TextFile(Box<TextFile>),
    BinaryFile(Box<BinaryFile>),
    Executable(fn()),
}


pub struct File {
    pub name: String,
    pub file_type: FileType,
}
impl File {
    pub fn new(name: String, file_type: FileType) -> Self {
        Self { name, file_type }
    }
}



pub struct TextFile {
    pub name: String,
    pub data: String,
}
impl TextFile {
    pub fn new(name: String, data: String) -> Self {
        Self { name, data }
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
}


*/