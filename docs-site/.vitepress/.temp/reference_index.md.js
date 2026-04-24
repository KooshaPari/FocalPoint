import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Reference","description":"Quick references, design tokens, traceability, and coverage metrics.","frontmatter":{"title":"Reference","description":"Quick references, design tokens, traceability, and coverage metrics."},"headers":[],"relativePath":"reference/index.md","filePath":"reference/index.md","lastUpdated":1777002546000}');
const _sfc_main = { name: "reference/index.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="reference-materials" tabindex="-1">Reference Materials <a class="header-anchor" href="#reference-materials" aria-label="Permalink to &quot;Reference Materials&quot;">​</a></h1><p>Central hub for quick-look documentation, design system tokens, test traceability, and coverage reports.</p><h2 id="quick-references" tabindex="-1">Quick References <a class="header-anchor" href="#quick-references" aria-label="Permalink to &quot;Quick References&quot;">​</a></h2><ul><li><strong><a href="./design-tokens">Design Tokens</a></strong> — Colors, typography, spacing, component sizes</li><li><strong><a href="./traceability">Traceability Matrix</a></strong> — Requirements ↔ Features ↔ Tests</li><li><strong><a href="./honest-coverage">Honest Coverage Report</a></strong> — Test coverage broken down by component</li></ul><h2 id="architecture-references" tabindex="-1">Architecture References <a class="header-anchor" href="#architecture-references" aria-label="Permalink to &quot;Architecture References&quot;">​</a></h2><p>See <a href="./../architecture/">Architecture</a> for:</p><ul><li>System diagram and subsystem responsibilities</li><li>FFI topology for iOS/Android bindings</li><li>Connector framework specifications</li><li>Architectural decision records (ADRs)</li></ul><h2 id="governance-process" tabindex="-1">Governance &amp; Process <a class="header-anchor" href="#governance-process" aria-label="Permalink to &quot;Governance &amp; Process&quot;">​</a></h2><p>See <a href="./../governance/">Governance</a> for:</p><ul><li>Contribution guidelines</li><li>Code of Conduct</li><li>Verification &amp; security criteria</li></ul><h2 id="full-documentation-index" tabindex="-1">Full Documentation Index <a class="header-anchor" href="#full-documentation-index" aria-label="Permalink to &quot;Full Documentation Index&quot;">​</a></h2><p>All documentation is organized as:</p><div class="language- vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang"></span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>docs-site/</span></span>
<span class="line"><span>├── index.md (home)</span></span>
<span class="line"><span>├── getting-started/ (setup, first rule, troubleshooting)</span></span>
<span class="line"><span>├── architecture/ (system design, ADRs)</span></span>
<span class="line"><span>├── connectors/ (integrations)</span></span>
<span class="line"><span>├── connector-sdk/ (building connectors)</span></span>
<span class="line"><span>├── ecosystem/ (marketplace, rule packs)</span></span>
<span class="line"><span>├── rules/ (DSL, conditions, actions)</span></span>
<span class="line"><span>├── mascot/ (Coachy personality &amp; coaching)</span></span>
<span class="line"><span>├── rituals/ (morning brief, evening shutdown)</span></span>
<span class="line"><span>├── reference/ (this section)</span></span>
<span class="line"><span>├── governance/ (contribution, conduct, verification)</span></span>
<span class="line"><span>├── journeys/ (user personas &amp; workflows)</span></span>
<span class="line"><span>├── guides/ (how-to articles)</span></span>
<span class="line"><span>└── reports/ (audit reports, status)</span></span></code></pre></div></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("reference/index.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const index = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  index as default
};
