use std::sync::Arc;
use std::task::{Context, Poll};

use tower_service::Service;

use super::types::{LlmRequest, LlmResponse};
use crate::client::{BoxFuture, LlmClient};
use crate::error::{LiterLmError, Result};

/// A thin tower [`Service`] wrapper around any [`LlmClient`] implementation.
///
/// Because [`LlmClient`] methods take `&self`, the inner client is stored
/// behind an [`Arc`] so the service can be cloned without owning a unique
/// reference.  `tower::Service::call` takes `&mut self`, but the actual
/// async work is dispatched through the shared reference inside the arc.
pub struct LlmService<C> {
    inner: Arc<C>,
}

impl<C> LlmService<C> {
    /// Wrap `client` in a tower-compatible service.
    pub fn new(client: C) -> Self {
        Self {
            inner: Arc::new(client),
        }
    }

    /// Return a reference to the inner client.
    pub fn inner(&self) -> &C {
        &self.inner
    }
}

impl<C: Clone> Clone for LlmService<C> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<C> Service<LlmRequest> for LlmService<C>
where
    C: LlmClient + Send + Sync + 'static,
{
    type Response = LlmResponse;
    type Error = LiterLmError;
    type Future = BoxFuture<'static, LlmResponse>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: LlmRequest) -> Self::Future {
        let client = Arc::clone(&self.inner);
        Box::pin(async move {
            match req {
                LlmRequest::Chat(r) => {
                    let resp = client.chat(r).await?;
                    Ok(LlmResponse::Chat(resp))
                }
                LlmRequest::ChatStream(r) => {
                    let stream = client.chat_stream(r).await?;
                    // SAFETY: `stream` is a `BoxStream<'_, ChatCompletionChunk>` where
                    // `'_` is the borrow lifetime tied to `client` (via Arc).  The
                    // stream is heap-allocated (`Pin<Box<dyn Stream + Send>>`); once
                    // the `chat_stream` future resolves the stream is fully
                    // self-contained on the heap.  We keep the Arc (`client`) alive
                    // until after the transmute, ensuring any heap data the stream
                    // might hold through the Arc remains valid for the `'static`
                    // lifetime we assert here.
                    let static_stream: crate::client::BoxStream<'static, crate::types::ChatCompletionChunk> =
                        unsafe { std::mem::transmute(stream) };
                    drop(client);
                    Ok(LlmResponse::ChatStream(static_stream))
                }
                LlmRequest::Embed(r) => {
                    let resp = client.embed(r).await?;
                    Ok(LlmResponse::Embed(resp))
                }
                LlmRequest::ListModels => {
                    let resp = client.list_models().await?;
                    Ok(LlmResponse::ListModels(resp))
                }
            }
        })
    }
}
