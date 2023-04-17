use my_azure_storage_sdk::{
    page_blob::{AzurePageBlobStorage, PageBlobAbstractions},
    AzureStorageError,
};
use tokio::sync::Mutex;

use crate::{PageBlobRandomAccessError, PageBlobRandomAccessInner, ReadChunk};

pub struct PageBlobRandomAccess {
    inner: Mutex<PageBlobRandomAccessInner>,
}

impl PageBlobRandomAccess {
    pub fn new(
        page_blob: AzurePageBlobStorage,
        auto_resize_on_write: bool,
        max_pages_to_write_per_request: usize,
    ) -> Self {
        Self {
            inner: Mutex::new(PageBlobRandomAccessInner::new(
                page_blob,
                auto_resize_on_write,
                max_pages_to_write_per_request,
            )),
        }
    }

    pub async fn get_blob_size(&self) -> Result<usize, AzureStorageError> {
        let mut write_access = self.inner.lock().await;
        write_access.get_blob_size().await
    }

    pub async fn read(
        &self,
        start_pos: usize,
        size: usize,
    ) -> Result<ReadChunk, PageBlobRandomAccessError> {
        let mut write_access = self.inner.lock().await;
        write_access.read(start_pos, size).await
    }

    pub async fn write(
        &self,
        start_pos: usize,
        payload: &[u8],
    ) -> Result<(), PageBlobRandomAccessError> {
        let mut write_access = self.inner.lock().await;
        write_access.write(start_pos, payload).await
    }

    pub async fn create_blob_if_not_exists(
        &self,
        init_pages_amount: usize,
    ) -> Result<usize, AzureStorageError> {
        let mut write_access = self.inner.lock().await;
        write_access.create_new(init_pages_amount).await
    }

    pub async fn create_container_if_not_exists(&self) -> Result<(), AzureStorageError> {
        let write_access: tokio::sync::MutexGuard<PageBlobRandomAccessInner> =
            self.inner.lock().await;
        write_access
            .page_blob
            .create_container_if_not_exists()
            .await
    }
}

#[async_trait::async_trait]
impl PageBlobAbstractions for PageBlobRandomAccess {
    async fn create_container_if_not_exists(&self) -> Result<(), AzureStorageError> {
        self.create_container_if_not_exists().await
    }
    async fn create_blob_if_not_exists(
        &self,
        init_pages_amounts: usize,
    ) -> Result<usize, AzureStorageError> {
        self.create_blob_if_not_exists(init_pages_amounts).await
    }
}
