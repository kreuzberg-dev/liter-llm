#[cfg(test)]
mod serde_tests {
    use crate::types::*;

    #[test]
    fn chat_request_round_trip() {
        let req = ChatCompletionRequest {
            model: "gpt-4".into(),
            messages: vec![
                Message::System(SystemMessage {
                    content: "You are helpful.".into(),
                    name: None,
                }),
                Message::User(UserMessage {
                    content: UserContent::Text("Hello!".into()),
                    name: None,
                }),
            ],
            temperature: Some(0.7),
            max_tokens: Some(100),
            ..Default::default()
        };

        let json = serde_json::to_string(&req).unwrap();
        let parsed: ChatCompletionRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.model, "gpt-4");
        assert_eq!(parsed.messages.len(), 2);
        assert_eq!(parsed.temperature, Some(0.7));
        assert_eq!(parsed.max_tokens, Some(100));
    }

    #[test]
    fn chat_response_deserialize() {
        let json = r#"{
            "id": "chatcmpl-abc123",
            "object": "chat.completion",
            "created": 1700000000,
            "model": "gpt-4",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "Hello!"
                },
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 5,
                "total_tokens": 15
            }
        }"#;

        let resp: ChatCompletionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.id, "chatcmpl-abc123");
        assert_eq!(resp.choices.len(), 1);
        assert_eq!(resp.choices[0].message.content.as_deref(), Some("Hello!"));
        assert_eq!(resp.usage.as_ref().unwrap().total_tokens, 15);
    }

    #[test]
    fn stream_chunk_deserialize() {
        let json = r#"{
            "id": "chatcmpl-abc123",
            "object": "chat.completion.chunk",
            "created": 1700000000,
            "model": "gpt-4",
            "choices": [{
                "index": 0,
                "delta": {
                    "content": "Hello"
                },
                "finish_reason": null
            }]
        }"#;

        let chunk: ChatCompletionChunk = serde_json::from_str(json).unwrap();
        assert_eq!(chunk.choices[0].delta.content.as_deref(), Some("Hello"));
        assert!(chunk.choices[0].finish_reason.is_none());
    }

    #[test]
    fn tool_call_message_round_trip() {
        let msg = Message::Assistant(AssistantMessage {
            content: None,
            name: None,
            tool_calls: Some(vec![ToolCall {
                id: "call_123".into(),
                call_type: ToolType::Function,
                function: FunctionCall {
                    name: "get_weather".into(),
                    arguments: r#"{"location": "NYC"}"#.into(),
                },
            }]),
            refusal: None,
            function_call: None,
        });

        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();

        if let Message::Assistant(a) = parsed {
            let calls = a.tool_calls.unwrap();
            assert_eq!(calls.len(), 1);
            assert_eq!(calls[0].function.name, "get_weather");
        } else {
            panic!("expected assistant message");
        }
    }

    #[test]
    fn multipart_content_round_trip() {
        let msg = Message::User(UserMessage {
            content: UserContent::Parts(vec![
                ContentPart::Text {
                    text: "What's in this image?".into(),
                },
                ContentPart::ImageUrl {
                    image_url: ImageUrl {
                        url: "https://example.com/image.png".into(),
                        detail: Some(ImageDetail::High),
                    },
                },
            ]),
            name: None,
        });

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("image_url"));
        let _: Message = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn embedding_request_round_trip() {
        let req = EmbeddingRequest {
            model: "text-embedding-3-small".into(),
            input: EmbeddingInput::Multiple(vec!["hello".into(), "world".into()]),
            encoding_format: None,
            dimensions: Some(256),
            user: None,
        };

        let json = serde_json::to_string(&req).unwrap();
        let parsed: EmbeddingRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.model, "text-embedding-3-small");
        assert_eq!(parsed.dimensions, Some(256));
    }

    #[test]
    fn embedding_response_deserialize() {
        let json = r#"{
            "object": "list",
            "data": [{
                "object": "embedding",
                "embedding": [0.1, 0.2, 0.3],
                "index": 0
            }],
            "model": "text-embedding-3-small",
            "usage": {
                "prompt_tokens": 5,
                "completion_tokens": 0,
                "total_tokens": 5
            }
        }"#;

        let resp: EmbeddingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].embedding.len(), 3);
    }

    #[test]
    fn developer_message_round_trip() {
        let msg = Message::Developer(DeveloperMessage {
            content: "You are a dev assistant.".into(),
            name: Some("devbot".into()),
        });
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"role\":\"developer\""));
        let parsed: Message = serde_json::from_str(&json).unwrap();
        if let Message::Developer(d) = parsed {
            assert_eq!(d.content, "You are a dev assistant.");
            assert_eq!(d.name.as_deref(), Some("devbot"));
        } else {
            panic!("expected developer message");
        }
    }

    #[test]
    fn function_message_round_trip() {
        let msg = Message::Function(FunctionMessage {
            content: r#"{"temperature": 72}"#.into(),
            name: "get_weather".into(),
        });
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"role\":\"function\""));
        let parsed: Message = serde_json::from_str(&json).unwrap();
        if let Message::Function(f) = parsed {
            assert_eq!(f.name, "get_weather");
        } else {
            panic!("expected function message");
        }
    }

    #[test]
    fn assistant_message_with_refusal() {
        let msg = Message::Assistant(AssistantMessage {
            content: None,
            name: None,
            tool_calls: None,
            refusal: Some("I cannot help with that.".into()),
            function_call: None,
        });
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("refusal"));
        let parsed: Message = serde_json::from_str(&json).unwrap();
        if let Message::Assistant(a) = parsed {
            assert_eq!(a.refusal.as_deref(), Some("I cannot help with that."));
        } else {
            panic!("expected assistant message");
        }
    }

    #[test]
    fn finish_reason_function_call_serde() {
        let reason = FinishReason::FunctionCall;
        let json = serde_json::to_string(&reason).unwrap();
        assert_eq!(json, "\"function_call\"");
        let parsed: FinishReason = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, FinishReason::FunctionCall);
    }

    #[test]
    fn service_tier_in_response() {
        let json = r#"{
            "id": "chatcmpl-abc",
            "object": "chat.completion",
            "created": 1700000000,
            "model": "gpt-4",
            "choices": [{
                "index": 0,
                "message": {"role": "assistant", "content": "Hi"},
                "finish_reason": "stop"
            }],
            "service_tier": "default"
        }"#;
        let resp: ChatCompletionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.service_tier.as_deref(), Some("default"));
    }

    #[test]
    fn response_format_json_schema() {
        let fmt = ResponseFormat::JsonSchema {
            json_schema: JsonSchemaFormat {
                name: "my_schema".into(),
                description: None,
                schema: serde_json::json!({"type": "object"}),
                strict: Some(true),
            },
        };

        let json = serde_json::to_string(&fmt).unwrap();
        assert!(json.contains("json_schema"));
        let _: ResponseFormat = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn finish_reason_other_unknown_string() {
        // Catch-all variant for unknown finish reasons from non-OpenAI providers
        let json = r#""custom_stop_reason""#;
        let reason: FinishReason = serde_json::from_str(json).unwrap();
        assert_eq!(reason, FinishReason::Other);
    }

    #[test]
    fn finish_reason_stop_serde() {
        let reason = FinishReason::Stop;
        let json = serde_json::to_string(&reason).unwrap();
        assert_eq!(json, "\"stop\"");
        let parsed: FinishReason = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, FinishReason::Stop);
    }

    #[test]
    fn finish_reason_length_serde() {
        let reason = FinishReason::Length;
        let json = serde_json::to_string(&reason).unwrap();
        assert_eq!(json, "\"length\"");
        let parsed: FinishReason = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, FinishReason::Length);
    }

    #[test]
    fn embedding_format_float_serde() {
        let fmt = EmbeddingFormat::Float;
        let json = serde_json::to_string(&fmt).unwrap();
        assert_eq!(json, "\"float\"");
        let parsed: EmbeddingFormat = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, EmbeddingFormat::Float);
    }

    #[test]
    fn embedding_format_base64_serde() {
        let fmt = EmbeddingFormat::Base64;
        let json = serde_json::to_string(&fmt).unwrap();
        assert_eq!(json, "\"base64\"");
        let parsed: EmbeddingFormat = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, EmbeddingFormat::Base64);
    }

    #[test]
    fn embedding_input_single_string() {
        let input = EmbeddingInput::Single("hello world".into());
        let json = serde_json::to_string(&input).unwrap();
        assert_eq!(json, "\"hello world\"");
        let parsed: EmbeddingInput = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, input);
    }

    #[test]
    fn embedding_input_multiple_strings() {
        let input = EmbeddingInput::Multiple(vec!["hello".into(), "world".into(), "test".into()]);
        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("hello"));
        assert!(json.contains("world"));
        let parsed: EmbeddingInput = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, input);
    }

    #[test]
    fn embedding_request_with_format_and_dimensions() {
        let req = EmbeddingRequest {
            model: "text-embedding-3-large".into(),
            input: EmbeddingInput::Single("test".into()),
            encoding_format: Some(EmbeddingFormat::Base64),
            dimensions: Some(1024),
            user: Some("user-123".into()),
        };

        let json = serde_json::to_string(&req).unwrap();
        let parsed: EmbeddingRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.encoding_format, Some(EmbeddingFormat::Base64));
        assert_eq!(parsed.dimensions, Some(1024));
        assert_eq!(parsed.user, Some("user-123".into()));
    }

    #[test]
    fn stop_sequence_single_serde() {
        let stop = StopSequence::Single("\\n".into());
        let json = serde_json::to_string(&stop).unwrap();
        assert_eq!(json, "\"\\\\n\"");
        let parsed: StopSequence = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, stop);
    }

    #[test]
    fn stop_sequence_multiple_serde() {
        let stop = StopSequence::Multiple(vec!["\\n".into(), "\\n\\n".into(), "[END]".into()]);
        let json = serde_json::to_string(&stop).unwrap();
        assert!(json.contains("END"));
        let parsed: StopSequence = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, stop);
    }

    #[test]
    fn tool_choice_mode_auto_serde() {
        let choice = ToolChoice::Mode(ToolChoiceMode::Auto);
        let json = serde_json::to_string(&choice).unwrap();
        assert_eq!(json, "\"auto\"");
        let parsed: ToolChoice = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, choice);
    }

    #[test]
    fn tool_choice_mode_required_serde() {
        let choice = ToolChoice::Mode(ToolChoiceMode::Required);
        let json = serde_json::to_string(&choice).unwrap();
        assert_eq!(json, "\"required\"");
        let parsed: ToolChoice = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, choice);
    }

    #[test]
    fn tool_choice_mode_none_serde() {
        let choice = ToolChoice::Mode(ToolChoiceMode::None);
        let json = serde_json::to_string(&choice).unwrap();
        assert_eq!(json, "\"none\"");
        let parsed: ToolChoice = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, choice);
    }

    #[test]
    fn tool_choice_specific_serde() {
        let choice = ToolChoice::Specific(SpecificToolChoice {
            choice_type: ToolType::Function,
            function: SpecificFunction {
                name: "get_weather".into(),
            },
        });
        let json = serde_json::to_string(&choice).unwrap();
        assert!(json.contains("get_weather"));
        let parsed: ToolChoice = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, choice);
    }

    #[test]
    fn response_format_text_serde() {
        let fmt = ResponseFormat::Text;
        let json = serde_json::to_string(&fmt).unwrap();
        assert_eq!(json, "{\"type\":\"text\"}");
        let parsed: ResponseFormat = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, fmt);
    }

    #[test]
    fn response_format_json_object_serde() {
        let fmt = ResponseFormat::JsonObject;
        let json = serde_json::to_string(&fmt).unwrap();
        assert_eq!(json, "{\"type\":\"json_object\"}");
        let parsed: ResponseFormat = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, fmt);
    }

    #[test]
    fn chat_completion_request_default_round_trip() {
        let req = ChatCompletionRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let parsed: ChatCompletionRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, req);
        assert!(parsed.model.is_empty());
        assert!(parsed.messages.is_empty());
    }

    #[test]
    fn chat_completion_request_full_fields_partial_eq() {
        let req1 = ChatCompletionRequest {
            model: "gpt-4".into(),
            messages: vec![Message::System(SystemMessage {
                content: "You are helpful.".into(),
                name: None,
            })],
            temperature: Some(0.7),
            max_tokens: Some(100),
            top_p: Some(0.95),
            n: Some(1),
            stream: Some(false),
            stop: Some(StopSequence::Single("\\n".into())),
            presence_penalty: Some(0.0),
            frequency_penalty: Some(0.0),
            logit_bias: None,
            user: Some("user-1".into()),
            tools: None,
            tool_choice: None,
            parallel_tool_calls: None,
            response_format: None,
            stream_options: None,
            seed: None,
        };

        let json = serde_json::to_string(&req1).unwrap();
        let req2: ChatCompletionRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req1, req2);
    }

    #[test]
    fn message_variant_equality() {
        // Test that Message variants support PartialEq correctly
        let msg1 = Message::System(SystemMessage {
            content: "test".into(),
            name: None,
        });
        let msg2 = Message::System(SystemMessage {
            content: "test".into(),
            name: None,
        });
        assert_eq!(msg1, msg2);

        let msg3 = Message::System(SystemMessage {
            content: "different".into(),
            name: None,
        });
        assert_ne!(msg1, msg3);
    }

    #[test]
    fn message_assistant_round_trip_equality() {
        let msg = Message::Assistant(AssistantMessage {
            content: Some("Hello!".into()),
            name: None,
            tool_calls: None,
            refusal: None,
            function_call: None,
        });
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, parsed);
    }

    #[test]
    fn message_user_round_trip_equality() {
        let msg = Message::User(UserMessage {
            content: UserContent::Text("What's up?".into()),
            name: None,
        });
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, parsed);
    }

    #[test]
    fn message_tool_round_trip() {
        let msg = Message::Tool(ToolMessage {
            content: r#"{"result": "sunny"}"#.into(),
            tool_call_id: "call_456".into(),
            name: Some("get_weather".into()),
        });
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        if let Message::Tool(t) = parsed {
            assert_eq!(t.tool_call_id, "call_456");
            assert_eq!(t.name.as_deref(), Some("get_weather"));
        } else {
            panic!("expected tool message");
        }
    }

    #[test]
    fn user_content_parts_image_detail_low() {
        let msg = Message::User(UserMessage {
            content: UserContent::Parts(vec![
                ContentPart::Text {
                    text: "Describe this".into(),
                },
                ContentPart::ImageUrl {
                    image_url: ImageUrl {
                        url: "https://example.com/img.png".into(),
                        detail: Some(ImageDetail::Low),
                    },
                },
            ]),
            name: None,
        });
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, parsed);
    }

    #[test]
    fn user_content_parts_image_detail_auto() {
        let msg = Message::User(UserMessage {
            content: UserContent::Parts(vec![ContentPart::ImageUrl {
                image_url: ImageUrl {
                    url: "https://example.com/img.png".into(),
                    detail: Some(ImageDetail::Auto),
                },
            }]),
            name: None,
        });
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"detail\":\"auto\""));
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, parsed);
    }
}

