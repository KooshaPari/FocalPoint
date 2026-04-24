import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Core Loop","description":"FocalPoint documentation - Core Loop","frontmatter":{"title":"Core Loop","description":"FocalPoint documentation - Core Loop"},"headers":[],"relativePath":"guides/core-loop.md","filePath":"guides/core-loop.md","lastUpdated":null}');
const _sfc_main = { name: "guides/core-loop.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="core-loop" tabindex="-1">Core Loop <a class="header-anchor" href="#core-loop" aria-label="Permalink to &quot;Core Loop&quot;">​</a></h1><p>This page is coming soon. Check back soon for more information.</p></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("guides/core-loop.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const coreLoop = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  coreLoop as default
};
