use my_azure_storage_sdk::page_blob::consts::BLOB_PAGE_SIZE;

pub struct RandomAccessPageOffsets {
    pub start_page_no: usize,
    pub offset_in_page: usize,
    pub total_pages_amount_to_read: usize,
}

impl RandomAccessPageOffsets {
    pub fn new(start_pos: usize, data_size: usize) -> Self {
        let start_page_no = start_pos / BLOB_PAGE_SIZE;

        let offset_in_page = start_pos - start_page_no * BLOB_PAGE_SIZE;

        let total_pages_amount_to_read =
            get_pages_amount_by_size(offset_in_page + data_size, BLOB_PAGE_SIZE);

        Self {
            start_page_no,
            offset_in_page,
            total_pages_amount_to_read,
        }
    }

    pub fn get_total_size_to_read(&self) -> usize {
        self.total_pages_amount_to_read * BLOB_PAGE_SIZE
    }

    pub fn get_required_blob_pages_amount(&self) -> usize {
        self.start_page_no + self.total_pages_amount_to_read
    }
}

pub fn get_pages_amount_by_size(data_size: usize, page_size: usize) -> usize {
    if data_size == 0 {
        return 0;
    }
    return (data_size - 1) / page_size + 1;
}

#[cfg(test)]
mod tests {
    use my_azure_storage_sdk::page_blob::consts::BLOB_PAGE_SIZE;

    use super::RandomAccessPageOffsets;

    #[test]
    fn test_first_page_2_bytes_in_the_middle() {
        let page_data = RandomAccessPageOffsets::new(2, 2);

        assert_eq!(page_data.offset_in_page, 2);
        assert_eq!(page_data.get_total_size_to_read(), BLOB_PAGE_SIZE);
        assert_eq!(page_data.total_pages_amount_to_read, 1);
        assert_eq!(page_data.start_page_no, 0);
        assert_eq!(page_data.get_required_blob_pages_amount(), 1);
    }

    #[test]
    fn test_second_page_2_bytes_in_the_middle() {
        let page_data = RandomAccessPageOffsets::new(2 + BLOB_PAGE_SIZE, 2);

        assert_eq!(page_data.offset_in_page, 2);
        assert_eq!(page_data.get_total_size_to_read(), BLOB_PAGE_SIZE);
        assert_eq!(page_data.total_pages_amount_to_read, 1);
        assert_eq!(page_data.start_page_no, 1);
        assert_eq!(page_data.get_required_blob_pages_amount(), 2);
    }

    #[test]
    fn test_whole_first_page() {
        let page_data = RandomAccessPageOffsets::new(0, BLOB_PAGE_SIZE);

        assert_eq!(page_data.offset_in_page, 0);
        assert_eq!(page_data.get_total_size_to_read(), BLOB_PAGE_SIZE);
        assert_eq!(page_data.total_pages_amount_to_read, 1);
        assert_eq!(page_data.start_page_no, 0);
        assert_eq!(page_data.get_required_blob_pages_amount(), 1);
    }

    #[test]
    fn test_read_from_middle_of_the_page_which_takes_two_pages() {
        let page_data = RandomAccessPageOffsets::new(BLOB_PAGE_SIZE + 4, BLOB_PAGE_SIZE);

        assert_eq!(page_data.offset_in_page, 4);
        assert_eq!(page_data.get_total_size_to_read(), BLOB_PAGE_SIZE * 2);
        assert_eq!(page_data.total_pages_amount_to_read, 2);
        assert_eq!(page_data.start_page_no, 1);
        assert_eq!(page_data.get_required_blob_pages_amount(), 3);
    }
}