#[cfg(test)]
mod provider_tests {
    use crate::provider::{
        AuthConfig, AuthType, ConfigDrivenProvider, OpenAiProvider, Provider, ProviderConfig, detect_provider,
    };

    #[test]
    fn openai_matches() {
        let p = OpenAiProvider;
        assert!(p.matches_model("gpt-4"));
        assert!(p.matches_model("gpt-4o-mini"));
        assert!(p.matches_model("o1-preview"));
        assert!(p.matches_model("o3-mini"));
        assert!(p.matches_model("text-embedding-3-small"));
        assert!(!p.matches_model("claude-3-opus"));
        assert!(!p.matches_model("groq/llama3"));
    }

    #[test]
    fn detect_openai() {
        let p = detect_provider("gpt-4").unwrap();
        assert_eq!(p.name(), "openai");
        assert_eq!(p.base_url(), "https://api.openai.com/v1");
    }

    #[test]
    fn detect_groq() {
        let p = detect_provider("groq/llama3-70b").unwrap();
        assert_eq!(p.name(), "groq");
        assert_eq!(p.base_url(), "https://api.groq.com/openai/v1");
    }

    #[test]
    fn detect_mistral_prefix() {
        // The registry uses the "mistral/" routing prefix for Mistral models.
        // Bare model names without a slash prefix return None; callers should
        // use the prefixed form "mistral/mistral-large-latest".
        let p = detect_provider("mistral/mistral-large-latest").unwrap();
        assert_eq!(p.name(), "mistral");
        assert_eq!(p.base_url(), "https://api.mistral.ai/v1");
    }

