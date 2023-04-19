use my_azure_storage_sdk::{
    page_blob::{AzurePageBlobStorage, PageBlobAbstractions},
    AzureStorageError,
};
use rust_extensions::AsSliceOrVec;
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

    pub async fn write<'s>(
        &self,
        start_pos: usize,
        payload: impl Into<AsSliceOrVec<'s, u8>>,
    ) -> Result<(), PageBlobRandomAccessError> {
        let payload = payload.into();
        let mut write_access = self.inner.lock().await;
        write_access.write(start_pos, payload.as_slice()).await
    }

    pub async fn create_blob_if_not_exists(
        &self,
        init_pages_amount: usize,
    ) -> Result<usize, AzureStorageError> {
        let mut write_access = self.inner.lock().await;
        write_access.create_new(init_pages_amount).await
    }

    pub async fn create_container_if_not_exists(&self) -> Result<(), AzureStorageError> {
        let access = self.inner.lock().await;
        access.page_blob.create_container_if_not_exists().await
    }

    pub async fn resize(&self, pages_amount: usize) -> Result<(), AzureStorageError> {
        let mut write_access = self.inner.lock().await;
        write_access.resize(pages_amount).await
    }

    pub async fn download(&self) -> Result<Vec<u8>, AzureStorageError> {
        let access = self.inner.lock().await;
        access.page_blob.download().await
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
