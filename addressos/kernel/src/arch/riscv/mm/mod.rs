use bitflags::bitflags;

bitflags! {
    #[repr(C)]
    pub(crate) struct PageTableFlags: u8 {
        /// 使能位
        const Valid = 1 << 0;
        /// 可读
        const Read = 1 << 1;
        /// 可写
        const Write = 1 << 2;
        /// 可执行
        const Execute = 1 << 3;
        /// 可用户模式下访问
        const User = 1 << 4;
        /// 全局位
        const Global = 1 << 5;
        /// 访问记录
        const Accessed = 1 << 6;
        /// 修改记录
        const Dirty = 1 << 7;
    }
}