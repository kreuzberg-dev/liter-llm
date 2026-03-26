defmodule LiterLlm.ErrorTest do
  use ExUnit.Case, async: true

  alias LiterLlm.Error

  # ─── Error Constructors ───────────────────────────────────────────────────

  describe "error constructors" do
    test "unknown/1 sets correct kind and code" do
      err = Error.unknown("something went wrong")

      assert err.kind == :unknown
      assert err.code == 1000
      assert err.message =~ "something went wrong"
    end

    test "invalid_request/1 sets correct kind and code" do
      err = Error.invalid_request("missing field")

      assert err.kind == :invalid_request
      assert err.code == 1400
      assert err.message =~ "invalid request"
      assert err.message =~ "missing field"
    end

    test "authentication/1 sets correct kind and code" do
      err = Error.authentication("bad key")

      assert err.kind == :authentication
      assert err.code == 1401
    end

    test "not_found/1 sets correct kind and code" do
      err = Error.not_found("model not found")

      assert err.kind == :not_found
      assert err.code == 1404
    end

    test "rate_limit/1 sets correct kind and code" do
      err = Error.rate_limit("slow down")

      assert err.kind == :rate_limit
      assert err.code == 1429
    end

    test "provider_error/2 includes http_status" do
      err = Error.provider_error(503, "Service Unavailable")

      assert err.kind == :provider_error
      assert err.code == 1500
      assert err.http_status == 503
      assert err.message =~ "503"
    end

    test "stream_error/1 sets correct kind and code" do
      err = Error.stream_error("unexpected EOF")

      assert err.kind == :stream_error
      assert err.code == 1600
    end

    test "serialization/1 sets correct kind and code" do
      err = Error.serialization("bad JSON")

      assert err.kind == :serialization
      assert err.code == 1700
    end
  end

  # ─── from_http_status/2 ───────────────────────────────────────────────────

  describe "from_http_status/2" do
    test "maps 400 to :invalid_request" do
      err = Error.from_http_status(400, "bad request")
      assert err.kind == :invalid_request
    end

    test "maps 401 to :authentication" do
      err = Error.from_http_status(401, "unauthorized")
      assert err.kind == :authentication
    end

    test "maps 403 to :authentication" do
      err = Error.from_http_status(403, "forbidden")
      assert err.kind == :authentication
    end

    test "maps 404 to :not_found" do
      err = Error.from_http_status(404, "not found")
      assert err.kind == :not_found
    end

    test "maps 429 to :rate_limit" do
      err = Error.from_http_status(429, "too many requests")
      assert err.kind == :rate_limit
    end

    test "maps 500 to :provider_error" do
      err = Error.from_http_status(500, "internal server error")
      assert err.kind == :provider_error
      assert err.http_status == 500
    end

    test "maps 503 to :provider_error" do
      err = Error.from_http_status(503, "service unavailable")
      assert err.kind == :provider_error
    end
  end

  # ─── extract_message/1 ────────────────────────────────────────────────────

  describe "extract_message/1" do
    test "extracts message from nested error map" do
      body = %{"error" => %{"message" => "Invalid API key", "type" => "auth_error"}}
      assert Error.extract_message(body) == "Invalid API key"
    end

    test "handles nil body" do
      assert Error.extract_message(nil) == "empty response body"
    end

    test "handles empty string" do
      assert Error.extract_message("") == "empty response body"
    end

    test "decodes JSON string and extracts message" do
      body = Jason.encode!(%{"error" => %{"message" => "Rate limit exceeded"}})
      assert Error.extract_message(body) == "Rate limit exceeded"
    end

    test "returns truncated body for unknown format" do
      body = "plain text error"
      assert Error.extract_message(body) == "plain text error"
    end

    test "truncates very long error bodies" do
      body = String.duplicate("x", 300)
      result = Error.extract_message(body)
      assert String.length(result) <= 204
      assert result =~ "…"
    end
  end

  # ─── String.Chars protocol ────────────────────────────────────────────────

  describe "String.Chars implementation" do
    test "converts error to string via message field" do
      err = Error.rate_limit("too many requests")
      assert to_string(err) =~ "rate limit"
    end
  end
end