    #[test]
    fn detect_ollama() {
        let p = detect_provider("ollama/llama3").unwrap();
        assert_eq!(p.name(), "ollama");
        assert_eq!(p.base_url(), "http://localhost:11434/v1");
    }

    #[test]
    fn detect_unknown_returns_none() {
        assert!(detect_provider("some-random-model").is_none());
    }

    fn make_provider(auth_type: AuthType) -> ConfigDrivenProvider {
        // Box::leak gives us a &'static reference, which ConfigDrivenProvider
        // now requires.  Leaking in tests is acceptable — each test process is
        // short-lived and the total number of leaked configs is tiny.
        let cfg: &'static ProviderConfig = Box::leak(Box::new(ProviderConfig {
            name: "test-provider".into(),
            display_name: None,
            base_url: Some("https://api.example.com/v1".into()),
            auth: Some(AuthConfig {
                auth_type,
                env_var: Some("TEST_API_KEY".into()),
            }),
            endpoints: None,
            model_prefixes: None,
            param_mappings: None,
        }));
        ConfigDrivenProvider::new(cfg)
    }

    #[test]
    fn config_driven_bearer_auth() {
        let provider = make_provider(AuthType::Bearer);
        let header = provider.auth_header("my-secret-key");
        assert!(header.is_some());
        let (name, value) = header.unwrap();
        assert_eq!(name, "Authorization");
        assert_eq!(value, "Bearer my-secret-key");
    }

    #[test]
    fn config_driven_api_key_auth() {
        let provider = make_provider(AuthType::ApiKey);
        let header = provider.auth_header("my-secret-key");
        assert!(header.is_some());
        let (name, value) = header.unwrap();
        assert_eq!(name, "x-api-key");
        assert_eq!(value, "my-secret-key");
    }

    #[test]
    fn config_driven_no_auth() {
        let provider = make_provider(AuthType::None);
        let header = provider.auth_header("my-secret-key");
        assert!(header.is_none(), "AuthType::None should return no auth header");
    }

    #[test]
    fn detect_deepseek() {
        let p = detect_provider("deepseek/deepseek-chat").unwrap();
        assert_eq!(p.name(), "deepseek");
    }

    #[test]
    fn detect_cerebras() {
        let p = detect_provider("cerebras/llama-3.1-70b").unwrap();
        assert_eq!(p.name(), "cerebras");
    }

    #[test]
    fn detect_openrouter() {
        let p = detect_provider("openrouter/auto").unwrap();
        assert_eq!(p.name(), "openrouter");
    }

    #[test]
    fn config_driven_unknown_auth_defaults_to_bearer() {
        let provider = make_provider(AuthType::Unknown);
        let header = provider.auth_header("my-secret-key");
        assert!(header.is_some());
        let (name, value) = header.unwrap();
        assert_eq!(name, "Authorization");
        assert!(value.contains("Bearer"));
    }

    #[test]
    fn provider_base_url_contract() {
        let provider = OpenAiProvider;
        assert_eq!(provider.base_url(), "https://api.openai.com/v1");
    }

    #[test]
    fn provider_strip_model_prefix_groq() {
        let p = detect_provider("groq/llama3-70b").unwrap();
        let stripped = p.strip_model_prefix("groq/llama3-70b");
        assert_eq!(stripped, "llama3-70b");
    }

    #[test]
    fn provider_strip_model_prefix_openai() {
        let p = OpenAiProvider;
        let stripped = p.strip_model_prefix("gpt-4");
        assert_eq!(stripped, "gpt-4"); // OpenAI has no prefix
    }

    #[test]
    fn detect_anthropic_by_claude_prefix() {
        let p = detect_provider("claude-3-5-sonnet-20241022").unwrap();
        assert_eq!(p.name(), "anthropic");
        assert_eq!(p.base_url(), "https://api.anthropic.com/v1");
    }

    #[test]
    fn detect_anthropic_by_slash_prefix() {
        let p = detect_provider("anthropic/claude-3-5-sonnet-20241022").unwrap();
        assert_eq!(p.name(), "anthropic");
    }

    #[test]
    fn anthropic_auth_header_uses_x_api_key() {
        let p = detect_provider("claude-3-5-sonnet-20241022").unwrap();
        let header = p.auth_header("sk-ant-test");
        assert!(header.is_some());
        let (name, value) = header.unwrap();
        assert_eq!(name, "x-api-key");
        assert_eq!(value, "sk-ant-test");
    }

    #[test]
    fn anthropic_extra_headers_contain_version() {
        use crate::provider::anthropic::AnthropicProvider;
        let p = AnthropicProvider;
        let extras = p.extra_headers();
        assert_eq!(extras.len(), 1);
        assert_eq!(extras[0].0, "anthropic-version");
        assert_eq!(extras[0].1, "2023-06-01");
    }

    #[test]
    fn anthropic_strips_prefix() {
        let p = detect_provider("anthropic/claude-3-5-sonnet-20241022").unwrap();
        assert_eq!(
            p.strip_model_prefix("anthropic/claude-3-5-sonnet-20241022"),
            "claude-3-5-sonnet-20241022"
        );
    }

    #[test]
    fn anthropic_bare_model_not_stripped() {
        use crate::provider::anthropic::AnthropicProvider;
        let p = AnthropicProvider;
        assert_eq!(
            p.strip_model_prefix("claude-3-5-sonnet-20241022"),
            "claude-3-5-sonnet-20241022"
        );
    }

    #[test]
    fn detect_azure_by_prefix() {
        let p = detect_provider("azure/gpt-4").unwrap();
        assert_eq!(p.name(), "azure");
    }

    #[test]
    fn azure_auth_header_uses_api_key() {
        let p = detect_provider("azure/gpt-4").unwrap();
        let header = p.auth_header("my-azure-key");
        assert!(header.is_some());
        let (name, value) = header.unwrap();
        assert_eq!(name, "api-key");
        assert_eq!(value, "my-azure-key");
    }

    #[test]
    fn azure_strips_prefix() {
        let p = detect_provider("azure/gpt-4").unwrap();
        assert_eq!(p.strip_model_prefix("azure/gpt-4"), "gpt-4");
    }

    #[test]
    fn azure_extra_headers_are_empty() {
        use crate::provider::azure::AzureProvider;
        let p = AzureProvider;
        assert!(p.extra_headers().is_empty());
    }
}

