use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures_core::Stream;
use tower_service::Service;

use super::types::{LlmRequest, LlmResponse};
use crate::client::{BoxFuture, LlmClient};
use crate::error::{LiterLmError, Result};
use crate::types::ChatCompletionChunk;

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
                    // Collect the stream into a Vec while the Arc-backed client is
                    // alive.  This avoids the unsound transmute that would otherwise
                    // be needed to extend the stream's borrow lifetime to 'static.
                    // The cost is that streaming chunks are buffered before being
                    // yielded; this is acceptable because tower middleware cannot
                    // express borrowed lifetimes across the Service boundary.
                    let stream = client.chat_stream(r).await?;
                    let chunks = collect_stream(stream).await?;
                    let static_stream: crate::client::BoxStream<'static, ChatCompletionChunk> =
                        Box::pin(OwnedChunksStream { chunks, index: 0 });
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

/// Collect all items from a stream into a `Vec`, stopping on the first error.
async fn collect_stream<'a>(
    mut stream: crate::client::BoxStream<'a, ChatCompletionChunk>,
) -> Result<Vec<ChatCompletionChunk>> {
    let mut chunks = Vec::new();
    loop {
        // Drive the stream by polling it inside a future::poll_fn.
        let item = std::future::poll_fn(|cx| Pin::as_mut(&mut stream).poll_next(cx)).await;
        match item {
            Some(Ok(chunk)) => chunks.push(chunk),
            Some(Err(e)) => return Err(e),
            None => break,
        }
    }
    Ok(chunks)
}

/// A `Stream` that yields items from an owned `Vec` in order.
///
/// Used to wrap collected streaming chunks so they can be returned as a
/// `BoxStream<'static, ...>` without any lifetime dependencies.
struct OwnedChunksStream {
    chunks: Vec<ChatCompletionChunk>,
    index: usize,
}

impl Stream for OwnedChunksStream {
    type Item = Result<ChatCompletionChunk>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.index < self.chunks.len() {
            let chunk = self.chunks[self.index].clone();
            self.index += 1;
            Poll::Ready(Some(Ok(chunk)))
        } else {
            Poll::Ready(None)
        }
    }
}
