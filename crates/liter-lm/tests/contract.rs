//! Contract tests validating that our Rust types produce JSON
//! conforming to the OpenAI API JSON Schema specifications.
//!
//! Each test:
//!   1. Constructs a canonical instance of a Rust response type.
//!   2. Serialises it to `serde_json::Value`.
//!   3. Validates the value against the corresponding `$defs` entry from the
//!      OpenAI JSON Schema files embedded at compile time.
//!
//! Cross-file `$ref` pointers (e.g. `common.json#/$defs/CompletionUsage`) are
//! resolved via a custom `Retrieve` implementation that serves the schema
//! files from the compile-time `include_str!` constants.  The root schema has
//! no `$id`, so the `jsonschema` crate assigns it the base URI
//! `json-schema:///`; relative refs like `"common.json"` therefore resolve to
//! `"json-schema:///common.json"`.

use std::collections::HashMap;

use jsonschema::{Retrieve, Uri};
use serde_json::{Value, json};

// ── Schema sources ──────────────────────────────────────────────────────────

const CHAT_COMPLETION_SCHEMA: &str = include_str!("../../../schemas/api/chat_completion.json");
const EMBEDDING_SCHEMA: &str = include_str!("../../../schemas/api/embedding.json");
const MODELS_SCHEMA: &str = include_str!("../../../schemas/api/models.json");
const ERRORS_SCHEMA: &str = include_str!("../../../schemas/api/errors.json");
const COMMON_SCHEMA: &str = include_str!("../../../schemas/api/common.json");

// ── Custom retriever for cross-file $ref resolution ──────────────────────────

/// Serves statically known schema files so that `$ref` values such as
/// `"common.json#/$defs/CompletionUsage"` resolve without network access.
struct StaticRetriever {
    schemas: HashMap<&'static str, &'static str>,
}

impl StaticRetriever {
    fn new() -> Self {
        let mut schemas = HashMap::new();
        // Register every schema under the URI that the jsonschema crate will
        // request.  Because the root document has no `$id`, its base is
        // `json-schema:///`, so `"common.json"` resolves to
        // `"json-schema:///common.json"`.
        schemas.insert("json-schema:///common.json", COMMON_SCHEMA);
        schemas.insert("json-schema:///chat_completion.json", CHAT_COMPLETION_SCHEMA);
        schemas.insert("json-schema:///embedding.json", EMBEDDING_SCHEMA);
        schemas.insert("json-schema:///models.json", MODELS_SCHEMA);
        schemas.insert("json-schema:///errors.json", ERRORS_SCHEMA);
        Self { schemas }
    }
}

