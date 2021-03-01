mod mmu {

    /// Describes a reference to a memory page. Each page should
    /// consist of 256 bytes. Each page may be readable, writable,
    /// or both.
    pub enum Memory<'a> {
        ReadOnly(&'a [u8]),
        WriteOnly(&'a mut [u8]),
        ReadWrite(&'a mut [u8]),
    }

    /// Maximum number of memory pages.
    pub const PAGE_COUNT: usize = 256;

    /// Type alias for array of memory page specifications.
    pub type PageArray<'a> = [Option<Memory<'a>>; PAGE_COUNT];

}