#[cfg(test)]
mod error_tests {
    use crate::error::LiterLmError;

    #[test]
    fn error_from_401() {
        let err = LiterLmError::from_status(
            401,
            r#"{"error":{"message":"Invalid API key","type":"invalid_request_error"}}"#,
            None,
        );
        assert!(matches!(err, LiterLmError::Authentication { .. }));
    }

    #[test]
    fn error_from_429() {
        let err = LiterLmError::from_status(
            429,
            r#"{"error":{"message":"Rate limited","type":"rate_limit_error"}}"#,
            None,
        );
        assert!(matches!(err, LiterLmError::RateLimited { .. }));
    }

    #[test]
    fn error_from_context_window() {
        let err = LiterLmError::from_status(
            400,
            r#"{"error":{"message":"maximum context length exceeded","type":"invalid_request_error"}}"#,
            None,
        );
        assert!(matches!(err, LiterLmError::ContextWindowExceeded { .. }));
    }

    #[test]
    fn error_from_plain_text() {
        let err = LiterLmError::from_status(500, "Internal Server Error", None);
        assert!(matches!(err, LiterLmError::ServerError { .. }));
    }

    #[test]
    fn error_from_503() {
        let err = LiterLmError::from_status(503, "Service Unavailable", None);
        assert!(matches!(err, LiterLmError::ServiceUnavailable { .. }));
    }

