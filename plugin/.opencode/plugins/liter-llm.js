// AI-RULEZ :: GENERATED FILE — DO NOT EDIT
// Content-Hash: blake3:1ad3d813d05d2ed27d81f6e6848ad8cf124882181d5f4d5ca2e62e81d7bbc91c
// Source-Hash: blake3:5982f9e920534d61a2cd6166e32a8ee98938c731fc6e892800aaecbeaf8c1221
// Schema-Version: v1

/**
 * OpenCode-specific plugin entrypoint.
 *
 * ai-rulez copies this source module into the generated OpenCode package.
 * Shared skills, commands, agents, and MCP configuration belong in their normal
 * `.ai-rulez` sources; add only OpenCode-specific tools or hooks here.
 *
 * To extend this plugin:
 * 1. Import `tool` from `@opencode-ai/plugin`.
 * 2. Define tool arguments with `tool.schema` and validate every external
 * input.
 * 3. Return the OpenCode hooks object from this function.
 * 4. Preview with `ai-rulez generate --plugin --dry-run` before regenerating.
 *
 * Pass subprocess arguments as an array. Never interpolate external input into
 * a shell command.
 */
const LiterLlmPlugin = async () => ({});

export default LiterLlmPlugin;
