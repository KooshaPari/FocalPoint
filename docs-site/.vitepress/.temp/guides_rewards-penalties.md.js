import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Rewards & Penalties","description":"FocalPoint documentation - Rewards & Penalties","frontmatter":{"title":"Rewards & Penalties","description":"FocalPoint documentation - Rewards & Penalties"},"headers":[],"relativePath":"guides/rewards-penalties.md","filePath":"guides/rewards-penalties.md","lastUpdated":null}');
const _sfc_main = { name: "guides/rewards-penalties.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="rewards-penalties" tabindex="-1">Rewards &amp; Penalties <a class="header-anchor" href="#rewards-penalties" aria-label="Permalink to &quot;Rewards &amp; Penalties&quot;">​</a></h1><p>This page is coming soon. Check back soon for more information.</p></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("guides/rewards-penalties.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const rewardsPenalties = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  rewardsPenalties as default
};