    #[test]
    fn error_from_403_forbidden() {
        // 403 Forbidden indicates invalid credentials / insufficient
        // permissions — map to Authentication, not ServerError.
        let err = LiterLmError::from_status(403, "Forbidden", None);
        assert!(matches!(err, LiterLmError::Authentication { .. }));
    }

    #[test]
    fn error_from_502_bad_gateway() {
        let err = LiterLmError::from_status(502, "Bad Gateway", None);
        assert!(matches!(err, LiterLmError::ServiceUnavailable { .. }));
    }

    #[test]
    fn error_from_504_gateway_timeout() {
        let err = LiterLmError::from_status(504, "Gateway Timeout", None);
        assert!(matches!(err, LiterLmError::ServiceUnavailable { .. }));
    }

    #[test]
    fn error_from_content_policy_filter() {
        let err = LiterLmError::from_status(
            400,
            r#"{"error":{"message":"Request violates content_filter policy","type":"invalid_request_error"}}"#,
            None,
        );
        assert!(matches!(err, LiterLmError::ContentPolicy { .. }));
    }

    #[test]
    fn error_from_content_policy_explicit() {
        let err = LiterLmError::from_status(
            400,
            r#"{"error":{"message":"content_policy violation detected","type":"invalid_request_error"}}"#,
            None,
        );
        assert!(matches!(err, LiterLmError::ContentPolicy { .. }));
    }

