use my_azure_storage_sdk::{
    page_blob::{consts::BLOB_PAGE_SIZE, AzurePageBlobStorage, AzurePageBlobStorageExts},
    AzureStorageError,
};

use crate::{PageBlobRandomAccessError, RandomAccessPageOffsets, ReadChunk};

pub struct PageBlobRandomAccess {
    pub page_blob: AzurePageBlobStorage,
    page_blob_size: Option<usize>,
    auto_resize_on_write: bool,
    max_pages_to_write_per_request: usize,
}

impl PageBlobRandomAccess {
    pub fn new(
        page_blob: AzurePageBlobStorage,
        auto_resize_on_write: bool,
        max_pages_to_write_per_request: usize,
    ) -> Self {
        Self {
            page_blob,
            page_blob_size: None,
            auto_resize_on_write,
            max_pages_to_write_per_request,
        }
    }

    pub async fn get_blob_size(&mut self) -> Result<usize, AzureStorageError> {
        if self.page_blob_size.is_some() {
            return Ok(self.page_blob_size.unwrap());
        }

        let props = self.page_blob.get_blob_properties().await?;

        self.page_blob_size = Some(props.blob_size);

        return Ok(props.blob_size);
    }

    async fn resize(&mut self, pages_amount: usize) -> Result<(), AzureStorageError> {
        self.page_blob.resize(pages_amount).await?;

        self.page_blob_size = Some(pages_amount * BLOB_PAGE_SIZE);

        Ok(())
    }

    pub async fn read(
        &mut self,
        start_pos: usize,
        size: usize,
    ) -> Result<ReadChunk, PageBlobRandomAccessError> {
        let available_pages_amount = self.get_blob_size().await? / BLOB_PAGE_SIZE;

        let offsets = RandomAccessPageOffsets::new(start_pos, size);

        let required_pages_amount = offsets.get_required_blob_pages_amount();

        if required_pages_amount > available_pages_amount {
            let msg = format!(
                "To perform read operation required blob pages amount is {}. Blob has pages: {}",
                required_pages_amount, available_pages_amount
            );
            return Err(PageBlobRandomAccessError::IndexRangeViolation(msg));
        }

        let result = self
            .page_blob
            .get_pages(offsets.start_page_no, offsets.total_pages_amount_to_read)
            .await?;

        Ok(ReadChunk::new(offsets.offset_in_page, size, result))
    }

    pub async fn write(
        &mut self,
        start_pos: usize,
        payload: &[u8],
    ) -> Result<(), PageBlobRandomAccessError> {
        let available_pages_amount = self.get_blob_size().await? / BLOB_PAGE_SIZE;

        let offsets = RandomAccessPageOffsets::new(start_pos, payload.len());

        let required_pages_amount = offsets.get_required_blob_pages_amount();

        if available_pages_amount < required_pages_amount {
            if self.auto_resize_on_write {
                self.resize(required_pages_amount).await?;
            }
        }

        let mut payload_from_blob = self
            .page_blob
            .get_pages(offsets.start_page_no, offsets.total_pages_amount_to_read)
            .await?;

        let payload_to_copy =
            &mut payload_from_blob[offsets.offset_in_page..offsets.offset_in_page + payload.len()];

        payload_to_copy.copy_from_slice(payload);

        self.page_blob
            .save_pages_ext(
                offsets.start_page_no,
                payload_from_blob,
                self.max_pages_to_write_per_request,
            )
            .await?;

        Ok(())
    }

    pub async fn create_new(&mut self, pages: usize) -> Result<(), AzureStorageError> {
        self.page_blob.create_if_not_exists(pages).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use my_azure_storage_sdk::AzureStorageConnection;

    use super::*;

    #[tokio::test]
    async fn test_basic_cases() {
        let connection = AzureStorageConnection::new_in_memory();

        let page_blob = AzurePageBlobStorage::new(Arc::new(connection), "Test", "Test").await;
        page_blob.create_container_if_not_exist().await.unwrap();
        page_blob.create_if_not_exists(0).await.unwrap();

        let mut random_access = PageBlobRandomAccess::new(page_blob, true, 10);

        random_access
            .write(3, vec![1u8, 2u8, 3u8].as_slice())
            .await
            .unwrap();

        let result = random_access.page_blob.download().await.unwrap();

        assert_eq!(vec![0u8, 0u8, 0u8, 1u8, 2u8, 3u8, 0u8, 0u8], result[0..8]);

        let result = random_access.read(3, 4).await.unwrap();

        assert_eq!(vec![1u8, 2u8, 3u8, 0u8], result.as_slice());
    }
}
