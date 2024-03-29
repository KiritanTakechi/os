use alloc::{
    borrow::ToOwned,
    collections::{btree_map::Entry, BTreeMap},
    sync::Arc,
    vec::Vec,
};
use log::{info, warn};
use spin::{mutex::SpinMutex, once::Once};

use crate::{
    arch::mm::{mm_csr, PageTableEntry, PageTableFlags},
    config::{MEMORY_END, MMIO, PAGE_SIZE, TRAMPOLINE, TRAP_CONTEXT, USER_STACK_SIZE},
    error::Error,
    mm::{
        address::VirtPageNum,
        is_page_aligned,
        page_table::{PageTableEntryTrait, PageTableFlagsTrait},
    },
};

use super::{
    address::{PhysAddr, VirtAddr},
    frame::{VirtMemFrame, VirtMemReader, VirtMemWriter},
    option::VirtMemAllocOption,
    page_table::PageTable,
};

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn ebss();
    fn ekernel();
    fn strampoline();
}

pub static KERNEL_SPACE: Once<Arc<SpinMutex<MemorySet>>> = Once::new();

#[derive(Debug)]
pub struct MapArea {
    pub flags: PageTableFlags,
    pub start_va: VirtAddr,
    pub size: usize,
    pub map_type: MapType,
    pub mapper: BTreeMap<VirtAddr, VirtMemFrame>,
}

pub struct MemorySet {
    pub pt: PageTable<PageTableEntry>,
    areas: BTreeMap<VirtAddr, MapArea>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MapType {
    Identical,
    Framed,
}

impl Clone for MapArea {
    fn clone(&self) -> Self {
        let mut mapper = BTreeMap::new();
        for (&va, old) in &self.mapper {
            let new = VirtMemAllocOption::new(1).alloc_single().unwrap();
            new.copy_from_frame(old);
            mapper.insert(va, new.clone());
        }
        Self {
            start_va: self.start_va,
            size: self.size,
            flags: self.flags,
            map_type: self.map_type,
            mapper,
        }
    }
}

impl MapArea {
    pub fn mapped_size(&self) -> usize {
        self.size
    }

    pub fn new_with_frames(
        start_va: VirtAddr,
        size: usize,
        flags: PageTableFlags,
        map_type: MapType,
        physical_frames: Vec<VirtMemFrame>,
    ) -> Self {
        assert!(
            is_page_aligned(start_va.into())
                && is_page_aligned(size)
                && physical_frames.len() == (size / PAGE_SIZE)
        );

        println!(
            "mapping from {:#x?} to {:#x?}",
            start_va.0,
            start_va.0 + size
        );

        let mut map_area = Self {
            flags,
            start_va,
            size,
            map_type,
            mapper: BTreeMap::new(),
        };
        let mut current_va = start_va;
        let page_size = size / PAGE_SIZE;
        let mut phy_frame_iter = physical_frames.iter();

        for i in 0..page_size {
            let vm_frame = phy_frame_iter.next().unwrap();
            map_area.map_with_physical_address(current_va, vm_frame.clone());
            current_va.0 += PAGE_SIZE;
        }

        map_area
    }

    pub fn new(start_va: VirtAddr, size: usize, flags: PageTableFlags, map_type: MapType) -> Self {
        assert!(is_page_aligned(start_va.into()) && is_page_aligned(size));

        println!(
            "mapping from {:#x?} to {:#x?}",
            start_va.0,
            start_va.0 + size
        );

        Self {
            flags,
            start_va,
            size,
            map_type,
            mapper: BTreeMap::new(),
        }
    }

    pub fn map_with_physical_address(&mut self, va: VirtAddr, pa: VirtMemFrame) -> PhysAddr {
        assert!(is_page_aligned(va.into()));

        match self.mapper.entry(va) {
            Entry::Occupied(e) => panic!("already mapped a input physical address"),
            Entry::Vacant(e) => e.insert(pa).start_phys_addr(),
        }
    }

    pub fn map(&mut self, va: VirtAddr) -> PhysAddr {
        assert!(is_page_aligned(va.into()));

        match self.mapper.entry(va) {
            Entry::Occupied(e) => e.get().start_phys_addr(),
            Entry::Vacant(e) => e
                .insert(VirtMemAllocOption::new(1).alloc_single().unwrap())
                .start_phys_addr(),
        }
    }

    pub fn unmap(&mut self, va: VirtAddr) -> Option<VirtMemFrame> {
        self.mapper.remove(&va)
    }

