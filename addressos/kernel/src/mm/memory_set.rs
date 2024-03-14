use alloc::{
    collections::{btree_map::Entry, BTreeMap},
    sync::Arc,
    vec::Vec,
};
use spin::{mutex::SpinMutex, once::Once};

use crate::{
    arch::mm::{PageTableEntry, PageTableFlags},
    config::{MEMORY_END, MMIO, PAGE_SIZE, TRAMPOLINE},
    error::Error,
    mm::{
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

    pub fn new(
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

        println!("mapping from {:#x?} to {:#x?}", start_va.0, start_va.0 + size);

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

    pub fn new_kernel() -> Self {
        let mut memory_set: MemorySet = Self::new();

        let rflag = PageTableFlags::new().set_valid(true).set_readable(true);

        let rxflag = PageTableFlags::new()
            .set_valid(true)
            .set_readable(true)
            .set_executable(true);

        let rwflag = PageTableFlags::new()
            .set_valid(true)
            .set_readable(true)
            .set_writable(true);

        memory_set
            .pt
            .map(VirtAddr(TRAMPOLINE), PhysAddr(strampoline as usize), rxflag)
            .unwrap();

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
            VirtMemAllocOption::new((etext as usize - stext as usize) / PAGE_SIZE)
                .alloc()
                .unwrap(),
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
            VirtMemAllocOption::new((erodata as usize - srodata as usize) / PAGE_SIZE)
                .alloc()
                .unwrap(),
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
            VirtMemAllocOption::new((edata as usize - sdata as usize) / PAGE_SIZE)
                .alloc()
                .unwrap(),
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
            VirtMemAllocOption::new((ebss as usize - sbss_with_stack as usize) / PAGE_SIZE)
                .alloc()
                .unwrap(),
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
            VirtMemAllocOption::new((MEMORY_END - ekernel as usize) / PAGE_SIZE)
                .alloc()
                .unwrap(),
        );

        println!(
            "physical memory mapped from {:#x?} to {:#x?}",
            ekernel as usize, MEMORY_END
        );

        memory_set.map(mem_area);

        println!("mapping memory-mapped registers");

        for (start, end) in MMIO.iter() {
            let mmio_area = MapArea::new(
                VirtAddr(*start),
                *end - *start,
                rwflag,
                MapType::Identical,
                VirtMemAllocOption::new((*end - *start) / PAGE_SIZE)
                    .alloc()
                    .unwrap(),
            );

            memory_set.map(mmio_area);
        }

        println!("kernel space initialized");

        memory_set
    }

    pub fn new_elf() -> Self {
        todo!()
    }

    pub fn map(&mut self, area: MapArea) {
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
}

pub fn init() {
    KERNEL_SPACE.call_once(|| Arc::new(SpinMutex::new(MemorySet::new_kernel())));
    let addr = KERNEL_SPACE.get().unwrap().lock().pt.get_root_paddr();
    //mm_csr(addr);
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
