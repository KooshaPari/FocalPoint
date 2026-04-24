import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Feedback","description":"FocalPoint documentation - Feedback","frontmatter":{"title":"Feedback","description":"FocalPoint documentation - Feedback"},"headers":[],"relativePath":"guides/feedback.md","filePath":"guides/feedback.md","lastUpdated":null}');
const _sfc_main = { name: "guides/feedback.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="feedback" tabindex="-1">Feedback <a class="header-anchor" href="#feedback" aria-label="Permalink to &quot;Feedback&quot;">​</a></h1><p>This page is coming soon. Check back soon for more information.</p></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("guides/feedback.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const feedback = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  feedback as default
};