    pub fn write_data(&mut self, addr: usize, data: &[u8]) {
        let mut current_start_address = addr;
        let mut buf_reader: VirtMemReader = data.into();
        info!("write_data: addr: {:#x?}", addr);
        for (va, pa) in self.mapper.iter() {
            if current_start_address >= va.0 && current_start_address < va.0 + PAGE_SIZE {
                let offset = current_start_address - va.0;
                let _ = pa.writer().skip(offset).write(&mut buf_reader);
                if !buf_reader.has_remain() {
                    return;
                }
                current_start_address = va.0 + PAGE_SIZE;
            }
        }
    }

    pub fn read_data(&self, addr: usize, data: &mut [u8]) {
        let mut start = addr;
        let mut buf_writer: VirtMemWriter = data.into();
        for (va, pa) in self.mapper.iter() {
            if start >= va.0 && start < va.0 + PAGE_SIZE {
                let offset = start - va.0;
                let _ = pa.reader().skip(offset).read(&mut buf_writer);
                if !buf_writer.has_avail() {
                    return;
                }
                start = va.0 + PAGE_SIZE;
            }
        }
    }
}

impl MemorySet {
    pub fn new() -> Self {
        let page_table = PageTable::<PageTableEntry>::new();

        Self {
            pt: page_table,
            areas: BTreeMap::new(),
        }
    }

    fn map_trampoline(&mut self) {
        self.pt
            .map(
                VirtAddr::from(TRAMPOLINE),
                PhysAddr::from(strampoline as usize),
                PageTableFlags::new()
                    .set_valid(true)
                    .set_readable(true)
                    .set_executable(true),
            )
            .unwrap()
    }

