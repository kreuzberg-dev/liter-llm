/**
 * Smoke tests for the @kreuzberg/liter-llm-wasm published package.
 *
 * Validates the published WASM package works against real LLM APIs.
 * Requires API keys in environment variables or .env file at repo root.
 *
 * Run: npm install && node main.mjs
 */

import { readFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

// Load .env from repo root
const __dirname = dirname(fileURLToPath(import.meta.url));
{
	let dir = __dirname;
	for (let i = 0; i < 5; i++) {
		const envPath = join(dir, ".env");
		if (existsSync(envPath)) {
			for (const line of readFileSync(envPath, "utf-8").split("\n")) {
				const trimmed = line.trim();
				if (trimmed && !trimmed.startsWith("#") && trimmed.includes("=")) {
					const idx = trimmed.indexOf("=");
					const key = trimmed.slice(0, idx).trim();
					const value = trimmed.slice(idx + 1).trim();
					if (!(key in process.env)) {
						process.env[key] = value;
					}
				}
			}
			break;
		}
		dir = dirname(dir);
	}
}

import { LlmClient } from "@kreuzberg/liter-llm-wasm";

// ─── Test runner ──────────────────────────────────────────────────────────────

let passed = 0;
let failed = 0;
let skipped = 0;

async function run(name, fn) {
	process.stdout.write(`  ${name}... `);
	try {
		const result = await fn();
		if (result === null) {
			process.stdout.write("SKIP\n");
			skipped++;
		} else {
			process.stdout.write("PASS\n");
			passed++;
		}
	} catch (err) {
		process.stdout.write(`FAIL: ${err}\n`);
		failed++;
	}
}

// ─── Test cases ───────────────────────────────────────────────────────────────

async function testChatOpenAI() {
	const key = process.env["OPENAI_API_KEY"];
	if (!key) return null;
	const client = new LlmClient({ apiKey: key });
	const r = await client.chat({
		model: "openai/gpt-4o-mini",
		messages: [{ role: "user", content: "Say hello in one word." }],
		max_tokens: 10,
	});
	if (!r.choices || r.choices.length === 0) throw new Error("no choices in response");
	if (!r.choices[0].message.content) throw new Error("empty content");
	if (!r.usage || r.usage.total_tokens <= 0) throw new Error("no usage data");
	return "ok";
}

async function testChatAnthropic() {
	const key = process.env["ANTHROPIC_API_KEY"];
	if (!key) return null;
	const client = new LlmClient({ apiKey: key });
	const r = await client.chat({
		model: "anthropic/claude-sonnet-4-20250514",
		messages: [{ role: "user", content: "Say hello in one word." }],
		max_tokens: 10,
	});
	if (!r.choices || r.choices.length === 0) throw new Error("no choices");
	if (!r.choices[0].message.content) throw new Error("empty content");
	return "ok";
}

async function testChatGemini() {
	const key = process.env["GEMINI_API_KEY"];
	if (!key) return null;
	const client = new LlmClient({ apiKey: key });
	const r = await client.chat({
		model: "gemini/gemini-2.5-flash-lite",
		messages: [{ role: "user", content: "Say hello in one word." }],
		max_tokens: 10,
	});
	if (!r.choices || r.choices.length === 0) throw new Error("no choices");
	if (!r.choices[0].message.content) throw new Error("empty content");
	return "ok";
}

async function testStreamingOpenAI() {
	const key = process.env["OPENAI_API_KEY"];
	if (!key) return null;
	const client = new LlmClient({ apiKey: key });
	const chunks = await client.chatStream({
		model: "openai/gpt-4o-mini",
		messages: [{ role: "user", content: "Count from 1 to 5." }],
		max_tokens: 50,
	});
	if (!chunks || chunks.length === 0) throw new Error("no chunks received");
	return "ok";
}

async function testEmbedOpenAI() {
	const key = process.env["OPENAI_API_KEY"];
	if (!key) return null;
	const client = new LlmClient({ apiKey: key });
	const r = await client.embed({
		model: "openai/text-embedding-3-small",
		input: ["Hello, world!"],
	});
	if (!r.data || r.data.length === 0) throw new Error("no embeddings");
	if (!r.data[0].embedding || r.data[0].embedding.length === 0) throw new Error("empty embedding vector");
	return "ok";
}

async function testListModelsOpenAI() {
	const key = process.env["OPENAI_API_KEY"];
	if (!key) return null;
	const client = new LlmClient({ apiKey: key });
	const r = await client.listModels();
	if (!r.data || r.data.length === 0) throw new Error("no models returned");
	return "ok";
}

async function testProviderRouting() {
	const openaiKey = process.env["OPENAI_API_KEY"];
	const anthropicKey = process.env["ANTHROPIC_API_KEY"];
	if (!openaiKey || !anthropicKey) return null;

	const messages = [{ role: "user", content: "Say hi." }];

	const clientOpenAI = new LlmClient({ apiKey: openaiKey });
	const r1 = await clientOpenAI.chat({
		model: "openai/gpt-4o-mini",
		messages,
		max_tokens: 5,
	});
	if (!r1.choices || r1.choices.length === 0) throw new Error("OpenAI failed");

	const clientAnthropic = new LlmClient({ apiKey: anthropicKey });
	const r2 = await clientAnthropic.chat({
		model: "anthropic/claude-sonnet-4-20250514",
		messages,
		max_tokens: 5,
	});
	if (!r2.choices || r2.choices.length === 0) throw new Error("Anthropic failed");
	return "ok";
}

async function testCacheMemory() {
	const key = process.env["OPENAI_API_KEY"];
	if (!key) return null;
	const client = new LlmClient({
		apiKey: key,
		cache: { maxEntries: 10, ttlSeconds: 60 },
	});
	const messages = [{ role: "user", content: "What is 2+2? Answer with just the number." }];
	const r1 = await client.chat({
		model: "openai/gpt-4o-mini",
		messages,
		max_tokens: 5,
	});
	const r2 = await client.chat({
		model: "openai/gpt-4o-mini",
		messages,
		max_tokens: 5,
	});
	if (!r1.choices || r1.choices.length === 0) throw new Error("first request failed");
	if (!r2.choices || r2.choices.length === 0) throw new Error("second request failed");
	if (r1.choices[0].message.content !== r2.choices[0].message.content) {
		throw new Error("cache miss - responses differ");
	}
	return "ok";
}

// ─── Main ─────────────────────────────────────────────────────────────────────

async function main() {
	process.stdout.write("liter-llm Smoke Tests (WASM)\n");
	process.stdout.write("=".repeat(60) + "\n\n");

	process.stdout.write("Chat Completions:\n");
	await run("OpenAI gpt-4o-mini", testChatOpenAI);
	await run("Anthropic claude-3-5-haiku", testChatAnthropic);
	await run("Google gemini-2.5-flash-lite", testChatGemini);

	await run("OpenAI streaming", testStreamingOpenAI);
	await run("OpenAI text-embedding-3-small", testEmbedOpenAI);
	await run("OpenAI list models", testListModelsOpenAI);
	await run("Multi-provider routing", testProviderRouting);
	await run("In-memory cache hit", testCacheMemory);

	const total = passed + failed + skipped;
	process.stdout.write(`\n${"=".repeat(60)}\n`);
	process.stdout.write(`Results: ${passed} passed, ${failed} failed, ${skipped} skipped (${total} total)\n`);

	return failed > 0 ? 1 : 0;
}

main().then((code) => process.exit(code));
