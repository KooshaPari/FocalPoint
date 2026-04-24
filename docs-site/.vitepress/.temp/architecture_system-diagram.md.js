import { resolveComponent, useSSRContext } from "vue";
import { ssrRenderAttrs, ssrRenderSuspense, ssrRenderComponent } from "vue/server-renderer";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"System diagram","description":"FocalPoint documentation","frontmatter":{"title":"System diagram","description":"FocalPoint documentation"},"headers":[],"relativePath":"architecture/system-diagram.md","filePath":"architecture/system-diagram.md","lastUpdated":1776935227000}');
const _sfc_main = { name: "architecture/system-diagram.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  const _component_Mermaid = resolveComponent("Mermaid");
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="system-diagram" tabindex="-1">System diagram <a class="header-anchor" href="#system-diagram" aria-label="Permalink to &quot;System diagram&quot;">​</a></h1><p>End-to-end view of a single rule-fire cycle, from external event to audited block.</p>`);
  ssrRenderSuspense(_push, {
    default: () => {
      _push(ssrRenderComponent(_component_Mermaid, {
        id: "mermaid-6",
        class: "mermaid",
        graph: "sequenceDiagram%0A%20%20autonumber%0A%20%20participant%20Ext%20as%20External%20system%3Cbr%2F%3E(Canvas)%0A%20%20participant%20Conn%20as%20connector-canvas%0A%20%20participant%20Runtime%20as%20focus-connectors%3Cbr%2F%3Eruntime%0A%20%20participant%20Evt%20as%20focus-events%3Cbr%2F%3E(append-only)%0A%20%20participant%20Rules%20as%20focus-rules%3Cbr%2F%3Eengine%0A%20%20participant%20Policy%20as%20focus-policy%3Cbr%2F%3Edecision%0A%20%20participant%20Ledger%20as%20focus-rewards%20%2F%3Cbr%2F%3Efocus-penalties%0A%20%20participant%20Audit%20as%20focus-audit%3Cbr%2F%3Ehash%20chain%0A%20%20participant%20Store%20as%20focus-storage%3Cbr%2F%3E(SQLite)%0A%20%20participant%20Swift%20as%20iOS%20app%3Cbr%2F%3E(UniFFI)%0A%20%20participant%20FC%20as%20FamilyControls%20%2F%3Cbr%2F%3EManagedSettings%0A%20%20participant%20Coachy%20as%20Coachy%20mascot%0A%0A%20%20Ext-%3E%3EConn%3A%20HTTP%20GET%20%2Fassignments%20(polled)%0A%20%20Conn-%3E%3ERuntime%3A%20yield%20Events%20%5Bassignment.upcoming%5D%0A%20%20Runtime-%3E%3EEvt%3A%20append(event%2C%20cursor)%0A%20%20Evt-%3E%3EStore%3A%20INSERT%20INTO%20events%0A%20%20Evt--%3E%3ERules%3A%20notify(new%20events)%0A%20%20Rules-%3E%3ERules%3A%20match%20rules%20vs%20event%20window%0A%20%20Rules-%3E%3EPolicy%3A%20fire(rule_id%2C%20event%2C%20targets)%0A%20%20Policy-%3E%3ELedger%3A%20apply(reward%3F%20penalty%3F)%0A%20%20Policy-%3E%3EAudit%3A%20append(decision%20record)%0A%20%20Audit-%3E%3EStore%3A%20INSERT%20INTO%20audit%20(hash%20%3D%20sha256(prev%7C%7Crecord))%0A%20%20Policy--%3E%3ESwift%3A%20notify(decision)%0A%20%20Swift-%3E%3EFC%3A%20ManagedSettings.shield(%5Bbundle_ids%5D)%0A%20%20Swift-%3E%3ECoachy%3A%20transition(state%20%3D%20%22locked%22)%0A%20%20Coachy-%3E%3ESwift%3A%20render(line%20%3D%20%22Instagram%20is%20locked.%20Canvas%20says%20PSYC%20101%20is%20due%20in%203h.%22)%0A"
      }, null, _parent));
    },
    fallback: () => {
      _push(` Loading... `);
    },
    _: 1
  });
  _push(`<h2 id="data-flows" tabindex="-1">Data flows <a class="header-anchor" href="#data-flows" aria-label="Permalink to &quot;Data flows&quot;">​</a></h2><h3 id="inbound-external-→-core" tabindex="-1">Inbound (external → core) <a class="header-anchor" href="#inbound-external-→-core" aria-label="Permalink to &quot;Inbound (external → core)&quot;">​</a></h3><ol><li>Connector runtime polls (or receives webhook, where supported).</li><li>Each raw payload is normalized into one or more <code>Event</code> values.</li><li>Events are appended to <code>focus-events</code> with a monotonically increasing cursor.</li><li>Rules engine subscribes to the event stream; each new event may match one or more rules.</li></ol><h3 id="outbound-core-→-platform" tabindex="-1">Outbound (core → platform) <a class="header-anchor" href="#outbound-core-→-platform" aria-label="Permalink to &quot;Outbound (core → platform)&quot;">​</a></h3><ol><li>Rule match → <code>Policy</code> produces a <code>Decision</code> (lock-apps, unlock-apps, reward, penalty, notify).</li><li><code>Decision</code> appends an <code>AuditRecord</code> before any side effect.</li><li>The platform adapter (Swift) observes <code>Decision</code> via a UniFFI callback and actuates: <code>ManagedSettings.store.shield.applications = ...</code>.</li><li><code>DeviceActivityMonitor</code> callbacks in Swift relay usage attempts back as <code>Event</code>s (for penalty escalation).</li></ol><h3 id="audit-chain-verification" tabindex="-1">Audit chain verification <a class="header-anchor" href="#audit-chain-verification" aria-label="Permalink to &quot;Audit chain verification&quot;">​</a></h3><p>At launch, the iOS app calls into <code>focus-audit::verify_chain_from_genesis</code>. The function walks every <code>AuditRecord</code> in order and recomputes <code>sha256(prev_hash || record_bytes)</code>. On mismatch it returns <code>ChainBroken { at_index, expected, actual }</code> and the app refuses to operate until the user resolves the break (restore from backup, or accept reset with evidence).</p><h2 id="threading-model" tabindex="-1">Threading model <a class="header-anchor" href="#threading-model" aria-label="Permalink to &quot;Threading model&quot;">​</a></h2><ul><li>The core exposes a single <code>Core</code> handle. All public methods are <code>Send + Sync</code>.</li><li>Under the hood the core uses a single-threaded runtime (<code>tokio::runtime::Builder::new_current_thread</code>) for the connector poll loop and a lock-free <code>crossbeam</code> channel for event delivery.</li><li>Swift calls UniFFI-bound methods from the main thread; long-running operations (connector sync, full-chain verify) are dispatched to <code>DispatchQueue.global(qos: .utility)</code> by the Swift adapter.</li></ul></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("architecture/system-diagram.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const systemDiagram = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  systemDiagram as default
};
