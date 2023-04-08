use crate::type_system::runtime::Runtime;
use std::io::Read;
struct SmartReader<R: Read> {
    offset: usize,
    src: R,
}
impl<R: Read> SmartReader<R> {
    fn new(src: R) -> Self {
        Self { offset: 0, src }
    }
    fn skip_to(&mut self, offset: usize) -> std::io::Result<()> {
        let mut by = offset - self.offset;
        self.skip(by)
    }
    fn skip(&mut self, mut ammount: usize) -> std::io::Result<()> {
        let mut discard = [0; 8];
        //println!("skipping {} bytes beginig at offset 0x{:x}, and ending at offset 0x{:x}!",by,self.offset,offset);
        while ammount > 0 {
            let curr = if ammount >= 8 { 8 } else { ammount };
            ammount -= curr;
            self.read_exact(&mut discard[..curr])?;
        }
        Ok(())
    }
    fn curr_offset(&self)->usize{
        self.offset
    }
}
impl<R: Read> Read for SmartReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = self.src.read(buf)?;
        self.offset += size;
        Ok(size)
    }
}
#[derive(Debug)]
pub enum ImportError {}
#[derive(Debug)]
struct SectionHeader {
    name: String,
    offset: u32,
    size: u32,
    virtual_adress:u32,
}
fn load_headers<R: Read>(
    data: &mut SmartReader<R>,
    count: usize,
) -> Result<Vec<SectionHeader>, ImportError> {
    let mut res = Vec::with_capacity(count);
    for _ in 0..count {
        let mut name = [0; 8];
        data.read_exact(&mut name);
        let name = std::str::from_utf8(&name).unwrap().to_owned();
        //Skip virtual size
        data.skip(4);
        let mut virtual_adress = [0; 4];
        data.read_exact(&mut virtual_adress);
        let virtual_adress = u32::from_le_bytes(virtual_adress);
        let mut raw_size = [0; 4];
        data.read_exact(&mut raw_size);
        let raw_size = u32::from_le_bytes(raw_size);
        let mut raw_offset = [0; 4];
        data.read_exact(&mut raw_offset);
        let raw_offset = u32::from_le_bytes(raw_offset);
        //Skip pointer to relocations
        data.skip(4);
        //Skip pointer to line numbers
        data.skip(4);
        //Skip number of relocations
        data.skip(2);
        //Skip number of line numbers
        data.skip(2);
        //Skip characteristics
        data.skip(4);
        res.push(SectionHeader {
            name,
            offset: raw_offset,
            size: raw_size,
            virtual_adress:virtual_adress
        });
    }
    Ok(res)
}
fn load_header<R: Read>(asm: &mut SmartReader<R>) -> Result<(u32, u32,u32), ImportError> {
    let mut dos_magic = [0; 2];
    asm.read_exact(&mut dos_magic).unwrap();
    //Is dos executable
    assert_eq!(dos_magic, *b"MZ");
    //DOS header, irrelewant
    let mut dos_header = [0; 0x3C - 2];
    asm.read_exact(&mut dos_header).unwrap();
    //PE offset, ought to be 0x80
    let mut pe_offset = [0; 4];
    asm.read_exact(&mut pe_offset).unwrap();
    let pe_offset = u32::from_le_bytes(pe_offset);
    assert_eq!(pe_offset, 0x80);
    //DOS stub, irrelevant
    asm.skip_to(0x80).unwrap();
    //PE magic, letters PE and 0x00 0x00
    let mut pe_magic = [0; 4];
    asm.read_exact(&mut pe_magic).unwrap();
    assert_eq!(pe_magic, *b"PE\0\0");
    asm.skip_to(0x84).unwrap();
    let mut section_count = [0; 2];
    asm.read_exact(&mut section_count).unwrap();
    let section_count = u16::from_le_bytes(section_count);
    asm.skip_to(0x96).unwrap();
    //PE file type, ends at x97
    let mut file_type = [0; 2];
    asm.read_exact(&mut file_type).unwrap();
    let file_type = u16::from_le_bytes(file_type);
    assert_eq!(
        file_type & 0x2000,
        0x2000,
        "Only dynamic libraries may be imported now!"
    );
    //Architecture, only AnyCPU supported.
    let mut architecture = [0; 2];
    asm.read_exact(&mut architecture).unwrap();
    let architecture = u16::from_le_bytes(architecture);
    assert_eq!(architecture, 0x10b, "Only AnyCPU assemblies are supported!");
    asm.skip_to(0xDB);
    //Subsystem field
    let mut subsystem = [0; 2];
    asm.read_exact(&mut subsystem).unwrap();
    let subsystem = u16::from_le_bytes(subsystem);
    assert_eq!(subsystem, 0x300, "Only console programs supported!");
    //Irrelevant NT fields
    //asm.skip_to(0xD4B);
    asm.skip_to(0xF7);
    //Data directories
    let mut data_dirs = [0; 9 * 14];
    asm.read_exact(&mut data_dirs).unwrap();
    let cil_rva = &data_dirs[8 * 14..8 * 14 + 4];
    let cil_rva = u32::from_le_bytes(cil_rva.try_into().unwrap());
    let cil_size = &data_dirs[8 * 14 + 4..8 * 14 + 8];
    let cil_size = u32::from_le_bytes(cil_size.try_into().unwrap());
    asm.skip_to(0x178);
    //TODO: Get header count
    let headers = load_headers(asm, 3)?;
    for header in &headers {
        if header.name == ".text\0\0\0" {
            return Ok((header.size, header.offset,header.virtual_adress));
        }
    }
    panic!("No .txt header, headers:{headers:?}");
}
pub(crate) fn import_assembly<R: Read>(
    asm: &mut R,
    runtime: &mut Runtime,
) -> Result<(), ImportError> {
    let mut asm = SmartReader::new(asm);
    let mut asm = &mut asm;
    let (text_size, text_offset,virtual_adress) = load_header(asm)?;
    asm.skip_to(text_offset as usize).unwrap();
    let mut cil_data = vec![0; text_size as usize];
    asm.read_exact(&mut cil_data).unwrap();
    load_managed_data(&mut SmartReader::new(cil_data.as_slice()), runtime,text_offset,virtual_adress)
}
fn load_managed_data<R: Read>(
    cil_data: &mut SmartReader<R>,
    runtime: &mut Runtime,
    text_offset:u32,
    text_rva:u32,
) -> Result<(), ImportError> {
    //clr loader stub
    cil_data.skip(8).unwrap();
    let mut clr_header_size = [0; 4];
    cil_data.read_exact(&mut clr_header_size).unwrap();
    let cil_header_size = u32::from_le_bytes(clr_header_size);
    assert_eq!(cil_header_size, 0x48);
    let mut clr_major = [0; 2];
    cil_data.read_exact(&mut clr_major).unwrap();
    let clr_major = u16::from_le_bytes(clr_major);
    assert_eq!(clr_major, 2);
    let mut clr_minor = [0; 2];
    cil_data.read_exact(&mut clr_minor).unwrap();
    let clr_minor = u16::from_le_bytes(clr_minor);
    assert_eq!(clr_minor, 5);
    let mut rva = [0; 4];
    cil_data.read_exact(&mut rva).unwrap();
    let rva = u32::from_le_bytes(rva);
    let mut metadata_header_size = [0; 4];
    cil_data.read_exact(&mut metadata_header_size).unwrap();
    let metadata_header_size = u32::from_le_bytes(metadata_header_size);
    //TODO:Check flags!
    let mut flags = [0; 4];
    cil_data.read_exact(&mut flags).unwrap();
    //TODO: handle entry
    let mut entry = [0; 4];
    cil_data.read_exact(&mut entry).unwrap();
    //Unused CIL header data
    cil_data.skip_to(0x50);
    //Strong name hash
    cil_data.skip_to(0xC0);
    println!("metatdata_rva:{rva}, text_rva:{text_rva}");
    let metadata_start = rva - text_rva + text_offset;
    println!("Metadata start {metadata_start} curr_offset:{}",cil_data.curr_offset());
    let mut method_bodys_len = metadata_start as usize - cil_data.curr_offset();
    let mut method_bodys = vec![0;method_bodys_len];
    cil_data.read_exact(&mut method_bodys);
    load_ops(&method_bodys);
    assert_eq!(method_bodys[method_bodys.len() - 1],0x2A);
    println!("Methods take {} bytes!",method_bodys_len);
    todo!();
}
fn load_ops(data:&[u8]){
    let mut index = 0;
    while index < data.len(){
        let curr = data[index];
        let op = match curr{
            _=>todo!("Unhandled op:0x{curr:x}"),
        };
        println!("op:{op:?}");
        index += 1;
    }
}
