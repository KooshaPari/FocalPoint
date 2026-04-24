import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Verification Tiers","description":"Verification requirements for published rule templates and connectors.","frontmatter":{"title":"Verification Tiers","description":"Verification requirements for published rule templates and connectors."},"headers":[],"relativePath":"ecosystem/verification-tiers.md","filePath":"ecosystem/verification-tiers.md","lastUpdated":null}');
const _sfc_main = { name: "ecosystem/verification-tiers.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="verification-tiers" tabindex="-1">Verification Tiers <a class="header-anchor" href="#verification-tiers" aria-label="Permalink to &quot;Verification Tiers&quot;">​</a></h1><p>FocalPoint uses a 3-tier verification system for community-contributed connectors and rule templates.</p><h2 id="tier-1-self-published" tabindex="-1">Tier 1: Self-Published <a class="header-anchor" href="#tier-1-self-published" aria-label="Permalink to &quot;Tier 1: Self-Published&quot;">​</a></h2><ul><li>No review required</li><li>Author takes full responsibility</li><li>Community flag system for abuse</li><li>No marketplace badge</li></ul><h2 id="tier-2-reviewed" tabindex="-1">Tier 2: Reviewed <a class="header-anchor" href="#tier-2-reviewed" aria-label="Permalink to &quot;Tier 2: Reviewed&quot;">​</a></h2><ul><li>Code review by FocalPoint maintainers</li><li>Test coverage requirements (&gt;50%)</li><li>Security audit</li><li>Marketplace badge: &quot;Reviewed&quot;</li></ul><h2 id="tier-3-official" tabindex="-1">Tier 3: Official <a class="header-anchor" href="#tier-3-official" aria-label="Permalink to &quot;Tier 3: Official&quot;">​</a></h2><ul><li>Full integration with FocalPoint team</li><li>Ongoing maintenance commitment</li><li>Quarterly security updates</li><li>Marketplace badge: &quot;Official&quot;</li></ul><h2 id="audit-criteria" tabindex="-1">Audit Criteria <a class="header-anchor" href="#audit-criteria" aria-label="Permalink to &quot;Audit Criteria&quot;">​</a></h2><p>All submissions must:</p><ul><li>Have clear documentation</li><li>Include example configurations</li><li>Pass automated lint checks</li><li>Declare all permissions</li></ul><p>See <a href="/governance/">Governance</a> for the full audit process.</p></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("ecosystem/verification-tiers.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const verificationTiers = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  verificationTiers as default
};