    #[test]
    fn error_message_preservation() {
        let msg = "Custom error message from provider";
        let err = LiterLmError::from_status(
            500,
            &format!(r#"{{"error":{{"message":"{}","type":"server_error"}}}}"#, msg),
            None,
        );
        if let LiterLmError::ServerError { message } = err {
            assert_eq!(message, msg);
        } else {
            panic!("expected ServerError");
        }
    }
}

#[cfg(test)]
mod retry_tests {
    use crate::http::retry::{parse_retry_after, should_retry};

    #[test]
    fn retry_on_429() {
        assert!(should_retry(429, 0, 3, None).is_some());
        assert!(should_retry(429, 1, 3, None).is_some());
        assert!(should_retry(429, 2, 3, None).is_some());
        assert!(should_retry(429, 3, 3, None).is_none()); // exhausted
    }

    #[test]
    fn retry_on_500() {
        assert!(should_retry(500, 0, 3, None).is_some());
        assert!(should_retry(503, 0, 3, None).is_some());
    }

    #[test]
    fn no_retry_on_400() {
        assert!(should_retry(400, 0, 3, None).is_none());
        assert!(should_retry(401, 0, 3, None).is_none());
        assert!(should_retry(404, 0, 3, None).is_none());
    }

    #[test]
    fn no_retry_when_disabled() {
        assert!(should_retry(429, 0, 0, None).is_none());
    }

