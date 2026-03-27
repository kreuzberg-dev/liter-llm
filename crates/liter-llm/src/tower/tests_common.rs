//! Shared test helpers for tower middleware tests.
//!
//! This module provides a [`MockClient`] and helper functions used across
//! multiple middleware test modules.
#![allow(dead_code)]

use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll};

use futures_core::Stream;

use crate::client::{BoxFuture, BoxStream, LlmClient};
use crate::error::{LiterLlmError, Result};
use crate::types::audio::{CreateSpeechRequest, CreateTranscriptionRequest, TranscriptionResponse};
use crate::types::image::{CreateImageRequest, ImagesResponse};
use crate::types::moderation::{ModerationRequest, ModerationResponse};
use crate::types::rerank::{RerankRequest, RerankResponse};
use crate::types::{
    AssistantMessage, ChatCompletionChunk, ChatCompletionRequest, ChatCompletionResponse, Choice, EmbeddingObject,
    EmbeddingRequest, EmbeddingResponse, FinishReason, Message, ModelsListResponse, SystemMessage, Usage,
};

/// A stream that yields no items.
pub struct EmptyStream;

impl Stream for EmptyStream {
    type Item = Result<ChatCompletionChunk>;
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(None)
    }
}

/// A serializable subset of [`LiterLlmError`] variants used in tests.
/// `LiterLlmError` is not `Clone`, so we store an enum of the variants we care about.
pub enum LiterLlmErrorKind {
    RateLimited { message: String },
    ServiceUnavailable { message: String },
    Timeout,
    Authentication { message: String },
}

impl LiterLlmErrorKind {
    pub fn into_error(&self) -> LiterLlmError {
        match self {
            Self::RateLimited { message } => LiterLlmError::RateLimited {
                message: message.clone(),
                retry_after: None,
            },
            Self::ServiceUnavailable { message } => LiterLlmError::ServiceUnavailable {
                message: message.clone(),
            },
            Self::Timeout => LiterLlmError::Timeout,
            Self::Authentication { message } => LiterLlmError::BadRequest {
                message: message.clone(),
            },
        }
    }
}

/// A synchronous mock client. All methods return configurable canned
/// responses or errors.
#[derive(Clone)]
pub struct MockClient {
    /// When set, `chat` returns this error instead of the canned response.
    chat_error: Option<Arc<LiterLlmErrorKind>>,
    /// Number of times `chat` / `chat_stream` has been called.
    pub call_count: Arc<AtomicUsize>,
}

pub fn make_chat_response(model: &str) -> ChatCompletionResponse {
    ChatCompletionResponse {
        id: "test-id".into(),
        object: "chat.completion".into(),
        created: 0,
        model: model.into(),
        choices: vec![Choice {
            index: 0,
            message: AssistantMessage {
                content: Some("Hello!".into()),
                name: None,
                tool_calls: None,
                refusal: None,
                function_call: None,
            },
            finish_reason: Some(FinishReason::Stop),
        }],
        usage: Some(Usage {
            prompt_tokens: 10,
            completion_tokens: 5,
            total_tokens: 15,
        }),
        system_fingerprint: None,
        service_tier: None,
    }
}

impl MockClient {
    fn new_with_error(error: Option<LiterLlmErrorKind>) -> Self {
        Self {
            chat_error: error.map(Arc::new),
            call_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn ok() -> Self {
        Self::new_with_error(None)
    }

    pub fn failing_rate_limited() -> Self {
        Self::new_with_error(Some(LiterLlmErrorKind::RateLimited {
            message: "too many requests".into(),
        }))
    }

    pub fn failing_service_unavailable() -> Self {
        Self::new_with_error(Some(LiterLlmErrorKind::ServiceUnavailable { message: "503".into() }))
    }

    pub fn failing_auth() -> Self {
        Self::new_with_error(Some(LiterLlmErrorKind::Authentication {
            message: "invalid key".into(),
        }))
    }

    pub fn failing_timeout() -> Self {
        Self::new_with_error(Some(LiterLlmErrorKind::Timeout))
    }
}

impl LlmClient for MockClient {
    fn chat(&self, req: ChatCompletionRequest) -> BoxFuture<'_, ChatCompletionResponse> {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        let result = match &self.chat_error {
            Some(kind) => Err(kind.into_error()),
            None => Ok(make_chat_response(&req.model)),
        };
        Box::pin(async move { result })
    }

    fn chat_stream(&self, _req: ChatCompletionRequest) -> BoxFuture<'_, BoxStream<'_, ChatCompletionChunk>> {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        Box::pin(async move {
            let stream: BoxStream<'_, ChatCompletionChunk> = Box::pin(EmptyStream);
            Ok(stream)
        })
    }

    fn embed(&self, req: EmbeddingRequest) -> BoxFuture<'_, EmbeddingResponse> {
        let resp = EmbeddingResponse {
            object: "list".into(),
            data: vec![EmbeddingObject {
                object: "embedding".into(),
                embedding: vec![0.1, 0.2, 0.3],
                index: 0,
            }],
            model: req.model.clone(),
            usage: Some(Usage {
                prompt_tokens: 4,
                completion_tokens: 0,
                total_tokens: 4,
            }),
        };
        Box::pin(async move { Ok(resp) })
    }

    fn list_models(&self) -> BoxFuture<'_, ModelsListResponse> {
        Box::pin(async move {
            Ok(ModelsListResponse {
                object: "list".into(),
                data: vec![],
            })
        })
    }

    fn image_generate(&self, _req: CreateImageRequest) -> BoxFuture<'_, ImagesResponse> {
        Box::pin(async move {
            Ok(ImagesResponse {
                created: 0,
                data: vec![],
            })
        })
    }

    fn speech(&self, _req: CreateSpeechRequest) -> BoxFuture<'_, bytes::Bytes> {
        Box::pin(async move { Ok(bytes::Bytes::new()) })
    }

    fn transcribe(&self, _req: CreateTranscriptionRequest) -> BoxFuture<'_, TranscriptionResponse> {
        Box::pin(async move {
            Ok(TranscriptionResponse {
                text: String::new(),
                language: None,
                duration: None,
                segments: None,
            })
        })
    }

    fn moderate(&self, _req: ModerationRequest) -> BoxFuture<'_, ModerationResponse> {
        Box::pin(async move {
            Ok(ModerationResponse {
                id: String::new(),
                model: String::new(),
                results: vec![],
            })
        })
    }

    fn rerank(&self, _req: RerankRequest) -> BoxFuture<'_, RerankResponse> {
        Box::pin(async move {
            Ok(RerankResponse {
                id: None,
                results: vec![],
                meta: None,
            })
        })
    }
}

/// Build a [`ChatCompletionRequest`] with the given model name.
pub fn chat_req(model: &str) -> ChatCompletionRequest {
    ChatCompletionRequest {
        model: model.into(),
        messages: vec![Message::System(SystemMessage {
            content: "test".into(),
            name: None,
        })],
        ..Default::default()
    }
}
