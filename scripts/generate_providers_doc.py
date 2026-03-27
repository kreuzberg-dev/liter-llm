#!/usr/bin/env python3
"""
Providers documentation generator for Liter-LLM.

Reads schemas/providers.json and generates docs/providers.md with a
searchable table of all supported LLM providers and their capabilities.

Supports --validate mode for CI (exits non-zero if docs are stale)
and --dry-run mode for preview.
"""

import argparse
import json
import logging
import sys
from pathlib import Path

logging.basicConfig(level=logging.INFO, format="%(levelname)s: %(message)s")
logger = logging.getLogger(__name__)

PROJECT_ROOT = Path(__file__).resolve().parent.parent
SCHEMA_PATH = PROJECT_ROOT / "schemas" / "providers.json"
OUTPUT_PATH = PROJECT_ROOT / "docs" / "providers.md"

# Endpoint columns to display in the table
ENDPOINT_COLUMNS = ["chat", "embedding", "image", "audio", "moderation"]

CHECK = ":white_check_mark:"
DASH = "--"


def load_providers(schema_path: Path) -> list[dict]:
    """Load and return the providers list from the JSON schema."""
    with schema_path.open() as f:
        data = json.load(f)
    return data["providers"]


def provider_prefix(provider: dict) -> str:
    """Derive the routing prefix for a provider."""
    name = provider["name"]
    return f"`{name}/`"


def endpoint_cell(provider: dict, endpoint: str) -> str:
    """Return a checkmark or dash for a given endpoint."""
    endpoints = provider.get("endpoints", [])
    return CHECK if endpoint in endpoints else DASH


def generate_markdown(providers: list[dict]) -> str:
    """Generate the full providers.md content."""
    sorted_providers = sorted(providers, key=lambda p: p["display_name"].lower())
    count = len(sorted_providers)

    lines: list[str] = []

    # Front matter
    lines.append("---")
    lines.append(f'description: "Complete list of {count} supported LLM providers"')
    lines.append("---")
    lines.append("")

    # Title and intro
    lines.append("# Supported Providers")
    lines.append("")
    lines.append(
        f"Liter-LLM supports **{count} providers** out of the box. "
        "Route requests to any provider using the `provider/model` prefix convention "
        "-- for example, `openai/gpt-4o` routes to OpenAI and `anthropic/claude-3-opus` "
        "routes to Anthropic. No extra configuration is needed beyond setting the "
        "provider's API key."
    )
    lines.append("")

    # Table header
    lines.append("| Provider | Prefix | Chat | Embeddings | Image | Audio | Moderation |")
    lines.append("| --- | --- | :---: | :---: | :---: | :---: | :---: |")

    # Table rows
    for p in sorted_providers:
        display = p["display_name"]
        prefix = provider_prefix(p)
        cells = [endpoint_cell(p, ep) for ep in ENDPOINT_COLUMNS]
        row = f"| {display} | {prefix} | {' | '.join(cells)} |"
        lines.append(row)

    lines.append("")
    lines.append(f"*{count} providers total.*")
    lines.append("")

    # Usage section
    lines.append("## Usage")
    lines.append("")
    lines.append("Use any provider by prefixing the model name with the provider's routing prefix:")
    lines.append("")
    lines.append("```python")
    lines.append("from liter_llm import LiterLLM")
    lines.append("")
    lines.append("client = LiterLLM()")
    lines.append("")
    lines.append("# OpenAI")
    lines.append('response = await client.chat("openai/gpt-4o", messages=[')
    lines.append('    {"role": "user", "content": "Hello!"}')
    lines.append("])")
    lines.append("")
    lines.append("# Anthropic")
    lines.append('response = await client.chat("anthropic/claude-3-opus", messages=[')
    lines.append('    {"role": "user", "content": "Hello!"}')
    lines.append("])")
    lines.append("")
    lines.append("# Groq")
    lines.append('response = await client.chat("groq/llama3-70b", messages=[')
    lines.append('    {"role": "user", "content": "Hello!"}')
    lines.append("])")
    lines.append("```")
    lines.append("")

    # Custom providers section
    lines.append("## Custom Providers")
    lines.append("")
    lines.append(
        "Any OpenAI-compatible API can be used as a custom provider by setting the base URL and API key directly:"
    )
    lines.append("")
    lines.append("```python")
    lines.append('response = await client.chat("custom/my-model",')
    lines.append('    base_url="https://my-api.example.com/v1",')
    lines.append('    api_key="my-key",')
    lines.append("    messages=[")
    lines.append('        {"role": "user", "content": "Hello!"}')
    lines.append("    ]")
    lines.append(")")
    lines.append("```")
    lines.append("")

    # Link to raw JSON
    lines.append("## Provider Registry")
    lines.append("")
    lines.append(
        "The full provider registry with base URLs, auth configuration, and model "
        "mappings is available at "
        "[schemas/providers.json]"
        "(https://github.com/kreuzberg-dev/liter-llm/blob/main/schemas/providers.json)."
    )
    lines.append("")

    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate providers documentation from schemas/providers.json")
    parser.add_argument(
        "--validate",
        action="store_true",
        help="Check if docs/providers.md matches generated output (for CI)",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print generated output to stdout without writing",
    )
    args = parser.parse_args()

    if not SCHEMA_PATH.exists():
        logger.error("Schema not found: %s", SCHEMA_PATH)
        return 1

    providers = load_providers(SCHEMA_PATH)
    logger.info("Loaded %d providers from %s", len(providers), SCHEMA_PATH.name)

    content = generate_markdown(providers)

    if args.dry_run:
        print(content)
        return 0

    if args.validate:
        if not OUTPUT_PATH.exists():
            logger.error("Output file does not exist: %s", OUTPUT_PATH)
            return 1
        existing = OUTPUT_PATH.read_text()
        if existing == content:
            logger.info("docs/providers.md is up-to-date")
            return 0
        logger.error("docs/providers.md is out of date. Run 'task generate:providers-doc' to regenerate.")
        return 1

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(content)
    logger.info("Generated %s (%d providers)", OUTPUT_PATH.relative_to(PROJECT_ROOT), len(providers))
    return 0


if __name__ == "__main__":
    sys.exit(main())