    #[test]
    fn exponential_backoff() {
        let d0 = should_retry(429, 0, 3, None).unwrap();
        let d1 = should_retry(429, 1, 3, None).unwrap();
        let d2 = should_retry(429, 2, 3, None).unwrap();
        assert!(d1 > d0);
        assert!(d2 > d1);
    }

    #[test]
    fn retry_after_header_respected_on_429() {
        use std::time::Duration;
        let server_delay = Duration::from_secs(42);
        let delay = should_retry(429, 0, 3, Some(server_delay)).unwrap();
        assert_eq!(delay, server_delay);
    }

    #[test]
    fn retry_after_header_ignored_on_500() {
        use std::time::Duration;
        // Retry-After is only honoured for 429; on 500 we use exponential backoff.
        let server_delay = Duration::from_secs(42);
        let delay = should_retry(500, 0, 3, Some(server_delay)).unwrap();
        // Exponential backoff for attempt 0 = 1 s, not 42 s.
        assert_eq!(delay, Duration::from_secs(1));
    }

    #[test]
    fn parse_retry_after_header() {
        use std::time::Duration;
        assert_eq!(parse_retry_after("30"), Some(Duration::from_secs(30)));
        assert_eq!(parse_retry_after("  5  "), Some(Duration::from_secs(5)));
        assert_eq!(parse_retry_after("not-a-number"), None);
    }