    pub fn new_kernel() -> Self {
        let mut memory_set: MemorySet = Self::new();

        let rflag = PageTableFlags::new()
            .set_valid(true)
            .set_readable(true)
            .set_valid(true);

        let rxflag = PageTableFlags::new()
            .set_valid(true)
            .set_readable(true)
            .set_executable(true)
            .set_valid(true);

        let rwflag = PageTableFlags::new()
            .set_valid(true)
            .set_readable(true)
            .set_writable(true)
            .set_valid(true);

        memory_set.map_trampoline();

        println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        println!(
            ".bss [{:#x}, {:#x})",
            sbss_with_stack as usize, ebss as usize
        );
        println!("mapping .text section");

        let text_area = MapArea::new(
            VirtAddr(stext as usize),
            etext as usize - stext as usize,
            rxflag,
            MapType::Identical,
        );

        println!(
            "text section mapped from {:#x?} to {:#x?}",
            stext as usize, etext as usize
        );

        memory_set.map(text_area);

        println!("mapping .rodata section");

        let rodata_area = MapArea::new(
            VirtAddr(srodata as usize),
            erodata as usize - srodata as usize,
            rflag,
            MapType::Identical,
        );

        println!(
            "rodata section mapped from {:#x?} to {:#x?}",
            srodata as usize, erodata as usize
        );

        memory_set.map(rodata_area);

        println!("mapping .data section");

        let data_area = MapArea::new(
            VirtAddr(sdata as usize),
            edata as usize - sdata as usize,
            rwflag,
            MapType::Identical,
        );

        println!(
            "data section mapped from {:#x?} to {:#x?}",
            sdata as usize, edata as usize
        );

        memory_set.map(data_area);

        println!("mapping .bss section");

        let bss_area = MapArea::new(
            VirtAddr(sbss_with_stack as usize),
            ebss as usize - sbss_with_stack as usize,
            rwflag,
            MapType::Identical,
        );

        println!(
            "bss section mapped from {:#x?} to {:#x?}",
            sbss_with_stack as usize, ebss as usize
        );

        memory_set.map(bss_area);

        println!("mapping physical memory");

        let mem_area = MapArea::new(
            VirtAddr(ekernel as usize),
            MEMORY_END - ekernel as usize,
            rwflag,
            MapType::Identical,
        );

        println!(
            "physical memory mapped from {:#x?} to {:#x?}",
            ekernel as usize, MEMORY_END
        );

        memory_set.map(mem_area);

        println!("mapping memory-mapped registers");

        for (start, size) in MMIO.iter() {
            let mmio_area = MapArea::new(VirtAddr(*start), *size, rwflag, MapType::Identical);

            memory_set.map(mmio_area);
        }

        memory_set
    }

    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize) {
        let mut memory_set = Self::new();
        // map trampoline
        memory_set.map_trampoline();
        // map program headers of elf, with U flag
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
        let ph_count = elf_header.pt2.ph_count();
        let mut max_end_vpn = VirtPageNum(0);

        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
                let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
                let mut flag = PageTableFlags::new()
                    .set_accessible_by_user(true)
                    .set_valid(true);
                let ph_flags = ph.flags();
                if ph_flags.is_read() {
                    flag.set_readable(true);
                }
                if ph_flags.is_write() {
                    flag.set_writable(true);
                }
                if ph_flags.is_execute() {
                    flag.set_executable(true);
                }

                let map_area = MapArea::new_with_frames(
                    start_va,
                    usize::from(end_va) - usize::from(start_va),
                    flag,
                    MapType::Framed,
                    VirtMemAllocOption::new(
                        (usize::from(end_va) - usize::from(start_va)) / PAGE_SIZE,
                    )
                    .alloc()
                    .unwrap(),
                );

                max_end_vpn = end_va.ceil();

                memory_set.map(map_area);

                memory_set
                    .write_bytes(
                        start_va.into(),
                        &elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize],
                    )
                    .unwrap();
            }
        }
        // map user stack with U flags
        let max_end_va: VirtAddr = max_end_vpn.into();
        let mut user_stack_bottom: usize = max_end_va.into();
        // guard page
        user_stack_bottom += PAGE_SIZE;
        let user_stack_top = user_stack_bottom + USER_STACK_SIZE;

        let user_stack_area = MapArea::new_with_frames(
            user_stack_bottom.into(),
            user_stack_top - user_stack_bottom,
            PageTableFlags::new()
                .set_accessible_by_user(true)
                .set_readable(true)
                .set_writable(true)
                .set_valid(true),
            MapType::Framed,
            VirtMemAllocOption::new((user_stack_top - user_stack_bottom) / PAGE_SIZE)
                .alloc()
                .unwrap(),
        );

        memory_set.map(user_stack_area);

        let trampoline_area = MapArea::new_with_frames(
            TRAP_CONTEXT.into(),
            TRAMPOLINE - TRAP_CONTEXT,
            PageTableFlags::new()
                .set_readable(true)
                .set_writable(true)
                .set_valid(true),
            MapType::Framed,
            VirtMemAllocOption::new((TRAMPOLINE - TRAP_CONTEXT) / PAGE_SIZE)
                .alloc()
                .unwrap(),
        );

        memory_set.map(trampoline_area);

        (
            memory_set,
            user_stack_top,
            elf.header.pt2.entry_point() as usize,
        )
    }

    pub fn map(&mut self, area: MapArea) {
        match area.map_type {
            MapType::Identical => {
                if area.size > 0 {
                    (area.start_va.0..area.start_va.0 + area.size)
                        .step_by(PAGE_SIZE)
                        .for_each(|va| {
                            //info!("mapping {:#x?}", va);
                            self.pt
                                .map(VirtAddr::from(va), PhysAddr::from(va), area.flags)
                                .unwrap();
                        });
                }
            }
            MapType::Framed => {
                if area.size > 0 {
                    if let Entry::Vacant(e) = self.areas.entry(area.start_va) {
                        let area = e.insert(area);
                        for (va, frame) in area.mapper.iter() {
                            self.pt
                                .map(*va, frame.start_phys_addr(), area.flags)
                                .unwrap();
                        }
                    } else {
                        panic!(
                            "MemorySet::map: MapArea starts from {:#x?} is existed!",
                            area.start_va
                        );
                    }
                }
            }
        }
    }

    pub fn is_mapped(&self, vaddr: VirtAddr) -> bool {
        for (start_address, map_area) in self.areas.iter() {
            if *start_address > vaddr {
                break;
            }
            if *start_address <= vaddr && vaddr.0 < start_address.0 + map_area.mapped_size() {
                return true;
            }
        }
        false
    }

    pub fn unmap(&mut self, va: VirtAddr) -> Result<(), crate::error::Error> {
        if let Some(area) = self.areas.remove(&va) {
            for (va, _) in area.mapper.iter() {
                self.pt.unmap(*va).unwrap();
            }
            Ok(())
        } else {
            Err(Error::PageFault)
        }
    }

    pub fn clear(&mut self) {
        for area in self.areas.values_mut() {
            for (va, _) in area.mapper.iter() {
                self.pt.unmap(*va).unwrap();
            }
        }
        self.areas.clear();
    }

    pub fn write_bytes(&mut self, addr: usize, data: &[u8]) -> Result<(), crate::error::Error> {
        let mut current_addr = addr;
        let mut remain = data.len();
        let start_write = false;
        let mut offset = 0usize;
        for (va, area) in self.areas.iter_mut() {
            if current_addr >= va.0 && current_addr < area.size + va.0 {
                if !area.flags.contains(PageTableFlags::Write) {
                    return Err(Error::PageFault);
                }
                let write_len = remain.min(area.size + va.0 - current_addr);
                area.write_data(current_addr, &data[offset..(offset + write_len)]);
                offset += write_len;
                remain -= write_len;
                // remain -= (va.0 + area.size - current_addr).min(remain);
                if remain == 0 {
                    return Ok(());
                }
                current_addr = va.0 + area.size;
            } else if start_write {
                return Err(Error::PageFault);
            }
        }
        Err(Error::PageFault)
    }

    pub fn read_bytes(&self, addr: usize, data: &mut [u8]) -> Result<(), crate::error::Error> {
        let mut current_addr = addr;
        let mut remain = data.len();
        let mut offset = 0usize;
        let start_read = false;
        for (va, area) in self.areas.iter() {
            if current_addr >= va.0 && current_addr < area.size + va.0 {
                let read_len = remain.min(area.size + va.0 - current_addr);
                area.read_data(current_addr, &mut data[offset..(offset + read_len)]);
                remain -= read_len;
                offset += read_len;
                // remain -= (va.0 + area.size - current_addr).min(remain);
                if remain == 0 {
                    return Ok(());
                }
                current_addr = va.0 + area.size;
            } else if start_read {
                return Err(Error::PageFault);
            }
        }
        Err(Error::PageFault)
    }

    pub fn token(&self) -> usize {
        self.pt.get_root_paddr().0
    }

}

