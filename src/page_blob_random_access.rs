use my_azure_storage_sdk::{
    page_blob::{MyAzurePageBlobStorage, PageBlobAbstractions, PageBlobProperties},
    AzureStorageError,
};
use rust_extensions::AsSliceOrVec;

use crate::{PageBlobRandomAccessError, RandomAccessPageOffsets, ReadChunk};

pub struct PageBlobRandomAccess<
    TMyAzurePageBlobStorage: MyAzurePageBlobStorage + Send + Sync + 'static,
> {
    page_blob: TMyAzurePageBlobStorage,
    auto_resize_on_write: bool,
    max_pages_to_write_per_request: usize,
}

impl<TMyAzurePageBlobStorage: MyAzurePageBlobStorage + Send + Sync + 'static>
    PageBlobRandomAccess<TMyAzurePageBlobStorage>
{
    pub fn new(
        page_blob: TMyAzurePageBlobStorage,
        auto_resize_on_write: bool,
        max_pages_to_write_per_request: usize,
    ) -> Self {
        Self {
            page_blob,
            auto_resize_on_write,
            max_pages_to_write_per_request,
        }
    }

    pub async fn get_blob_properties(&self) -> Result<PageBlobProperties, AzureStorageError> {
        self.page_blob.get_blob_properties().await
    }

    pub async fn resize(&self, pages_amount: usize) -> Result<(), AzureStorageError> {
        self.page_blob.resize(pages_amount).await
    }

    pub async fn read(
        &self,
        start_pos: usize,
        size: usize,
    ) -> Result<ReadChunk, PageBlobRandomAccessError> {
        let blob_properties = self.get_blob_properties().await?;

        let offsets = RandomAccessPageOffsets::new(start_pos, size);

        let required_pages_amount = offsets.get_required_blob_pages_amount();

        let available_pages_amount = blob_properties.get_pages_amount();

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

    pub async fn write<'s>(
        &self,
        start_pos: usize,
        payload: impl Into<AsSliceOrVec<'s, u8>>,
    ) -> Result<(), PageBlobRandomAccessError> {
        let blob_props = self.get_blob_properties().await?;

        let available_pages_amount = blob_props.get_pages_amount();

        let payload: AsSliceOrVec<'s, u8> = payload.into();

        let payload = payload.as_slice();

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

    pub async fn create_blob_if_not_exists(
        &self,
        init_pages_amount: usize,
        auto_create_container: bool,
    ) -> Result<PageBlobProperties, AzureStorageError> {
        self.page_blob
            .create_if_not_exists(init_pages_amount, auto_create_container)
            .await
    }

    pub async fn create_container_if_not_exists(&self) -> Result<(), AzureStorageError> {
        self.page_blob.create_container_if_not_exists().await
    }

    pub async fn download(&self) -> Result<Vec<u8>, AzureStorageError> {
        self.page_blob.download().await
    }
}

#[async_trait::async_trait]
impl<TMyAzurePageBlobStorage: MyAzurePageBlobStorage + Send + Sync + 'static> PageBlobAbstractions
    for PageBlobRandomAccess<TMyAzurePageBlobStorage>
{
    async fn create_container_if_not_exists(&self) -> Result<(), AzureStorageError> {
        self.create_container_if_not_exists().await
    }
    async fn create_blob_if_not_exists(
        &self,
        init_pages_amounts: usize,
        auto_create_container: bool,
    ) -> Result<PageBlobProperties, AzureStorageError> {
        self.create_blob_if_not_exists(init_pages_amounts, auto_create_container)
            .await
    }
}

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use my_azure_storage_sdk::{page_blob::AzurePageBlobStorage, AzureStorageConnection};

    use super::*;

    #[tokio::test]
    async fn test_basic_cases() {
        let connection = AzureStorageConnection::new_in_memory();

        let page_blob = AzurePageBlobStorage::new(Arc::new(connection), "Test", "Test").await;
        page_blob.create_if_not_exists(0, true).await.unwrap();

        let random_access = PageBlobRandomAccess::new(page_blob, true, 10);

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
