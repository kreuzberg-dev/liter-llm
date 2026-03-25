"""Pure-Python TypedDict definitions for liter-lm request parameter types.

These are ordinary Python classes — not compiled extension types — so they are
importable at any time without the native extension being available.  They exist
solely to provide structural typing for :meth:`LlmClient.chat` kwargs, etc.

All TypedDicts use ``total=False`` with individual fields marked
:class:`~typing.Required` so that optional fields do not need to be specified
explicitly by callers.
"""

from __future__ import annotations

import sys

if sys.version_info >= (3, 13):
    from typing import Literal, Required, TypeAlias, TypedDict
else:
    from typing import Literal, TypeAlias

    from typing_extensions import Required, TypedDict

# ─── Content parts ────────────────────────────────────────────────────────────


class ImageUrlParam(TypedDict, total=False):
    url: Required[str]
    detail: Literal["low", "high", "auto"] | None


class ContentPartTextParam(TypedDict, total=False):
    type: Required[Literal["text"]]
    text: Required[str]


class ContentPartImageParam(TypedDict, total=False):
    type: Required[Literal["image_url"]]
    image_url: Required[ImageUrlParam]


ContentPartParam: TypeAlias = ContentPartTextParam | ContentPartImageParam

# ─── Tool types ───────────────────────────────────────────────────────────────


class FunctionCallParam(TypedDict, total=False):
    name: Required[str]
    arguments: Required[str]


class ToolCallParam(TypedDict, total=False):
    id: Required[str]
    type: Required[Literal["function"]]
    function: Required[FunctionCallParam]


class FunctionDefinitionParam(TypedDict, total=False):
    name: Required[str]
    description: str | None
    parameters: object | None
    strict: bool | None


class ToolParam(TypedDict, total=False):
    type: Required[Literal["function"]]
    function: Required[FunctionDefinitionParam]


class SpecificFunctionParam(TypedDict, total=False):
    name: Required[str]


class SpecificToolChoiceParam(TypedDict, total=False):
    type: Required[Literal["function"]]
    function: Required[SpecificFunctionParam]


ToolChoiceParam: TypeAlias = Literal["auto", "required", "none"] | SpecificToolChoiceParam

# ─── Response format ──────────────────────────────────────────────────────────


class JsonSchemaParam(TypedDict, total=False):
    name: Required[str]
    schema: Required[object]
    description: str | None
    strict: bool | None


class ResponseFormatTextParam(TypedDict, total=False):
    type: Required[Literal["text"]]


class ResponseFormatJsonObjectParam(TypedDict, total=False):
    type: Required[Literal["json_object"]]


class ResponseFormatJsonSchemaParam(TypedDict, total=False):
    type: Required[Literal["json_schema"]]
    json_schema: Required[JsonSchemaParam]


ResponseFormatParam: TypeAlias = ResponseFormatTextParam | ResponseFormatJsonObjectParam | ResponseFormatJsonSchemaParam

# ─── Stream options ───────────────────────────────────────────────────────────


class StreamOptionsParam(TypedDict, total=False):
    include_usage: bool | None


# ─── Message ──────────────────────────────────────────────────────────────────


class MessageParam(TypedDict, total=False):
    role: Required[Literal["system", "user", "assistant", "tool", "developer", "function"]]
    content: str | list[ContentPartParam] | None
    name: str | None
    tool_call_id: str | None
    tool_calls: list[ToolCallParam] | None


# ─── Chat completion request ──────────────────────────────────────────────────


class ChatCompletionRequestParams(TypedDict, total=False):
    model: Required[str]
    messages: Required[list[MessageParam]]
    temperature: float | None
    top_p: float | None
    n: int | None
    stream: bool | None
    stop: str | list[str] | None
    max_tokens: int | None
    presence_penalty: float | None
    frequency_penalty: float | None
    logit_bias: dict[str, float] | None
    user: str | None
    tools: list[ToolParam] | None
    tool_choice: ToolChoiceParam | None
    parallel_tool_calls: bool | None
    response_format: ResponseFormatParam | None
    stream_options: StreamOptionsParam | None
    seed: int | None


# ─── Embedding request ────────────────────────────────────────────────────────


class EmbeddingRequestParams(TypedDict, total=False):
    model: Required[str]
    input: Required[str | list[str]]
    encoding_format: str | None
    dimensions: int | None
    user: str | None
