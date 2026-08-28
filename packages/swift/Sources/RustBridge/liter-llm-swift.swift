// swift-format-ignore-file
import RustBridgeC

public func assistantMessageTextFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> RustString {
    try { let val = __swift_bridge__$assistant_message_text_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return RustString(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func assistantMessageRefusalTextFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> RustString {
    try { let val = __swift_bridge__$assistant_message_refusal_text_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return RustString(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func assistantMessageReasoningTextFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> RustString {
    try { let val = __swift_bridge__$assistant_message_reasoning_text_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return RustString(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func assistantMessageOutputImagesFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> RustString {
    try { let val = __swift_bridge__$assistant_message_output_images_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return RustString(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func assistantMessageOutputAudioFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> RustString {
    try { let val = __swift_bridge__$assistant_message_output_audio_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return RustString(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func createDefaultClient<GenericIntoRustString: IntoRustString>(_ api_key: GenericIntoRustString, _ base_url: Optional<GenericIntoRustString>) throws -> DefaultClient {
    try { let val = __swift_bridge__$create_default_client({ let rustString = api_key.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let rustString = optionalStringIntoRustString(base_url) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()); if val.is_ok { return DefaultClient(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientChat(_ client: DefaultClientRef, _ req: ChatCompletionRequest) throws -> ChatCompletionResponse {
    try { let val = __swift_bridge__$default_client_chat(client.ptr, {req.isOwned = false; return req.ptr;}()); if val.is_ok { return ChatCompletionResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientEmbed(_ client: DefaultClientRef, _ req: EmbeddingRequest) throws -> EmbeddingResponse {
    try { let val = __swift_bridge__$default_client_embed(client.ptr, {req.isOwned = false; return req.ptr;}()); if val.is_ok { return EmbeddingResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientListModels(_ client: DefaultClientRef) throws -> ModelsListResponse {
    try { let val = __swift_bridge__$default_client_list_models(client.ptr); if val.is_ok { return ModelsListResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientImageGenerate(_ client: DefaultClientRef, _ req: CreateImageRequest) throws -> ImagesResponse {
    try { let val = __swift_bridge__$default_client_image_generate(client.ptr, {req.isOwned = false; return req.ptr;}()); if val.is_ok { return ImagesResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientSpeech(_ client: DefaultClientRef, _ req: CreateSpeechRequest) throws -> RustVec<UInt8> {
    try { let val = __swift_bridge__$default_client_speech(client.ptr, {req.isOwned = false; return req.ptr;}()); if val.is_ok { return RustVec(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientTranscribe(_ client: DefaultClientRef, _ req: CreateTranscriptionRequest) throws -> TranscriptionResponse {
    try { let val = __swift_bridge__$default_client_transcribe(client.ptr, {req.isOwned = false; return req.ptr;}()); if val.is_ok { return TranscriptionResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientModerate(_ client: DefaultClientRef, _ req: ModerationRequest) throws -> ModerationResponse {
    try { let val = __swift_bridge__$default_client_moderate(client.ptr, {req.isOwned = false; return req.ptr;}()); if val.is_ok { return ModerationResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientRerank(_ client: DefaultClientRef, _ req: RerankRequest) throws -> RerankResponse {
    try { let val = __swift_bridge__$default_client_rerank(client.ptr, {req.isOwned = false; return req.ptr;}()); if val.is_ok { return RerankResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientSearch(_ client: DefaultClientRef, _ req: SearchRequest) throws -> SearchResponse {
    try { let val = __swift_bridge__$default_client_search(client.ptr, {req.isOwned = false; return req.ptr;}()); if val.is_ok { return SearchResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientOcr(_ client: DefaultClientRef, _ req: OcrRequest) throws -> OcrResponse {
    try { let val = __swift_bridge__$default_client_ocr(client.ptr, {req.isOwned = false; return req.ptr;}()); if val.is_ok { return OcrResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientCreateFile(_ client: DefaultClientRef, _ req: CreateFileRequest) throws -> FileObject {
    try { let val = __swift_bridge__$default_client_create_file(client.ptr, {req.isOwned = false; return req.ptr;}()); if val.is_ok { return FileObject(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientRetrieveFile<GenericIntoRustString: IntoRustString>(_ client: DefaultClientRef, _ file_id: GenericIntoRustString) throws -> FileObject {
    try { let val = __swift_bridge__$default_client_retrieve_file(client.ptr, { let rustString = file_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return FileObject(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientDeleteFile<GenericIntoRustString: IntoRustString>(_ client: DefaultClientRef, _ file_id: GenericIntoRustString) throws -> DeleteResponse {
    try { let val = __swift_bridge__$default_client_delete_file(client.ptr, { let rustString = file_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return DeleteResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientListFiles(_ client: DefaultClientRef, _ query: Optional<FileListQuery>) throws -> FileListResponse {
    try { let val = __swift_bridge__$default_client_list_files(client.ptr, { if let val = query { val.isOwned = false; return val.ptr } else { return nil } }()); if val.is_ok { return FileListResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientFileContent<GenericIntoRustString: IntoRustString>(_ client: DefaultClientRef, _ file_id: GenericIntoRustString) throws -> RustVec<UInt8> {
    try { let val = __swift_bridge__$default_client_file_content(client.ptr, { let rustString = file_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return RustVec(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientCreateBatch(_ client: DefaultClientRef, _ req: CreateBatchRequest) throws -> BatchObject {
    try { let val = __swift_bridge__$default_client_create_batch(client.ptr, {req.isOwned = false; return req.ptr;}()); if val.is_ok { return BatchObject(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientRetrieveBatch<GenericIntoRustString: IntoRustString>(_ client: DefaultClientRef, _ batch_id: GenericIntoRustString) throws -> BatchObject {
    try { let val = __swift_bridge__$default_client_retrieve_batch(client.ptr, { let rustString = batch_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return BatchObject(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientListBatches(_ client: DefaultClientRef, _ query: Optional<BatchListQuery>) throws -> BatchListResponse {
    try { let val = __swift_bridge__$default_client_list_batches(client.ptr, { if let val = query { val.isOwned = false; return val.ptr } else { return nil } }()); if val.is_ok { return BatchListResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientCancelBatch<GenericIntoRustString: IntoRustString>(_ client: DefaultClientRef, _ batch_id: GenericIntoRustString) throws -> BatchObject {
    try { let val = __swift_bridge__$default_client_cancel_batch(client.ptr, { let rustString = batch_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return BatchObject(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientFetchBatchForPolling<GenericIntoRustString: IntoRustString>(_ client: DefaultClientRef, _ batch_id: GenericIntoRustString) throws -> BatchObject {
    try { let val = __swift_bridge__$default_client_fetch_batch_for_polling(client.ptr, { let rustString = batch_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return BatchObject(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientWaitForBatch<GenericIntoRustString: IntoRustString>(_ client: DefaultClientRef, _ batch_id: GenericIntoRustString, _ config: WaitForBatchConfig) throws -> BatchObject {
    try { let val = __swift_bridge__$default_client_wait_for_batch(client.ptr, { let rustString = batch_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), {config.isOwned = false; return config.ptr;}()); if val.is_ok { return BatchObject(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientCreateResponse(_ client: DefaultClientRef, _ req: CreateResponseRequest) throws -> ResponseObject {
    try { let val = __swift_bridge__$default_client_create_response(client.ptr, {req.isOwned = false; return req.ptr;}()); if val.is_ok { return ResponseObject(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientRetrieveResponse<GenericIntoRustString: IntoRustString>(_ client: DefaultClientRef, _ response_id: GenericIntoRustString) throws -> ResponseObject {
    try { let val = __swift_bridge__$default_client_retrieve_response(client.ptr, { let rustString = response_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ResponseObject(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func defaultClientCancelResponse<GenericIntoRustString: IntoRustString>(_ client: DefaultClientRef, _ response_id: GenericIntoRustString) throws -> ResponseObject {
    try { let val = __swift_bridge__$default_client_cancel_response(client.ptr, { let rustString = response_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ResponseObject(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func llmConfigProvidersFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> RustString {
    try { let val = __swift_bridge__$llm_config_providers_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return RustString(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func singleflight_result_noop(_ client: SingleflightResultRef) {
    __swift_bridge__$singleflight_result_noop(client.ptr)
}
public func capabilities<GenericIntoRustString: IntoRustString>(_ provider_name: GenericIntoRustString) -> ProviderCapabilities {
    ProviderCapabilities(ptr: __swift_bridge__$capabilities({ let rustString = provider_name.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func checkBound<GenericIntoRustString: IntoRustString>(_ context: GenericIntoRustString, _ current_len: UInt, _ incoming: UInt, _ limit: UInt) throws -> () {
    try { let val = __swift_bridge__$check_bound({ let rustString = context.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), current_len, incoming, limit); if val != nil { throw RustString(ptr: val!) } else { return } }()
}
public func clear() -> () {
    __swift_bridge__$clear()
}
public func clearCatalogOverlay() -> () {
    __swift_bridge__$clear_catalog_overlay()
}
public func completionCost<GenericIntoRustString: IntoRustString>(_ model: GenericIntoRustString, _ prompt_tokens: UInt64, _ completion_tokens: UInt64) -> RustString {
    RustString(ptr: __swift_bridge__$completion_cost({ let rustString = model.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), prompt_tokens, completion_tokens))
}
public func completionCostWithCache<GenericIntoRustString: IntoRustString>(_ model: GenericIntoRustString, _ prompt_tokens: UInt64, _ cached_tokens: UInt64, _ completion_tokens: UInt64) -> RustString {
    RustString(ptr: __swift_bridge__$completion_cost_with_cache({ let rustString = model.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), prompt_tokens, cached_tokens, completion_tokens))
}
public func complexProviderNames() throws -> RustVec<RustString> {
    try { let val = __swift_bridge__$complex_provider_names(); if val.is_ok { return RustVec(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func decodeDataUrl<GenericIntoRustString: IntoRustString>(_ url: GenericIntoRustString) -> Optional<DecodedDataUrl> {
    { let val = __swift_bridge__$decode_data_url({ let rustString = url.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val != nil { return DecodedDataUrl(ptr: val!) } else { return nil } }()
}
public func encodeDataUrl<GenericIntoRustString: IntoRustString>(_ bytes: RustVec<UInt8>, _ mime: Optional<GenericIntoRustString>) -> RustString {
    RustString(ptr: __swift_bridge__$encode_data_url({ let val = bytes; val.isOwned = false; return val.ptr }(), { if let rustString = optionalStringIntoRustString(mime) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
}
public func installCatalogOverlayFromStr<GenericIntoRustString: IntoRustString>(_ catalog_json: GenericIntoRustString) throws -> () {
    try { let val = __swift_bridge__$install_catalog_overlay_from_str({ let rustString = catalog_json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val != nil { throw RustString(ptr: val!) } else { return } }()
}
public func modelInfo<GenericIntoRustString: IntoRustString>(_ model: GenericIntoRustString) -> Optional<ModelInfo> {
    { let val = __swift_bridge__$model_info({ let rustString = model.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val != nil { return ModelInfo(ptr: val!) } else { return nil } }()
}
public func refreshCatalog(_ config: CatalogRefreshConfig) throws -> RefreshOutcome {
    try { let val = __swift_bridge__$refresh_catalog({config.isOwned = false; return config.ptr;}()); if val.is_ok { return RefreshOutcome(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func registerCustomProvider(_ config: CustomProviderConfig) throws -> () {
    try { let val = __swift_bridge__$register_custom_provider({config.isOwned = false; return config.ptr;}()); if val != nil { throw RustString(ptr: val!) } else { return } }()
}
public func unregisterCustomProvider<GenericIntoRustString: IntoRustString>(_ name: GenericIntoRustString) throws -> Bool {
    try { let val = __swift_bridge__$unregister_custom_provider({ let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); switch val.tag { case __swift_bridge__$ResultBoolAndString$ResultOk: return val.payload.ok case __swift_bridge__$ResultBoolAndString$ResultErr: throw RustString(ptr: val.payload.err) default: fatalError() } }()
}
public func createClient<GenericIntoRustString: IntoRustString>(_ api_key: GenericIntoRustString, _ base_url: Optional<GenericIntoRustString>, _ timeout_secs: Optional<UInt64>, _ max_retries: Optional<UInt32>, _ model_hint: Optional<GenericIntoRustString>) throws -> DefaultClient {
    try { let val = __swift_bridge__$create_client({ let rustString = api_key.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let rustString = optionalStringIntoRustString(base_url) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), timeout_secs.intoFfiRepr(), max_retries.intoFfiRepr(), { if let rustString = optionalStringIntoRustString(model_hint) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()); if val.is_ok { return DefaultClient(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func createClientFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> DefaultClient {
    try { let val = __swift_bridge__$create_client_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return DefaultClient(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func ensureCryptoProvider() -> () {
    __swift_bridge__$ensure_crypto_provider()
}
public func countRequestTokens<GenericIntoRustString: IntoRustString>(_ model: GenericIntoRustString, _ req: ChatCompletionRequest) throws -> UInt {
    try { let val = __swift_bridge__$count_request_tokens({ let rustString = model.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), {req.isOwned = false; return req.ptr;}()); switch val.tag { case __swift_bridge__$ResultUIntAndString$ResultOk: return val.payload.ok case __swift_bridge__$ResultUIntAndString$ResultErr: throw RustString(ptr: val.payload.err) default: fatalError() } }()
}
public func countTokens<GenericIntoRustString: IntoRustString>(_ model: GenericIntoRustString, _ text: GenericIntoRustString) throws -> UInt {
    try { let val = __swift_bridge__$count_tokens({ let rustString = model.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = text.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); switch val.tag { case __swift_bridge__$ResultUIntAndString$ResultOk: return val.payload.ok case __swift_bridge__$ResultUIntAndString$ResultErr: throw RustString(ptr: val.payload.err) default: fatalError() } }()
}
public func recordCostUsd<GenericIntoRustString: IntoRustString>(_ system: GenericIntoRustString, _ model: GenericIntoRustString, _ operation: GenericIntoRustString, _ cost_usd: Double) -> () {
    __swift_bridge__$record_cost_usd({ let rustString = system.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = model.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = operation.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), cost_usd)
}
public func defaultClientChatStreamStart(_ client: DefaultClientRef, _ req: ChatCompletionRequestRef) throws -> DefaultClientChatStreamStreamHandle {
    try { let val = __swift_bridge__$default_client_chat_stream_start(client.ptr, req.ptr); if val.is_ok { return DefaultClientChatStreamStreamHandle(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func assistantMessageFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> AssistantMessage {
    try { let val = __swift_bridge__$assistant_message_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return AssistantMessage(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func audioContentFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> AudioContent {
    try { let val = __swift_bridge__$audio_content_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return AudioContent(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func authConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> AuthConfig {
    try { let val = __swift_bridge__$auth_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return AuthConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func batchListQueryFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> BatchListQuery {
    try { let val = __swift_bridge__$batch_list_query_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return BatchListQuery(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func batchListResponseFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> BatchListResponse {
    try { let val = __swift_bridge__$batch_list_response_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return BatchListResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func batchObjectFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> BatchObject {
    try { let val = __swift_bridge__$batch_object_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return BatchObject(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func batchRequestCountsFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> BatchRequestCounts {
    try { let val = __swift_bridge__$batch_request_counts_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return BatchRequestCounts(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func bedrockConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> BedrockConfig {
    try { let val = __swift_bridge__$bedrock_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return BedrockConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func budgetConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> BudgetConfig {
    try { let val = __swift_bridge__$budget_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return BudgetConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func cacheConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> CacheConfig {
    try { let val = __swift_bridge__$cache_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return CacheConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func catalogRefreshConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> CatalogRefreshConfig {
    try { let val = __swift_bridge__$catalog_refresh_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return CatalogRefreshConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func chatCompletionRequestFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ChatCompletionRequest {
    try { let val = __swift_bridge__$chat_completion_request_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ChatCompletionRequest(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func chatCompletionResponseFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ChatCompletionResponse {
    try { let val = __swift_bridge__$chat_completion_response_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ChatCompletionResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func chatCompletionToolFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ChatCompletionTool {
    try { let val = __swift_bridge__$chat_completion_tool_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ChatCompletionTool(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func choiceFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> Choice {
    try { let val = __swift_bridge__$choice_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return Choice(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func createBatchRequestFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> CreateBatchRequest {
    try { let val = __swift_bridge__$create_batch_request_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return CreateBatchRequest(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func createFileRequestFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> CreateFileRequest {
    try { let val = __swift_bridge__$create_file_request_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return CreateFileRequest(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func createImageRequestFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> CreateImageRequest {
    try { let val = __swift_bridge__$create_image_request_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return CreateImageRequest(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func createResponseRequestFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> CreateResponseRequest {
    try { let val = __swift_bridge__$create_response_request_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return CreateResponseRequest(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func createSpeechRequestFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> CreateSpeechRequest {
    try { let val = __swift_bridge__$create_speech_request_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return CreateSpeechRequest(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func createTranscriptionRequestFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> CreateTranscriptionRequest {
    try { let val = __swift_bridge__$create_transcription_request_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return CreateTranscriptionRequest(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func customProviderConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> CustomProviderConfig {
    try { let val = __swift_bridge__$custom_provider_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return CustomProviderConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func decodedDataUrlFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> DecodedDataUrl {
    try { let val = __swift_bridge__$decoded_data_url_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return DecodedDataUrl(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func deleteResponseFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> DeleteResponse {
    try { let val = __swift_bridge__$delete_response_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return DeleteResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func embeddingObjectFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> EmbeddingObject {
    try { let val = __swift_bridge__$embedding_object_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return EmbeddingObject(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func embeddingRequestFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> EmbeddingRequest {
    try { let val = __swift_bridge__$embedding_request_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return EmbeddingRequest(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func embeddingResponseFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> EmbeddingResponse {
    try { let val = __swift_bridge__$embedding_response_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return EmbeddingResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func fileListQueryFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> FileListQuery {
    try { let val = __swift_bridge__$file_list_query_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return FileListQuery(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func fileListResponseFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> FileListResponse {
    try { let val = __swift_bridge__$file_list_response_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return FileListResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func fileObjectFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> FileObject {
    try { let val = __swift_bridge__$file_object_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return FileObject(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func functionCallFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> FunctionCall {
    try { let val = __swift_bridge__$function_call_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return FunctionCall(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func functionDefinitionFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> FunctionDefinition {
    try { let val = __swift_bridge__$function_definition_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return FunctionDefinition(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func imageFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> Image {
    try { let val = __swift_bridge__$image_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return Image(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func imageUrlFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ImageUrl {
    try { let val = __swift_bridge__$image_url_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ImageUrl(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func imagesResponseFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ImagesResponse {
    try { let val = __swift_bridge__$images_response_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ImagesResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func llmBudgetConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> LlmBudgetConfig {
    try { let val = __swift_bridge__$llm_budget_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return LlmBudgetConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func llmCacheConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> LlmCacheConfig {
    try { let val = __swift_bridge__$llm_cache_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return LlmCacheConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func llmInFlightLimitConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> LlmInFlightLimitConfig {
    try { let val = __swift_bridge__$llm_in_flight_limit_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return LlmInFlightLimitConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func llmProviderConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> LlmProviderConfig {
    try { let val = __swift_bridge__$llm_provider_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return LlmProviderConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func llmRateLimitConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> LlmRateLimitConfig {
    try { let val = __swift_bridge__$llm_rate_limit_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return LlmRateLimitConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func modelInfoFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ModelInfo {
    try { let val = __swift_bridge__$model_info_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ModelInfo(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func modelObjectFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ModelObject {
    try { let val = __swift_bridge__$model_object_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ModelObject(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func modelTierFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ModelTier {
    try { let val = __swift_bridge__$model_tier_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ModelTier(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func modelsListResponseFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ModelsListResponse {
    try { let val = __swift_bridge__$models_list_response_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ModelsListResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func moderationCategoriesFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ModerationCategories {
    try { let val = __swift_bridge__$moderation_categories_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ModerationCategories(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func moderationCategoryScoresFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ModerationCategoryScores {
    try { let val = __swift_bridge__$moderation_category_scores_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ModerationCategoryScores(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func moderationRequestFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ModerationRequest {
    try { let val = __swift_bridge__$moderation_request_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ModerationRequest(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func moderationResponseFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ModerationResponse {
    try { let val = __swift_bridge__$moderation_response_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ModerationResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func moderationResultFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ModerationResult {
    try { let val = __swift_bridge__$moderation_result_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ModerationResult(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func ocrImageFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> OcrImage {
    try { let val = __swift_bridge__$ocr_image_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return OcrImage(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func ocrPageFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> OcrPage {
    try { let val = __swift_bridge__$ocr_page_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return OcrPage(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func ocrRequestFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> OcrRequest {
    try { let val = __swift_bridge__$ocr_request_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return OcrRequest(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func ocrResponseFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> OcrResponse {
    try { let val = __swift_bridge__$ocr_response_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return OcrResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func pageDimensionsFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> PageDimensions {
    try { let val = __swift_bridge__$page_dimensions_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return PageDimensions(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func promptTokensDetailsFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> PromptTokensDetails {
    try { let val = __swift_bridge__$prompt_tokens_details_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return PromptTokensDetails(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func providerCapabilitiesFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ProviderCapabilities {
    try { let val = __swift_bridge__$provider_capabilities_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ProviderCapabilities(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func rateLimitConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> RateLimitConfig {
    try { let val = __swift_bridge__$rate_limit_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return RateLimitConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func rerankRequestFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> RerankRequest {
    try { let val = __swift_bridge__$rerank_request_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return RerankRequest(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func rerankResponseFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> RerankResponse {
    try { let val = __swift_bridge__$rerank_response_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return RerankResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func rerankResultFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> RerankResult {
    try { let val = __swift_bridge__$rerank_result_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return RerankResult(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func rerankResultDocumentFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> RerankResultDocument {
    try { let val = __swift_bridge__$rerank_result_document_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return RerankResultDocument(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func responseObjectFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ResponseObject {
    try { let val = __swift_bridge__$response_object_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ResponseObject(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func responseOutputItemFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ResponseOutputItem {
    try { let val = __swift_bridge__$response_output_item_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ResponseOutputItem(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func responseToolFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ResponseTool {
    try { let val = __swift_bridge__$response_tool_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ResponseTool(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func responseUsageFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ResponseUsage {
    try { let val = __swift_bridge__$response_usage_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ResponseUsage(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func searchRequestFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> SearchRequest {
    try { let val = __swift_bridge__$search_request_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return SearchRequest(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func searchResponseFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> SearchResponse {
    try { let val = __swift_bridge__$search_response_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return SearchResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func searchResultFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> SearchResult {
    try { let val = __swift_bridge__$search_result_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return SearchResult(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func specificFunctionFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> SpecificFunction {
    try { let val = __swift_bridge__$specific_function_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return SpecificFunction(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func streamChoiceFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> StreamChoice {
    try { let val = __swift_bridge__$stream_choice_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return StreamChoice(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func streamDeltaFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> StreamDelta {
    try { let val = __swift_bridge__$stream_delta_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return StreamDelta(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func streamFunctionCallFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> StreamFunctionCall {
    try { let val = __swift_bridge__$stream_function_call_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return StreamFunctionCall(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func streamOptionsFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> StreamOptions {
    try { let val = __swift_bridge__$stream_options_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return StreamOptions(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func streamToolCallFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> StreamToolCall {
    try { let val = __swift_bridge__$stream_tool_call_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return StreamToolCall(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func toolCallFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ToolCall {
    try { let val = __swift_bridge__$tool_call_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ToolCall(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func transcriptionResponseFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> TranscriptionResponse {
    try { let val = __swift_bridge__$transcription_response_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return TranscriptionResponse(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func transcriptionSegmentFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> TranscriptionSegment {
    try { let val = __swift_bridge__$transcription_segment_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return TranscriptionSegment(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func usageFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> Usage {
    try { let val = __swift_bridge__$usage_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return Usage(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func waitForBatchConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> WaitForBatchConfig {
    try { let val = __swift_bridge__$wait_for_batch_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return WaitForBatchConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func chatCompletionChunkFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ChatCompletionChunk {
    try { let val = __swift_bridge__$chat_completion_chunk_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ChatCompletionChunk(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func developerMessageFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> DeveloperMessage {
    try { let val = __swift_bridge__$developer_message_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return DeveloperMessage(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func documentContentFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> DocumentContent {
    try { let val = __swift_bridge__$document_content_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return DocumentContent(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func functionMessageFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> FunctionMessage {
    try { let val = __swift_bridge__$function_message_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return FunctionMessage(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func inFlightLimitConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> InFlightLimitConfig {
    try { let val = __swift_bridge__$in_flight_limit_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return InFlightLimitConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func jsonSchemaFormatFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> JsonSchemaFormat {
    try { let val = __swift_bridge__$json_schema_format_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return JsonSchemaFormat(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func llmConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> LlmConfig {
    try { let val = __swift_bridge__$llm_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return LlmConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func providerConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ProviderConfig {
    try { let val = __swift_bridge__$provider_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ProviderConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func specificToolChoiceFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> SpecificToolChoice {
    try { let val = __swift_bridge__$specific_tool_choice_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return SpecificToolChoice(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func systemMessageFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> SystemMessage {
    try { let val = __swift_bridge__$system_message_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return SystemMessage(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func toolMessageFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ToolMessage {
    try { let val = __swift_bridge__$tool_message_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ToolMessage(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func userMessageFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> UserMessage {
    try { let val = __swift_bridge__$user_message_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return UserMessage(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func assistantContentFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> AssistantContent {
    try { let val = __swift_bridge__$assistant_content_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return AssistantContent(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func assistantPartFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> AssistantPart {
    try { let val = __swift_bridge__$assistant_part_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return AssistantPart(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func authHeaderFormatFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> AuthHeaderFormat {
    try { let val = __swift_bridge__$auth_header_format_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return AuthHeaderFormat(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func authTypeFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> AuthType {
    try { let val = __swift_bridge__$auth_type_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return AuthType(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func batchStatusFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> BatchStatus {
    try { let val = __swift_bridge__$batch_status_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return BatchStatus(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func cacheBackendFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> CacheBackend {
    try { let val = __swift_bridge__$cache_backend_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return CacheBackend(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func contentPartFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ContentPart {
    try { let val = __swift_bridge__$content_part_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ContentPart(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func embeddingContentPartFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> EmbeddingContentPart {
    try { let val = __swift_bridge__$embedding_content_part_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return EmbeddingContentPart(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func embeddingFormatFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> EmbeddingFormat {
    try { let val = __swift_bridge__$embedding_format_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return EmbeddingFormat(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func embeddingInputFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> EmbeddingInput {
    try { let val = __swift_bridge__$embedding_input_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return EmbeddingInput(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func enforcementFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> Enforcement {
    try { let val = __swift_bridge__$enforcement_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return Enforcement(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func filePurposeFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> FilePurpose {
    try { let val = __swift_bridge__$file_purpose_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return FilePurpose(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func finishReasonFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> FinishReason {
    try { let val = __swift_bridge__$finish_reason_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return FinishReason(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func imageDetailFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ImageDetail {
    try { let val = __swift_bridge__$image_detail_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ImageDetail(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func messageFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> Message {
    try { let val = __swift_bridge__$message_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return Message(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func modalityFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> Modality {
    try { let val = __swift_bridge__$modality_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return Modality(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func moderationInputFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ModerationInput {
    try { let val = __swift_bridge__$moderation_input_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ModerationInput(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func ocrDocumentFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> OcrDocument {
    try { let val = __swift_bridge__$ocr_document_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return OcrDocument(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func reasoningEffortFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ReasoningEffort {
    try { let val = __swift_bridge__$reasoning_effort_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ReasoningEffort(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func refreshOutcomeFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> RefreshOutcome {
    try { let val = __swift_bridge__$refresh_outcome_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return RefreshOutcome(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func rerankDocumentFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> RerankDocument {
    try { let val = __swift_bridge__$rerank_document_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return RerankDocument(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func responseFormatFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ResponseFormat {
    try { let val = __swift_bridge__$response_format_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ResponseFormat(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func stopSequenceFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> StopSequence {
    try { let val = __swift_bridge__$stop_sequence_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return StopSequence(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func streamFormatFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> StreamFormat {
    try { let val = __swift_bridge__$stream_format_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return StreamFormat(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func toolChoiceFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ToolChoice {
    try { let val = __swift_bridge__$tool_choice_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ToolChoice(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func toolChoiceModeFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ToolChoiceMode {
    try { let val = __swift_bridge__$tool_choice_mode_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ToolChoiceMode(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func toolTypeFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ToolType {
    try { let val = __swift_bridge__$tool_type_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ToolType(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func userContentFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> UserContent {
    try { let val = __swift_bridge__$user_content_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return UserContent(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func __alef_phantom_vec_assistant_message() -> RustVec<AssistantMessage> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_assistant_message())
}
public func __alef_phantom_vec_audio_content() -> RustVec<AudioContent> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_audio_content())
}
public func __alef_phantom_vec_auth_config() -> RustVec<AuthConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_auth_config())
}
public func __alef_phantom_vec_batch_list_query() -> RustVec<BatchListQuery> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_batch_list_query())
}
public func __alef_phantom_vec_batch_list_response() -> RustVec<BatchListResponse> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_batch_list_response())
}
public func __alef_phantom_vec_batch_object() -> RustVec<BatchObject> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_batch_object())
}
public func __alef_phantom_vec_batch_request_counts() -> RustVec<BatchRequestCounts> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_batch_request_counts())
}
public func __alef_phantom_vec_bedrock_config() -> RustVec<BedrockConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_bedrock_config())
}
public func __alef_phantom_vec_catalog_refresh_config() -> RustVec<CatalogRefreshConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_catalog_refresh_config())
}
public func __alef_phantom_vec_chat_completion_chunk() -> RustVec<ChatCompletionChunk> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_chat_completion_chunk())
}
public func __alef_phantom_vec_chat_completion_request() -> RustVec<ChatCompletionRequest> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_chat_completion_request())
}
public func __alef_phantom_vec_chat_completion_response() -> RustVec<ChatCompletionResponse> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_chat_completion_response())
}
public func __alef_phantom_vec_chat_completion_tool() -> RustVec<ChatCompletionTool> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_chat_completion_tool())
}
public func __alef_phantom_vec_choice() -> RustVec<Choice> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_choice())
}
public func __alef_phantom_vec_create_batch_request() -> RustVec<CreateBatchRequest> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_create_batch_request())
}
public func __alef_phantom_vec_create_file_request() -> RustVec<CreateFileRequest> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_create_file_request())
}
public func __alef_phantom_vec_create_image_request() -> RustVec<CreateImageRequest> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_create_image_request())
}
public func __alef_phantom_vec_create_response_request() -> RustVec<CreateResponseRequest> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_create_response_request())
}
public func __alef_phantom_vec_create_speech_request() -> RustVec<CreateSpeechRequest> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_create_speech_request())
}
public func __alef_phantom_vec_create_transcription_request() -> RustVec<CreateTranscriptionRequest> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_create_transcription_request())
}
public func __alef_phantom_vec_custom_provider_config() -> RustVec<CustomProviderConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_custom_provider_config())
}
public func __alef_phantom_vec_decoded_data_url() -> RustVec<DecodedDataUrl> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_decoded_data_url())
}
public func __alef_phantom_vec_delete_response() -> RustVec<DeleteResponse> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_delete_response())
}
public func __alef_phantom_vec_developer_message() -> RustVec<DeveloperMessage> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_developer_message())
}
public func __alef_phantom_vec_document_content() -> RustVec<DocumentContent> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_document_content())
}
public func __alef_phantom_vec_embedding_object() -> RustVec<EmbeddingObject> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_embedding_object())
}
public func __alef_phantom_vec_embedding_request() -> RustVec<EmbeddingRequest> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_embedding_request())
}
public func __alef_phantom_vec_embedding_response() -> RustVec<EmbeddingResponse> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_embedding_response())
}
public func __alef_phantom_vec_file_list_query() -> RustVec<FileListQuery> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_file_list_query())
}
public func __alef_phantom_vec_file_list_response() -> RustVec<FileListResponse> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_file_list_response())
}
public func __alef_phantom_vec_file_object() -> RustVec<FileObject> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_file_object())
}
public func __alef_phantom_vec_function_call() -> RustVec<FunctionCall> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_function_call())
}
public func __alef_phantom_vec_function_definition() -> RustVec<FunctionDefinition> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_function_definition())
}
public func __alef_phantom_vec_function_message() -> RustVec<FunctionMessage> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_function_message())
}
public func __alef_phantom_vec_image() -> RustVec<Image> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_image())
}
public func __alef_phantom_vec_image_url() -> RustVec<ImageUrl> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_image_url())
}
public func __alef_phantom_vec_images_response() -> RustVec<ImagesResponse> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_images_response())
}
public func __alef_phantom_vec_json_schema_format() -> RustVec<JsonSchemaFormat> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_json_schema_format())
}
public func __alef_phantom_vec_llm_budget_config() -> RustVec<LlmBudgetConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_llm_budget_config())
}
public func __alef_phantom_vec_llm_cache_config() -> RustVec<LlmCacheConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_llm_cache_config())
}
public func __alef_phantom_vec_llm_config() -> RustVec<LlmConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_llm_config())
}
public func __alef_phantom_vec_llm_in_flight_limit_config() -> RustVec<LlmInFlightLimitConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_llm_in_flight_limit_config())
}
public func __alef_phantom_vec_llm_provider_config() -> RustVec<LlmProviderConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_llm_provider_config())
}
public func __alef_phantom_vec_llm_rate_limit_config() -> RustVec<LlmRateLimitConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_llm_rate_limit_config())
}
public func __alef_phantom_vec_model_info() -> RustVec<ModelInfo> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_model_info())
}
public func __alef_phantom_vec_model_object() -> RustVec<ModelObject> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_model_object())
}
public func __alef_phantom_vec_model_tier() -> RustVec<ModelTier> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_model_tier())
}
public func __alef_phantom_vec_models_list_response() -> RustVec<ModelsListResponse> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_models_list_response())
}
public func __alef_phantom_vec_moderation_categories() -> RustVec<ModerationCategories> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_moderation_categories())
}
public func __alef_phantom_vec_moderation_category_scores() -> RustVec<ModerationCategoryScores> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_moderation_category_scores())
}
public func __alef_phantom_vec_moderation_request() -> RustVec<ModerationRequest> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_moderation_request())
}
public func __alef_phantom_vec_moderation_response() -> RustVec<ModerationResponse> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_moderation_response())
}
public func __alef_phantom_vec_moderation_result() -> RustVec<ModerationResult> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_moderation_result())
}
public func __alef_phantom_vec_ocr_image() -> RustVec<OcrImage> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_ocr_image())
}
public func __alef_phantom_vec_ocr_page() -> RustVec<OcrPage> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_ocr_page())
}
public func __alef_phantom_vec_ocr_request() -> RustVec<OcrRequest> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_ocr_request())
}
public func __alef_phantom_vec_ocr_response() -> RustVec<OcrResponse> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_ocr_response())
}
public func __alef_phantom_vec_page_dimensions() -> RustVec<PageDimensions> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_page_dimensions())
}
public func __alef_phantom_vec_prompt_tokens_details() -> RustVec<PromptTokensDetails> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_prompt_tokens_details())
}
public func __alef_phantom_vec_provider_capabilities() -> RustVec<ProviderCapabilities> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_provider_capabilities())
}
public func __alef_phantom_vec_provider_config() -> RustVec<ProviderConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_provider_config())
}
public func __alef_phantom_vec_rerank_request() -> RustVec<RerankRequest> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_rerank_request())
}
public func __alef_phantom_vec_rerank_response() -> RustVec<RerankResponse> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_rerank_response())
}
public func __alef_phantom_vec_rerank_result() -> RustVec<RerankResult> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_rerank_result())
}
public func __alef_phantom_vec_rerank_result_document() -> RustVec<RerankResultDocument> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_rerank_result_document())
}
public func __alef_phantom_vec_response_object() -> RustVec<ResponseObject> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_response_object())
}
public func __alef_phantom_vec_response_output_item() -> RustVec<ResponseOutputItem> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_response_output_item())
}
public func __alef_phantom_vec_response_tool() -> RustVec<ResponseTool> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_response_tool())
}
public func __alef_phantom_vec_response_usage() -> RustVec<ResponseUsage> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_response_usage())
}
public func __alef_phantom_vec_search_request() -> RustVec<SearchRequest> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_search_request())
}
public func __alef_phantom_vec_search_response() -> RustVec<SearchResponse> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_search_response())
}
public func __alef_phantom_vec_search_result() -> RustVec<SearchResult> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_search_result())
}
public func __alef_phantom_vec_specific_function() -> RustVec<SpecificFunction> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_specific_function())
}
public func __alef_phantom_vec_specific_tool_choice() -> RustVec<SpecificToolChoice> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_specific_tool_choice())
}
public func __alef_phantom_vec_stream_choice() -> RustVec<StreamChoice> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_stream_choice())
}
public func __alef_phantom_vec_stream_delta() -> RustVec<StreamDelta> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_stream_delta())
}
public func __alef_phantom_vec_stream_function_call() -> RustVec<StreamFunctionCall> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_stream_function_call())
}
public func __alef_phantom_vec_stream_options() -> RustVec<StreamOptions> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_stream_options())
}
public func __alef_phantom_vec_stream_tool_call() -> RustVec<StreamToolCall> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_stream_tool_call())
}
public func __alef_phantom_vec_system_message() -> RustVec<SystemMessage> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_system_message())
}
public func __alef_phantom_vec_tool_call() -> RustVec<ToolCall> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_tool_call())
}
public func __alef_phantom_vec_tool_message() -> RustVec<ToolMessage> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_tool_message())
}
public func __alef_phantom_vec_transcription_response() -> RustVec<TranscriptionResponse> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_transcription_response())
}
public func __alef_phantom_vec_transcription_segment() -> RustVec<TranscriptionSegment> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_transcription_segment())
}
public func __alef_phantom_vec_usage() -> RustVec<Usage> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_usage())
}
public func __alef_phantom_vec_user_message() -> RustVec<UserMessage> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_user_message())
}
public func __alef_phantom_vec_assistant_content() -> RustVec<AssistantContent> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_assistant_content())
}
public func __alef_phantom_vec_assistant_part() -> RustVec<AssistantPart> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_assistant_part())
}
public func __alef_phantom_vec_auth_header_format() -> RustVec<AuthHeaderFormat> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_auth_header_format())
}
public func __alef_phantom_vec_auth_type() -> RustVec<AuthType> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_auth_type())
}
public func __alef_phantom_vec_batch_status() -> RustVec<BatchStatus> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_batch_status())
}
public func __alef_phantom_vec_content_part() -> RustVec<ContentPart> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_content_part())
}
public func __alef_phantom_vec_embedding_content_part() -> RustVec<EmbeddingContentPart> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_embedding_content_part())
}
public func __alef_phantom_vec_embedding_format() -> RustVec<EmbeddingFormat> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_embedding_format())
}
public func __alef_phantom_vec_embedding_input() -> RustVec<EmbeddingInput> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_embedding_input())
}
public func __alef_phantom_vec_file_purpose() -> RustVec<FilePurpose> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_file_purpose())
}
public func __alef_phantom_vec_finish_reason() -> RustVec<FinishReason> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_finish_reason())
}
public func __alef_phantom_vec_image_detail() -> RustVec<ImageDetail> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_image_detail())
}
public func __alef_phantom_vec_message() -> RustVec<Message> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_message())
}
public func __alef_phantom_vec_modality() -> RustVec<Modality> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_modality())
}
public func __alef_phantom_vec_moderation_input() -> RustVec<ModerationInput> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_moderation_input())
}
public func __alef_phantom_vec_ocr_document() -> RustVec<OcrDocument> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_ocr_document())
}
public func __alef_phantom_vec_reasoning_effort() -> RustVec<ReasoningEffort> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_reasoning_effort())
}
public func __alef_phantom_vec_refresh_outcome() -> RustVec<RefreshOutcome> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_refresh_outcome())
}
public func __alef_phantom_vec_rerank_document() -> RustVec<RerankDocument> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_rerank_document())
}
public func __alef_phantom_vec_response_format() -> RustVec<ResponseFormat> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_response_format())
}
public func __alef_phantom_vec_stop_sequence() -> RustVec<StopSequence> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_stop_sequence())
}
public func __alef_phantom_vec_stream_format() -> RustVec<StreamFormat> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_stream_format())
}
public func __alef_phantom_vec_tool_choice() -> RustVec<ToolChoice> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_tool_choice())
}
public func __alef_phantom_vec_tool_choice_mode() -> RustVec<ToolChoiceMode> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_tool_choice_mode())
}
public func __alef_phantom_vec_tool_type() -> RustVec<ToolType> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_tool_type())
}
public func __alef_phantom_vec_user_content() -> RustVec<UserContent> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_user_content())
}
public func __alef_phantom_vec_budget_config() -> RustVec<BudgetConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_budget_config())
}
public func __alef_phantom_vec_cache_config() -> RustVec<CacheConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_cache_config())
}
public func __alef_phantom_vec_in_flight_limit_config() -> RustVec<InFlightLimitConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_in_flight_limit_config())
}
public func __alef_phantom_vec_intent_prototype() -> RustVec<IntentPrototype> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_intent_prototype())
}
public func __alef_phantom_vec_rate_limit_config() -> RustVec<RateLimitConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_rate_limit_config())
}
public func __alef_phantom_vec_singleflight_result() -> RustVec<SingleflightResult> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_singleflight_result())
}
public func __alef_phantom_vec_wait_for_batch_config() -> RustVec<WaitForBatchConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_wait_for_batch_config())
}
public func __alef_phantom_vec_cache_backend() -> RustVec<CacheBackend> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_cache_backend())
}
public func __alef_phantom_vec_circuit_state() -> RustVec<CircuitState> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_circuit_state())
}
public func __alef_phantom_vec_enforcement() -> RustVec<Enforcement> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_enforcement())
}
public func __alef_phantom_vec_health_status() -> RustVec<HealthStatus> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_health_status())
}

public class AssistantMessage: AssistantMessageRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$AssistantMessage$_free(ptr)
        }
    }
}
public class AssistantMessageRefMut: AssistantMessageRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class AssistantMessageRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension AssistantMessageRef {
    public func content() -> Optional<RustString> {
        { let val = __swift_bridge__$AssistantMessage$content(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func name() -> Optional<RustString> {
        { let val = __swift_bridge__$AssistantMessage$name(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func toolCalls() -> RustString {
        RustString(ptr: __swift_bridge__$AssistantMessage$tool_calls(ptr))
    }

    public func refusal() -> Optional<RustString> {
        { let val = __swift_bridge__$AssistantMessage$refusal(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func functionCall() -> Optional<FunctionCall> {
        { let val = __swift_bridge__$AssistantMessage$function_call(ptr); if val != nil { return FunctionCall(ptr: val!) } else { return nil } }()
    }

    public func reasoningContent() -> Optional<RustString> {
        { let val = __swift_bridge__$AssistantMessage$reasoning_content(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension AssistantMessage: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_AssistantMessage$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_AssistantMessage$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: AssistantMessage) {
        __swift_bridge__$Vec_AssistantMessage$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_AssistantMessage$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (AssistantMessage(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AssistantMessageRef> {
        let pointer = __swift_bridge__$Vec_AssistantMessage$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AssistantMessageRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AssistantMessageRefMut> {
        let pointer = __swift_bridge__$Vec_AssistantMessage$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AssistantMessageRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<AssistantMessageRef> {
        UnsafePointer<AssistantMessageRef>(OpaquePointer(__swift_bridge__$Vec_AssistantMessage$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_AssistantMessage$len(vecPtr)
    }
}


public class AudioContent: AudioContentRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$AudioContent$_free(ptr)
        }
    }
}
extension AudioContent {
    public convenience init<GenericIntoRustString: IntoRustString>(_ data: GenericIntoRustString, _ format: GenericIntoRustString) {
        self.init(ptr: __swift_bridge__$AudioContent$new({ let rustString = data.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = format.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class AudioContentRefMut: AudioContentRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class AudioContentRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension AudioContentRef {
    public func data() -> RustString {
        RustString(ptr: __swift_bridge__$AudioContent$data(ptr))
    }

    public func format() -> RustString {
        RustString(ptr: __swift_bridge__$AudioContent$format(ptr))
    }
}
extension AudioContent: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_AudioContent$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_AudioContent$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: AudioContent) {
        __swift_bridge__$Vec_AudioContent$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_AudioContent$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (AudioContent(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AudioContentRef> {
        let pointer = __swift_bridge__$Vec_AudioContent$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AudioContentRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AudioContentRefMut> {
        let pointer = __swift_bridge__$Vec_AudioContent$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AudioContentRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<AudioContentRef> {
        UnsafePointer<AudioContentRef>(OpaquePointer(__swift_bridge__$Vec_AudioContent$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_AudioContent$len(vecPtr)
    }
}


public class AuthConfig: AuthConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$AuthConfig$_free(ptr)
        }
    }
}
public class AuthConfigRefMut: AuthConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class AuthConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension AuthConfigRef {
    public func authType() -> RustString {
        RustString(ptr: __swift_bridge__$AuthConfig$auth_type(ptr))
    }

    public func envVar() -> Optional<RustString> {
        { let val = __swift_bridge__$AuthConfig$env_var(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension AuthConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_AuthConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_AuthConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: AuthConfig) {
        __swift_bridge__$Vec_AuthConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_AuthConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (AuthConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AuthConfigRef> {
        let pointer = __swift_bridge__$Vec_AuthConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AuthConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AuthConfigRefMut> {
        let pointer = __swift_bridge__$Vec_AuthConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AuthConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<AuthConfigRef> {
        UnsafePointer<AuthConfigRef>(OpaquePointer(__swift_bridge__$Vec_AuthConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_AuthConfig$len(vecPtr)
    }
}


public class BatchListQuery: BatchListQueryRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$BatchListQuery$_free(ptr)
        }
    }
}
extension BatchListQuery {
    public convenience init<GenericIntoRustString: IntoRustString>(_ limit: Optional<UInt32>, _ after: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$BatchListQuery$new(limit.intoFfiRepr(), { if let rustString = optionalStringIntoRustString(after) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class BatchListQueryRefMut: BatchListQueryRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class BatchListQueryRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension BatchListQueryRef {
    public func limit() -> Optional<UInt32> {
        __swift_bridge__$BatchListQuery$limit(ptr).intoSwiftRepr()
    }

    public func after() -> Optional<RustString> {
        { let val = __swift_bridge__$BatchListQuery$after(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension BatchListQuery: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_BatchListQuery$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_BatchListQuery$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BatchListQuery) {
        __swift_bridge__$Vec_BatchListQuery$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_BatchListQuery$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (BatchListQuery(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BatchListQueryRef> {
        let pointer = __swift_bridge__$Vec_BatchListQuery$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return BatchListQueryRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BatchListQueryRefMut> {
        let pointer = __swift_bridge__$Vec_BatchListQuery$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return BatchListQueryRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<BatchListQueryRef> {
        UnsafePointer<BatchListQueryRef>(OpaquePointer(__swift_bridge__$Vec_BatchListQuery$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_BatchListQuery$len(vecPtr)
    }
}


public class BatchListResponse: BatchListResponseRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$BatchListResponse$_free(ptr)
        }
    }
}
extension BatchListResponse {
    public convenience init<GenericIntoRustString: IntoRustString>(_ object: GenericIntoRustString, _ data: RustVec<BatchObject>, _ has_more: Optional<Bool>, _ first_id: Optional<GenericIntoRustString>, _ last_id: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$BatchListResponse$new({ let rustString = object.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let val = data; val.isOwned = false; return val.ptr }(), has_more.intoFfiRepr(), { if let rustString = optionalStringIntoRustString(first_id) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(last_id) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class BatchListResponseRefMut: BatchListResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class BatchListResponseRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension BatchListResponseRef {
    public func object() -> RustString {
        RustString(ptr: __swift_bridge__$BatchListResponse$object(ptr))
    }

    public func data() -> RustVec<BatchObject> {
        RustVec(ptr: __swift_bridge__$BatchListResponse$data(ptr))
    }

    public func hasMore() -> Optional<Bool> {
        __swift_bridge__$BatchListResponse$has_more(ptr).intoSwiftRepr()
    }

    public func firstId() -> Optional<RustString> {
        { let val = __swift_bridge__$BatchListResponse$first_id(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func lastId() -> Optional<RustString> {
        { let val = __swift_bridge__$BatchListResponse$last_id(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension BatchListResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_BatchListResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_BatchListResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BatchListResponse) {
        __swift_bridge__$Vec_BatchListResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_BatchListResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (BatchListResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BatchListResponseRef> {
        let pointer = __swift_bridge__$Vec_BatchListResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return BatchListResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BatchListResponseRefMut> {
        let pointer = __swift_bridge__$Vec_BatchListResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return BatchListResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<BatchListResponseRef> {
        UnsafePointer<BatchListResponseRef>(OpaquePointer(__swift_bridge__$Vec_BatchListResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_BatchListResponse$len(vecPtr)
    }
}


public class BatchObject: BatchObjectRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$BatchObject$_free(ptr)
        }
    }
}
extension BatchObject {
    public convenience init<GenericIntoRustString: IntoRustString>(_ id: GenericIntoRustString, _ object: GenericIntoRustString, _ endpoint: GenericIntoRustString, _ input_file_id: GenericIntoRustString, _ completion_window: GenericIntoRustString, _ status: BatchStatus, _ output_file_id: Optional<GenericIntoRustString>, _ error_file_id: Optional<GenericIntoRustString>, _ created_at: UInt64, _ completed_at: Optional<UInt64>, _ failed_at: Optional<UInt64>, _ expired_at: Optional<UInt64>, _ request_counts: Optional<BatchRequestCounts>, _ metadata: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$BatchObject$new({ let rustString = id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = object.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = endpoint.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = input_file_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = completion_window.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), {status.isOwned = false; return status.ptr;}(), { if let rustString = optionalStringIntoRustString(output_file_id) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(error_file_id) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), created_at, completed_at.intoFfiRepr(), failed_at.intoFfiRepr(), expired_at.intoFfiRepr(), { if let val = request_counts { val.isOwned = false; return val.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(metadata) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class BatchObjectRefMut: BatchObjectRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class BatchObjectRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension BatchObjectRef {
    public func id() -> RustString {
        RustString(ptr: __swift_bridge__$BatchObject$id(ptr))
    }

    public func object() -> RustString {
        RustString(ptr: __swift_bridge__$BatchObject$object(ptr))
    }

    public func endpoint() -> RustString {
        RustString(ptr: __swift_bridge__$BatchObject$endpoint(ptr))
    }

    public func inputFileId() -> RustString {
        RustString(ptr: __swift_bridge__$BatchObject$input_file_id(ptr))
    }

    public func completionWindow() -> RustString {
        RustString(ptr: __swift_bridge__$BatchObject$completion_window(ptr))
    }

    public func status() -> RustString {
        RustString(ptr: __swift_bridge__$BatchObject$status(ptr))
    }

    public func outputFileId() -> Optional<RustString> {
        { let val = __swift_bridge__$BatchObject$output_file_id(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func errorFileId() -> Optional<RustString> {
        { let val = __swift_bridge__$BatchObject$error_file_id(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func createdAt() -> UInt64 {
        __swift_bridge__$BatchObject$created_at(ptr)
    }

    public func completedAt() -> Optional<UInt64> {
        __swift_bridge__$BatchObject$completed_at(ptr).intoSwiftRepr()
    }

    public func failedAt() -> Optional<UInt64> {
        __swift_bridge__$BatchObject$failed_at(ptr).intoSwiftRepr()
    }

    public func expiredAt() -> Optional<UInt64> {
        __swift_bridge__$BatchObject$expired_at(ptr).intoSwiftRepr()
    }

    public func requestCounts() -> Optional<BatchRequestCounts> {
        { let val = __swift_bridge__$BatchObject$request_counts(ptr); if val != nil { return BatchRequestCounts(ptr: val!) } else { return nil } }()
    }

    public func metadata() -> Optional<RustString> {
        { let val = __swift_bridge__$BatchObject$metadata(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension BatchObject: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_BatchObject$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_BatchObject$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BatchObject) {
        __swift_bridge__$Vec_BatchObject$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_BatchObject$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (BatchObject(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BatchObjectRef> {
        let pointer = __swift_bridge__$Vec_BatchObject$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return BatchObjectRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BatchObjectRefMut> {
        let pointer = __swift_bridge__$Vec_BatchObject$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return BatchObjectRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<BatchObjectRef> {
        UnsafePointer<BatchObjectRef>(OpaquePointer(__swift_bridge__$Vec_BatchObject$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_BatchObject$len(vecPtr)
    }
}


public class BatchRequestCounts: BatchRequestCountsRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$BatchRequestCounts$_free(ptr)
        }
    }
}
extension BatchRequestCounts {
    public convenience init(_ total: UInt64, _ completed: UInt64, _ failed: UInt64) {
        self.init(ptr: __swift_bridge__$BatchRequestCounts$new(total, completed, failed))
    }
}
public class BatchRequestCountsRefMut: BatchRequestCountsRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class BatchRequestCountsRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension BatchRequestCountsRef {
    public func total() -> UInt64 {
        __swift_bridge__$BatchRequestCounts$total(ptr)
    }

    public func completed() -> UInt64 {
        __swift_bridge__$BatchRequestCounts$completed(ptr)
    }

    public func failed() -> UInt64 {
        __swift_bridge__$BatchRequestCounts$failed(ptr)
    }
}
extension BatchRequestCounts: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_BatchRequestCounts$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_BatchRequestCounts$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BatchRequestCounts) {
        __swift_bridge__$Vec_BatchRequestCounts$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_BatchRequestCounts$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (BatchRequestCounts(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BatchRequestCountsRef> {
        let pointer = __swift_bridge__$Vec_BatchRequestCounts$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return BatchRequestCountsRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BatchRequestCountsRefMut> {
        let pointer = __swift_bridge__$Vec_BatchRequestCounts$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return BatchRequestCountsRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<BatchRequestCountsRef> {
        UnsafePointer<BatchRequestCountsRef>(OpaquePointer(__swift_bridge__$Vec_BatchRequestCounts$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_BatchRequestCounts$len(vecPtr)
    }
}


public class BedrockConfig: BedrockConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$BedrockConfig$_free(ptr)
        }
    }
}
extension BedrockConfig {
    public convenience init<GenericIntoRustString: IntoRustString>(_ region: Optional<GenericIntoRustString>, _ cross_region_prefix: Optional<GenericIntoRustString>, _ access_key_id: Optional<GenericIntoRustString>, _ secret_access_key: Optional<GenericIntoRustString>, _ session_token: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$BedrockConfig$new({ if let rustString = optionalStringIntoRustString(region) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(cross_region_prefix) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(access_key_id) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(secret_access_key) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(session_token) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class BedrockConfigRefMut: BedrockConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class BedrockConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension BedrockConfigRef {
    public func region() -> Optional<RustString> {
        { let val = __swift_bridge__$BedrockConfig$region(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func crossRegionPrefix() -> Optional<RustString> {
        { let val = __swift_bridge__$BedrockConfig$cross_region_prefix(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func accessKeyId() -> Optional<RustString> {
        { let val = __swift_bridge__$BedrockConfig$access_key_id(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func secretAccessKey() -> Optional<RustString> {
        { let val = __swift_bridge__$BedrockConfig$secret_access_key(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func sessionToken() -> Optional<RustString> {
        { let val = __swift_bridge__$BedrockConfig$session_token(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension BedrockConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_BedrockConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_BedrockConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BedrockConfig) {
        __swift_bridge__$Vec_BedrockConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_BedrockConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (BedrockConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BedrockConfigRef> {
        let pointer = __swift_bridge__$Vec_BedrockConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return BedrockConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BedrockConfigRefMut> {
        let pointer = __swift_bridge__$Vec_BedrockConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return BedrockConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<BedrockConfigRef> {
        UnsafePointer<BedrockConfigRef>(OpaquePointer(__swift_bridge__$Vec_BedrockConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_BedrockConfig$len(vecPtr)
    }
}


public class BudgetConfig: BudgetConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$BudgetConfig$_free(ptr)
        }
    }
}
public class BudgetConfigRefMut: BudgetConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class BudgetConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension BudgetConfigRef {
    public func globalLimit() -> Optional<Double> {
        __swift_bridge__$BudgetConfig$global_limit(ptr).intoSwiftRepr()
    }

    public func modelLimits() -> RustString {
        RustString(ptr: __swift_bridge__$BudgetConfig$model_limits(ptr))
    }

    public func enforcement() -> RustString {
        RustString(ptr: __swift_bridge__$BudgetConfig$enforcement(ptr))
    }
}
extension BudgetConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_BudgetConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_BudgetConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BudgetConfig) {
        __swift_bridge__$Vec_BudgetConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_BudgetConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (BudgetConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BudgetConfigRef> {
        let pointer = __swift_bridge__$Vec_BudgetConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return BudgetConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BudgetConfigRefMut> {
        let pointer = __swift_bridge__$Vec_BudgetConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return BudgetConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<BudgetConfigRef> {
        UnsafePointer<BudgetConfigRef>(OpaquePointer(__swift_bridge__$Vec_BudgetConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_BudgetConfig$len(vecPtr)
    }
}


public class CacheConfig: CacheConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$CacheConfig$_free(ptr)
        }
    }
}
extension CacheConfig {
    public convenience init(_ max_entries: UInt, _ ttl: UInt64, _ backend: CacheBackend) {
        self.init(ptr: __swift_bridge__$CacheConfig$new(max_entries, ttl, {backend.isOwned = false; return backend.ptr;}()))
    }
}
public class CacheConfigRefMut: CacheConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class CacheConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension CacheConfigRef {
    public func maxEntries() -> UInt {
        __swift_bridge__$CacheConfig$max_entries(ptr)
    }

    public func ttl() -> UInt64 {
        __swift_bridge__$CacheConfig$ttl(ptr)
    }

    public func backend() -> RustString {
        RustString(ptr: __swift_bridge__$CacheConfig$backend(ptr))
    }
}
extension CacheConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_CacheConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_CacheConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CacheConfig) {
        __swift_bridge__$Vec_CacheConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_CacheConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (CacheConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CacheConfigRef> {
        let pointer = __swift_bridge__$Vec_CacheConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CacheConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CacheConfigRefMut> {
        let pointer = __swift_bridge__$Vec_CacheConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CacheConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CacheConfigRef> {
        UnsafePointer<CacheConfigRef>(OpaquePointer(__swift_bridge__$Vec_CacheConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_CacheConfig$len(vecPtr)
    }
}


public class CatalogRefreshConfig: CatalogRefreshConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$CatalogRefreshConfig$_free(ptr)
        }
    }
}
extension CatalogRefreshConfig {
    public convenience init<GenericIntoRustString: IntoRustString>(_ enabled: Bool, _ source_url: GenericIntoRustString, _ ttl_seconds: UInt64, _ cache_path: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$CatalogRefreshConfig$new(enabled, { let rustString = source_url.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), ttl_seconds, { if let rustString = optionalStringIntoRustString(cache_path) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class CatalogRefreshConfigRefMut: CatalogRefreshConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class CatalogRefreshConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension CatalogRefreshConfigRef {
    public func enabled() -> Bool {
        __swift_bridge__$CatalogRefreshConfig$enabled(ptr)
    }

    public func sourceUrl() -> RustString {
        RustString(ptr: __swift_bridge__$CatalogRefreshConfig$source_url(ptr))
    }

    public func ttlSeconds() -> UInt64 {
        __swift_bridge__$CatalogRefreshConfig$ttl_seconds(ptr)
    }

    public func cachePath() -> Optional<RustString> {
        { let val = __swift_bridge__$CatalogRefreshConfig$cache_path(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension CatalogRefreshConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_CatalogRefreshConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_CatalogRefreshConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CatalogRefreshConfig) {
        __swift_bridge__$Vec_CatalogRefreshConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_CatalogRefreshConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (CatalogRefreshConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CatalogRefreshConfigRef> {
        let pointer = __swift_bridge__$Vec_CatalogRefreshConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CatalogRefreshConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CatalogRefreshConfigRefMut> {
        let pointer = __swift_bridge__$Vec_CatalogRefreshConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CatalogRefreshConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CatalogRefreshConfigRef> {
        UnsafePointer<CatalogRefreshConfigRef>(OpaquePointer(__swift_bridge__$Vec_CatalogRefreshConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_CatalogRefreshConfig$len(vecPtr)
    }
}


public class ChatCompletionChunk: ChatCompletionChunkRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ChatCompletionChunk$_free(ptr)
        }
    }
}
extension ChatCompletionChunk {
    public convenience init<GenericIntoRustString: IntoRustString>(_ id: GenericIntoRustString, _ object: GenericIntoRustString, _ created: UInt64, _ model: GenericIntoRustString, _ choices: RustVec<StreamChoice>, _ usage: Optional<Usage>, _ system_fingerprint: Optional<GenericIntoRustString>, _ service_tier: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$ChatCompletionChunk$new({ let rustString = id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = object.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), created, { let rustString = model.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let val = choices; val.isOwned = false; return val.ptr }(), { if let val = usage { val.isOwned = false; return val.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(system_fingerprint) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(service_tier) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class ChatCompletionChunkRefMut: ChatCompletionChunkRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ChatCompletionChunkRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ChatCompletionChunkRef {
    public func id() -> RustString {
        RustString(ptr: __swift_bridge__$ChatCompletionChunk$id(ptr))
    }

    public func object() -> RustString {
        RustString(ptr: __swift_bridge__$ChatCompletionChunk$object(ptr))
    }

    public func created() -> UInt64 {
        __swift_bridge__$ChatCompletionChunk$created(ptr)
    }

    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$ChatCompletionChunk$model(ptr))
    }

    public func choices() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$ChatCompletionChunk$choices(ptr))
    }

    public func usage() -> Optional<Usage> {
        { let val = __swift_bridge__$ChatCompletionChunk$usage(ptr); if val != nil { return Usage(ptr: val!) } else { return nil } }()
    }

    public func systemFingerprint() -> Optional<RustString> {
        { let val = __swift_bridge__$ChatCompletionChunk$system_fingerprint(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func serviceTier() -> Optional<RustString> {
        { let val = __swift_bridge__$ChatCompletionChunk$service_tier(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension ChatCompletionChunk: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ChatCompletionChunk$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ChatCompletionChunk$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ChatCompletionChunk) {
        __swift_bridge__$Vec_ChatCompletionChunk$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ChatCompletionChunk$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ChatCompletionChunk(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ChatCompletionChunkRef> {
        let pointer = __swift_bridge__$Vec_ChatCompletionChunk$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ChatCompletionChunkRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ChatCompletionChunkRefMut> {
        let pointer = __swift_bridge__$Vec_ChatCompletionChunk$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ChatCompletionChunkRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ChatCompletionChunkRef> {
        UnsafePointer<ChatCompletionChunkRef>(OpaquePointer(__swift_bridge__$Vec_ChatCompletionChunk$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ChatCompletionChunk$len(vecPtr)
    }
}


public class ChatCompletionRequest: ChatCompletionRequestRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ChatCompletionRequest$_free(ptr)
        }
    }
}
public class ChatCompletionRequestRefMut: ChatCompletionRequestRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ChatCompletionRequestRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ChatCompletionRequestRef {
    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$ChatCompletionRequest$model(ptr))
    }

    public func messages() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$ChatCompletionRequest$messages(ptr))
    }

    public func temperature() -> Optional<Double> {
        __swift_bridge__$ChatCompletionRequest$temperature(ptr).intoSwiftRepr()
    }

    public func topP() -> Optional<Double> {
        __swift_bridge__$ChatCompletionRequest$top_p(ptr).intoSwiftRepr()
    }

    public func n() -> Optional<UInt32> {
        __swift_bridge__$ChatCompletionRequest$n(ptr).intoSwiftRepr()
    }

    public func stream() -> Optional<Bool> {
        __swift_bridge__$ChatCompletionRequest$stream(ptr).intoSwiftRepr()
    }

    public func stop() -> Optional<RustString> {
        { let val = __swift_bridge__$ChatCompletionRequest$stop(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func maxTokens() -> Optional<UInt64> {
        __swift_bridge__$ChatCompletionRequest$max_tokens(ptr).intoSwiftRepr()
    }

    public func presencePenalty() -> Optional<Double> {
        __swift_bridge__$ChatCompletionRequest$presence_penalty(ptr).intoSwiftRepr()
    }

    public func frequencyPenalty() -> Optional<Double> {
        __swift_bridge__$ChatCompletionRequest$frequency_penalty(ptr).intoSwiftRepr()
    }

    public func logitBias() -> RustString {
        RustString(ptr: __swift_bridge__$ChatCompletionRequest$logit_bias(ptr))
    }

    public func user() -> Optional<RustString> {
        { let val = __swift_bridge__$ChatCompletionRequest$user(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func tools() -> RustString {
        RustString(ptr: __swift_bridge__$ChatCompletionRequest$tools(ptr))
    }

    public func toolChoice() -> Optional<RustString> {
        { let val = __swift_bridge__$ChatCompletionRequest$tool_choice(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func parallelToolCalls() -> Optional<Bool> {
        __swift_bridge__$ChatCompletionRequest$parallel_tool_calls(ptr).intoSwiftRepr()
    }

    public func responseFormat() -> Optional<RustString> {
        { let val = __swift_bridge__$ChatCompletionRequest$response_format(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func streamOptions() -> Optional<StreamOptions> {
        { let val = __swift_bridge__$ChatCompletionRequest$stream_options(ptr); if val != nil { return StreamOptions(ptr: val!) } else { return nil } }()
    }

    public func seed() -> Optional<Int64> {
        __swift_bridge__$ChatCompletionRequest$seed(ptr).intoSwiftRepr()
    }

    public func reasoningEffort() -> Optional<RustString> {
        { let val = __swift_bridge__$ChatCompletionRequest$reasoning_effort(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func modalities() -> RustString {
        RustString(ptr: __swift_bridge__$ChatCompletionRequest$modalities(ptr))
    }

    public func logprobs() -> Optional<Bool> {
        __swift_bridge__$ChatCompletionRequest$logprobs(ptr).intoSwiftRepr()
    }

    public func topLogprobs() -> Optional<UInt32> {
        __swift_bridge__$ChatCompletionRequest$top_logprobs(ptr).intoSwiftRepr()
    }

    public func maxCompletionTokens() -> Optional<UInt64> {
        __swift_bridge__$ChatCompletionRequest$max_completion_tokens(ptr).intoSwiftRepr()
    }

    public func serviceTier() -> Optional<RustString> {
        { let val = __swift_bridge__$ChatCompletionRequest$service_tier(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func store() -> Optional<Bool> {
        __swift_bridge__$ChatCompletionRequest$store(ptr).intoSwiftRepr()
    }

    public func metadata() -> RustString {
        RustString(ptr: __swift_bridge__$ChatCompletionRequest$metadata(ptr))
    }

    public func prediction() -> Optional<RustString> {
        { let val = __swift_bridge__$ChatCompletionRequest$prediction(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func audio() -> Optional<RustString> {
        { let val = __swift_bridge__$ChatCompletionRequest$audio(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func webSearchOptions() -> Optional<RustString> {
        { let val = __swift_bridge__$ChatCompletionRequest$web_search_options(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func extraBody() -> Optional<RustString> {
        { let val = __swift_bridge__$ChatCompletionRequest$extra_body(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension ChatCompletionRequest: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ChatCompletionRequest$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ChatCompletionRequest$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ChatCompletionRequest) {
        __swift_bridge__$Vec_ChatCompletionRequest$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ChatCompletionRequest$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ChatCompletionRequest(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ChatCompletionRequestRef> {
        let pointer = __swift_bridge__$Vec_ChatCompletionRequest$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ChatCompletionRequestRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ChatCompletionRequestRefMut> {
        let pointer = __swift_bridge__$Vec_ChatCompletionRequest$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ChatCompletionRequestRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ChatCompletionRequestRef> {
        UnsafePointer<ChatCompletionRequestRef>(OpaquePointer(__swift_bridge__$Vec_ChatCompletionRequest$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ChatCompletionRequest$len(vecPtr)
    }
}


public class ChatCompletionResponse: ChatCompletionResponseRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ChatCompletionResponse$_free(ptr)
        }
    }
}
extension ChatCompletionResponse {
    public convenience init<GenericIntoRustString: IntoRustString>(_ id: GenericIntoRustString, _ object: GenericIntoRustString, _ created: UInt64, _ model: GenericIntoRustString, _ choices: RustVec<Choice>, _ usage: Optional<Usage>, _ system_fingerprint: Optional<GenericIntoRustString>, _ service_tier: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$ChatCompletionResponse$new({ let rustString = id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = object.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), created, { let rustString = model.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let val = choices; val.isOwned = false; return val.ptr }(), { if let val = usage { val.isOwned = false; return val.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(system_fingerprint) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(service_tier) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class ChatCompletionResponseRefMut: ChatCompletionResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ChatCompletionResponseRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ChatCompletionResponseRef {
    public func id() -> RustString {
        RustString(ptr: __swift_bridge__$ChatCompletionResponse$id(ptr))
    }

    public func object() -> RustString {
        RustString(ptr: __swift_bridge__$ChatCompletionResponse$object(ptr))
    }

    public func created() -> UInt64 {
        __swift_bridge__$ChatCompletionResponse$created(ptr)
    }

    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$ChatCompletionResponse$model(ptr))
    }

    public func choices() -> RustVec<Choice> {
        RustVec(ptr: __swift_bridge__$ChatCompletionResponse$choices(ptr))
    }

    public func usage() -> Optional<Usage> {
        { let val = __swift_bridge__$ChatCompletionResponse$usage(ptr); if val != nil { return Usage(ptr: val!) } else { return nil } }()
    }

    public func systemFingerprint() -> Optional<RustString> {
        { let val = __swift_bridge__$ChatCompletionResponse$system_fingerprint(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func serviceTier() -> Optional<RustString> {
        { let val = __swift_bridge__$ChatCompletionResponse$service_tier(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension ChatCompletionResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ChatCompletionResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ChatCompletionResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ChatCompletionResponse) {
        __swift_bridge__$Vec_ChatCompletionResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ChatCompletionResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ChatCompletionResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ChatCompletionResponseRef> {
        let pointer = __swift_bridge__$Vec_ChatCompletionResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ChatCompletionResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ChatCompletionResponseRefMut> {
        let pointer = __swift_bridge__$Vec_ChatCompletionResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ChatCompletionResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ChatCompletionResponseRef> {
        UnsafePointer<ChatCompletionResponseRef>(OpaquePointer(__swift_bridge__$Vec_ChatCompletionResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ChatCompletionResponse$len(vecPtr)
    }
}


public class ChatCompletionTool: ChatCompletionToolRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ChatCompletionTool$_free(ptr)
        }
    }
}
public class ChatCompletionToolRefMut: ChatCompletionToolRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ChatCompletionToolRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ChatCompletionToolRef {
    public func toolType() -> RustString {
        RustString(ptr: __swift_bridge__$ChatCompletionTool$tool_type(ptr))
    }

    public func function() -> FunctionDefinition {
        FunctionDefinition(ptr: __swift_bridge__$ChatCompletionTool$function(ptr))
    }
}
extension ChatCompletionTool: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ChatCompletionTool$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ChatCompletionTool$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ChatCompletionTool) {
        __swift_bridge__$Vec_ChatCompletionTool$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ChatCompletionTool$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ChatCompletionTool(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ChatCompletionToolRef> {
        let pointer = __swift_bridge__$Vec_ChatCompletionTool$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ChatCompletionToolRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ChatCompletionToolRefMut> {
        let pointer = __swift_bridge__$Vec_ChatCompletionTool$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ChatCompletionToolRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ChatCompletionToolRef> {
        UnsafePointer<ChatCompletionToolRef>(OpaquePointer(__swift_bridge__$Vec_ChatCompletionTool$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ChatCompletionTool$len(vecPtr)
    }
}


public class Choice: ChoiceRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Choice$_free(ptr)
        }
    }
}
extension Choice {
    public convenience init<GenericIntoRustString: IntoRustString>(_ index: UInt32, _ message: AssistantMessage, _ finish_reason: Optional<FinishReason>, _ logprobs: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$Choice$new(index, {message.isOwned = false; return message.ptr;}(), { if let val = finish_reason { val.isOwned = false; return val.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(logprobs) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class ChoiceRefMut: ChoiceRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ChoiceRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ChoiceRef {
    public func index() -> UInt32 {
        __swift_bridge__$Choice$index(ptr)
    }

    public func message() -> AssistantMessage {
        AssistantMessage(ptr: __swift_bridge__$Choice$message(ptr))
    }

    public func finishReason() -> Optional<RustString> {
        { let val = __swift_bridge__$Choice$finish_reason(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func logprobs() -> Optional<RustString> {
        { let val = __swift_bridge__$Choice$logprobs(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension Choice: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Choice$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Choice$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Choice) {
        __swift_bridge__$Vec_Choice$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Choice$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Choice(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ChoiceRef> {
        let pointer = __swift_bridge__$Vec_Choice$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ChoiceRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ChoiceRefMut> {
        let pointer = __swift_bridge__$Vec_Choice$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ChoiceRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ChoiceRef> {
        UnsafePointer<ChoiceRef>(OpaquePointer(__swift_bridge__$Vec_Choice$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Choice$len(vecPtr)
    }
}


public class CreateBatchRequest: CreateBatchRequestRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$CreateBatchRequest$_free(ptr)
        }
    }
}
extension CreateBatchRequest {
    public convenience init<GenericIntoRustString: IntoRustString>(_ input_file_id: GenericIntoRustString, _ endpoint: GenericIntoRustString, _ completion_window: GenericIntoRustString, _ metadata: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$CreateBatchRequest$new({ let rustString = input_file_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = endpoint.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = completion_window.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let rustString = optionalStringIntoRustString(metadata) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class CreateBatchRequestRefMut: CreateBatchRequestRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class CreateBatchRequestRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension CreateBatchRequestRef {
    public func inputFileId() -> RustString {
        RustString(ptr: __swift_bridge__$CreateBatchRequest$input_file_id(ptr))
    }

    public func endpoint() -> RustString {
        RustString(ptr: __swift_bridge__$CreateBatchRequest$endpoint(ptr))
    }

    public func completionWindow() -> RustString {
        RustString(ptr: __swift_bridge__$CreateBatchRequest$completion_window(ptr))
    }

    public func metadata() -> Optional<RustString> {
        { let val = __swift_bridge__$CreateBatchRequest$metadata(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension CreateBatchRequest: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_CreateBatchRequest$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_CreateBatchRequest$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CreateBatchRequest) {
        __swift_bridge__$Vec_CreateBatchRequest$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_CreateBatchRequest$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (CreateBatchRequest(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CreateBatchRequestRef> {
        let pointer = __swift_bridge__$Vec_CreateBatchRequest$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CreateBatchRequestRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CreateBatchRequestRefMut> {
        let pointer = __swift_bridge__$Vec_CreateBatchRequest$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CreateBatchRequestRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CreateBatchRequestRef> {
        UnsafePointer<CreateBatchRequestRef>(OpaquePointer(__swift_bridge__$Vec_CreateBatchRequest$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_CreateBatchRequest$len(vecPtr)
    }
}


public class CreateFileRequest: CreateFileRequestRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$CreateFileRequest$_free(ptr)
        }
    }
}
extension CreateFileRequest {
    public convenience init<GenericIntoRustString: IntoRustString>(_ file: GenericIntoRustString, _ purpose: FilePurpose, _ filename: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$CreateFileRequest$new({ let rustString = file.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), {purpose.isOwned = false; return purpose.ptr;}(), { if let rustString = optionalStringIntoRustString(filename) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class CreateFileRequestRefMut: CreateFileRequestRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class CreateFileRequestRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension CreateFileRequestRef {
    public func file() -> RustString {
        RustString(ptr: __swift_bridge__$CreateFileRequest$file(ptr))
    }

    public func purpose() -> RustString {
        RustString(ptr: __swift_bridge__$CreateFileRequest$purpose(ptr))
    }

    public func filename() -> Optional<RustString> {
        { let val = __swift_bridge__$CreateFileRequest$filename(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension CreateFileRequest: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_CreateFileRequest$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_CreateFileRequest$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CreateFileRequest) {
        __swift_bridge__$Vec_CreateFileRequest$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_CreateFileRequest$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (CreateFileRequest(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CreateFileRequestRef> {
        let pointer = __swift_bridge__$Vec_CreateFileRequest$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CreateFileRequestRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CreateFileRequestRefMut> {
        let pointer = __swift_bridge__$Vec_CreateFileRequest$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CreateFileRequestRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CreateFileRequestRef> {
        UnsafePointer<CreateFileRequestRef>(OpaquePointer(__swift_bridge__$Vec_CreateFileRequest$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_CreateFileRequest$len(vecPtr)
    }
}


public class CreateImageRequest: CreateImageRequestRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$CreateImageRequest$_free(ptr)
        }
    }
}
extension CreateImageRequest {
    public convenience init<GenericIntoRustString: IntoRustString>(_ prompt: GenericIntoRustString, _ model: Optional<GenericIntoRustString>, _ n: Optional<UInt32>, _ size: Optional<GenericIntoRustString>, _ quality: Optional<GenericIntoRustString>, _ style: Optional<GenericIntoRustString>, _ response_format: Optional<GenericIntoRustString>, _ user: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$CreateImageRequest$new({ let rustString = prompt.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let rustString = optionalStringIntoRustString(model) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), n.intoFfiRepr(), { if let rustString = optionalStringIntoRustString(size) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(quality) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(style) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(response_format) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(user) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class CreateImageRequestRefMut: CreateImageRequestRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class CreateImageRequestRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension CreateImageRequestRef {
    public func prompt() -> RustString {
        RustString(ptr: __swift_bridge__$CreateImageRequest$prompt(ptr))
    }

    public func model() -> Optional<RustString> {
        { let val = __swift_bridge__$CreateImageRequest$model(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func n() -> Optional<UInt32> {
        __swift_bridge__$CreateImageRequest$n(ptr).intoSwiftRepr()
    }

    public func size() -> Optional<RustString> {
        { let val = __swift_bridge__$CreateImageRequest$size(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func quality() -> Optional<RustString> {
        { let val = __swift_bridge__$CreateImageRequest$quality(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func style() -> Optional<RustString> {
        { let val = __swift_bridge__$CreateImageRequest$style(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func responseFormat() -> Optional<RustString> {
        { let val = __swift_bridge__$CreateImageRequest$response_format(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func user() -> Optional<RustString> {
        { let val = __swift_bridge__$CreateImageRequest$user(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension CreateImageRequest: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_CreateImageRequest$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_CreateImageRequest$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CreateImageRequest) {
        __swift_bridge__$Vec_CreateImageRequest$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_CreateImageRequest$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (CreateImageRequest(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CreateImageRequestRef> {
        let pointer = __swift_bridge__$Vec_CreateImageRequest$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CreateImageRequestRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CreateImageRequestRefMut> {
        let pointer = __swift_bridge__$Vec_CreateImageRequest$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CreateImageRequestRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CreateImageRequestRef> {
        UnsafePointer<CreateImageRequestRef>(OpaquePointer(__swift_bridge__$Vec_CreateImageRequest$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_CreateImageRequest$len(vecPtr)
    }
}


public class CreateResponseRequest: CreateResponseRequestRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$CreateResponseRequest$_free(ptr)
        }
    }
}
public class CreateResponseRequestRefMut: CreateResponseRequestRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class CreateResponseRequestRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension CreateResponseRequestRef {
    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$CreateResponseRequest$model(ptr))
    }

    public func input() -> RustString {
        RustString(ptr: __swift_bridge__$CreateResponseRequest$input(ptr))
    }

    public func instructions() -> Optional<RustString> {
        { let val = __swift_bridge__$CreateResponseRequest$instructions(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func tools() -> RustString {
        RustString(ptr: __swift_bridge__$CreateResponseRequest$tools(ptr))
    }

    public func temperature() -> Optional<Double> {
        __swift_bridge__$CreateResponseRequest$temperature(ptr).intoSwiftRepr()
    }

    public func maxOutputTokens() -> Optional<UInt64> {
        __swift_bridge__$CreateResponseRequest$max_output_tokens(ptr).intoSwiftRepr()
    }

    public func metadata() -> Optional<RustString> {
        { let val = __swift_bridge__$CreateResponseRequest$metadata(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func extraBody() -> Optional<RustString> {
        { let val = __swift_bridge__$CreateResponseRequest$extra_body(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func stream() -> Optional<Bool> {
        __swift_bridge__$CreateResponseRequest$stream(ptr).intoSwiftRepr()
    }
}
extension CreateResponseRequest: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_CreateResponseRequest$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_CreateResponseRequest$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CreateResponseRequest) {
        __swift_bridge__$Vec_CreateResponseRequest$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_CreateResponseRequest$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (CreateResponseRequest(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CreateResponseRequestRef> {
        let pointer = __swift_bridge__$Vec_CreateResponseRequest$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CreateResponseRequestRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CreateResponseRequestRefMut> {
        let pointer = __swift_bridge__$Vec_CreateResponseRequest$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CreateResponseRequestRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CreateResponseRequestRef> {
        UnsafePointer<CreateResponseRequestRef>(OpaquePointer(__swift_bridge__$Vec_CreateResponseRequest$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_CreateResponseRequest$len(vecPtr)
    }
}


public class CreateSpeechRequest: CreateSpeechRequestRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$CreateSpeechRequest$_free(ptr)
        }
    }
}
extension CreateSpeechRequest {
    public convenience init<GenericIntoRustString: IntoRustString>(_ model: GenericIntoRustString, _ input: GenericIntoRustString, _ voice: GenericIntoRustString, _ response_format: Optional<GenericIntoRustString>, _ speed: Optional<Double>) {
        self.init(ptr: __swift_bridge__$CreateSpeechRequest$new({ let rustString = model.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = input.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = voice.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let rustString = optionalStringIntoRustString(response_format) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), speed.intoFfiRepr()))
    }
}
public class CreateSpeechRequestRefMut: CreateSpeechRequestRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class CreateSpeechRequestRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension CreateSpeechRequestRef {
    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$CreateSpeechRequest$model(ptr))
    }

    public func input() -> RustString {
        RustString(ptr: __swift_bridge__$CreateSpeechRequest$input(ptr))
    }

    public func voice() -> RustString {
        RustString(ptr: __swift_bridge__$CreateSpeechRequest$voice(ptr))
    }

    public func responseFormat() -> Optional<RustString> {
        { let val = __swift_bridge__$CreateSpeechRequest$response_format(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func speed() -> Optional<Double> {
        __swift_bridge__$CreateSpeechRequest$speed(ptr).intoSwiftRepr()
    }
}
extension CreateSpeechRequest: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_CreateSpeechRequest$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_CreateSpeechRequest$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CreateSpeechRequest) {
        __swift_bridge__$Vec_CreateSpeechRequest$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_CreateSpeechRequest$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (CreateSpeechRequest(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CreateSpeechRequestRef> {
        let pointer = __swift_bridge__$Vec_CreateSpeechRequest$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CreateSpeechRequestRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CreateSpeechRequestRefMut> {
        let pointer = __swift_bridge__$Vec_CreateSpeechRequest$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CreateSpeechRequestRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CreateSpeechRequestRef> {
        UnsafePointer<CreateSpeechRequestRef>(OpaquePointer(__swift_bridge__$Vec_CreateSpeechRequest$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_CreateSpeechRequest$len(vecPtr)
    }
}


public class CreateTranscriptionRequest: CreateTranscriptionRequestRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$CreateTranscriptionRequest$_free(ptr)
        }
    }
}
extension CreateTranscriptionRequest {
    public convenience init<GenericIntoRustString: IntoRustString>(_ model: GenericIntoRustString, _ file: GenericIntoRustString, _ language: Optional<GenericIntoRustString>, _ prompt: Optional<GenericIntoRustString>, _ response_format: Optional<GenericIntoRustString>, _ temperature: Optional<Double>) {
        self.init(ptr: __swift_bridge__$CreateTranscriptionRequest$new({ let rustString = model.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = file.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let rustString = optionalStringIntoRustString(language) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(prompt) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(response_format) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), temperature.intoFfiRepr()))
    }
}
public class CreateTranscriptionRequestRefMut: CreateTranscriptionRequestRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class CreateTranscriptionRequestRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension CreateTranscriptionRequestRef {
    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$CreateTranscriptionRequest$model(ptr))
    }

    public func file() -> RustString {
        RustString(ptr: __swift_bridge__$CreateTranscriptionRequest$file(ptr))
    }

    public func language() -> Optional<RustString> {
        { let val = __swift_bridge__$CreateTranscriptionRequest$language(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func prompt() -> Optional<RustString> {
        { let val = __swift_bridge__$CreateTranscriptionRequest$prompt(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func responseFormat() -> Optional<RustString> {
        { let val = __swift_bridge__$CreateTranscriptionRequest$response_format(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func temperature() -> Optional<Double> {
        __swift_bridge__$CreateTranscriptionRequest$temperature(ptr).intoSwiftRepr()
    }
}
extension CreateTranscriptionRequest: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_CreateTranscriptionRequest$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_CreateTranscriptionRequest$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CreateTranscriptionRequest) {
        __swift_bridge__$Vec_CreateTranscriptionRequest$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_CreateTranscriptionRequest$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (CreateTranscriptionRequest(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CreateTranscriptionRequestRef> {
        let pointer = __swift_bridge__$Vec_CreateTranscriptionRequest$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CreateTranscriptionRequestRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CreateTranscriptionRequestRefMut> {
        let pointer = __swift_bridge__$Vec_CreateTranscriptionRequest$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CreateTranscriptionRequestRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CreateTranscriptionRequestRef> {
        UnsafePointer<CreateTranscriptionRequestRef>(OpaquePointer(__swift_bridge__$Vec_CreateTranscriptionRequest$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_CreateTranscriptionRequest$len(vecPtr)
    }
}


public class CustomProviderConfig: CustomProviderConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$CustomProviderConfig$_free(ptr)
        }
    }
}
public class CustomProviderConfigRefMut: CustomProviderConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class CustomProviderConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension CustomProviderConfigRef {
    public func name() -> RustString {
        RustString(ptr: __swift_bridge__$CustomProviderConfig$name(ptr))
    }

    public func baseUrl() -> RustString {
        RustString(ptr: __swift_bridge__$CustomProviderConfig$base_url(ptr))
    }

    public func authHeader() -> RustString {
        RustString(ptr: __swift_bridge__$CustomProviderConfig$auth_header(ptr))
    }

    public func modelPrefixes() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$CustomProviderConfig$model_prefixes(ptr))
    }
}
extension CustomProviderConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_CustomProviderConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_CustomProviderConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CustomProviderConfig) {
        __swift_bridge__$Vec_CustomProviderConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_CustomProviderConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (CustomProviderConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CustomProviderConfigRef> {
        let pointer = __swift_bridge__$Vec_CustomProviderConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CustomProviderConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CustomProviderConfigRefMut> {
        let pointer = __swift_bridge__$Vec_CustomProviderConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CustomProviderConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CustomProviderConfigRef> {
        UnsafePointer<CustomProviderConfigRef>(OpaquePointer(__swift_bridge__$Vec_CustomProviderConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_CustomProviderConfig$len(vecPtr)
    }
}


public class DecodedDataUrl: DecodedDataUrlRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$DecodedDataUrl$_free(ptr)
        }
    }
}
extension DecodedDataUrl {
    public convenience init<GenericIntoRustString: IntoRustString>(_ mime: GenericIntoRustString, _ data: RustVec<UInt8>) {
        self.init(ptr: __swift_bridge__$DecodedDataUrl$new({ let rustString = mime.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let val = data; val.isOwned = false; return val.ptr }()))
    }
}
public class DecodedDataUrlRefMut: DecodedDataUrlRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class DecodedDataUrlRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension DecodedDataUrlRef {
    public func mime() -> RustString {
        RustString(ptr: __swift_bridge__$DecodedDataUrl$mime(ptr))
    }

    public func data() -> RustVec<UInt8> {
        RustVec(ptr: __swift_bridge__$DecodedDataUrl$data(ptr))
    }
}
extension DecodedDataUrl: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_DecodedDataUrl$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_DecodedDataUrl$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: DecodedDataUrl) {
        __swift_bridge__$Vec_DecodedDataUrl$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_DecodedDataUrl$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (DecodedDataUrl(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DecodedDataUrlRef> {
        let pointer = __swift_bridge__$Vec_DecodedDataUrl$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DecodedDataUrlRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DecodedDataUrlRefMut> {
        let pointer = __swift_bridge__$Vec_DecodedDataUrl$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DecodedDataUrlRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<DecodedDataUrlRef> {
        UnsafePointer<DecodedDataUrlRef>(OpaquePointer(__swift_bridge__$Vec_DecodedDataUrl$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_DecodedDataUrl$len(vecPtr)
    }
}


public class DefaultClient: DefaultClientRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$DefaultClient$_free(ptr)
        }
    }
}
public class DefaultClientRefMut: DefaultClientRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class DefaultClientRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension DefaultClient: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_DefaultClient$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_DefaultClient$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: DefaultClient) {
        __swift_bridge__$Vec_DefaultClient$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_DefaultClient$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (DefaultClient(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DefaultClientRef> {
        let pointer = __swift_bridge__$Vec_DefaultClient$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DefaultClientRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DefaultClientRefMut> {
        let pointer = __swift_bridge__$Vec_DefaultClient$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DefaultClientRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<DefaultClientRef> {
        UnsafePointer<DefaultClientRef>(OpaquePointer(__swift_bridge__$Vec_DefaultClient$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_DefaultClient$len(vecPtr)
    }
}


public class DeleteResponse: DeleteResponseRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$DeleteResponse$_free(ptr)
        }
    }
}
extension DeleteResponse {
    public convenience init<GenericIntoRustString: IntoRustString>(_ id: GenericIntoRustString, _ object: GenericIntoRustString, _ deleted: Bool) {
        self.init(ptr: __swift_bridge__$DeleteResponse$new({ let rustString = id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = object.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), deleted))
    }
}
public class DeleteResponseRefMut: DeleteResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class DeleteResponseRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension DeleteResponseRef {
    public func id() -> RustString {
        RustString(ptr: __swift_bridge__$DeleteResponse$id(ptr))
    }

    public func object() -> RustString {
        RustString(ptr: __swift_bridge__$DeleteResponse$object(ptr))
    }

    public func deleted() -> Bool {
        __swift_bridge__$DeleteResponse$deleted(ptr)
    }
}
extension DeleteResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_DeleteResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_DeleteResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: DeleteResponse) {
        __swift_bridge__$Vec_DeleteResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_DeleteResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (DeleteResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DeleteResponseRef> {
        let pointer = __swift_bridge__$Vec_DeleteResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DeleteResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DeleteResponseRefMut> {
        let pointer = __swift_bridge__$Vec_DeleteResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DeleteResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<DeleteResponseRef> {
        UnsafePointer<DeleteResponseRef>(OpaquePointer(__swift_bridge__$Vec_DeleteResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_DeleteResponse$len(vecPtr)
    }
}


public class DeveloperMessage: DeveloperMessageRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$DeveloperMessage$_free(ptr)
        }
    }
}
extension DeveloperMessage {
    public convenience init<GenericIntoRustString: IntoRustString>(_ content: GenericIntoRustString, _ name: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$DeveloperMessage$new({ let rustString = content.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let rustString = optionalStringIntoRustString(name) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class DeveloperMessageRefMut: DeveloperMessageRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class DeveloperMessageRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension DeveloperMessageRef {
    public func content() -> RustString {
        RustString(ptr: __swift_bridge__$DeveloperMessage$content(ptr))
    }

    public func name() -> Optional<RustString> {
        { let val = __swift_bridge__$DeveloperMessage$name(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension DeveloperMessage: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_DeveloperMessage$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_DeveloperMessage$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: DeveloperMessage) {
        __swift_bridge__$Vec_DeveloperMessage$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_DeveloperMessage$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (DeveloperMessage(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DeveloperMessageRef> {
        let pointer = __swift_bridge__$Vec_DeveloperMessage$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DeveloperMessageRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DeveloperMessageRefMut> {
        let pointer = __swift_bridge__$Vec_DeveloperMessage$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DeveloperMessageRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<DeveloperMessageRef> {
        UnsafePointer<DeveloperMessageRef>(OpaquePointer(__swift_bridge__$Vec_DeveloperMessage$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_DeveloperMessage$len(vecPtr)
    }
}


public class DocumentContent: DocumentContentRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$DocumentContent$_free(ptr)
        }
    }
}
extension DocumentContent {
    public convenience init<GenericIntoRustString: IntoRustString>(_ data: GenericIntoRustString, _ media_type: GenericIntoRustString) {
        self.init(ptr: __swift_bridge__$DocumentContent$new({ let rustString = data.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = media_type.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class DocumentContentRefMut: DocumentContentRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class DocumentContentRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension DocumentContentRef {
    public func data() -> RustString {
        RustString(ptr: __swift_bridge__$DocumentContent$data(ptr))
    }

    public func mediaType() -> RustString {
        RustString(ptr: __swift_bridge__$DocumentContent$media_type(ptr))
    }
}
extension DocumentContent: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_DocumentContent$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_DocumentContent$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: DocumentContent) {
        __swift_bridge__$Vec_DocumentContent$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_DocumentContent$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (DocumentContent(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DocumentContentRef> {
        let pointer = __swift_bridge__$Vec_DocumentContent$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DocumentContentRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DocumentContentRefMut> {
        let pointer = __swift_bridge__$Vec_DocumentContent$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DocumentContentRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<DocumentContentRef> {
        UnsafePointer<DocumentContentRef>(OpaquePointer(__swift_bridge__$Vec_DocumentContent$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_DocumentContent$len(vecPtr)
    }
}


public class EmbeddingObject: EmbeddingObjectRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$EmbeddingObject$_free(ptr)
        }
    }
}
public class EmbeddingObjectRefMut: EmbeddingObjectRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class EmbeddingObjectRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension EmbeddingObjectRef {
    public func object() -> RustString {
        RustString(ptr: __swift_bridge__$EmbeddingObject$object(ptr))
    }

    public func embedding() -> RustVec<Float> {
        RustVec(ptr: __swift_bridge__$EmbeddingObject$embedding(ptr))
    }

    public func index() -> UInt32 {
        __swift_bridge__$EmbeddingObject$index(ptr)
    }
}
extension EmbeddingObject: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_EmbeddingObject$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_EmbeddingObject$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: EmbeddingObject) {
        __swift_bridge__$Vec_EmbeddingObject$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_EmbeddingObject$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (EmbeddingObject(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<EmbeddingObjectRef> {
        let pointer = __swift_bridge__$Vec_EmbeddingObject$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return EmbeddingObjectRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<EmbeddingObjectRefMut> {
        let pointer = __swift_bridge__$Vec_EmbeddingObject$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return EmbeddingObjectRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<EmbeddingObjectRef> {
        UnsafePointer<EmbeddingObjectRef>(OpaquePointer(__swift_bridge__$Vec_EmbeddingObject$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_EmbeddingObject$len(vecPtr)
    }
}


public class EmbeddingRequest: EmbeddingRequestRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$EmbeddingRequest$_free(ptr)
        }
    }
}
extension EmbeddingRequest {
    public convenience init<GenericIntoRustString: IntoRustString>(_ model: GenericIntoRustString, _ input: EmbeddingInput, _ encoding_format: Optional<EmbeddingFormat>, _ dimensions: Optional<UInt32>, _ user: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$EmbeddingRequest$new({ let rustString = model.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), {input.isOwned = false; return input.ptr;}(), { if let val = encoding_format { val.isOwned = false; return val.ptr } else { return nil } }(), dimensions.intoFfiRepr(), { if let rustString = optionalStringIntoRustString(user) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class EmbeddingRequestRefMut: EmbeddingRequestRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class EmbeddingRequestRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension EmbeddingRequestRef {
    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$EmbeddingRequest$model(ptr))
    }

    public func input() -> RustString {
        RustString(ptr: __swift_bridge__$EmbeddingRequest$input(ptr))
    }

    public func encodingFormat() -> Optional<RustString> {
        { let val = __swift_bridge__$EmbeddingRequest$encoding_format(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func dimensions() -> Optional<UInt32> {
        __swift_bridge__$EmbeddingRequest$dimensions(ptr).intoSwiftRepr()
    }

    public func user() -> Optional<RustString> {
        { let val = __swift_bridge__$EmbeddingRequest$user(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension EmbeddingRequest: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_EmbeddingRequest$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_EmbeddingRequest$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: EmbeddingRequest) {
        __swift_bridge__$Vec_EmbeddingRequest$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_EmbeddingRequest$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (EmbeddingRequest(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<EmbeddingRequestRef> {
        let pointer = __swift_bridge__$Vec_EmbeddingRequest$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return EmbeddingRequestRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<EmbeddingRequestRefMut> {
        let pointer = __swift_bridge__$Vec_EmbeddingRequest$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return EmbeddingRequestRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<EmbeddingRequestRef> {
        UnsafePointer<EmbeddingRequestRef>(OpaquePointer(__swift_bridge__$Vec_EmbeddingRequest$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_EmbeddingRequest$len(vecPtr)
    }
}


public class EmbeddingResponse: EmbeddingResponseRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$EmbeddingResponse$_free(ptr)
        }
    }
}
public class EmbeddingResponseRefMut: EmbeddingResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class EmbeddingResponseRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension EmbeddingResponseRef {
    public func object() -> RustString {
        RustString(ptr: __swift_bridge__$EmbeddingResponse$object(ptr))
    }

    public func data() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$EmbeddingResponse$data(ptr))
    }

    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$EmbeddingResponse$model(ptr))
    }

    public func usage() -> Optional<Usage> {
        { let val = __swift_bridge__$EmbeddingResponse$usage(ptr); if val != nil { return Usage(ptr: val!) } else { return nil } }()
    }
}
extension EmbeddingResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_EmbeddingResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_EmbeddingResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: EmbeddingResponse) {
        __swift_bridge__$Vec_EmbeddingResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_EmbeddingResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (EmbeddingResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<EmbeddingResponseRef> {
        let pointer = __swift_bridge__$Vec_EmbeddingResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return EmbeddingResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<EmbeddingResponseRefMut> {
        let pointer = __swift_bridge__$Vec_EmbeddingResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return EmbeddingResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<EmbeddingResponseRef> {
        UnsafePointer<EmbeddingResponseRef>(OpaquePointer(__swift_bridge__$Vec_EmbeddingResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_EmbeddingResponse$len(vecPtr)
    }
}


public class FileListQuery: FileListQueryRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$FileListQuery$_free(ptr)
        }
    }
}
extension FileListQuery {
    public convenience init<GenericIntoRustString: IntoRustString>(_ purpose: Optional<GenericIntoRustString>, _ limit: Optional<UInt32>, _ after: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$FileListQuery$new({ if let rustString = optionalStringIntoRustString(purpose) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), limit.intoFfiRepr(), { if let rustString = optionalStringIntoRustString(after) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class FileListQueryRefMut: FileListQueryRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class FileListQueryRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension FileListQueryRef {
    public func purpose() -> Optional<RustString> {
        { let val = __swift_bridge__$FileListQuery$purpose(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func limit() -> Optional<UInt32> {
        __swift_bridge__$FileListQuery$limit(ptr).intoSwiftRepr()
    }

    public func after() -> Optional<RustString> {
        { let val = __swift_bridge__$FileListQuery$after(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension FileListQuery: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_FileListQuery$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_FileListQuery$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: FileListQuery) {
        __swift_bridge__$Vec_FileListQuery$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_FileListQuery$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (FileListQuery(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FileListQueryRef> {
        let pointer = __swift_bridge__$Vec_FileListQuery$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FileListQueryRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FileListQueryRefMut> {
        let pointer = __swift_bridge__$Vec_FileListQuery$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FileListQueryRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<FileListQueryRef> {
        UnsafePointer<FileListQueryRef>(OpaquePointer(__swift_bridge__$Vec_FileListQuery$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_FileListQuery$len(vecPtr)
    }
}


public class FileListResponse: FileListResponseRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$FileListResponse$_free(ptr)
        }
    }
}
extension FileListResponse {
    public convenience init<GenericIntoRustString: IntoRustString>(_ object: GenericIntoRustString, _ data: RustVec<FileObject>, _ has_more: Optional<Bool>) {
        self.init(ptr: __swift_bridge__$FileListResponse$new({ let rustString = object.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let val = data; val.isOwned = false; return val.ptr }(), has_more.intoFfiRepr()))
    }
}
public class FileListResponseRefMut: FileListResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class FileListResponseRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension FileListResponseRef {
    public func object() -> RustString {
        RustString(ptr: __swift_bridge__$FileListResponse$object(ptr))
    }

    public func data() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$FileListResponse$data(ptr))
    }

    public func hasMore() -> Optional<Bool> {
        __swift_bridge__$FileListResponse$has_more(ptr).intoSwiftRepr()
    }
}
extension FileListResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_FileListResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_FileListResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: FileListResponse) {
        __swift_bridge__$Vec_FileListResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_FileListResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (FileListResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FileListResponseRef> {
        let pointer = __swift_bridge__$Vec_FileListResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FileListResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FileListResponseRefMut> {
        let pointer = __swift_bridge__$Vec_FileListResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FileListResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<FileListResponseRef> {
        UnsafePointer<FileListResponseRef>(OpaquePointer(__swift_bridge__$Vec_FileListResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_FileListResponse$len(vecPtr)
    }
}


public class FileObject: FileObjectRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$FileObject$_free(ptr)
        }
    }
}
extension FileObject {
    public convenience init<GenericIntoRustString: IntoRustString>(_ id: GenericIntoRustString, _ object: GenericIntoRustString, _ bytes: UInt64, _ created_at: UInt64, _ filename: GenericIntoRustString, _ purpose: GenericIntoRustString, _ status: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$FileObject$new({ let rustString = id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = object.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), bytes, created_at, { let rustString = filename.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = purpose.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let rustString = optionalStringIntoRustString(status) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class FileObjectRefMut: FileObjectRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class FileObjectRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension FileObjectRef {
    public func id() -> RustString {
        RustString(ptr: __swift_bridge__$FileObject$id(ptr))
    }

    public func object() -> RustString {
        RustString(ptr: __swift_bridge__$FileObject$object(ptr))
    }

    public func bytes() -> UInt64 {
        __swift_bridge__$FileObject$bytes(ptr)
    }

    public func createdAt() -> UInt64 {
        __swift_bridge__$FileObject$created_at(ptr)
    }

    public func filename() -> RustString {
        RustString(ptr: __swift_bridge__$FileObject$filename(ptr))
    }

    public func purpose() -> RustString {
        RustString(ptr: __swift_bridge__$FileObject$purpose(ptr))
    }

    public func status() -> Optional<RustString> {
        { let val = __swift_bridge__$FileObject$status(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension FileObject: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_FileObject$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_FileObject$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: FileObject) {
        __swift_bridge__$Vec_FileObject$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_FileObject$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (FileObject(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FileObjectRef> {
        let pointer = __swift_bridge__$Vec_FileObject$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FileObjectRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FileObjectRefMut> {
        let pointer = __swift_bridge__$Vec_FileObject$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FileObjectRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<FileObjectRef> {
        UnsafePointer<FileObjectRef>(OpaquePointer(__swift_bridge__$Vec_FileObject$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_FileObject$len(vecPtr)
    }
}


public class FunctionCall: FunctionCallRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$FunctionCall$_free(ptr)
        }
    }
}
public class FunctionCallRefMut: FunctionCallRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class FunctionCallRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension FunctionCallRef {
    public func name() -> RustString {
        RustString(ptr: __swift_bridge__$FunctionCall$name(ptr))
    }

    public func arguments() -> RustString {
        RustString(ptr: __swift_bridge__$FunctionCall$arguments(ptr))
    }
}
extension FunctionCall: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_FunctionCall$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_FunctionCall$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: FunctionCall) {
        __swift_bridge__$Vec_FunctionCall$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_FunctionCall$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (FunctionCall(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FunctionCallRef> {
        let pointer = __swift_bridge__$Vec_FunctionCall$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FunctionCallRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FunctionCallRefMut> {
        let pointer = __swift_bridge__$Vec_FunctionCall$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FunctionCallRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<FunctionCallRef> {
        UnsafePointer<FunctionCallRef>(OpaquePointer(__swift_bridge__$Vec_FunctionCall$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_FunctionCall$len(vecPtr)
    }
}


public class FunctionDefinition: FunctionDefinitionRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$FunctionDefinition$_free(ptr)
        }
    }
}
public class FunctionDefinitionRefMut: FunctionDefinitionRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class FunctionDefinitionRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension FunctionDefinitionRef {
    public func name() -> RustString {
        RustString(ptr: __swift_bridge__$FunctionDefinition$name(ptr))
    }

    public func description() -> Optional<RustString> {
        { let val = __swift_bridge__$FunctionDefinition$description(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func parameters() -> Optional<RustString> {
        { let val = __swift_bridge__$FunctionDefinition$parameters(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func strict() -> Optional<Bool> {
        __swift_bridge__$FunctionDefinition$strict(ptr).intoSwiftRepr()
    }
}
extension FunctionDefinition: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_FunctionDefinition$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_FunctionDefinition$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: FunctionDefinition) {
        __swift_bridge__$Vec_FunctionDefinition$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_FunctionDefinition$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (FunctionDefinition(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FunctionDefinitionRef> {
        let pointer = __swift_bridge__$Vec_FunctionDefinition$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FunctionDefinitionRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FunctionDefinitionRefMut> {
        let pointer = __swift_bridge__$Vec_FunctionDefinition$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FunctionDefinitionRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<FunctionDefinitionRef> {
        UnsafePointer<FunctionDefinitionRef>(OpaquePointer(__swift_bridge__$Vec_FunctionDefinition$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_FunctionDefinition$len(vecPtr)
    }
}


public class FunctionMessage: FunctionMessageRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$FunctionMessage$_free(ptr)
        }
    }
}
extension FunctionMessage {
    public convenience init<GenericIntoRustString: IntoRustString>(_ content: GenericIntoRustString, _ name: GenericIntoRustString) {
        self.init(ptr: __swift_bridge__$FunctionMessage$new({ let rustString = content.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class FunctionMessageRefMut: FunctionMessageRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class FunctionMessageRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension FunctionMessageRef {
    public func content() -> RustString {
        RustString(ptr: __swift_bridge__$FunctionMessage$content(ptr))
    }

    public func name() -> RustString {
        RustString(ptr: __swift_bridge__$FunctionMessage$name(ptr))
    }
}
extension FunctionMessage: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_FunctionMessage$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_FunctionMessage$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: FunctionMessage) {
        __swift_bridge__$Vec_FunctionMessage$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_FunctionMessage$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (FunctionMessage(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FunctionMessageRef> {
        let pointer = __swift_bridge__$Vec_FunctionMessage$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FunctionMessageRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FunctionMessageRefMut> {
        let pointer = __swift_bridge__$Vec_FunctionMessage$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FunctionMessageRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<FunctionMessageRef> {
        UnsafePointer<FunctionMessageRef>(OpaquePointer(__swift_bridge__$Vec_FunctionMessage$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_FunctionMessage$len(vecPtr)
    }
}


public class Image: ImageRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Image$_free(ptr)
        }
    }
}
extension Image {
    public convenience init<GenericIntoRustString: IntoRustString>(_ url: Optional<GenericIntoRustString>, _ b64_json: Optional<GenericIntoRustString>, _ revised_prompt: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$Image$new({ if let rustString = optionalStringIntoRustString(url) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(b64_json) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(revised_prompt) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class ImageRefMut: ImageRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ImageRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ImageRef {
    public func url() -> Optional<RustString> {
        { let val = __swift_bridge__$Image$url(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func b64Json() -> Optional<RustString> {
        { let val = __swift_bridge__$Image$b64_json(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func revisedPrompt() -> Optional<RustString> {
        { let val = __swift_bridge__$Image$revised_prompt(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension Image: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Image$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Image$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Image) {
        __swift_bridge__$Vec_Image$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Image$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Image(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ImageRef> {
        let pointer = __swift_bridge__$Vec_Image$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ImageRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ImageRefMut> {
        let pointer = __swift_bridge__$Vec_Image$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ImageRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ImageRef> {
        UnsafePointer<ImageRef>(OpaquePointer(__swift_bridge__$Vec_Image$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Image$len(vecPtr)
    }
}


public class ImageUrl: ImageUrlRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ImageUrl$_free(ptr)
        }
    }
}
extension ImageUrl {
    public convenience init<GenericIntoRustString: IntoRustString>(_ url: GenericIntoRustString, _ detail: Optional<ImageDetail>) {
        self.init(ptr: __swift_bridge__$ImageUrl$new({ let rustString = url.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let val = detail { val.isOwned = false; return val.ptr } else { return nil } }()))
    }
}
public class ImageUrlRefMut: ImageUrlRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ImageUrlRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ImageUrlRef {
    public func url() -> RustString {
        RustString(ptr: __swift_bridge__$ImageUrl$url(ptr))
    }

    public func detail() -> Optional<RustString> {
        { let val = __swift_bridge__$ImageUrl$detail(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension ImageUrl: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ImageUrl$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ImageUrl$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ImageUrl) {
        __swift_bridge__$Vec_ImageUrl$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ImageUrl$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ImageUrl(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ImageUrlRef> {
        let pointer = __swift_bridge__$Vec_ImageUrl$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ImageUrlRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ImageUrlRefMut> {
        let pointer = __swift_bridge__$Vec_ImageUrl$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ImageUrlRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ImageUrlRef> {
        UnsafePointer<ImageUrlRef>(OpaquePointer(__swift_bridge__$Vec_ImageUrl$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ImageUrl$len(vecPtr)
    }
}


public class ImagesResponse: ImagesResponseRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ImagesResponse$_free(ptr)
        }
    }
}
extension ImagesResponse {
    public convenience init(_ created: UInt64, _ data: RustVec<Image>) {
        self.init(ptr: __swift_bridge__$ImagesResponse$new(created, { let val = data; val.isOwned = false; return val.ptr }()))
    }
}
public class ImagesResponseRefMut: ImagesResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ImagesResponseRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ImagesResponseRef {
    public func created() -> UInt64 {
        __swift_bridge__$ImagesResponse$created(ptr)
    }

    public func data() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$ImagesResponse$data(ptr))
    }
}
extension ImagesResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ImagesResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ImagesResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ImagesResponse) {
        __swift_bridge__$Vec_ImagesResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ImagesResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ImagesResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ImagesResponseRef> {
        let pointer = __swift_bridge__$Vec_ImagesResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ImagesResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ImagesResponseRefMut> {
        let pointer = __swift_bridge__$Vec_ImagesResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ImagesResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ImagesResponseRef> {
        UnsafePointer<ImagesResponseRef>(OpaquePointer(__swift_bridge__$Vec_ImagesResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ImagesResponse$len(vecPtr)
    }
}


public class InFlightLimitConfig: InFlightLimitConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$InFlightLimitConfig$_free(ptr)
        }
    }
}
extension InFlightLimitConfig {
    public convenience init(_ max_in_flight: Optional<UInt>) {
        self.init(ptr: __swift_bridge__$InFlightLimitConfig$new(max_in_flight.intoFfiRepr()))
    }
}
public class InFlightLimitConfigRefMut: InFlightLimitConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class InFlightLimitConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension InFlightLimitConfigRef {
    public func maxInFlight() -> Optional<UInt> {
        __swift_bridge__$InFlightLimitConfig$max_in_flight(ptr).intoSwiftRepr()
    }
}
extension InFlightLimitConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_InFlightLimitConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_InFlightLimitConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: InFlightLimitConfig) {
        __swift_bridge__$Vec_InFlightLimitConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_InFlightLimitConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (InFlightLimitConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<InFlightLimitConfigRef> {
        let pointer = __swift_bridge__$Vec_InFlightLimitConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return InFlightLimitConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<InFlightLimitConfigRefMut> {
        let pointer = __swift_bridge__$Vec_InFlightLimitConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return InFlightLimitConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<InFlightLimitConfigRef> {
        UnsafePointer<InFlightLimitConfigRef>(OpaquePointer(__swift_bridge__$Vec_InFlightLimitConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_InFlightLimitConfig$len(vecPtr)
    }
}


public class IntentPrototype: IntentPrototypeRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IntentPrototype$_free(ptr)
        }
    }
}
public class IntentPrototypeRefMut: IntentPrototypeRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IntentPrototypeRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IntentPrototypeRef {
    public func name() -> RustString {
        RustString(ptr: __swift_bridge__$IntentPrototype$name(ptr))
    }

    public func embedding() -> RustVec<Double> {
        RustVec(ptr: __swift_bridge__$IntentPrototype$embedding(ptr))
    }

    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$IntentPrototype$model(ptr))
    }
}
extension IntentPrototype: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IntentPrototype$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IntentPrototype$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IntentPrototype) {
        __swift_bridge__$Vec_IntentPrototype$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IntentPrototype$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IntentPrototype(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IntentPrototypeRef> {
        let pointer = __swift_bridge__$Vec_IntentPrototype$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IntentPrototypeRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IntentPrototypeRefMut> {
        let pointer = __swift_bridge__$Vec_IntentPrototype$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IntentPrototypeRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IntentPrototypeRef> {
        UnsafePointer<IntentPrototypeRef>(OpaquePointer(__swift_bridge__$Vec_IntentPrototype$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IntentPrototype$len(vecPtr)
    }
}


public class JsonSchemaFormat: JsonSchemaFormatRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$JsonSchemaFormat$_free(ptr)
        }
    }
}
extension JsonSchemaFormat {
    public convenience init<GenericIntoRustString: IntoRustString>(_ name: GenericIntoRustString, _ description: Optional<GenericIntoRustString>, _ schema: GenericIntoRustString, _ strict: Optional<Bool>) {
        self.init(ptr: __swift_bridge__$JsonSchemaFormat$new({ let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let rustString = optionalStringIntoRustString(description) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { let rustString = schema.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), strict.intoFfiRepr()))
    }
}
public class JsonSchemaFormatRefMut: JsonSchemaFormatRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class JsonSchemaFormatRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension JsonSchemaFormatRef {
    public func name() -> RustString {
        RustString(ptr: __swift_bridge__$JsonSchemaFormat$name(ptr))
    }

    public func description() -> Optional<RustString> {
        { let val = __swift_bridge__$JsonSchemaFormat$description(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func schema() -> RustString {
        RustString(ptr: __swift_bridge__$JsonSchemaFormat$schema(ptr))
    }

    public func strict() -> Optional<Bool> {
        __swift_bridge__$JsonSchemaFormat$strict(ptr).intoSwiftRepr()
    }
}
extension JsonSchemaFormat: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_JsonSchemaFormat$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_JsonSchemaFormat$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: JsonSchemaFormat) {
        __swift_bridge__$Vec_JsonSchemaFormat$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_JsonSchemaFormat$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (JsonSchemaFormat(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<JsonSchemaFormatRef> {
        let pointer = __swift_bridge__$Vec_JsonSchemaFormat$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return JsonSchemaFormatRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<JsonSchemaFormatRefMut> {
        let pointer = __swift_bridge__$Vec_JsonSchemaFormat$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return JsonSchemaFormatRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<JsonSchemaFormatRef> {
        UnsafePointer<JsonSchemaFormatRef>(OpaquePointer(__swift_bridge__$Vec_JsonSchemaFormat$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_JsonSchemaFormat$len(vecPtr)
    }
}


public class LlmBudgetConfig: LlmBudgetConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$LlmBudgetConfig$_free(ptr)
        }
    }
}
public class LlmBudgetConfigRefMut: LlmBudgetConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class LlmBudgetConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension LlmBudgetConfigRef {
    public func globalLimit() -> Optional<Double> {
        __swift_bridge__$LlmBudgetConfig$global_limit(ptr).intoSwiftRepr()
    }

    public func modelLimits() -> RustString {
        RustString(ptr: __swift_bridge__$LlmBudgetConfig$model_limits(ptr))
    }

    public func enforcement() -> Optional<RustString> {
        { let val = __swift_bridge__$LlmBudgetConfig$enforcement(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension LlmBudgetConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_LlmBudgetConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_LlmBudgetConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: LlmBudgetConfig) {
        __swift_bridge__$Vec_LlmBudgetConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_LlmBudgetConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (LlmBudgetConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LlmBudgetConfigRef> {
        let pointer = __swift_bridge__$Vec_LlmBudgetConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LlmBudgetConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LlmBudgetConfigRefMut> {
        let pointer = __swift_bridge__$Vec_LlmBudgetConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LlmBudgetConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<LlmBudgetConfigRef> {
        UnsafePointer<LlmBudgetConfigRef>(OpaquePointer(__swift_bridge__$Vec_LlmBudgetConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_LlmBudgetConfig$len(vecPtr)
    }
}


public class LlmCacheConfig: LlmCacheConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$LlmCacheConfig$_free(ptr)
        }
    }
}
public class LlmCacheConfigRefMut: LlmCacheConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class LlmCacheConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension LlmCacheConfigRef {
    public func maxEntries() -> Optional<UInt> {
        __swift_bridge__$LlmCacheConfig$max_entries(ptr).intoSwiftRepr()
    }

    public func ttlSeconds() -> Optional<UInt64> {
        __swift_bridge__$LlmCacheConfig$ttl_seconds(ptr).intoSwiftRepr()
    }

    public func backend() -> Optional<RustString> {
        { let val = __swift_bridge__$LlmCacheConfig$backend(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func backendConfig() -> RustString {
        RustString(ptr: __swift_bridge__$LlmCacheConfig$backend_config(ptr))
    }
}
extension LlmCacheConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_LlmCacheConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_LlmCacheConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: LlmCacheConfig) {
        __swift_bridge__$Vec_LlmCacheConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_LlmCacheConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (LlmCacheConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LlmCacheConfigRef> {
        let pointer = __swift_bridge__$Vec_LlmCacheConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LlmCacheConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LlmCacheConfigRefMut> {
        let pointer = __swift_bridge__$Vec_LlmCacheConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LlmCacheConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<LlmCacheConfigRef> {
        UnsafePointer<LlmCacheConfigRef>(OpaquePointer(__swift_bridge__$Vec_LlmCacheConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_LlmCacheConfig$len(vecPtr)
    }
}


public class LlmConfig: LlmConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$LlmConfig$_free(ptr)
        }
    }
}
public class LlmConfigRefMut: LlmConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class LlmConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension LlmConfigRef {
    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$LlmConfig$model(ptr))
    }

    public func apiKey() -> Optional<RustString> {
        { let val = __swift_bridge__$LlmConfig$api_key(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func baseUrl() -> Optional<RustString> {
        { let val = __swift_bridge__$LlmConfig$base_url(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func timeoutSecs() -> Optional<UInt64> {
        __swift_bridge__$LlmConfig$timeout_secs(ptr).intoSwiftRepr()
    }

    public func maxRetries() -> Optional<UInt32> {
        __swift_bridge__$LlmConfig$max_retries(ptr).intoSwiftRepr()
    }

    public func temperature() -> Optional<Double> {
        __swift_bridge__$LlmConfig$temperature(ptr).intoSwiftRepr()
    }

    public func maxTokens() -> Optional<UInt64> {
        __swift_bridge__$LlmConfig$max_tokens(ptr).intoSwiftRepr()
    }

    public func loadEnv() -> Optional<Bool> {
        __swift_bridge__$LlmConfig$load_env(ptr).intoSwiftRepr()
    }

    public func headers() -> RustString {
        RustString(ptr: __swift_bridge__$LlmConfig$headers(ptr))
    }

    public func providers() -> RustString {
        RustString(ptr: __swift_bridge__$LlmConfig$providers(ptr))
    }

    public func cache() -> Optional<LlmCacheConfig> {
        { let val = __swift_bridge__$LlmConfig$cache(ptr); if val != nil { return LlmCacheConfig(ptr: val!) } else { return nil } }()
    }

    public func budget() -> Optional<LlmBudgetConfig> {
        { let val = __swift_bridge__$LlmConfig$budget(ptr); if val != nil { return LlmBudgetConfig(ptr: val!) } else { return nil } }()
    }

    public func rateLimit() -> Optional<LlmRateLimitConfig> {
        { let val = __swift_bridge__$LlmConfig$rate_limit(ptr); if val != nil { return LlmRateLimitConfig(ptr: val!) } else { return nil } }()
    }

    public func inFlightLimit() -> Optional<LlmInFlightLimitConfig> {
        { let val = __swift_bridge__$LlmConfig$in_flight_limit(ptr); if val != nil { return LlmInFlightLimitConfig(ptr: val!) } else { return nil } }()
    }

    public func costTracking() -> Optional<Bool> {
        __swift_bridge__$LlmConfig$cost_tracking(ptr).intoSwiftRepr()
    }

    public func tracing() -> Optional<Bool> {
        __swift_bridge__$LlmConfig$tracing(ptr).intoSwiftRepr()
    }

    public func cooldownSecs() -> Optional<UInt64> {
        __swift_bridge__$LlmConfig$cooldown_secs(ptr).intoSwiftRepr()
    }

    public func healthCheckSecs() -> Optional<UInt64> {
        __swift_bridge__$LlmConfig$health_check_secs(ptr).intoSwiftRepr()
    }

    public func bedrock() -> Optional<BedrockConfig> {
        { let val = __swift_bridge__$LlmConfig$bedrock(ptr); if val != nil { return BedrockConfig(ptr: val!) } else { return nil } }()
    }
}
extension LlmConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_LlmConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_LlmConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: LlmConfig) {
        __swift_bridge__$Vec_LlmConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_LlmConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (LlmConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LlmConfigRef> {
        let pointer = __swift_bridge__$Vec_LlmConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LlmConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LlmConfigRefMut> {
        let pointer = __swift_bridge__$Vec_LlmConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LlmConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<LlmConfigRef> {
        UnsafePointer<LlmConfigRef>(OpaquePointer(__swift_bridge__$Vec_LlmConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_LlmConfig$len(vecPtr)
    }
}


public class LlmInFlightLimitConfig: LlmInFlightLimitConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$LlmInFlightLimitConfig$_free(ptr)
        }
    }
}
extension LlmInFlightLimitConfig {
    public convenience init(_ max_in_flight: Optional<UInt>) {
        self.init(ptr: __swift_bridge__$LlmInFlightLimitConfig$new(max_in_flight.intoFfiRepr()))
    }
}
public class LlmInFlightLimitConfigRefMut: LlmInFlightLimitConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class LlmInFlightLimitConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension LlmInFlightLimitConfigRef {
    public func maxInFlight() -> Optional<UInt> {
        __swift_bridge__$LlmInFlightLimitConfig$max_in_flight(ptr).intoSwiftRepr()
    }
}
extension LlmInFlightLimitConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_LlmInFlightLimitConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_LlmInFlightLimitConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: LlmInFlightLimitConfig) {
        __swift_bridge__$Vec_LlmInFlightLimitConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_LlmInFlightLimitConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (LlmInFlightLimitConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LlmInFlightLimitConfigRef> {
        let pointer = __swift_bridge__$Vec_LlmInFlightLimitConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LlmInFlightLimitConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LlmInFlightLimitConfigRefMut> {
        let pointer = __swift_bridge__$Vec_LlmInFlightLimitConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LlmInFlightLimitConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<LlmInFlightLimitConfigRef> {
        UnsafePointer<LlmInFlightLimitConfigRef>(OpaquePointer(__swift_bridge__$Vec_LlmInFlightLimitConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_LlmInFlightLimitConfig$len(vecPtr)
    }
}


public class LlmProviderConfig: LlmProviderConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$LlmProviderConfig$_free(ptr)
        }
    }
}
extension LlmProviderConfig {
    public convenience init<GenericIntoRustString: IntoRustString>(_ name: GenericIntoRustString, _ base_url: GenericIntoRustString, _ auth_header: Optional<GenericIntoRustString>, _ model_prefixes: RustVec<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$LlmProviderConfig$new({ let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = base_url.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let rustString = optionalStringIntoRustString(auth_header) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { let val = model_prefixes; val.isOwned = false; return val.ptr }()))
    }
}
public class LlmProviderConfigRefMut: LlmProviderConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class LlmProviderConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension LlmProviderConfigRef {
    public func name() -> RustString {
        RustString(ptr: __swift_bridge__$LlmProviderConfig$name(ptr))
    }

    public func baseUrl() -> RustString {
        RustString(ptr: __swift_bridge__$LlmProviderConfig$base_url(ptr))
    }

    public func authHeader() -> Optional<RustString> {
        { let val = __swift_bridge__$LlmProviderConfig$auth_header(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func modelPrefixes() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$LlmProviderConfig$model_prefixes(ptr))
    }
}
extension LlmProviderConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_LlmProviderConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_LlmProviderConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: LlmProviderConfig) {
        __swift_bridge__$Vec_LlmProviderConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_LlmProviderConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (LlmProviderConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LlmProviderConfigRef> {
        let pointer = __swift_bridge__$Vec_LlmProviderConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LlmProviderConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LlmProviderConfigRefMut> {
        let pointer = __swift_bridge__$Vec_LlmProviderConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LlmProviderConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<LlmProviderConfigRef> {
        UnsafePointer<LlmProviderConfigRef>(OpaquePointer(__swift_bridge__$Vec_LlmProviderConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_LlmProviderConfig$len(vecPtr)
    }
}


public class LlmRateLimitConfig: LlmRateLimitConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$LlmRateLimitConfig$_free(ptr)
        }
    }
}
extension LlmRateLimitConfig {
    public convenience init(_ rpm: Optional<UInt32>, _ tpm: Optional<UInt64>, _ window_seconds: Optional<UInt64>) {
        self.init(ptr: __swift_bridge__$LlmRateLimitConfig$new(rpm.intoFfiRepr(), tpm.intoFfiRepr(), window_seconds.intoFfiRepr()))
    }
}
public class LlmRateLimitConfigRefMut: LlmRateLimitConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class LlmRateLimitConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension LlmRateLimitConfigRef {
    public func rpm() -> Optional<UInt32> {
        __swift_bridge__$LlmRateLimitConfig$rpm(ptr).intoSwiftRepr()
    }

    public func tpm() -> Optional<UInt64> {
        __swift_bridge__$LlmRateLimitConfig$tpm(ptr).intoSwiftRepr()
    }

    public func windowSeconds() -> Optional<UInt64> {
        __swift_bridge__$LlmRateLimitConfig$window_seconds(ptr).intoSwiftRepr()
    }
}
extension LlmRateLimitConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_LlmRateLimitConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_LlmRateLimitConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: LlmRateLimitConfig) {
        __swift_bridge__$Vec_LlmRateLimitConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_LlmRateLimitConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (LlmRateLimitConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LlmRateLimitConfigRef> {
        let pointer = __swift_bridge__$Vec_LlmRateLimitConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LlmRateLimitConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LlmRateLimitConfigRefMut> {
        let pointer = __swift_bridge__$Vec_LlmRateLimitConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LlmRateLimitConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<LlmRateLimitConfigRef> {
        UnsafePointer<LlmRateLimitConfigRef>(OpaquePointer(__swift_bridge__$Vec_LlmRateLimitConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_LlmRateLimitConfig$len(vecPtr)
    }
}


public class ModelInfo: ModelInfoRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ModelInfo$_free(ptr)
        }
    }
}
extension ModelInfo {
    public convenience init<GenericIntoRustString: IntoRustString>(_ input_cost_per_token: Double, _ output_cost_per_token: Double, _ cache_read_input_token_cost: Optional<Double>, _ cache_creation_input_token_cost: Optional<Double>, _ input_cost_per_audio_token: Optional<Double>, _ output_cost_per_audio_token: Optional<Double>, _ output_cost_per_reasoning_token: Optional<Double>, _ max_tokens: Optional<UInt64>, _ max_input_tokens: Optional<UInt64>, _ max_output_tokens: Optional<UInt64>, _ mode: Optional<GenericIntoRustString>, _ supports_vision: Optional<Bool>, _ supports_function_calling: Optional<Bool>, _ supports_reasoning: Optional<Bool>, _ supports_structured_output: Optional<Bool>, _ supports_audio_input: Optional<Bool>, _ supports_audio_output: Optional<Bool>, _ supports_prompt_caching: Optional<Bool>, _ tiers: RustVec<ModelTier>) {
        self.init(ptr: __swift_bridge__$ModelInfo$new(input_cost_per_token, output_cost_per_token, cache_read_input_token_cost.intoFfiRepr(), cache_creation_input_token_cost.intoFfiRepr(), input_cost_per_audio_token.intoFfiRepr(), output_cost_per_audio_token.intoFfiRepr(), output_cost_per_reasoning_token.intoFfiRepr(), max_tokens.intoFfiRepr(), max_input_tokens.intoFfiRepr(), max_output_tokens.intoFfiRepr(), { if let rustString = optionalStringIntoRustString(mode) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), supports_vision.intoFfiRepr(), supports_function_calling.intoFfiRepr(), supports_reasoning.intoFfiRepr(), supports_structured_output.intoFfiRepr(), supports_audio_input.intoFfiRepr(), supports_audio_output.intoFfiRepr(), supports_prompt_caching.intoFfiRepr(), { let val = tiers; val.isOwned = false; return val.ptr }()))
    }
}
public class ModelInfoRefMut: ModelInfoRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ModelInfoRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ModelInfoRef {
    public func inputCostPerToken() -> Double {
        __swift_bridge__$ModelInfo$input_cost_per_token(ptr)
    }

    public func outputCostPerToken() -> Double {
        __swift_bridge__$ModelInfo$output_cost_per_token(ptr)
    }

    public func cacheReadInputTokenCost() -> Optional<Double> {
        __swift_bridge__$ModelInfo$cache_read_input_token_cost(ptr).intoSwiftRepr()
    }

    public func cacheCreationInputTokenCost() -> Optional<Double> {
        __swift_bridge__$ModelInfo$cache_creation_input_token_cost(ptr).intoSwiftRepr()
    }

    public func inputCostPerAudioToken() -> Optional<Double> {
        __swift_bridge__$ModelInfo$input_cost_per_audio_token(ptr).intoSwiftRepr()
    }

    public func outputCostPerAudioToken() -> Optional<Double> {
        __swift_bridge__$ModelInfo$output_cost_per_audio_token(ptr).intoSwiftRepr()
    }

    public func outputCostPerReasoningToken() -> Optional<Double> {
        __swift_bridge__$ModelInfo$output_cost_per_reasoning_token(ptr).intoSwiftRepr()
    }

    public func maxTokens() -> Optional<UInt64> {
        __swift_bridge__$ModelInfo$max_tokens(ptr).intoSwiftRepr()
    }

    public func maxInputTokens() -> Optional<UInt64> {
        __swift_bridge__$ModelInfo$max_input_tokens(ptr).intoSwiftRepr()
    }

    public func maxOutputTokens() -> Optional<UInt64> {
        __swift_bridge__$ModelInfo$max_output_tokens(ptr).intoSwiftRepr()
    }

    public func mode() -> Optional<RustString> {
        { let val = __swift_bridge__$ModelInfo$mode(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func supportsVision() -> Optional<Bool> {
        __swift_bridge__$ModelInfo$supports_vision(ptr).intoSwiftRepr()
    }

    public func supportsFunctionCalling() -> Optional<Bool> {
        __swift_bridge__$ModelInfo$supports_function_calling(ptr).intoSwiftRepr()
    }

    public func supportsReasoning() -> Optional<Bool> {
        __swift_bridge__$ModelInfo$supports_reasoning(ptr).intoSwiftRepr()
    }

    public func supportsStructuredOutput() -> Optional<Bool> {
        __swift_bridge__$ModelInfo$supports_structured_output(ptr).intoSwiftRepr()
    }

    public func supportsAudioInput() -> Optional<Bool> {
        __swift_bridge__$ModelInfo$supports_audio_input(ptr).intoSwiftRepr()
    }

    public func supportsAudioOutput() -> Optional<Bool> {
        __swift_bridge__$ModelInfo$supports_audio_output(ptr).intoSwiftRepr()
    }

    public func supportsPromptCaching() -> Optional<Bool> {
        __swift_bridge__$ModelInfo$supports_prompt_caching(ptr).intoSwiftRepr()
    }

    public func tiers() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$ModelInfo$tiers(ptr))
    }
}
extension ModelInfo: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ModelInfo$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ModelInfo$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ModelInfo) {
        __swift_bridge__$Vec_ModelInfo$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ModelInfo$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ModelInfo(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModelInfoRef> {
        let pointer = __swift_bridge__$Vec_ModelInfo$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModelInfoRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModelInfoRefMut> {
        let pointer = __swift_bridge__$Vec_ModelInfo$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModelInfoRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ModelInfoRef> {
        UnsafePointer<ModelInfoRef>(OpaquePointer(__swift_bridge__$Vec_ModelInfo$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ModelInfo$len(vecPtr)
    }
}


public class ModelObject: ModelObjectRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ModelObject$_free(ptr)
        }
    }
}
extension ModelObject {
    public convenience init<GenericIntoRustString: IntoRustString>(_ id: GenericIntoRustString, _ object: GenericIntoRustString, _ created: UInt64, _ owned_by: GenericIntoRustString) {
        self.init(ptr: __swift_bridge__$ModelObject$new({ let rustString = id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = object.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), created, { let rustString = owned_by.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class ModelObjectRefMut: ModelObjectRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ModelObjectRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ModelObjectRef {
    public func id() -> RustString {
        RustString(ptr: __swift_bridge__$ModelObject$id(ptr))
    }

    public func object() -> RustString {
        RustString(ptr: __swift_bridge__$ModelObject$object(ptr))
    }

    public func created() -> UInt64 {
        __swift_bridge__$ModelObject$created(ptr)
    }

    public func ownedBy() -> RustString {
        RustString(ptr: __swift_bridge__$ModelObject$owned_by(ptr))
    }
}
extension ModelObject: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ModelObject$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ModelObject$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ModelObject) {
        __swift_bridge__$Vec_ModelObject$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ModelObject$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ModelObject(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModelObjectRef> {
        let pointer = __swift_bridge__$Vec_ModelObject$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModelObjectRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModelObjectRefMut> {
        let pointer = __swift_bridge__$Vec_ModelObject$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModelObjectRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ModelObjectRef> {
        UnsafePointer<ModelObjectRef>(OpaquePointer(__swift_bridge__$Vec_ModelObject$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ModelObject$len(vecPtr)
    }
}


public class ModelTier: ModelTierRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ModelTier$_free(ptr)
        }
    }
}
extension ModelTier {
    public convenience init(_ min_context_tokens: UInt64, _ input_cost_per_token: Double, _ output_cost_per_token: Double, _ cache_read_input_token_cost: Optional<Double>, _ cache_creation_input_token_cost: Optional<Double>, _ input_cost_per_audio_token: Optional<Double>, _ output_cost_per_audio_token: Optional<Double>, _ output_cost_per_reasoning_token: Optional<Double>) {
        self.init(ptr: __swift_bridge__$ModelTier$new(min_context_tokens, input_cost_per_token, output_cost_per_token, cache_read_input_token_cost.intoFfiRepr(), cache_creation_input_token_cost.intoFfiRepr(), input_cost_per_audio_token.intoFfiRepr(), output_cost_per_audio_token.intoFfiRepr(), output_cost_per_reasoning_token.intoFfiRepr()))
    }
}
public class ModelTierRefMut: ModelTierRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ModelTierRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ModelTierRef {
    public func minContextTokens() -> UInt64 {
        __swift_bridge__$ModelTier$min_context_tokens(ptr)
    }

    public func inputCostPerToken() -> Double {
        __swift_bridge__$ModelTier$input_cost_per_token(ptr)
    }

    public func outputCostPerToken() -> Double {
        __swift_bridge__$ModelTier$output_cost_per_token(ptr)
    }

    public func cacheReadInputTokenCost() -> Optional<Double> {
        __swift_bridge__$ModelTier$cache_read_input_token_cost(ptr).intoSwiftRepr()
    }

    public func cacheCreationInputTokenCost() -> Optional<Double> {
        __swift_bridge__$ModelTier$cache_creation_input_token_cost(ptr).intoSwiftRepr()
    }

    public func inputCostPerAudioToken() -> Optional<Double> {
        __swift_bridge__$ModelTier$input_cost_per_audio_token(ptr).intoSwiftRepr()
    }

    public func outputCostPerAudioToken() -> Optional<Double> {
        __swift_bridge__$ModelTier$output_cost_per_audio_token(ptr).intoSwiftRepr()
    }

    public func outputCostPerReasoningToken() -> Optional<Double> {
        __swift_bridge__$ModelTier$output_cost_per_reasoning_token(ptr).intoSwiftRepr()
    }
}
extension ModelTier: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ModelTier$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ModelTier$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ModelTier) {
        __swift_bridge__$Vec_ModelTier$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ModelTier$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ModelTier(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModelTierRef> {
        let pointer = __swift_bridge__$Vec_ModelTier$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModelTierRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModelTierRefMut> {
        let pointer = __swift_bridge__$Vec_ModelTier$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModelTierRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ModelTierRef> {
        UnsafePointer<ModelTierRef>(OpaquePointer(__swift_bridge__$Vec_ModelTier$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ModelTier$len(vecPtr)
    }
}


public class ModelsListResponse: ModelsListResponseRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ModelsListResponse$_free(ptr)
        }
    }
}
extension ModelsListResponse {
    public convenience init<GenericIntoRustString: IntoRustString>(_ object: GenericIntoRustString, _ data: RustVec<ModelObject>) {
        self.init(ptr: __swift_bridge__$ModelsListResponse$new({ let rustString = object.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let val = data; val.isOwned = false; return val.ptr }()))
    }
}
public class ModelsListResponseRefMut: ModelsListResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ModelsListResponseRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ModelsListResponseRef {
    public func object() -> RustString {
        RustString(ptr: __swift_bridge__$ModelsListResponse$object(ptr))
    }

    public func data() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$ModelsListResponse$data(ptr))
    }
}
extension ModelsListResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ModelsListResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ModelsListResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ModelsListResponse) {
        __swift_bridge__$Vec_ModelsListResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ModelsListResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ModelsListResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModelsListResponseRef> {
        let pointer = __swift_bridge__$Vec_ModelsListResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModelsListResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModelsListResponseRefMut> {
        let pointer = __swift_bridge__$Vec_ModelsListResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModelsListResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ModelsListResponseRef> {
        UnsafePointer<ModelsListResponseRef>(OpaquePointer(__swift_bridge__$Vec_ModelsListResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ModelsListResponse$len(vecPtr)
    }
}


public class ModerationCategories: ModerationCategoriesRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ModerationCategories$_free(ptr)
        }
    }
}
extension ModerationCategories {
    public convenience init(_ sexual: Bool, _ hate: Bool, _ harassment: Bool, _ self_harm: Bool, _ sexual_minors: Bool, _ hate_threatening: Bool, _ violence_graphic: Bool, _ self_harm_intent: Bool, _ self_harm_instructions: Bool, _ harassment_threatening: Bool, _ violence: Bool) {
        self.init(ptr: __swift_bridge__$ModerationCategories$new(sexual, hate, harassment, self_harm, sexual_minors, hate_threatening, violence_graphic, self_harm_intent, self_harm_instructions, harassment_threatening, violence))
    }
}
public class ModerationCategoriesRefMut: ModerationCategoriesRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ModerationCategoriesRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ModerationCategoriesRef {
    public func sexual() -> Bool {
        __swift_bridge__$ModerationCategories$sexual(ptr)
    }

    public func hate() -> Bool {
        __swift_bridge__$ModerationCategories$hate(ptr)
    }

    public func harassment() -> Bool {
        __swift_bridge__$ModerationCategories$harassment(ptr)
    }

    public func selfHarm() -> Bool {
        __swift_bridge__$ModerationCategories$self_harm(ptr)
    }

    public func sexualMinors() -> Bool {
        __swift_bridge__$ModerationCategories$sexual_minors(ptr)
    }

    public func hateThreatening() -> Bool {
        __swift_bridge__$ModerationCategories$hate_threatening(ptr)
    }

    public func violenceGraphic() -> Bool {
        __swift_bridge__$ModerationCategories$violence_graphic(ptr)
    }

    public func selfHarmIntent() -> Bool {
        __swift_bridge__$ModerationCategories$self_harm_intent(ptr)
    }

    public func selfHarmInstructions() -> Bool {
        __swift_bridge__$ModerationCategories$self_harm_instructions(ptr)
    }

    public func harassmentThreatening() -> Bool {
        __swift_bridge__$ModerationCategories$harassment_threatening(ptr)
    }

    public func violence() -> Bool {
        __swift_bridge__$ModerationCategories$violence(ptr)
    }
}
extension ModerationCategories: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ModerationCategories$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ModerationCategories$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ModerationCategories) {
        __swift_bridge__$Vec_ModerationCategories$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ModerationCategories$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ModerationCategories(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModerationCategoriesRef> {
        let pointer = __swift_bridge__$Vec_ModerationCategories$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModerationCategoriesRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModerationCategoriesRefMut> {
        let pointer = __swift_bridge__$Vec_ModerationCategories$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModerationCategoriesRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ModerationCategoriesRef> {
        UnsafePointer<ModerationCategoriesRef>(OpaquePointer(__swift_bridge__$Vec_ModerationCategories$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ModerationCategories$len(vecPtr)
    }
}


public class ModerationCategoryScores: ModerationCategoryScoresRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ModerationCategoryScores$_free(ptr)
        }
    }
}
extension ModerationCategoryScores {
    public convenience init(_ sexual: Double, _ hate: Double, _ harassment: Double, _ self_harm: Double, _ sexual_minors: Double, _ hate_threatening: Double, _ violence_graphic: Double, _ self_harm_intent: Double, _ self_harm_instructions: Double, _ harassment_threatening: Double, _ violence: Double) {
        self.init(ptr: __swift_bridge__$ModerationCategoryScores$new(sexual, hate, harassment, self_harm, sexual_minors, hate_threatening, violence_graphic, self_harm_intent, self_harm_instructions, harassment_threatening, violence))
    }
}
public class ModerationCategoryScoresRefMut: ModerationCategoryScoresRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ModerationCategoryScoresRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ModerationCategoryScoresRef {
    public func sexual() -> Double {
        __swift_bridge__$ModerationCategoryScores$sexual(ptr)
    }

    public func hate() -> Double {
        __swift_bridge__$ModerationCategoryScores$hate(ptr)
    }

    public func harassment() -> Double {
        __swift_bridge__$ModerationCategoryScores$harassment(ptr)
    }

    public func selfHarm() -> Double {
        __swift_bridge__$ModerationCategoryScores$self_harm(ptr)
    }

    public func sexualMinors() -> Double {
        __swift_bridge__$ModerationCategoryScores$sexual_minors(ptr)
    }

    public func hateThreatening() -> Double {
        __swift_bridge__$ModerationCategoryScores$hate_threatening(ptr)
    }

    public func violenceGraphic() -> Double {
        __swift_bridge__$ModerationCategoryScores$violence_graphic(ptr)
    }

    public func selfHarmIntent() -> Double {
        __swift_bridge__$ModerationCategoryScores$self_harm_intent(ptr)
    }

    public func selfHarmInstructions() -> Double {
        __swift_bridge__$ModerationCategoryScores$self_harm_instructions(ptr)
    }

    public func harassmentThreatening() -> Double {
        __swift_bridge__$ModerationCategoryScores$harassment_threatening(ptr)
    }

    public func violence() -> Double {
        __swift_bridge__$ModerationCategoryScores$violence(ptr)
    }
}
extension ModerationCategoryScores: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ModerationCategoryScores$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ModerationCategoryScores$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ModerationCategoryScores) {
        __swift_bridge__$Vec_ModerationCategoryScores$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ModerationCategoryScores$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ModerationCategoryScores(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModerationCategoryScoresRef> {
        let pointer = __swift_bridge__$Vec_ModerationCategoryScores$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModerationCategoryScoresRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModerationCategoryScoresRefMut> {
        let pointer = __swift_bridge__$Vec_ModerationCategoryScores$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModerationCategoryScoresRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ModerationCategoryScoresRef> {
        UnsafePointer<ModerationCategoryScoresRef>(OpaquePointer(__swift_bridge__$Vec_ModerationCategoryScores$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ModerationCategoryScores$len(vecPtr)
    }
}


public class ModerationRequest: ModerationRequestRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ModerationRequest$_free(ptr)
        }
    }
}
extension ModerationRequest {
    public convenience init<GenericIntoRustString: IntoRustString>(_ input: ModerationInput, _ model: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$ModerationRequest$new({input.isOwned = false; return input.ptr;}(), { if let rustString = optionalStringIntoRustString(model) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class ModerationRequestRefMut: ModerationRequestRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ModerationRequestRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ModerationRequestRef {
    public func input() -> RustString {
        RustString(ptr: __swift_bridge__$ModerationRequest$input(ptr))
    }

    public func model() -> Optional<RustString> {
        { let val = __swift_bridge__$ModerationRequest$model(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension ModerationRequest: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ModerationRequest$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ModerationRequest$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ModerationRequest) {
        __swift_bridge__$Vec_ModerationRequest$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ModerationRequest$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ModerationRequest(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModerationRequestRef> {
        let pointer = __swift_bridge__$Vec_ModerationRequest$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModerationRequestRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModerationRequestRefMut> {
        let pointer = __swift_bridge__$Vec_ModerationRequest$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModerationRequestRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ModerationRequestRef> {
        UnsafePointer<ModerationRequestRef>(OpaquePointer(__swift_bridge__$Vec_ModerationRequest$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ModerationRequest$len(vecPtr)
    }
}


public class ModerationResponse: ModerationResponseRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ModerationResponse$_free(ptr)
        }
    }
}
public class ModerationResponseRefMut: ModerationResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ModerationResponseRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ModerationResponseRef {
    public func id() -> RustString {
        RustString(ptr: __swift_bridge__$ModerationResponse$id(ptr))
    }

    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$ModerationResponse$model(ptr))
    }

    public func results() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$ModerationResponse$results(ptr))
    }
}
extension ModerationResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ModerationResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ModerationResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ModerationResponse) {
        __swift_bridge__$Vec_ModerationResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ModerationResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ModerationResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModerationResponseRef> {
        let pointer = __swift_bridge__$Vec_ModerationResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModerationResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModerationResponseRefMut> {
        let pointer = __swift_bridge__$Vec_ModerationResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModerationResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ModerationResponseRef> {
        UnsafePointer<ModerationResponseRef>(OpaquePointer(__swift_bridge__$Vec_ModerationResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ModerationResponse$len(vecPtr)
    }
}


public class ModerationResult: ModerationResultRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ModerationResult$_free(ptr)
        }
    }
}
public class ModerationResultRefMut: ModerationResultRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ModerationResultRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ModerationResultRef {
    public func flagged() -> Bool {
        __swift_bridge__$ModerationResult$flagged(ptr)
    }

    public func categories() -> ModerationCategories {
        ModerationCategories(ptr: __swift_bridge__$ModerationResult$categories(ptr))
    }

    public func categoryScores() -> ModerationCategoryScores {
        ModerationCategoryScores(ptr: __swift_bridge__$ModerationResult$category_scores(ptr))
    }
}
extension ModerationResult: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ModerationResult$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ModerationResult$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ModerationResult) {
        __swift_bridge__$Vec_ModerationResult$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ModerationResult$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ModerationResult(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModerationResultRef> {
        let pointer = __swift_bridge__$Vec_ModerationResult$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModerationResultRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModerationResultRefMut> {
        let pointer = __swift_bridge__$Vec_ModerationResult$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModerationResultRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ModerationResultRef> {
        UnsafePointer<ModerationResultRef>(OpaquePointer(__swift_bridge__$Vec_ModerationResult$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ModerationResult$len(vecPtr)
    }
}


public class OcrImage: OcrImageRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$OcrImage$_free(ptr)
        }
    }
}
public class OcrImageRefMut: OcrImageRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class OcrImageRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension OcrImageRef {
    public func id() -> RustString {
        RustString(ptr: __swift_bridge__$OcrImage$id(ptr))
    }

    public func imageBase64() -> Optional<RustString> {
        { let val = __swift_bridge__$OcrImage$image_base64(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension OcrImage: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_OcrImage$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_OcrImage$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: OcrImage) {
        __swift_bridge__$Vec_OcrImage$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_OcrImage$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (OcrImage(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<OcrImageRef> {
        let pointer = __swift_bridge__$Vec_OcrImage$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return OcrImageRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<OcrImageRefMut> {
        let pointer = __swift_bridge__$Vec_OcrImage$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return OcrImageRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<OcrImageRef> {
        UnsafePointer<OcrImageRef>(OpaquePointer(__swift_bridge__$Vec_OcrImage$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_OcrImage$len(vecPtr)
    }
}


public class OcrPage: OcrPageRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$OcrPage$_free(ptr)
        }
    }
}
public class OcrPageRefMut: OcrPageRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class OcrPageRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension OcrPageRef {
    public func index() -> UInt32 {
        __swift_bridge__$OcrPage$index(ptr)
    }

    public func markdown() -> RustString {
        RustString(ptr: __swift_bridge__$OcrPage$markdown(ptr))
    }

    public func images() -> RustString {
        RustString(ptr: __swift_bridge__$OcrPage$images(ptr))
    }

    public func dimensions() -> Optional<PageDimensions> {
        { let val = __swift_bridge__$OcrPage$dimensions(ptr); if val != nil { return PageDimensions(ptr: val!) } else { return nil } }()
    }
}
extension OcrPage: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_OcrPage$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_OcrPage$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: OcrPage) {
        __swift_bridge__$Vec_OcrPage$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_OcrPage$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (OcrPage(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<OcrPageRef> {
        let pointer = __swift_bridge__$Vec_OcrPage$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return OcrPageRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<OcrPageRefMut> {
        let pointer = __swift_bridge__$Vec_OcrPage$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return OcrPageRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<OcrPageRef> {
        UnsafePointer<OcrPageRef>(OpaquePointer(__swift_bridge__$Vec_OcrPage$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_OcrPage$len(vecPtr)
    }
}


public class OcrRequest: OcrRequestRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$OcrRequest$_free(ptr)
        }
    }
}
public class OcrRequestRefMut: OcrRequestRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class OcrRequestRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension OcrRequestRef {
    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$OcrRequest$model(ptr))
    }

    public func document() -> RustString {
        RustString(ptr: __swift_bridge__$OcrRequest$document(ptr))
    }

    public func pages() -> RustString {
        RustString(ptr: __swift_bridge__$OcrRequest$pages(ptr))
    }

    public func includeImageBase64() -> Optional<Bool> {
        __swift_bridge__$OcrRequest$include_image_base64(ptr).intoSwiftRepr()
    }
}
extension OcrRequest: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_OcrRequest$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_OcrRequest$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: OcrRequest) {
        __swift_bridge__$Vec_OcrRequest$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_OcrRequest$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (OcrRequest(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<OcrRequestRef> {
        let pointer = __swift_bridge__$Vec_OcrRequest$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return OcrRequestRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<OcrRequestRefMut> {
        let pointer = __swift_bridge__$Vec_OcrRequest$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return OcrRequestRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<OcrRequestRef> {
        UnsafePointer<OcrRequestRef>(OpaquePointer(__swift_bridge__$Vec_OcrRequest$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_OcrRequest$len(vecPtr)
    }
}


public class OcrResponse: OcrResponseRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$OcrResponse$_free(ptr)
        }
    }
}
public class OcrResponseRefMut: OcrResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class OcrResponseRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension OcrResponseRef {
    public func pages() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$OcrResponse$pages(ptr))
    }

    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$OcrResponse$model(ptr))
    }

    public func usage() -> Optional<Usage> {
        { let val = __swift_bridge__$OcrResponse$usage(ptr); if val != nil { return Usage(ptr: val!) } else { return nil } }()
    }
}
extension OcrResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_OcrResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_OcrResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: OcrResponse) {
        __swift_bridge__$Vec_OcrResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_OcrResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (OcrResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<OcrResponseRef> {
        let pointer = __swift_bridge__$Vec_OcrResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return OcrResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<OcrResponseRefMut> {
        let pointer = __swift_bridge__$Vec_OcrResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return OcrResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<OcrResponseRef> {
        UnsafePointer<OcrResponseRef>(OpaquePointer(__swift_bridge__$Vec_OcrResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_OcrResponse$len(vecPtr)
    }
}


public class PageDimensions: PageDimensionsRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$PageDimensions$_free(ptr)
        }
    }
}
extension PageDimensions {
    public convenience init(_ width: UInt32, _ height: UInt32) {
        self.init(ptr: __swift_bridge__$PageDimensions$new(width, height))
    }
}
public class PageDimensionsRefMut: PageDimensionsRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class PageDimensionsRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension PageDimensionsRef {
    public func width() -> UInt32 {
        __swift_bridge__$PageDimensions$width(ptr)
    }

    public func height() -> UInt32 {
        __swift_bridge__$PageDimensions$height(ptr)
    }
}
extension PageDimensions: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_PageDimensions$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_PageDimensions$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: PageDimensions) {
        __swift_bridge__$Vec_PageDimensions$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_PageDimensions$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (PageDimensions(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<PageDimensionsRef> {
        let pointer = __swift_bridge__$Vec_PageDimensions$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return PageDimensionsRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<PageDimensionsRefMut> {
        let pointer = __swift_bridge__$Vec_PageDimensions$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return PageDimensionsRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<PageDimensionsRef> {
        UnsafePointer<PageDimensionsRef>(OpaquePointer(__swift_bridge__$Vec_PageDimensions$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_PageDimensions$len(vecPtr)
    }
}


public class PromptTokensDetails: PromptTokensDetailsRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$PromptTokensDetails$_free(ptr)
        }
    }
}
extension PromptTokensDetails {
    public convenience init(_ cached_tokens: UInt64, _ audio_tokens: UInt64) {
        self.init(ptr: __swift_bridge__$PromptTokensDetails$new(cached_tokens, audio_tokens))
    }
}
public class PromptTokensDetailsRefMut: PromptTokensDetailsRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class PromptTokensDetailsRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension PromptTokensDetailsRef {
    public func cachedTokens() -> UInt64 {
        __swift_bridge__$PromptTokensDetails$cached_tokens(ptr)
    }

    public func audioTokens() -> UInt64 {
        __swift_bridge__$PromptTokensDetails$audio_tokens(ptr)
    }
}
extension PromptTokensDetails: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_PromptTokensDetails$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_PromptTokensDetails$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: PromptTokensDetails) {
        __swift_bridge__$Vec_PromptTokensDetails$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_PromptTokensDetails$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (PromptTokensDetails(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<PromptTokensDetailsRef> {
        let pointer = __swift_bridge__$Vec_PromptTokensDetails$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return PromptTokensDetailsRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<PromptTokensDetailsRefMut> {
        let pointer = __swift_bridge__$Vec_PromptTokensDetails$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return PromptTokensDetailsRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<PromptTokensDetailsRef> {
        UnsafePointer<PromptTokensDetailsRef>(OpaquePointer(__swift_bridge__$Vec_PromptTokensDetails$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_PromptTokensDetails$len(vecPtr)
    }
}


public class ProviderCapabilities: ProviderCapabilitiesRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ProviderCapabilities$_free(ptr)
        }
    }
}
extension ProviderCapabilities {
    public convenience init(_ vision: Bool, _ reasoning: Bool, _ structured_output: Bool, _ function_calling: Bool, _ audio_in: Bool, _ audio_out: Bool, _ video_in: Bool) {
        self.init(ptr: __swift_bridge__$ProviderCapabilities$new(vision, reasoning, structured_output, function_calling, audio_in, audio_out, video_in))
    }
}
public class ProviderCapabilitiesRefMut: ProviderCapabilitiesRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ProviderCapabilitiesRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ProviderCapabilitiesRef {
    public func vision() -> Bool {
        __swift_bridge__$ProviderCapabilities$vision(ptr)
    }

    public func reasoning() -> Bool {
        __swift_bridge__$ProviderCapabilities$reasoning(ptr)
    }

    public func structuredOutput() -> Bool {
        __swift_bridge__$ProviderCapabilities$structured_output(ptr)
    }

    public func functionCalling() -> Bool {
        __swift_bridge__$ProviderCapabilities$function_calling(ptr)
    }

    public func audioIn() -> Bool {
        __swift_bridge__$ProviderCapabilities$audio_in(ptr)
    }

    public func audioOut() -> Bool {
        __swift_bridge__$ProviderCapabilities$audio_out(ptr)
    }

    public func videoIn() -> Bool {
        __swift_bridge__$ProviderCapabilities$video_in(ptr)
    }
}
extension ProviderCapabilities: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ProviderCapabilities$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ProviderCapabilities$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ProviderCapabilities) {
        __swift_bridge__$Vec_ProviderCapabilities$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ProviderCapabilities$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ProviderCapabilities(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ProviderCapabilitiesRef> {
        let pointer = __swift_bridge__$Vec_ProviderCapabilities$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ProviderCapabilitiesRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ProviderCapabilitiesRefMut> {
        let pointer = __swift_bridge__$Vec_ProviderCapabilities$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ProviderCapabilitiesRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ProviderCapabilitiesRef> {
        UnsafePointer<ProviderCapabilitiesRef>(OpaquePointer(__swift_bridge__$Vec_ProviderCapabilities$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ProviderCapabilities$len(vecPtr)
    }
}


public class ProviderConfig: ProviderConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ProviderConfig$_free(ptr)
        }
    }
}
public class ProviderConfigRefMut: ProviderConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ProviderConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ProviderConfigRef {
    public func name() -> RustString {
        RustString(ptr: __swift_bridge__$ProviderConfig$name(ptr))
    }

    public func displayName() -> Optional<RustString> {
        { let val = __swift_bridge__$ProviderConfig$display_name(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func baseUrl() -> Optional<RustString> {
        { let val = __swift_bridge__$ProviderConfig$base_url(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func auth() -> Optional<AuthConfig> {
        { let val = __swift_bridge__$ProviderConfig$auth(ptr); if val != nil { return AuthConfig(ptr: val!) } else { return nil } }()
    }

    public func endpoints() -> RustString {
        RustString(ptr: __swift_bridge__$ProviderConfig$endpoints(ptr))
    }

    public func modelPrefixes() -> RustString {
        RustString(ptr: __swift_bridge__$ProviderConfig$model_prefixes(ptr))
    }

    public func paramMappings() -> RustString {
        RustString(ptr: __swift_bridge__$ProviderConfig$param_mappings(ptr))
    }
}
extension ProviderConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ProviderConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ProviderConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ProviderConfig) {
        __swift_bridge__$Vec_ProviderConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ProviderConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ProviderConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ProviderConfigRef> {
        let pointer = __swift_bridge__$Vec_ProviderConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ProviderConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ProviderConfigRefMut> {
        let pointer = __swift_bridge__$Vec_ProviderConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ProviderConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ProviderConfigRef> {
        UnsafePointer<ProviderConfigRef>(OpaquePointer(__swift_bridge__$Vec_ProviderConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ProviderConfig$len(vecPtr)
    }
}


public class RateLimitConfig: RateLimitConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$RateLimitConfig$_free(ptr)
        }
    }
}
extension RateLimitConfig {
    public convenience init(_ rpm: Optional<UInt32>, _ tpm: Optional<UInt64>, _ window: UInt64) {
        self.init(ptr: __swift_bridge__$RateLimitConfig$new(rpm.intoFfiRepr(), tpm.intoFfiRepr(), window))
    }
}
public class RateLimitConfigRefMut: RateLimitConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class RateLimitConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension RateLimitConfigRef {
    public func rpm() -> Optional<UInt32> {
        __swift_bridge__$RateLimitConfig$rpm(ptr).intoSwiftRepr()
    }

    public func tpm() -> Optional<UInt64> {
        __swift_bridge__$RateLimitConfig$tpm(ptr).intoSwiftRepr()
    }

    public func window() -> UInt64 {
        __swift_bridge__$RateLimitConfig$window(ptr)
    }
}
extension RateLimitConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_RateLimitConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_RateLimitConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: RateLimitConfig) {
        __swift_bridge__$Vec_RateLimitConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_RateLimitConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (RateLimitConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RateLimitConfigRef> {
        let pointer = __swift_bridge__$Vec_RateLimitConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RateLimitConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RateLimitConfigRefMut> {
        let pointer = __swift_bridge__$Vec_RateLimitConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RateLimitConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<RateLimitConfigRef> {
        UnsafePointer<RateLimitConfigRef>(OpaquePointer(__swift_bridge__$Vec_RateLimitConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_RateLimitConfig$len(vecPtr)
    }
}


public class RerankRequest: RerankRequestRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$RerankRequest$_free(ptr)
        }
    }
}
extension RerankRequest {
    public convenience init<GenericIntoRustString: IntoRustString>(_ model: GenericIntoRustString, _ query: GenericIntoRustString, _ documents: RustVec<RerankDocument>, _ top_n: Optional<UInt32>, _ return_documents: Optional<Bool>) {
        self.init(ptr: __swift_bridge__$RerankRequest$new({ let rustString = model.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = query.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let val = documents; val.isOwned = false; return val.ptr }(), top_n.intoFfiRepr(), return_documents.intoFfiRepr()))
    }
}
public class RerankRequestRefMut: RerankRequestRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class RerankRequestRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension RerankRequestRef {
    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$RerankRequest$model(ptr))
    }

    public func query() -> RustString {
        RustString(ptr: __swift_bridge__$RerankRequest$query(ptr))
    }

    public func documents() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$RerankRequest$documents(ptr))
    }

    public func topN() -> Optional<UInt32> {
        __swift_bridge__$RerankRequest$top_n(ptr).intoSwiftRepr()
    }

    public func returnDocuments() -> Optional<Bool> {
        __swift_bridge__$RerankRequest$return_documents(ptr).intoSwiftRepr()
    }
}
extension RerankRequest: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_RerankRequest$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_RerankRequest$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: RerankRequest) {
        __swift_bridge__$Vec_RerankRequest$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_RerankRequest$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (RerankRequest(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RerankRequestRef> {
        let pointer = __swift_bridge__$Vec_RerankRequest$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RerankRequestRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RerankRequestRefMut> {
        let pointer = __swift_bridge__$Vec_RerankRequest$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RerankRequestRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<RerankRequestRef> {
        UnsafePointer<RerankRequestRef>(OpaquePointer(__swift_bridge__$Vec_RerankRequest$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_RerankRequest$len(vecPtr)
    }
}


public class RerankResponse: RerankResponseRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$RerankResponse$_free(ptr)
        }
    }
}
public class RerankResponseRefMut: RerankResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class RerankResponseRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension RerankResponseRef {
    public func id() -> Optional<RustString> {
        { let val = __swift_bridge__$RerankResponse$id(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func results() -> RustVec<RerankResult> {
        RustVec(ptr: __swift_bridge__$RerankResponse$results(ptr))
    }

    public func meta() -> Optional<RustString> {
        { let val = __swift_bridge__$RerankResponse$meta(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension RerankResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_RerankResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_RerankResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: RerankResponse) {
        __swift_bridge__$Vec_RerankResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_RerankResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (RerankResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RerankResponseRef> {
        let pointer = __swift_bridge__$Vec_RerankResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RerankResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RerankResponseRefMut> {
        let pointer = __swift_bridge__$Vec_RerankResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RerankResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<RerankResponseRef> {
        UnsafePointer<RerankResponseRef>(OpaquePointer(__swift_bridge__$Vec_RerankResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_RerankResponse$len(vecPtr)
    }
}


public class RerankResult: RerankResultRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$RerankResult$_free(ptr)
        }
    }
}
public class RerankResultRefMut: RerankResultRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class RerankResultRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension RerankResultRef {
    public func index() -> UInt32 {
        __swift_bridge__$RerankResult$index(ptr)
    }

    public func relevanceScore() -> Double {
        __swift_bridge__$RerankResult$relevance_score(ptr)
    }

    public func document() -> Optional<RerankResultDocument> {
        { let val = __swift_bridge__$RerankResult$document(ptr); if val != nil { return RerankResultDocument(ptr: val!) } else { return nil } }()
    }
}
extension RerankResult: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_RerankResult$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_RerankResult$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: RerankResult) {
        __swift_bridge__$Vec_RerankResult$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_RerankResult$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (RerankResult(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RerankResultRef> {
        let pointer = __swift_bridge__$Vec_RerankResult$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RerankResultRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RerankResultRefMut> {
        let pointer = __swift_bridge__$Vec_RerankResult$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RerankResultRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<RerankResultRef> {
        UnsafePointer<RerankResultRef>(OpaquePointer(__swift_bridge__$Vec_RerankResult$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_RerankResult$len(vecPtr)
    }
}


public class RerankResultDocument: RerankResultDocumentRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$RerankResultDocument$_free(ptr)
        }
    }
}
public class RerankResultDocumentRefMut: RerankResultDocumentRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class RerankResultDocumentRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension RerankResultDocumentRef {
    public func text() -> RustString {
        RustString(ptr: __swift_bridge__$RerankResultDocument$text(ptr))
    }
}
extension RerankResultDocument: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_RerankResultDocument$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_RerankResultDocument$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: RerankResultDocument) {
        __swift_bridge__$Vec_RerankResultDocument$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_RerankResultDocument$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (RerankResultDocument(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RerankResultDocumentRef> {
        let pointer = __swift_bridge__$Vec_RerankResultDocument$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RerankResultDocumentRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RerankResultDocumentRefMut> {
        let pointer = __swift_bridge__$Vec_RerankResultDocument$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RerankResultDocumentRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<RerankResultDocumentRef> {
        UnsafePointer<RerankResultDocumentRef>(OpaquePointer(__swift_bridge__$Vec_RerankResultDocument$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_RerankResultDocument$len(vecPtr)
    }
}


public class ResponseObject: ResponseObjectRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ResponseObject$_free(ptr)
        }
    }
}
extension ResponseObject {
    public convenience init<GenericIntoRustString: IntoRustString>(_ id: GenericIntoRustString, _ object: GenericIntoRustString, _ created_at: UInt64, _ model: GenericIntoRustString, _ status: GenericIntoRustString, _ output: RustVec<ResponseOutputItem>, _ usage: Optional<ResponseUsage>, _ error: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$ResponseObject$new({ let rustString = id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = object.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), created_at, { let rustString = model.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = status.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let val = output; val.isOwned = false; return val.ptr }(), { if let val = usage { val.isOwned = false; return val.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(error) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class ResponseObjectRefMut: ResponseObjectRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ResponseObjectRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ResponseObjectRef {
    public func id() -> RustString {
        RustString(ptr: __swift_bridge__$ResponseObject$id(ptr))
    }

    public func object() -> RustString {
        RustString(ptr: __swift_bridge__$ResponseObject$object(ptr))
    }

    public func createdAt() -> UInt64 {
        __swift_bridge__$ResponseObject$created_at(ptr)
    }

    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$ResponseObject$model(ptr))
    }

    public func status() -> RustString {
        RustString(ptr: __swift_bridge__$ResponseObject$status(ptr))
    }

    public func output() -> RustVec<ResponseOutputItem> {
        RustVec(ptr: __swift_bridge__$ResponseObject$output(ptr))
    }

    public func usage() -> Optional<ResponseUsage> {
        { let val = __swift_bridge__$ResponseObject$usage(ptr); if val != nil { return ResponseUsage(ptr: val!) } else { return nil } }()
    }

    public func error() -> Optional<RustString> {
        { let val = __swift_bridge__$ResponseObject$error(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension ResponseObject: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ResponseObject$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ResponseObject$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ResponseObject) {
        __swift_bridge__$Vec_ResponseObject$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ResponseObject$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ResponseObject(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ResponseObjectRef> {
        let pointer = __swift_bridge__$Vec_ResponseObject$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ResponseObjectRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ResponseObjectRefMut> {
        let pointer = __swift_bridge__$Vec_ResponseObject$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ResponseObjectRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ResponseObjectRef> {
        UnsafePointer<ResponseObjectRef>(OpaquePointer(__swift_bridge__$Vec_ResponseObject$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ResponseObject$len(vecPtr)
    }
}


public class ResponseOutputItem: ResponseOutputItemRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ResponseOutputItem$_free(ptr)
        }
    }
}
extension ResponseOutputItem {
    public convenience init<GenericIntoRustString: IntoRustString>(_ item_type: GenericIntoRustString, _ content: GenericIntoRustString) {
        self.init(ptr: __swift_bridge__$ResponseOutputItem$new({ let rustString = item_type.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = content.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class ResponseOutputItemRefMut: ResponseOutputItemRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ResponseOutputItemRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ResponseOutputItemRef {
    public func itemType() -> RustString {
        RustString(ptr: __swift_bridge__$ResponseOutputItem$item_type(ptr))
    }

    public func content() -> RustString {
        RustString(ptr: __swift_bridge__$ResponseOutputItem$content(ptr))
    }
}
extension ResponseOutputItem: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ResponseOutputItem$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ResponseOutputItem$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ResponseOutputItem) {
        __swift_bridge__$Vec_ResponseOutputItem$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ResponseOutputItem$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ResponseOutputItem(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ResponseOutputItemRef> {
        let pointer = __swift_bridge__$Vec_ResponseOutputItem$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ResponseOutputItemRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ResponseOutputItemRefMut> {
        let pointer = __swift_bridge__$Vec_ResponseOutputItem$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ResponseOutputItemRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ResponseOutputItemRef> {
        UnsafePointer<ResponseOutputItemRef>(OpaquePointer(__swift_bridge__$Vec_ResponseOutputItem$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ResponseOutputItem$len(vecPtr)
    }
}


public class ResponseTool: ResponseToolRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ResponseTool$_free(ptr)
        }
    }
}
extension ResponseTool {
    public convenience init<GenericIntoRustString: IntoRustString>(_ tool_type: GenericIntoRustString, _ config: GenericIntoRustString) {
        self.init(ptr: __swift_bridge__$ResponseTool$new({ let rustString = tool_type.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = config.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class ResponseToolRefMut: ResponseToolRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ResponseToolRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ResponseToolRef {
    public func toolType() -> RustString {
        RustString(ptr: __swift_bridge__$ResponseTool$tool_type(ptr))
    }

    public func config() -> RustString {
        RustString(ptr: __swift_bridge__$ResponseTool$config(ptr))
    }
}
extension ResponseTool: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ResponseTool$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ResponseTool$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ResponseTool) {
        __swift_bridge__$Vec_ResponseTool$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ResponseTool$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ResponseTool(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ResponseToolRef> {
        let pointer = __swift_bridge__$Vec_ResponseTool$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ResponseToolRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ResponseToolRefMut> {
        let pointer = __swift_bridge__$Vec_ResponseTool$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ResponseToolRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ResponseToolRef> {
        UnsafePointer<ResponseToolRef>(OpaquePointer(__swift_bridge__$Vec_ResponseTool$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ResponseTool$len(vecPtr)
    }
}


public class ResponseUsage: ResponseUsageRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ResponseUsage$_free(ptr)
        }
    }
}
extension ResponseUsage {
    public convenience init(_ input_tokens: UInt64, _ output_tokens: UInt64, _ total_tokens: UInt64) {
        self.init(ptr: __swift_bridge__$ResponseUsage$new(input_tokens, output_tokens, total_tokens))
    }
}
public class ResponseUsageRefMut: ResponseUsageRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ResponseUsageRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ResponseUsageRef {
    public func inputTokens() -> UInt64 {
        __swift_bridge__$ResponseUsage$input_tokens(ptr)
    }

    public func outputTokens() -> UInt64 {
        __swift_bridge__$ResponseUsage$output_tokens(ptr)
    }

    public func totalTokens() -> UInt64 {
        __swift_bridge__$ResponseUsage$total_tokens(ptr)
    }
}
extension ResponseUsage: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ResponseUsage$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ResponseUsage$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ResponseUsage) {
        __swift_bridge__$Vec_ResponseUsage$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ResponseUsage$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ResponseUsage(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ResponseUsageRef> {
        let pointer = __swift_bridge__$Vec_ResponseUsage$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ResponseUsageRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ResponseUsageRefMut> {
        let pointer = __swift_bridge__$Vec_ResponseUsage$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ResponseUsageRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ResponseUsageRef> {
        UnsafePointer<ResponseUsageRef>(OpaquePointer(__swift_bridge__$Vec_ResponseUsage$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ResponseUsage$len(vecPtr)
    }
}


public class SearchRequest: SearchRequestRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$SearchRequest$_free(ptr)
        }
    }
}
public class SearchRequestRefMut: SearchRequestRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class SearchRequestRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension SearchRequestRef {
    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$SearchRequest$model(ptr))
    }

    public func query() -> RustString {
        RustString(ptr: __swift_bridge__$SearchRequest$query(ptr))
    }

    public func maxResults() -> Optional<UInt32> {
        __swift_bridge__$SearchRequest$max_results(ptr).intoSwiftRepr()
    }

    public func searchDomainFilter() -> RustString {
        RustString(ptr: __swift_bridge__$SearchRequest$search_domain_filter(ptr))
    }

    public func country() -> Optional<RustString> {
        { let val = __swift_bridge__$SearchRequest$country(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension SearchRequest: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_SearchRequest$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_SearchRequest$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: SearchRequest) {
        __swift_bridge__$Vec_SearchRequest$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_SearchRequest$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (SearchRequest(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SearchRequestRef> {
        let pointer = __swift_bridge__$Vec_SearchRequest$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SearchRequestRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SearchRequestRefMut> {
        let pointer = __swift_bridge__$Vec_SearchRequest$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SearchRequestRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<SearchRequestRef> {
        UnsafePointer<SearchRequestRef>(OpaquePointer(__swift_bridge__$Vec_SearchRequest$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_SearchRequest$len(vecPtr)
    }
}


public class SearchResponse: SearchResponseRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$SearchResponse$_free(ptr)
        }
    }
}
public class SearchResponseRefMut: SearchResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class SearchResponseRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension SearchResponseRef {
    public func results() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$SearchResponse$results(ptr))
    }

    public func model() -> RustString {
        RustString(ptr: __swift_bridge__$SearchResponse$model(ptr))
    }
}
extension SearchResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_SearchResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_SearchResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: SearchResponse) {
        __swift_bridge__$Vec_SearchResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_SearchResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (SearchResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SearchResponseRef> {
        let pointer = __swift_bridge__$Vec_SearchResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SearchResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SearchResponseRefMut> {
        let pointer = __swift_bridge__$Vec_SearchResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SearchResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<SearchResponseRef> {
        UnsafePointer<SearchResponseRef>(OpaquePointer(__swift_bridge__$Vec_SearchResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_SearchResponse$len(vecPtr)
    }
}


public class SearchResult: SearchResultRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$SearchResult$_free(ptr)
        }
    }
}
public class SearchResultRefMut: SearchResultRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class SearchResultRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension SearchResultRef {
    public func title() -> RustString {
        RustString(ptr: __swift_bridge__$SearchResult$title(ptr))
    }

    public func url() -> RustString {
        RustString(ptr: __swift_bridge__$SearchResult$url(ptr))
    }

    public func snippet() -> RustString {
        RustString(ptr: __swift_bridge__$SearchResult$snippet(ptr))
    }

    public func date() -> Optional<RustString> {
        { let val = __swift_bridge__$SearchResult$date(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension SearchResult: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_SearchResult$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_SearchResult$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: SearchResult) {
        __swift_bridge__$Vec_SearchResult$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_SearchResult$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (SearchResult(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SearchResultRef> {
        let pointer = __swift_bridge__$Vec_SearchResult$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SearchResultRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SearchResultRefMut> {
        let pointer = __swift_bridge__$Vec_SearchResult$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SearchResultRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<SearchResultRef> {
        UnsafePointer<SearchResultRef>(OpaquePointer(__swift_bridge__$Vec_SearchResult$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_SearchResult$len(vecPtr)
    }
}


public class SingleflightResult: SingleflightResultRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$SingleflightResult$_free(ptr)
        }
    }
}
public class SingleflightResultRefMut: SingleflightResultRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class SingleflightResultRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension SingleflightResult: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_SingleflightResult$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_SingleflightResult$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: SingleflightResult) {
        __swift_bridge__$Vec_SingleflightResult$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_SingleflightResult$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (SingleflightResult(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SingleflightResultRef> {
        let pointer = __swift_bridge__$Vec_SingleflightResult$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SingleflightResultRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SingleflightResultRefMut> {
        let pointer = __swift_bridge__$Vec_SingleflightResult$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SingleflightResultRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<SingleflightResultRef> {
        UnsafePointer<SingleflightResultRef>(OpaquePointer(__swift_bridge__$Vec_SingleflightResult$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_SingleflightResult$len(vecPtr)
    }
}


public class SpecificFunction: SpecificFunctionRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$SpecificFunction$_free(ptr)
        }
    }
}
extension SpecificFunction {
    public convenience init<GenericIntoRustString: IntoRustString>(_ name: GenericIntoRustString) {
        self.init(ptr: __swift_bridge__$SpecificFunction$new({ let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class SpecificFunctionRefMut: SpecificFunctionRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class SpecificFunctionRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension SpecificFunctionRef {
    public func name() -> RustString {
        RustString(ptr: __swift_bridge__$SpecificFunction$name(ptr))
    }
}
extension SpecificFunction: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_SpecificFunction$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_SpecificFunction$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: SpecificFunction) {
        __swift_bridge__$Vec_SpecificFunction$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_SpecificFunction$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (SpecificFunction(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SpecificFunctionRef> {
        let pointer = __swift_bridge__$Vec_SpecificFunction$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SpecificFunctionRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SpecificFunctionRefMut> {
        let pointer = __swift_bridge__$Vec_SpecificFunction$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SpecificFunctionRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<SpecificFunctionRef> {
        UnsafePointer<SpecificFunctionRef>(OpaquePointer(__swift_bridge__$Vec_SpecificFunction$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_SpecificFunction$len(vecPtr)
    }
}


public class SpecificToolChoice: SpecificToolChoiceRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$SpecificToolChoice$_free(ptr)
        }
    }
}
extension SpecificToolChoice {
    public convenience init(_ choice_type: ToolType, _ function: SpecificFunction) {
        self.init(ptr: __swift_bridge__$SpecificToolChoice$new({choice_type.isOwned = false; return choice_type.ptr;}(), {function.isOwned = false; return function.ptr;}()))
    }
}
public class SpecificToolChoiceRefMut: SpecificToolChoiceRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class SpecificToolChoiceRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension SpecificToolChoiceRef {
    public func choiceType() -> RustString {
        RustString(ptr: __swift_bridge__$SpecificToolChoice$choice_type(ptr))
    }

    public func function() -> SpecificFunction {
        SpecificFunction(ptr: __swift_bridge__$SpecificToolChoice$function(ptr))
    }
}
extension SpecificToolChoice: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_SpecificToolChoice$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_SpecificToolChoice$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: SpecificToolChoice) {
        __swift_bridge__$Vec_SpecificToolChoice$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_SpecificToolChoice$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (SpecificToolChoice(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SpecificToolChoiceRef> {
        let pointer = __swift_bridge__$Vec_SpecificToolChoice$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SpecificToolChoiceRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SpecificToolChoiceRefMut> {
        let pointer = __swift_bridge__$Vec_SpecificToolChoice$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SpecificToolChoiceRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<SpecificToolChoiceRef> {
        UnsafePointer<SpecificToolChoiceRef>(OpaquePointer(__swift_bridge__$Vec_SpecificToolChoice$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_SpecificToolChoice$len(vecPtr)
    }
}


public class StreamChoice: StreamChoiceRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$StreamChoice$_free(ptr)
        }
    }
}
extension StreamChoice {
    public convenience init(_ index: UInt32, _ delta: StreamDelta, _ finish_reason: Optional<FinishReason>) {
        self.init(ptr: __swift_bridge__$StreamChoice$new(index, {delta.isOwned = false; return delta.ptr;}(), { if let val = finish_reason { val.isOwned = false; return val.ptr } else { return nil } }()))
    }
}
public class StreamChoiceRefMut: StreamChoiceRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class StreamChoiceRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension StreamChoiceRef {
    public func index() -> UInt32 {
        __swift_bridge__$StreamChoice$index(ptr)
    }

    public func delta() -> StreamDelta {
        StreamDelta(ptr: __swift_bridge__$StreamChoice$delta(ptr))
    }

    public func finishReason() -> Optional<RustString> {
        { let val = __swift_bridge__$StreamChoice$finish_reason(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension StreamChoice: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_StreamChoice$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_StreamChoice$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: StreamChoice) {
        __swift_bridge__$Vec_StreamChoice$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_StreamChoice$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (StreamChoice(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StreamChoiceRef> {
        let pointer = __swift_bridge__$Vec_StreamChoice$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StreamChoiceRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StreamChoiceRefMut> {
        let pointer = __swift_bridge__$Vec_StreamChoice$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StreamChoiceRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<StreamChoiceRef> {
        UnsafePointer<StreamChoiceRef>(OpaquePointer(__swift_bridge__$Vec_StreamChoice$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_StreamChoice$len(vecPtr)
    }
}


public class StreamDelta: StreamDeltaRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$StreamDelta$_free(ptr)
        }
    }
}
public class StreamDeltaRefMut: StreamDeltaRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class StreamDeltaRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension StreamDeltaRef {
    public func role() -> Optional<RustString> {
        { let val = __swift_bridge__$StreamDelta$role(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func content() -> Optional<RustString> {
        { let val = __swift_bridge__$StreamDelta$content(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func toolCalls() -> RustString {
        RustString(ptr: __swift_bridge__$StreamDelta$tool_calls(ptr))
    }

    public func functionCall() -> Optional<StreamFunctionCall> {
        { let val = __swift_bridge__$StreamDelta$function_call(ptr); if val != nil { return StreamFunctionCall(ptr: val!) } else { return nil } }()
    }

    public func refusal() -> Optional<RustString> {
        { let val = __swift_bridge__$StreamDelta$refusal(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func reasoningContent() -> Optional<RustString> {
        { let val = __swift_bridge__$StreamDelta$reasoning_content(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension StreamDelta: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_StreamDelta$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_StreamDelta$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: StreamDelta) {
        __swift_bridge__$Vec_StreamDelta$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_StreamDelta$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (StreamDelta(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StreamDeltaRef> {
        let pointer = __swift_bridge__$Vec_StreamDelta$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StreamDeltaRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StreamDeltaRefMut> {
        let pointer = __swift_bridge__$Vec_StreamDelta$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StreamDeltaRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<StreamDeltaRef> {
        UnsafePointer<StreamDeltaRef>(OpaquePointer(__swift_bridge__$Vec_StreamDelta$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_StreamDelta$len(vecPtr)
    }
}


public class StreamFunctionCall: StreamFunctionCallRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$StreamFunctionCall$_free(ptr)
        }
    }
}
extension StreamFunctionCall {
    public convenience init<GenericIntoRustString: IntoRustString>(_ name: Optional<GenericIntoRustString>, _ arguments: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$StreamFunctionCall$new({ if let rustString = optionalStringIntoRustString(name) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(arguments) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class StreamFunctionCallRefMut: StreamFunctionCallRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class StreamFunctionCallRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension StreamFunctionCallRef {
    public func name() -> Optional<RustString> {
        { let val = __swift_bridge__$StreamFunctionCall$name(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func arguments() -> Optional<RustString> {
        { let val = __swift_bridge__$StreamFunctionCall$arguments(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension StreamFunctionCall: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_StreamFunctionCall$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_StreamFunctionCall$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: StreamFunctionCall) {
        __swift_bridge__$Vec_StreamFunctionCall$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_StreamFunctionCall$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (StreamFunctionCall(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StreamFunctionCallRef> {
        let pointer = __swift_bridge__$Vec_StreamFunctionCall$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StreamFunctionCallRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StreamFunctionCallRefMut> {
        let pointer = __swift_bridge__$Vec_StreamFunctionCall$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StreamFunctionCallRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<StreamFunctionCallRef> {
        UnsafePointer<StreamFunctionCallRef>(OpaquePointer(__swift_bridge__$Vec_StreamFunctionCall$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_StreamFunctionCall$len(vecPtr)
    }
}


public class StreamOptions: StreamOptionsRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$StreamOptions$_free(ptr)
        }
    }
}
extension StreamOptions {
    public convenience init(_ include_usage: Optional<Bool>) {
        self.init(ptr: __swift_bridge__$StreamOptions$new(include_usage.intoFfiRepr()))
    }
}
public class StreamOptionsRefMut: StreamOptionsRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class StreamOptionsRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension StreamOptionsRef {
    public func includeUsage() -> Optional<Bool> {
        __swift_bridge__$StreamOptions$include_usage(ptr).intoSwiftRepr()
    }
}
extension StreamOptions: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_StreamOptions$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_StreamOptions$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: StreamOptions) {
        __swift_bridge__$Vec_StreamOptions$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_StreamOptions$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (StreamOptions(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StreamOptionsRef> {
        let pointer = __swift_bridge__$Vec_StreamOptions$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StreamOptionsRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StreamOptionsRefMut> {
        let pointer = __swift_bridge__$Vec_StreamOptions$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StreamOptionsRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<StreamOptionsRef> {
        UnsafePointer<StreamOptionsRef>(OpaquePointer(__swift_bridge__$Vec_StreamOptions$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_StreamOptions$len(vecPtr)
    }
}


public class StreamToolCall: StreamToolCallRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$StreamToolCall$_free(ptr)
        }
    }
}
extension StreamToolCall {
    public convenience init<GenericIntoRustString: IntoRustString>(_ index: UInt32, _ id: Optional<GenericIntoRustString>, _ call_type: Optional<ToolType>, _ function: Optional<StreamFunctionCall>) {
        self.init(ptr: __swift_bridge__$StreamToolCall$new(index, { if let rustString = optionalStringIntoRustString(id) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let val = call_type { val.isOwned = false; return val.ptr } else { return nil } }(), { if let val = function { val.isOwned = false; return val.ptr } else { return nil } }()))
    }
}
public class StreamToolCallRefMut: StreamToolCallRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class StreamToolCallRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension StreamToolCallRef {
    public func index() -> UInt32 {
        __swift_bridge__$StreamToolCall$index(ptr)
    }

    public func id() -> Optional<RustString> {
        { let val = __swift_bridge__$StreamToolCall$id(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func callType() -> Optional<RustString> {
        { let val = __swift_bridge__$StreamToolCall$call_type(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func function() -> Optional<StreamFunctionCall> {
        { let val = __swift_bridge__$StreamToolCall$function(ptr); if val != nil { return StreamFunctionCall(ptr: val!) } else { return nil } }()
    }
}
extension StreamToolCall: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_StreamToolCall$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_StreamToolCall$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: StreamToolCall) {
        __swift_bridge__$Vec_StreamToolCall$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_StreamToolCall$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (StreamToolCall(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StreamToolCallRef> {
        let pointer = __swift_bridge__$Vec_StreamToolCall$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StreamToolCallRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StreamToolCallRefMut> {
        let pointer = __swift_bridge__$Vec_StreamToolCall$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StreamToolCallRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<StreamToolCallRef> {
        UnsafePointer<StreamToolCallRef>(OpaquePointer(__swift_bridge__$Vec_StreamToolCall$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_StreamToolCall$len(vecPtr)
    }
}


public class SystemMessage: SystemMessageRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$SystemMessage$_free(ptr)
        }
    }
}
extension SystemMessage {
    public convenience init<GenericIntoRustString: IntoRustString>(_ content: UserContent, _ name: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$SystemMessage$new({content.isOwned = false; return content.ptr;}(), { if let rustString = optionalStringIntoRustString(name) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class SystemMessageRefMut: SystemMessageRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class SystemMessageRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension SystemMessageRef {
    public func content() -> RustString {
        RustString(ptr: __swift_bridge__$SystemMessage$content(ptr))
    }

    public func name() -> Optional<RustString> {
        { let val = __swift_bridge__$SystemMessage$name(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension SystemMessage: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_SystemMessage$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_SystemMessage$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: SystemMessage) {
        __swift_bridge__$Vec_SystemMessage$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_SystemMessage$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (SystemMessage(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SystemMessageRef> {
        let pointer = __swift_bridge__$Vec_SystemMessage$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SystemMessageRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SystemMessageRefMut> {
        let pointer = __swift_bridge__$Vec_SystemMessage$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SystemMessageRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<SystemMessageRef> {
        UnsafePointer<SystemMessageRef>(OpaquePointer(__swift_bridge__$Vec_SystemMessage$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_SystemMessage$len(vecPtr)
    }
}


public class ToolCall: ToolCallRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ToolCall$_free(ptr)
        }
    }
}
public class ToolCallRefMut: ToolCallRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ToolCallRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ToolCallRef {
    public func id() -> RustString {
        RustString(ptr: __swift_bridge__$ToolCall$id(ptr))
    }

    public func callType() -> RustString {
        RustString(ptr: __swift_bridge__$ToolCall$call_type(ptr))
    }

    public func function() -> FunctionCall {
        FunctionCall(ptr: __swift_bridge__$ToolCall$function(ptr))
    }
}
extension ToolCall: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ToolCall$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ToolCall$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ToolCall) {
        __swift_bridge__$Vec_ToolCall$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ToolCall$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ToolCall(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ToolCallRef> {
        let pointer = __swift_bridge__$Vec_ToolCall$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ToolCallRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ToolCallRefMut> {
        let pointer = __swift_bridge__$Vec_ToolCall$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ToolCallRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ToolCallRef> {
        UnsafePointer<ToolCallRef>(OpaquePointer(__swift_bridge__$Vec_ToolCall$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ToolCall$len(vecPtr)
    }
}


public class ToolMessage: ToolMessageRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ToolMessage$_free(ptr)
        }
    }
}
extension ToolMessage {
    public convenience init<GenericIntoRustString: IntoRustString>(_ content: UserContent, _ tool_call_id: GenericIntoRustString, _ name: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$ToolMessage$new({content.isOwned = false; return content.ptr;}(), { let rustString = tool_call_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let rustString = optionalStringIntoRustString(name) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class ToolMessageRefMut: ToolMessageRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ToolMessageRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ToolMessageRef {
    public func content() -> RustString {
        RustString(ptr: __swift_bridge__$ToolMessage$content(ptr))
    }

    public func toolCallId() -> RustString {
        RustString(ptr: __swift_bridge__$ToolMessage$tool_call_id(ptr))
    }

    public func name() -> Optional<RustString> {
        { let val = __swift_bridge__$ToolMessage$name(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension ToolMessage: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ToolMessage$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ToolMessage$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ToolMessage) {
        __swift_bridge__$Vec_ToolMessage$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ToolMessage$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ToolMessage(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ToolMessageRef> {
        let pointer = __swift_bridge__$Vec_ToolMessage$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ToolMessageRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ToolMessageRefMut> {
        let pointer = __swift_bridge__$Vec_ToolMessage$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ToolMessageRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ToolMessageRef> {
        UnsafePointer<ToolMessageRef>(OpaquePointer(__swift_bridge__$Vec_ToolMessage$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ToolMessage$len(vecPtr)
    }
}


public class TranscriptionResponse: TranscriptionResponseRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$TranscriptionResponse$_free(ptr)
        }
    }
}
public class TranscriptionResponseRefMut: TranscriptionResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class TranscriptionResponseRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension TranscriptionResponseRef {
    public func text() -> RustString {
        RustString(ptr: __swift_bridge__$TranscriptionResponse$text(ptr))
    }

    public func language() -> Optional<RustString> {
        { let val = __swift_bridge__$TranscriptionResponse$language(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func duration() -> Optional<Double> {
        __swift_bridge__$TranscriptionResponse$duration(ptr).intoSwiftRepr()
    }

    public func segments() -> RustString {
        RustString(ptr: __swift_bridge__$TranscriptionResponse$segments(ptr))
    }
}
extension TranscriptionResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_TranscriptionResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_TranscriptionResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: TranscriptionResponse) {
        __swift_bridge__$Vec_TranscriptionResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_TranscriptionResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (TranscriptionResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<TranscriptionResponseRef> {
        let pointer = __swift_bridge__$Vec_TranscriptionResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return TranscriptionResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<TranscriptionResponseRefMut> {
        let pointer = __swift_bridge__$Vec_TranscriptionResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return TranscriptionResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<TranscriptionResponseRef> {
        UnsafePointer<TranscriptionResponseRef>(OpaquePointer(__swift_bridge__$Vec_TranscriptionResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_TranscriptionResponse$len(vecPtr)
    }
}


public class TranscriptionSegment: TranscriptionSegmentRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$TranscriptionSegment$_free(ptr)
        }
    }
}
extension TranscriptionSegment {
    public convenience init<GenericIntoRustString: IntoRustString>(_ id: UInt32, _ start: Double, _ end: Double, _ text: GenericIntoRustString) {
        self.init(ptr: __swift_bridge__$TranscriptionSegment$new(id, start, end, { let rustString = text.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class TranscriptionSegmentRefMut: TranscriptionSegmentRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class TranscriptionSegmentRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension TranscriptionSegmentRef {
    public func id() -> UInt32 {
        __swift_bridge__$TranscriptionSegment$id(ptr)
    }

    public func start() -> Double {
        __swift_bridge__$TranscriptionSegment$start(ptr)
    }

    public func end() -> Double {
        __swift_bridge__$TranscriptionSegment$end(ptr)
    }

    public func text() -> RustString {
        RustString(ptr: __swift_bridge__$TranscriptionSegment$text(ptr))
    }
}
extension TranscriptionSegment: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_TranscriptionSegment$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_TranscriptionSegment$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: TranscriptionSegment) {
        __swift_bridge__$Vec_TranscriptionSegment$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_TranscriptionSegment$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (TranscriptionSegment(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<TranscriptionSegmentRef> {
        let pointer = __swift_bridge__$Vec_TranscriptionSegment$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return TranscriptionSegmentRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<TranscriptionSegmentRefMut> {
        let pointer = __swift_bridge__$Vec_TranscriptionSegment$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return TranscriptionSegmentRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<TranscriptionSegmentRef> {
        UnsafePointer<TranscriptionSegmentRef>(OpaquePointer(__swift_bridge__$Vec_TranscriptionSegment$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_TranscriptionSegment$len(vecPtr)
    }
}


public class Usage: UsageRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Usage$_free(ptr)
        }
    }
}
extension Usage {
    public convenience init(_ prompt_tokens: UInt64, _ completion_tokens: UInt64, _ total_tokens: UInt64, _ prompt_tokens_details: Optional<PromptTokensDetails>) {
        self.init(ptr: __swift_bridge__$Usage$new(prompt_tokens, completion_tokens, total_tokens, { if let val = prompt_tokens_details { val.isOwned = false; return val.ptr } else { return nil } }()))
    }
}
public class UsageRefMut: UsageRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class UsageRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension UsageRef {
    public func promptTokens() -> UInt64 {
        __swift_bridge__$Usage$prompt_tokens(ptr)
    }

    public func completionTokens() -> UInt64 {
        __swift_bridge__$Usage$completion_tokens(ptr)
    }

    public func totalTokens() -> UInt64 {
        __swift_bridge__$Usage$total_tokens(ptr)
    }

    public func promptTokensDetails() -> Optional<PromptTokensDetails> {
        { let val = __swift_bridge__$Usage$prompt_tokens_details(ptr); if val != nil { return PromptTokensDetails(ptr: val!) } else { return nil } }()
    }
}
extension Usage: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Usage$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Usage$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Usage) {
        __swift_bridge__$Vec_Usage$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Usage$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Usage(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<UsageRef> {
        let pointer = __swift_bridge__$Vec_Usage$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return UsageRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<UsageRefMut> {
        let pointer = __swift_bridge__$Vec_Usage$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return UsageRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<UsageRef> {
        UnsafePointer<UsageRef>(OpaquePointer(__swift_bridge__$Vec_Usage$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Usage$len(vecPtr)
    }
}


public class UserMessage: UserMessageRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$UserMessage$_free(ptr)
        }
    }
}
extension UserMessage {
    public convenience init<GenericIntoRustString: IntoRustString>(_ content: UserContent, _ name: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$UserMessage$new({content.isOwned = false; return content.ptr;}(), { if let rustString = optionalStringIntoRustString(name) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class UserMessageRefMut: UserMessageRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class UserMessageRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension UserMessageRef {
    public func content() -> RustString {
        RustString(ptr: __swift_bridge__$UserMessage$content(ptr))
    }

    public func name() -> Optional<RustString> {
        { let val = __swift_bridge__$UserMessage$name(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension UserMessage: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_UserMessage$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_UserMessage$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: UserMessage) {
        __swift_bridge__$Vec_UserMessage$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_UserMessage$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (UserMessage(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<UserMessageRef> {
        let pointer = __swift_bridge__$Vec_UserMessage$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return UserMessageRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<UserMessageRefMut> {
        let pointer = __swift_bridge__$Vec_UserMessage$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return UserMessageRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<UserMessageRef> {
        UnsafePointer<UserMessageRef>(OpaquePointer(__swift_bridge__$Vec_UserMessage$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_UserMessage$len(vecPtr)
    }
}


public class WaitForBatchConfig: WaitForBatchConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$WaitForBatchConfig$_free(ptr)
        }
    }
}
extension WaitForBatchConfig {
    public convenience init(_ initial_interval_secs: Double, _ max_interval_secs: Double, _ backoff_multiplier: Float, _ timeout_secs: Optional<Double>) {
        self.init(ptr: __swift_bridge__$WaitForBatchConfig$new(initial_interval_secs, max_interval_secs, backoff_multiplier, timeout_secs.intoFfiRepr()))
    }
}
public class WaitForBatchConfigRefMut: WaitForBatchConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class WaitForBatchConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension WaitForBatchConfigRef {
    public func initialIntervalSecs() -> Double {
        __swift_bridge__$WaitForBatchConfig$initial_interval_secs(ptr)
    }

    public func maxIntervalSecs() -> Double {
        __swift_bridge__$WaitForBatchConfig$max_interval_secs(ptr)
    }

    public func backoffMultiplier() -> Float {
        __swift_bridge__$WaitForBatchConfig$backoff_multiplier(ptr)
    }

    public func timeoutSecs() -> Optional<Double> {
        __swift_bridge__$WaitForBatchConfig$timeout_secs(ptr).intoSwiftRepr()
    }
}
extension WaitForBatchConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_WaitForBatchConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_WaitForBatchConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: WaitForBatchConfig) {
        __swift_bridge__$Vec_WaitForBatchConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_WaitForBatchConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (WaitForBatchConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<WaitForBatchConfigRef> {
        let pointer = __swift_bridge__$Vec_WaitForBatchConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return WaitForBatchConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<WaitForBatchConfigRefMut> {
        let pointer = __swift_bridge__$Vec_WaitForBatchConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return WaitForBatchConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<WaitForBatchConfigRef> {
        UnsafePointer<WaitForBatchConfigRef>(OpaquePointer(__swift_bridge__$Vec_WaitForBatchConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_WaitForBatchConfig$len(vecPtr)
    }
}


public class AssistantContent: AssistantContentRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$AssistantContent$_free(ptr)
        }
    }
}
public class AssistantContentRefMut: AssistantContentRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class AssistantContentRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension AssistantContentRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$AssistantContent$to_string(ptr))
    }
}
extension AssistantContent: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_AssistantContent$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_AssistantContent$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: AssistantContent) {
        __swift_bridge__$Vec_AssistantContent$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_AssistantContent$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (AssistantContent(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AssistantContentRef> {
        let pointer = __swift_bridge__$Vec_AssistantContent$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AssistantContentRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AssistantContentRefMut> {
        let pointer = __swift_bridge__$Vec_AssistantContent$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AssistantContentRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<AssistantContentRef> {
        UnsafePointer<AssistantContentRef>(OpaquePointer(__swift_bridge__$Vec_AssistantContent$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_AssistantContent$len(vecPtr)
    }
}


public class AssistantPart: AssistantPartRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$AssistantPart$_free(ptr)
        }
    }
}
public class AssistantPartRefMut: AssistantPartRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class AssistantPartRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension AssistantPartRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$AssistantPart$to_string(ptr))
    }
}
extension AssistantPart: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_AssistantPart$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_AssistantPart$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: AssistantPart) {
        __swift_bridge__$Vec_AssistantPart$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_AssistantPart$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (AssistantPart(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AssistantPartRef> {
        let pointer = __swift_bridge__$Vec_AssistantPart$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AssistantPartRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AssistantPartRefMut> {
        let pointer = __swift_bridge__$Vec_AssistantPart$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AssistantPartRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<AssistantPartRef> {
        UnsafePointer<AssistantPartRef>(OpaquePointer(__swift_bridge__$Vec_AssistantPart$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_AssistantPart$len(vecPtr)
    }
}


public class AuthHeaderFormat: AuthHeaderFormatRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$AuthHeaderFormat$_free(ptr)
        }
    }
}
public class AuthHeaderFormatRefMut: AuthHeaderFormatRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class AuthHeaderFormatRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension AuthHeaderFormatRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$AuthHeaderFormat$to_string(ptr))
    }
}
extension AuthHeaderFormat: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_AuthHeaderFormat$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_AuthHeaderFormat$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: AuthHeaderFormat) {
        __swift_bridge__$Vec_AuthHeaderFormat$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_AuthHeaderFormat$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (AuthHeaderFormat(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AuthHeaderFormatRef> {
        let pointer = __swift_bridge__$Vec_AuthHeaderFormat$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AuthHeaderFormatRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AuthHeaderFormatRefMut> {
        let pointer = __swift_bridge__$Vec_AuthHeaderFormat$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AuthHeaderFormatRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<AuthHeaderFormatRef> {
        UnsafePointer<AuthHeaderFormatRef>(OpaquePointer(__swift_bridge__$Vec_AuthHeaderFormat$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_AuthHeaderFormat$len(vecPtr)
    }
}


public class AuthType: AuthTypeRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$AuthType$_free(ptr)
        }
    }
}
public class AuthTypeRefMut: AuthTypeRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class AuthTypeRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension AuthTypeRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$AuthType$to_string(ptr))
    }
}
extension AuthType: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_AuthType$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_AuthType$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: AuthType) {
        __swift_bridge__$Vec_AuthType$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_AuthType$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (AuthType(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AuthTypeRef> {
        let pointer = __swift_bridge__$Vec_AuthType$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AuthTypeRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AuthTypeRefMut> {
        let pointer = __swift_bridge__$Vec_AuthType$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AuthTypeRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<AuthTypeRef> {
        UnsafePointer<AuthTypeRef>(OpaquePointer(__swift_bridge__$Vec_AuthType$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_AuthType$len(vecPtr)
    }
}


public class BatchStatus: BatchStatusRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$BatchStatus$_free(ptr)
        }
    }
}
public class BatchStatusRefMut: BatchStatusRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class BatchStatusRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension BatchStatusRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$BatchStatus$to_string(ptr))
    }
}
extension BatchStatus: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_BatchStatus$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_BatchStatus$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BatchStatus) {
        __swift_bridge__$Vec_BatchStatus$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_BatchStatus$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (BatchStatus(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BatchStatusRef> {
        let pointer = __swift_bridge__$Vec_BatchStatus$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return BatchStatusRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BatchStatusRefMut> {
        let pointer = __swift_bridge__$Vec_BatchStatus$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return BatchStatusRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<BatchStatusRef> {
        UnsafePointer<BatchStatusRef>(OpaquePointer(__swift_bridge__$Vec_BatchStatus$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_BatchStatus$len(vecPtr)
    }
}


public class CacheBackend: CacheBackendRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$CacheBackend$_free(ptr)
        }
    }
}
public class CacheBackendRefMut: CacheBackendRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class CacheBackendRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension CacheBackendRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$CacheBackend$to_string(ptr))
    }
}
extension CacheBackend: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_CacheBackend$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_CacheBackend$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CacheBackend) {
        __swift_bridge__$Vec_CacheBackend$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_CacheBackend$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (CacheBackend(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CacheBackendRef> {
        let pointer = __swift_bridge__$Vec_CacheBackend$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CacheBackendRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CacheBackendRefMut> {
        let pointer = __swift_bridge__$Vec_CacheBackend$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CacheBackendRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CacheBackendRef> {
        UnsafePointer<CacheBackendRef>(OpaquePointer(__swift_bridge__$Vec_CacheBackend$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_CacheBackend$len(vecPtr)
    }
}


public class CircuitState: CircuitStateRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$CircuitState$_free(ptr)
        }
    }
}
public class CircuitStateRefMut: CircuitStateRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class CircuitStateRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension CircuitStateRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$CircuitState$to_string(ptr))
    }
}
extension CircuitState: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_CircuitState$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_CircuitState$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CircuitState) {
        __swift_bridge__$Vec_CircuitState$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_CircuitState$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (CircuitState(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CircuitStateRef> {
        let pointer = __swift_bridge__$Vec_CircuitState$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CircuitStateRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CircuitStateRefMut> {
        let pointer = __swift_bridge__$Vec_CircuitState$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CircuitStateRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CircuitStateRef> {
        UnsafePointer<CircuitStateRef>(OpaquePointer(__swift_bridge__$Vec_CircuitState$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_CircuitState$len(vecPtr)
    }
}


public class ContentPart: ContentPartRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ContentPart$_free(ptr)
        }
    }
}
public class ContentPartRefMut: ContentPartRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ContentPartRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ContentPartRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$ContentPart$to_string(ptr))
    }
}
extension ContentPart: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ContentPart$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ContentPart$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ContentPart) {
        __swift_bridge__$Vec_ContentPart$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ContentPart$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ContentPart(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ContentPartRef> {
        let pointer = __swift_bridge__$Vec_ContentPart$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ContentPartRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ContentPartRefMut> {
        let pointer = __swift_bridge__$Vec_ContentPart$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ContentPartRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ContentPartRef> {
        UnsafePointer<ContentPartRef>(OpaquePointer(__swift_bridge__$Vec_ContentPart$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ContentPart$len(vecPtr)
    }
}


public class EmbeddingContentPart: EmbeddingContentPartRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$EmbeddingContentPart$_free(ptr)
        }
    }
}
public class EmbeddingContentPartRefMut: EmbeddingContentPartRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class EmbeddingContentPartRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension EmbeddingContentPartRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$EmbeddingContentPart$to_string(ptr))
    }
}
extension EmbeddingContentPart: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_EmbeddingContentPart$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_EmbeddingContentPart$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: EmbeddingContentPart) {
        __swift_bridge__$Vec_EmbeddingContentPart$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_EmbeddingContentPart$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (EmbeddingContentPart(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<EmbeddingContentPartRef> {
        let pointer = __swift_bridge__$Vec_EmbeddingContentPart$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return EmbeddingContentPartRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<EmbeddingContentPartRefMut> {
        let pointer = __swift_bridge__$Vec_EmbeddingContentPart$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return EmbeddingContentPartRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<EmbeddingContentPartRef> {
        UnsafePointer<EmbeddingContentPartRef>(OpaquePointer(__swift_bridge__$Vec_EmbeddingContentPart$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_EmbeddingContentPart$len(vecPtr)
    }
}


public class EmbeddingFormat: EmbeddingFormatRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$EmbeddingFormat$_free(ptr)
        }
    }
}
public class EmbeddingFormatRefMut: EmbeddingFormatRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class EmbeddingFormatRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension EmbeddingFormatRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$EmbeddingFormat$to_string(ptr))
    }
}
extension EmbeddingFormat: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_EmbeddingFormat$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_EmbeddingFormat$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: EmbeddingFormat) {
        __swift_bridge__$Vec_EmbeddingFormat$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_EmbeddingFormat$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (EmbeddingFormat(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<EmbeddingFormatRef> {
        let pointer = __swift_bridge__$Vec_EmbeddingFormat$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return EmbeddingFormatRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<EmbeddingFormatRefMut> {
        let pointer = __swift_bridge__$Vec_EmbeddingFormat$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return EmbeddingFormatRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<EmbeddingFormatRef> {
        UnsafePointer<EmbeddingFormatRef>(OpaquePointer(__swift_bridge__$Vec_EmbeddingFormat$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_EmbeddingFormat$len(vecPtr)
    }
}


public class EmbeddingInput: EmbeddingInputRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$EmbeddingInput$_free(ptr)
        }
    }
}
public class EmbeddingInputRefMut: EmbeddingInputRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class EmbeddingInputRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension EmbeddingInputRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$EmbeddingInput$to_string(ptr))
    }
}
extension EmbeddingInput: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_EmbeddingInput$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_EmbeddingInput$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: EmbeddingInput) {
        __swift_bridge__$Vec_EmbeddingInput$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_EmbeddingInput$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (EmbeddingInput(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<EmbeddingInputRef> {
        let pointer = __swift_bridge__$Vec_EmbeddingInput$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return EmbeddingInputRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<EmbeddingInputRefMut> {
        let pointer = __swift_bridge__$Vec_EmbeddingInput$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return EmbeddingInputRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<EmbeddingInputRef> {
        UnsafePointer<EmbeddingInputRef>(OpaquePointer(__swift_bridge__$Vec_EmbeddingInput$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_EmbeddingInput$len(vecPtr)
    }
}


public class Enforcement: EnforcementRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Enforcement$_free(ptr)
        }
    }
}
public class EnforcementRefMut: EnforcementRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class EnforcementRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension EnforcementRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$Enforcement$to_string(ptr))
    }
}
extension Enforcement: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Enforcement$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Enforcement$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Enforcement) {
        __swift_bridge__$Vec_Enforcement$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Enforcement$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Enforcement(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<EnforcementRef> {
        let pointer = __swift_bridge__$Vec_Enforcement$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return EnforcementRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<EnforcementRefMut> {
        let pointer = __swift_bridge__$Vec_Enforcement$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return EnforcementRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<EnforcementRef> {
        UnsafePointer<EnforcementRef>(OpaquePointer(__swift_bridge__$Vec_Enforcement$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Enforcement$len(vecPtr)
    }
}


public class FilePurpose: FilePurposeRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$FilePurpose$_free(ptr)
        }
    }
}
public class FilePurposeRefMut: FilePurposeRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class FilePurposeRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension FilePurposeRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$FilePurpose$to_string(ptr))
    }
}
extension FilePurpose: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_FilePurpose$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_FilePurpose$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: FilePurpose) {
        __swift_bridge__$Vec_FilePurpose$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_FilePurpose$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (FilePurpose(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FilePurposeRef> {
        let pointer = __swift_bridge__$Vec_FilePurpose$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FilePurposeRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FilePurposeRefMut> {
        let pointer = __swift_bridge__$Vec_FilePurpose$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FilePurposeRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<FilePurposeRef> {
        UnsafePointer<FilePurposeRef>(OpaquePointer(__swift_bridge__$Vec_FilePurpose$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_FilePurpose$len(vecPtr)
    }
}


public class FinishReason: FinishReasonRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$FinishReason$_free(ptr)
        }
    }
}
public class FinishReasonRefMut: FinishReasonRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class FinishReasonRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension FinishReasonRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$FinishReason$to_string(ptr))
    }
}
extension FinishReason: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_FinishReason$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_FinishReason$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: FinishReason) {
        __swift_bridge__$Vec_FinishReason$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_FinishReason$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (FinishReason(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FinishReasonRef> {
        let pointer = __swift_bridge__$Vec_FinishReason$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FinishReasonRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FinishReasonRefMut> {
        let pointer = __swift_bridge__$Vec_FinishReason$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FinishReasonRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<FinishReasonRef> {
        UnsafePointer<FinishReasonRef>(OpaquePointer(__swift_bridge__$Vec_FinishReason$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_FinishReason$len(vecPtr)
    }
}


public class HealthStatus: HealthStatusRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$HealthStatus$_free(ptr)
        }
    }
}
public class HealthStatusRefMut: HealthStatusRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class HealthStatusRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension HealthStatusRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$HealthStatus$to_string(ptr))
    }
}
extension HealthStatus: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_HealthStatus$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_HealthStatus$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: HealthStatus) {
        __swift_bridge__$Vec_HealthStatus$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_HealthStatus$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (HealthStatus(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<HealthStatusRef> {
        let pointer = __swift_bridge__$Vec_HealthStatus$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return HealthStatusRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<HealthStatusRefMut> {
        let pointer = __swift_bridge__$Vec_HealthStatus$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return HealthStatusRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<HealthStatusRef> {
        UnsafePointer<HealthStatusRef>(OpaquePointer(__swift_bridge__$Vec_HealthStatus$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_HealthStatus$len(vecPtr)
    }
}


public class ImageDetail: ImageDetailRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ImageDetail$_free(ptr)
        }
    }
}
public class ImageDetailRefMut: ImageDetailRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ImageDetailRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ImageDetailRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$ImageDetail$to_string(ptr))
    }
}
extension ImageDetail: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ImageDetail$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ImageDetail$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ImageDetail) {
        __swift_bridge__$Vec_ImageDetail$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ImageDetail$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ImageDetail(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ImageDetailRef> {
        let pointer = __swift_bridge__$Vec_ImageDetail$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ImageDetailRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ImageDetailRefMut> {
        let pointer = __swift_bridge__$Vec_ImageDetail$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ImageDetailRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ImageDetailRef> {
        UnsafePointer<ImageDetailRef>(OpaquePointer(__swift_bridge__$Vec_ImageDetail$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ImageDetail$len(vecPtr)
    }
}


public class Message: MessageRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Message$_free(ptr)
        }
    }
}
public class MessageRefMut: MessageRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class MessageRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension MessageRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$Message$to_string(ptr))
    }
}
extension Message: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Message$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Message$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Message) {
        __swift_bridge__$Vec_Message$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Message$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Message(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<MessageRef> {
        let pointer = __swift_bridge__$Vec_Message$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return MessageRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<MessageRefMut> {
        let pointer = __swift_bridge__$Vec_Message$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return MessageRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<MessageRef> {
        UnsafePointer<MessageRef>(OpaquePointer(__swift_bridge__$Vec_Message$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Message$len(vecPtr)
    }
}


public class Modality: ModalityRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Modality$_free(ptr)
        }
    }
}
public class ModalityRefMut: ModalityRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ModalityRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ModalityRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$Modality$to_string(ptr))
    }
}
extension Modality: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Modality$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Modality$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Modality) {
        __swift_bridge__$Vec_Modality$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Modality$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Modality(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModalityRef> {
        let pointer = __swift_bridge__$Vec_Modality$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModalityRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModalityRefMut> {
        let pointer = __swift_bridge__$Vec_Modality$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModalityRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ModalityRef> {
        UnsafePointer<ModalityRef>(OpaquePointer(__swift_bridge__$Vec_Modality$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Modality$len(vecPtr)
    }
}


public class ModerationInput: ModerationInputRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ModerationInput$_free(ptr)
        }
    }
}
public class ModerationInputRefMut: ModerationInputRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ModerationInputRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ModerationInputRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$ModerationInput$to_string(ptr))
    }
}
extension ModerationInput: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ModerationInput$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ModerationInput$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ModerationInput) {
        __swift_bridge__$Vec_ModerationInput$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ModerationInput$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ModerationInput(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModerationInputRef> {
        let pointer = __swift_bridge__$Vec_ModerationInput$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModerationInputRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ModerationInputRefMut> {
        let pointer = __swift_bridge__$Vec_ModerationInput$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ModerationInputRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ModerationInputRef> {
        UnsafePointer<ModerationInputRef>(OpaquePointer(__swift_bridge__$Vec_ModerationInput$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ModerationInput$len(vecPtr)
    }
}


public class OcrDocument: OcrDocumentRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$OcrDocument$_free(ptr)
        }
    }
}
public class OcrDocumentRefMut: OcrDocumentRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class OcrDocumentRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension OcrDocumentRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$OcrDocument$to_string(ptr))
    }
}
extension OcrDocument: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_OcrDocument$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_OcrDocument$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: OcrDocument) {
        __swift_bridge__$Vec_OcrDocument$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_OcrDocument$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (OcrDocument(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<OcrDocumentRef> {
        let pointer = __swift_bridge__$Vec_OcrDocument$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return OcrDocumentRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<OcrDocumentRefMut> {
        let pointer = __swift_bridge__$Vec_OcrDocument$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return OcrDocumentRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<OcrDocumentRef> {
        UnsafePointer<OcrDocumentRef>(OpaquePointer(__swift_bridge__$Vec_OcrDocument$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_OcrDocument$len(vecPtr)
    }
}


public class ReasoningEffort: ReasoningEffortRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ReasoningEffort$_free(ptr)
        }
    }
}
public class ReasoningEffortRefMut: ReasoningEffortRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ReasoningEffortRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ReasoningEffortRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$ReasoningEffort$to_string(ptr))
    }
}
extension ReasoningEffort: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ReasoningEffort$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ReasoningEffort$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ReasoningEffort) {
        __swift_bridge__$Vec_ReasoningEffort$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ReasoningEffort$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ReasoningEffort(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ReasoningEffortRef> {
        let pointer = __swift_bridge__$Vec_ReasoningEffort$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ReasoningEffortRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ReasoningEffortRefMut> {
        let pointer = __swift_bridge__$Vec_ReasoningEffort$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ReasoningEffortRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ReasoningEffortRef> {
        UnsafePointer<ReasoningEffortRef>(OpaquePointer(__swift_bridge__$Vec_ReasoningEffort$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ReasoningEffort$len(vecPtr)
    }
}


public class RefreshOutcome: RefreshOutcomeRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$RefreshOutcome$_free(ptr)
        }
    }
}
public class RefreshOutcomeRefMut: RefreshOutcomeRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class RefreshOutcomeRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension RefreshOutcomeRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$RefreshOutcome$to_string(ptr))
    }
}
extension RefreshOutcome: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_RefreshOutcome$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_RefreshOutcome$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: RefreshOutcome) {
        __swift_bridge__$Vec_RefreshOutcome$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_RefreshOutcome$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (RefreshOutcome(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RefreshOutcomeRef> {
        let pointer = __swift_bridge__$Vec_RefreshOutcome$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RefreshOutcomeRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RefreshOutcomeRefMut> {
        let pointer = __swift_bridge__$Vec_RefreshOutcome$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RefreshOutcomeRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<RefreshOutcomeRef> {
        UnsafePointer<RefreshOutcomeRef>(OpaquePointer(__swift_bridge__$Vec_RefreshOutcome$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_RefreshOutcome$len(vecPtr)
    }
}


public class RerankDocument: RerankDocumentRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$RerankDocument$_free(ptr)
        }
    }
}
public class RerankDocumentRefMut: RerankDocumentRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class RerankDocumentRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension RerankDocumentRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$RerankDocument$to_string(ptr))
    }
}
extension RerankDocument: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_RerankDocument$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_RerankDocument$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: RerankDocument) {
        __swift_bridge__$Vec_RerankDocument$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_RerankDocument$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (RerankDocument(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RerankDocumentRef> {
        let pointer = __swift_bridge__$Vec_RerankDocument$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RerankDocumentRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RerankDocumentRefMut> {
        let pointer = __swift_bridge__$Vec_RerankDocument$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RerankDocumentRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<RerankDocumentRef> {
        UnsafePointer<RerankDocumentRef>(OpaquePointer(__swift_bridge__$Vec_RerankDocument$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_RerankDocument$len(vecPtr)
    }
}


public class ResponseFormat: ResponseFormatRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ResponseFormat$_free(ptr)
        }
    }
}
public class ResponseFormatRefMut: ResponseFormatRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ResponseFormatRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ResponseFormatRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$ResponseFormat$to_string(ptr))
    }
}
extension ResponseFormat: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ResponseFormat$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ResponseFormat$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ResponseFormat) {
        __swift_bridge__$Vec_ResponseFormat$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ResponseFormat$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ResponseFormat(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ResponseFormatRef> {
        let pointer = __swift_bridge__$Vec_ResponseFormat$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ResponseFormatRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ResponseFormatRefMut> {
        let pointer = __swift_bridge__$Vec_ResponseFormat$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ResponseFormatRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ResponseFormatRef> {
        UnsafePointer<ResponseFormatRef>(OpaquePointer(__swift_bridge__$Vec_ResponseFormat$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ResponseFormat$len(vecPtr)
    }
}


public class StopSequence: StopSequenceRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$StopSequence$_free(ptr)
        }
    }
}
public class StopSequenceRefMut: StopSequenceRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class StopSequenceRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension StopSequenceRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$StopSequence$to_string(ptr))
    }
}
extension StopSequence: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_StopSequence$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_StopSequence$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: StopSequence) {
        __swift_bridge__$Vec_StopSequence$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_StopSequence$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (StopSequence(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StopSequenceRef> {
        let pointer = __swift_bridge__$Vec_StopSequence$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StopSequenceRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StopSequenceRefMut> {
        let pointer = __swift_bridge__$Vec_StopSequence$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StopSequenceRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<StopSequenceRef> {
        UnsafePointer<StopSequenceRef>(OpaquePointer(__swift_bridge__$Vec_StopSequence$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_StopSequence$len(vecPtr)
    }
}


public class StreamFormat: StreamFormatRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$StreamFormat$_free(ptr)
        }
    }
}
public class StreamFormatRefMut: StreamFormatRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class StreamFormatRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension StreamFormatRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$StreamFormat$to_string(ptr))
    }
}
extension StreamFormat: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_StreamFormat$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_StreamFormat$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: StreamFormat) {
        __swift_bridge__$Vec_StreamFormat$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_StreamFormat$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (StreamFormat(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StreamFormatRef> {
        let pointer = __swift_bridge__$Vec_StreamFormat$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StreamFormatRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StreamFormatRefMut> {
        let pointer = __swift_bridge__$Vec_StreamFormat$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StreamFormatRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<StreamFormatRef> {
        UnsafePointer<StreamFormatRef>(OpaquePointer(__swift_bridge__$Vec_StreamFormat$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_StreamFormat$len(vecPtr)
    }
}


public class ToolChoice: ToolChoiceRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ToolChoice$_free(ptr)
        }
    }
}
public class ToolChoiceRefMut: ToolChoiceRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ToolChoiceRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ToolChoiceRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$ToolChoice$to_string(ptr))
    }
}
extension ToolChoice: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ToolChoice$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ToolChoice$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ToolChoice) {
        __swift_bridge__$Vec_ToolChoice$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ToolChoice$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ToolChoice(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ToolChoiceRef> {
        let pointer = __swift_bridge__$Vec_ToolChoice$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ToolChoiceRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ToolChoiceRefMut> {
        let pointer = __swift_bridge__$Vec_ToolChoice$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ToolChoiceRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ToolChoiceRef> {
        UnsafePointer<ToolChoiceRef>(OpaquePointer(__swift_bridge__$Vec_ToolChoice$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ToolChoice$len(vecPtr)
    }
}


public class ToolChoiceMode: ToolChoiceModeRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ToolChoiceMode$_free(ptr)
        }
    }
}
public class ToolChoiceModeRefMut: ToolChoiceModeRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ToolChoiceModeRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ToolChoiceModeRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$ToolChoiceMode$to_string(ptr))
    }
}
extension ToolChoiceMode: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ToolChoiceMode$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ToolChoiceMode$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ToolChoiceMode) {
        __swift_bridge__$Vec_ToolChoiceMode$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ToolChoiceMode$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ToolChoiceMode(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ToolChoiceModeRef> {
        let pointer = __swift_bridge__$Vec_ToolChoiceMode$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ToolChoiceModeRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ToolChoiceModeRefMut> {
        let pointer = __swift_bridge__$Vec_ToolChoiceMode$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ToolChoiceModeRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ToolChoiceModeRef> {
        UnsafePointer<ToolChoiceModeRef>(OpaquePointer(__swift_bridge__$Vec_ToolChoiceMode$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ToolChoiceMode$len(vecPtr)
    }
}


public class ToolType: ToolTypeRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ToolType$_free(ptr)
        }
    }
}
public class ToolTypeRefMut: ToolTypeRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ToolTypeRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ToolTypeRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$ToolType$to_string(ptr))
    }
}
extension ToolType: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ToolType$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ToolType$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ToolType) {
        __swift_bridge__$Vec_ToolType$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ToolType$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ToolType(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ToolTypeRef> {
        let pointer = __swift_bridge__$Vec_ToolType$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ToolTypeRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ToolTypeRefMut> {
        let pointer = __swift_bridge__$Vec_ToolType$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ToolTypeRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ToolTypeRef> {
        UnsafePointer<ToolTypeRef>(OpaquePointer(__swift_bridge__$Vec_ToolType$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ToolType$len(vecPtr)
    }
}


public class UserContent: UserContentRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$UserContent$_free(ptr)
        }
    }
}
public class UserContentRefMut: UserContentRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class UserContentRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension UserContentRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$UserContent$to_string(ptr))
    }
}
extension UserContent: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_UserContent$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_UserContent$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: UserContent) {
        __swift_bridge__$Vec_UserContent$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_UserContent$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (UserContent(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<UserContentRef> {
        let pointer = __swift_bridge__$Vec_UserContent$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return UserContentRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<UserContentRefMut> {
        let pointer = __swift_bridge__$Vec_UserContent$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return UserContentRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<UserContentRef> {
        UnsafePointer<UserContentRef>(OpaquePointer(__swift_bridge__$Vec_UserContent$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_UserContent$len(vecPtr)
    }
}


public class DefaultClientChatStreamStreamHandle: DefaultClientChatStreamStreamHandleRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$DefaultClientChatStreamStreamHandle$_free(ptr)
        }
    }
}
public class DefaultClientChatStreamStreamHandleRefMut: DefaultClientChatStreamStreamHandleRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
extension DefaultClientChatStreamStreamHandleRefMut {
    public func next() throws -> RustString {
        try { let val = __swift_bridge__$DefaultClientChatStreamStreamHandle$next(ptr); if val.is_ok { return RustString(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
    }
}
public class DefaultClientChatStreamStreamHandleRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension DefaultClientChatStreamStreamHandle: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_DefaultClientChatStreamStreamHandle$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_DefaultClientChatStreamStreamHandle$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: DefaultClientChatStreamStreamHandle) {
        __swift_bridge__$Vec_DefaultClientChatStreamStreamHandle$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_DefaultClientChatStreamStreamHandle$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (DefaultClientChatStreamStreamHandle(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DefaultClientChatStreamStreamHandleRef> {
        let pointer = __swift_bridge__$Vec_DefaultClientChatStreamStreamHandle$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DefaultClientChatStreamStreamHandleRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DefaultClientChatStreamStreamHandleRefMut> {
        let pointer = __swift_bridge__$Vec_DefaultClientChatStreamStreamHandle$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DefaultClientChatStreamStreamHandleRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<DefaultClientChatStreamStreamHandleRef> {
        UnsafePointer<DefaultClientChatStreamStreamHandleRef>(OpaquePointer(__swift_bridge__$Vec_DefaultClientChatStreamStreamHandle$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_DefaultClientChatStreamStreamHandle$len(vecPtr)
    }
}