    #[test]
    fn retry_on_504() {
        use std::time::Duration;
        assert_eq!(should_retry(504, 0, 3, None), Some(Duration::from_secs(1)));
        assert_eq!(should_retry(504, 1, 3, None), Some(Duration::from_secs(2)));
        assert_eq!(should_retry(504, 2, 3, None), Some(Duration::from_secs(4)));
    }

    #[test]
    fn retry_on_502() {
        assert!(should_retry(502, 0, 3, None).is_some());
        assert!(should_retry(502, 1, 3, None).is_some());
    }

    #[test]
    fn server_retry_after_capped_at_60s() {
        use std::time::Duration;
        let server_delay = Duration::from_secs(120);
        let delay = should_retry(429, 0, 3, Some(server_delay)).unwrap();
        // Should be capped at 60 seconds
        assert_eq!(delay, Duration::from_secs(60));
    }

    #[test]
    fn server_retry_after_under_cap() {
        use std::time::Duration;
        let server_delay = Duration::from_secs(30);
        let delay = should_retry(429, 0, 3, Some(server_delay)).unwrap();
        // Under 60s cap, should use server value
        assert_eq!(delay, Duration::from_secs(30));
    }

    #[test]
    fn exponential_backoff_caps_at_30s() {
        use std::time::Duration;
        // Attempt 5 would be 2^5 = 32 seconds, but capped at 30
        let delay = should_retry(500, 5, 10, None).unwrap();
        assert_eq!(delay, Duration::from_secs(30));
    }
}

#[cfg(test)]
mod sse_tests {
    use crate::http::streaming::parse_sse_line;

    #[test]
    fn parse_valid_chunk() {
        let line = r#"data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1700000000,"model":"gpt-4","choices":[{"index":0,"delta":{"content":"Hi"},"finish_reason":null}]}"#;
        let result = parse_sse_line(line).unwrap().unwrap();
        assert_eq!(result.choices[0].delta.content.as_deref(), Some("Hi"));
    }

    #[test]
    fn parse_done_returns_none() {
        assert!(parse_sse_line("data: [DONE]").is_none());
    }

    #[test]
    fn parse_non_data_returns_none() {
        assert!(parse_sse_line("event: ping").is_none());
        assert!(parse_sse_line(": comment").is_none());
    }

    #[test]
    fn parse_invalid_json() {
        let result = parse_sse_line("data: {invalid}").unwrap();
        assert!(result.is_err());
    }

    #[test]
    fn parse_data_without_space() {
        // Some implementations emit "data:{json}" without a trailing space.
        let line = r#"data:{"id":"chatcmpl-123","object":"chat.completion.chunk","created":1700000000,"model":"gpt-4","choices":[{"index":0,"delta":{"content":"Hi"},"finish_reason":null}]}"#;
        let result = parse_sse_line(line).unwrap().unwrap();
        assert_eq!(result.choices[0].delta.content.as_deref(), Some("Hi"));
    }

    #[test]
    fn parse_done_without_space() {
        assert!(parse_sse_line("data:[DONE]").is_none());
    }
}
