#[derive(Debug)]
pub struct Pagination {
    pub limit: Option<u32>,
    pub offset: u32,
}
