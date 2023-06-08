#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum PageSize {
    NORMAL = 1 << 12,
    LARGE = 1 << 21,
}
