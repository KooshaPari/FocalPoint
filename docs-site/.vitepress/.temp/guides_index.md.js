import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"User Guides","description":"How-to guides for using FocalPoint to manage your screen time.","frontmatter":{"title":"User Guides","description":"How-to guides for using FocalPoint to manage your screen time."},"headers":[],"relativePath":"guides/index.md","filePath":"guides/index.md","lastUpdated":null}');
const _sfc_main = { name: "guides/index.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="user-guides" tabindex="-1">User Guides <a class="header-anchor" href="#user-guides" aria-label="Permalink to &quot;User Guides&quot;">​</a></h1><p>Step-by-step guides to help you get the most out of FocalPoint.</p><h2 id="getting-started" tabindex="-1">Getting Started <a class="header-anchor" href="#getting-started" aria-label="Permalink to &quot;Getting Started&quot;">​</a></h2><ul><li><a href="/getting-started/">Quick Start</a> — Install and set up FocalPoint</li><li><a href="/getting-started/first-rule">First Rule Walkthrough</a> — Write your first rule in 5 minutes</li></ul><h2 id="features-workflows" tabindex="-1">Features &amp; Workflows <a class="header-anchor" href="#features-workflows" aria-label="Permalink to &quot;Features &amp; Workflows&quot;">​</a></h2><ul><li><a href="/rules/">Write a Rule</a> — Create custom automation rules</li><li><a href="/guides/focus-mode">Focus Mode</a> — Block distracting apps during focused work</li><li><a href="/guides/rewards-penalties">Rewards &amp; Penalties</a> — Build healthy streaks with the dual ledger system</li><li><a href="/ecosystem/">Install a Template Pack</a> — Use pre-built rules for common workflows</li><li><a href="/guides/backup-restore">Backup &amp; Restore</a> — Sync your settings and rules across devices</li></ul><h2 id="community" tabindex="-1">Community <a class="header-anchor" href="#community" aria-label="Permalink to &quot;Community&quot;">​</a></h2><ul><li><a href="/guides/feedback">Feedback</a> — Share ideas and report bugs</li><li><a href="/governance/contributing">Contributing</a> — Help improve FocalPoint</li></ul></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("guides/index.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const index = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  index as default
};
