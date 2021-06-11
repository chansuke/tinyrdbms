use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek},
    path::Path,
};

pub struct PageId(pub u64);

pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}

impl DiskManager {
    pub fn new(data_file: File) -> std::io::Result<Self> {
        // ファイルサイズを取得
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;

        Ok(Self {
            heap_file,
            next_page_id,
        })
    }

    pub fn open(data_file_path: impl AsRef<Path>) -> std::io::Result<Self> {
        // ヒープファイルを開く処理
        let heap_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(heap_file_path)?;
        Self::new(heap_file);
    }

    pub fn allocate_page(&mut self) -> PageId {
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        PageId(page_id)
    }

    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> std::io::Result<()> {}

    pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> std::io::Result<()> {
        // オフセットを計算
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        // ページ先頭へシーク
        self.heap_file.seek(SeekFrom::Start(offset))?;
        // データを読み出す
        self.heap_file.read_exact(data)
    }
}
