import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Audit Chain","description":"FocalPoint documentation - Audit Chain","frontmatter":{"title":"Audit Chain","description":"FocalPoint documentation - Audit Chain"},"headers":[],"relativePath":"architecture/audit-chain.md","filePath":"architecture/audit-chain.md","lastUpdated":null}');
const _sfc_main = { name: "architecture/audit-chain.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="audit-chain" tabindex="-1">Audit Chain <a class="header-anchor" href="#audit-chain" aria-label="Permalink to &quot;Audit Chain&quot;">​</a></h1><p>This page is coming soon. Check back soon for more information.</p></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("architecture/audit-chain.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const auditChain = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  auditChain as default
};