impl Clone for MemorySet {
    fn clone(&self) -> Self {
        let mut ms = Self::new();
        for area in self.areas.values() {
            ms.map(area.clone());
        }
        ms
    }
}
impl Drop for MemorySet {
    fn drop(&mut self) {
        self.clear();
    }
}

pub fn init() {
    KERNEL_SPACE.call_once(|| Arc::new(SpinMutex::new(MemorySet::new_kernel())));
    let table = &mut KERNEL_SPACE.get().unwrap().lock().pt;

    //let tmp = table.translate(VirtAddr(0x80202000)).unwrap();

    //println!("entry::{:x}", tmp.phys_page_num().0 << 12);

    let addr = table.get_root_paddr();

    println!("kernel space initialing");

    info!("root_addr:0x{:x?}", addr.0);

    mm_csr(addr);

    println!("kernel space initialized");
}

#[allow(unused)]
pub fn remap_test() {
    let mut kernel_space = KERNEL_SPACE.get().unwrap().lock();
    let mid_text: VirtAddr = ((stext as usize + etext as usize) / 2).into();
    let mid_rodata: VirtAddr = ((srodata as usize + erodata as usize) / 2).into();
    let mid_data: VirtAddr = ((sdata as usize + edata as usize) / 2).into();
    assert!(!kernel_space
        .pt
        .translate(mid_text)
        .unwrap()
        .flags()
        .is_writable(),);
    assert!(!kernel_space
        .pt
        .translate(mid_rodata)
        .unwrap()
        .flags()
        .is_writable(),);
    assert!(!kernel_space
        .pt
        .translate(mid_data)
        .unwrap()
        .flags()
        .is_executable(),);
    println!("remap_test passed!");
}

#[allow(unused)]
pub fn write_test() {
    let mut set = MemorySet::new();
    let data = [1u8, 2, 3, 4, 5];
    let va = VirtAddr(0x100000000);
    let mut area = MapArea::new_with_frames(
        va,
        32 * PAGE_SIZE,
        PageTableFlags::new()
            .set_readable(true)
            .set_writable(true)
            .set_valid(true),
        MapType::Framed,
        VirtMemAllocOption::new(32).alloc().unwrap(),
    );

    set.map(area.to_owned());

    info!("{:?}", set.areas.len());

    set.write_bytes(va.into(), &data);

    let mut buf = [0u8; 5];

    set.read_bytes(va.into(), &mut buf);

    assert_eq!(data, buf);
}
