import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Backup & Restore","description":"FocalPoint documentation - Backup & Restore","frontmatter":{"title":"Backup & Restore","description":"FocalPoint documentation - Backup & Restore"},"headers":[],"relativePath":"guides/backup-restore.md","filePath":"guides/backup-restore.md","lastUpdated":null}');
const _sfc_main = { name: "guides/backup-restore.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="backup-restore" tabindex="-1">Backup &amp; Restore <a class="header-anchor" href="#backup-restore" aria-label="Permalink to &quot;Backup &amp; Restore&quot;">​</a></h1><p>This page is coming soon. Check back soon for more information.</p></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("guides/backup-restore.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const backupRestore = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  backupRestore as default
};
