>>> dispatch-worker tier=worker model=Worker endpoint=http://localhost:20128/v1
>>> routed to backend: gemini-3-flash-preview
### 1. Plan Summary

#### Current Public Surface
* `src/index.ts`: Entry point for the MCP server.
* `src/server.ts`: Implementation of the `CheapLLMServer` class and MCP tool handlers.
* `src/llm-provider.ts`: Abstract base and concrete implementations for LLM providers (OpenAI, Anthropic).
* `src/types.ts`: Internal type definitions for configuration and API responses.

#### Re-Export List
* `src/index.ts` -> `CheapLLMServer` (from `src/server.ts`)
* `src/index.ts` -> `LLMProvider`, `OpenAIProvider`, `AnthropicProvider` (from `src/llm-provider.ts`)
* `src/index.ts` -> `ConfigSchema` (from `src/types.ts`)

#### Exact PR Title / Summary / Validation
* **Title**: `refactor: modularize codebase and expose public API surface`
* **Summary**: This PR restructures the project to support programmatic usage as a library while maintaining its functionality as a standalone MCP server. It introduces a clean export structure in `index.ts` and decouples the server logic from the CLI entry point.
* **Validation**:
    * Run `npm run build` to ensure types are generated correctly.
    * Execute `npm test` to verify tool handlers and provider logic.
    * Link the package locally and import `CheapLLMServer` in a test script.

#### Exact Diff
```diff
--- a/src/index.ts
+++ b/src/index.ts
@@ -1,15 +1,6 @@
 #!/usr/bin/env node
-import { Server } from "@modelcontextprotocol/sdk/server/index.js";
-import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
-import {
-  CallToolRequestSchema,
-  ListToolsRequestSchema,
-} from "@modelcontextprotocol/sdk/types.js";
-import { CheapLLMServer } from "./server.js";
+export { CheapLLMServer } from "./server.js";
+export { OpenAIProvider, AnthropicProvider } from "./llm-provider.js";
+export * from "./types.js";
 
-const server = new CheapLLMServer();
-const transport = new StdioServerTransport();
-server.connect(transport).catch((error) => {
-  console.error("Server error:", error);
-  process.exit(1);
-});
+import { main } from "./cli.js";
+if (require.main === module) { main(); }
```

### 2. Implementation Checklist

- [ ] `src/index.ts` (Modify: ~15 lines)
- [ ] `src/cli.ts` (Create: ~20 lines)
- [ ] `src/server.ts` (Modify: ~10 lines)
- [ ] `src/llm-provider.ts` (Modify: ~5 lines)
- [ ] `package.json` (Modify: ~5 lines)

### 3. Validation Commands

```bash
# Clean and build the project
npm run clean && npm run build

# Run unit tests
npm test

# Verify CLI execution
node build/index.js --help

# Verify programmatic import (requires local link or manual check)
node -e 'require("./build/index.js").CheapLLMServer ? process.exit(0) : process.exit(1)'
```

### 4. Risks / Blockers

* **Circular Dependencies**: Moving `main()` logic to a new `cli.ts` might create issues if `server.ts` imports from `index.ts`. Ensure unidirectional flow.
* **Breaking Changes**: External consumers using the CLI via `npx` must ensure the `bin` field in `package.json` still points to the transpiled `index.js`.
* **Environment Variables**: Programmatic usage may bypass standard `.env` loading if not handled explicitly in the `CheapLLMServer` constructor.