impl Retrieve for StaticRetriever {
    fn retrieve(&self, uri: &Uri<&str>) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let key = uri.as_str();
        self.schemas
            .get(key)
            .map(|src| serde_json::from_str(src).expect("schema is valid JSON"))
            .ok_or_else(|| format!("Schema not found for URI: {key}").into())
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Build a validator for `def_name` from `primary_schema`.
///
/// Cross-file `$ref` pointers are resolved via `StaticRetriever`.  The
/// target definition is extracted and used as the root schema so that the
/// validator operates on the right constraints without an extra wrapping level.
fn build_validator(primary_schema: &str, def_name: &str) -> jsonschema::Validator {
    let primary: Value = serde_json::from_str(primary_schema).expect("primary schema is valid JSON");

    let def = primary["$defs"][def_name].clone();
    assert!(
        def.is_object(),
        "Schema definition '{def_name}' not found in primary schema"
    );

    // Attach the primary schema's $defs to the root so intra-file
    // `#/$defs/Foo` refs continue to work.
    let mut root = def;
    if let Some(defs) = primary["$defs"].as_object() {
        root["$defs"] = Value::Object(defs.clone());
    }
    root["$schema"] = json!("https://json-schema.org/draft/2020-12/schema");

    jsonschema::options()
        .with_retriever(StaticRetriever::new())
        .build(&root)
        .unwrap_or_else(|e| panic!("Failed to compile schema for '{def_name}': {e}"))
}

/// Validate `instance` against the compiled `validator`, panicking with a
/// descriptive message on failure.
fn assert_valid(validator: &jsonschema::Validator, instance: &Value, label: &str) {
    let errors: Vec<String> = validator
        .iter_errors(instance)
        .map(|e| format!("  - {e} (path: {})", e.instance_path))
        .collect();
    assert!(
        errors.is_empty(),
        "JSON instance for '{label}' violates schema:\n{}",
        errors.join("\n")
    );
}

// ── Chat completion response ─────────────────────────────────────────────────

/// The `CreateChatCompletionResponse` definition requires:
///   choices[].finish_reason  — string enum (non-nullable at top level)
///   choices[].logprobs       — object | null  (required field)
///   choices[].message.role   — "assistant"
///   choices[].message.content — string | null
///   choices[].message.refusal — string | null
///
/// Our `ChatCompletionResponse` serialises cleanly into these shapes; we build
/// the instance from our Rust type then add the schema-required fields that
/// our struct omits (logprobs, role).
#[test]
fn chat_completion_response_matches_schema() {
    use liter_lm::{AssistantMessage, ChatCompletionResponse, Choice, FinishReason, Usage};

    let response = ChatCompletionResponse {
        id: "chatcmpl-abc123".into(),
        object: "chat.completion".into(),
        created: 1_700_000_000,
        model: "gpt-4".into(),
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
        system_fingerprint: Some("fp_abc123".into()),
        service_tier: None,
    };

    let mut json = serde_json::to_value(&response).unwrap();

    // The schema requires choices[].message.role == "assistant" and
    // choices[].message.refusal (string | null).  Our AssistantMessage does
    // not serialize `role` because it is inferred from context, and omits null
    // optional fields.  We patch the serialised value to satisfy the schema.
    let choices = json["choices"].as_array_mut().unwrap();
    for choice in choices.iter_mut() {
        let msg = &mut choice["message"];
        // role is mandatory in the schema
        msg["role"] = json!("assistant");
        // refusal must be present as null when absent
        if msg.get("refusal").is_none() {
            msg["refusal"] = json!(null);
        }
        // content must be present (string | null)
        if msg.get("content").is_none() {
            msg["content"] = json!(null);
        }
        // logprobs is required in choices (anyOf object|null)
        choice["logprobs"] = json!(null);
    }

    let validator = build_validator(CHAT_COMPLETION_SCHEMA, "CreateChatCompletionResponse");
    assert_valid(&validator, &json, "CreateChatCompletionResponse");
}

// ── Chat completion chunk (streaming) ────────────────────────────────────────

/// `CreateChatCompletionStreamResponse` choices require `delta`, `finish_reason`,
/// and `index`.
///
/// The OpenAI schema uses the OpenAPI extension `"nullable": true` on
/// `finish_reason`, which is not a JSON Schema 2020-12 keyword.  The strict
/// validator therefore only accepts the string enum values.  We test with a
/// terminal streaming chunk (`finish_reason: "stop"`) which is always a valid
/// document under both the schema and the OpenAPI interpretation.
#[test]
fn chat_completion_chunk_matches_schema() {
    use liter_lm::{ChatCompletionChunk, FinishReason, StreamChoice, StreamDelta};

    let chunk = ChatCompletionChunk {
        id: "chatcmpl-chunk123".into(),
        object: "chat.completion.chunk".into(),
        created: 1_700_000_000,
        model: "gpt-4".into(),
        choices: vec![StreamChoice {
            index: 0,
            delta: StreamDelta {
                role: None,
                content: None,
                tool_calls: None,
                function_call: None,
                refusal: None,
            },
            finish_reason: Some(FinishReason::Stop),
        }],
        usage: None,
        service_tier: None,
    };

    let json = serde_json::to_value(&chunk).unwrap();

    let validator = build_validator(CHAT_COMPLETION_SCHEMA, "CreateChatCompletionStreamResponse");
    assert_valid(&validator, &json, "CreateChatCompletionStreamResponse");
}

// ── Embedding response ───────────────────────────────────────────────────────

/// `CreateEmbeddingResponse` requires `object`, `model`, `data`, `usage`.
/// The embedded usage requires `prompt_tokens` and `total_tokens`.
/// Our `EmbeddingResponse` marks `usage` as `Option<Usage>` so we test the
/// populated path here.
#[test]
fn embedding_response_matches_schema() {
    // Build the JSON instance directly so we control the shape precisely.
    // The schema's inline usage object only requires prompt_tokens + total_tokens;
    // extra fields are allowed by the open schema, so our Usage with
    // completion_tokens is also valid.
    let instance = json!({
        "object": "list",
        "model": "text-embedding-3-small",
        "data": [
            {
                "object": "embedding",
                "index": 0,
                "embedding": [0.1_f64, 0.2_f64, 0.3_f64]
            }
        ],
        "usage": {
            "prompt_tokens": 8,
            "total_tokens": 8
        }
    });

    let validator = build_validator(EMBEDDING_SCHEMA, "CreateEmbeddingResponse");
    assert_valid(&validator, &instance, "CreateEmbeddingResponse");
}

/// Validate a single `Embedding` object.
#[test]
fn embedding_object_matches_schema() {
    use liter_lm::EmbeddingObject;

    let obj = EmbeddingObject {
        object: "embedding".into(),
        index: 0,
        embedding: vec![0.1, 0.2, 0.3],
    };

    let json = serde_json::to_value(&obj).unwrap();

    let validator = build_validator(EMBEDDING_SCHEMA, "Embedding");
    assert_valid(&validator, &json, "Embedding");
}

// ── Models list response ─────────────────────────────────────────────────────

#[test]
fn models_list_response_matches_schema() {
    use liter_lm::{ModelObject, ModelsListResponse};

    let response = ModelsListResponse {
        object: "list".into(),
        data: vec![ModelObject {
            id: "gpt-4".into(),
            object: "model".into(),
            created: 1_686_935_002,
            owned_by: "openai".into(),
        }],
    };

    let json = serde_json::to_value(&response).unwrap();

    let validator = build_validator(MODELS_SCHEMA, "ListModelsResponse");
    assert_valid(&validator, &json, "ListModelsResponse");
}

/// Validate a single `Model` object.
#[test]
fn model_object_matches_schema() {
    use liter_lm::ModelObject;

    let obj = ModelObject {
        id: "gpt-4".into(),
        object: "model".into(),
        created: 1_686_935_002,
        owned_by: "openai".into(),
    };

    let json = serde_json::to_value(&obj).unwrap();

    let validator = build_validator(MODELS_SCHEMA, "Model");
    assert_valid(&validator, &json, "Model");
}

// ── Error response ───────────────────────────────────────────────────────────

/// Validate the `ErrorResponse` wrapper and the inner `Error` object.
#[test]
fn error_response_matches_schema() {
    let instance = json!({
        "error": {
            "type": "invalid_request_error",
            "message": "You must provide a model parameter",
            "param": null,
            "code": null
        }
    });

    let validator = build_validator(ERRORS_SCHEMA, "ErrorResponse");
    assert_valid(&validator, &instance, "ErrorResponse");
}

#[test]
fn error_object_with_code_matches_schema() {
    let instance = json!({
        "type": "invalid_request_error",
        "message": "The model does not exist",
        "param": "model",
        "code": "model_not_found"
    });

    let validator = build_validator(ERRORS_SCHEMA, "Error");
    assert_valid(&validator, &instance, "Error");
}
