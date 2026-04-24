import { ssrRenderAttrs, ssrRenderStyle } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Evening Shutdown Ritual","description":"Reflect on your day, plan tomorrow, and wind down intentionally.","frontmatter":{"title":"Evening Shutdown Ritual","description":"Reflect on your day, plan tomorrow, and wind down intentionally."},"headers":[],"relativePath":"rituals/evening-shutdown.md","filePath":"rituals/evening-shutdown.md","lastUpdated":1777002546000}');
const _sfc_main = { name: "rituals/evening-shutdown.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="evening-shutdown-ritual" tabindex="-1">Evening Shutdown Ritual <a class="header-anchor" href="#evening-shutdown-ritual" aria-label="Permalink to &quot;Evening Shutdown Ritual&quot;">​</a></h1><p>The <strong>Evening Shutdown</strong> is a 5–10 minute reflection ceremony before bed.</p><h2 id="what-you-answer" tabindex="-1">What You Answer <a class="header-anchor" href="#what-you-answer" aria-label="Permalink to &quot;What You Answer&quot;">​</a></h2><h3 id="_1-time-audit" tabindex="-1">1. Time Audit <a class="header-anchor" href="#_1-time-audit" aria-label="Permalink to &quot;1. Time Audit&quot;">​</a></h3><div class="language- vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang"></span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>📊 How you spent your time today:</span></span>
<span class="line"><span>   </span></span>
<span class="line"><span>   Focus time: 3h 45m (goal: 4h) ✓</span></span>
<span class="line"><span>   Distraction time: 1h 30m (goal: &lt;2h) ✓</span></span>
<span class="line"><span>   Deep work: 2h 15m (goal: 2h) ✓</span></span>
<span class="line"><span>   Breaks: 45m (goal: 45m) ✓</span></span></code></pre></div><p>Shows your actual screen-time breakdown vs. your goals.</p><h3 id="_2-assignment-progress" tabindex="-1">2. Assignment Progress <a class="header-anchor" href="#_2-assignment-progress" aria-label="Permalink to &quot;2. Assignment Progress&quot;">​</a></h3><div class="language- vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang"></span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>📚 Canvas assignments:</span></span>
<span class="line"><span>   ✓ Submitted: 3</span></span>
<span class="line"><span>   ⏳ In progress: 2</span></span>
<span class="line"><span>   ⚠️ Overdue: 0</span></span></code></pre></div><p>If Canvas is connected, shows submission status.</p><h3 id="_3-reflection-questions" tabindex="-1">3. Reflection Questions <a class="header-anchor" href="#_3-reflection-questions" aria-label="Permalink to &quot;3. Reflection Questions&quot;">​</a></h3><div class="language- vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang"></span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Q1: &quot;Did you hit your focus goals today?&quot;</span></span>
<span class="line"><span>    [Yes] [Mostly] [No] [N/A]</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Q2: &quot;What blocked your focus most?&quot;</span></span>
<span class="line"><span>    - Social media</span></span>
<span class="line"><span>    - Procrastination</span></span>
<span class="line"><span>    - Other tasks</span></span>
<span class="line"><span>    - Distractions</span></span>
<span class="line"><span>    (Multiple select)</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Q3: &quot;What&#39;s your priority for tomorrow?&quot;</span></span>
<span class="line"><span>    [Open text field, 30 chars]</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Q4: &quot;How&#39;s your energy level?&quot;</span></span>
<span class="line"><span>    [Very low] [Low] [Medium] [High] [Very high]</span></span></code></pre></div><h3 id="_4-coaching-reflection" tabindex="-1">4. Coaching Reflection <a class="header-anchor" href="#_4-coaching-reflection" aria-label="Permalink to &quot;4. Coaching Reflection&quot;">​</a></h3><p>Based on answers, Coachy offers:</p><div class="language- vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang"></span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>You&#39;re averaging 3h 45m focus per day (great!).</span></span>
<span class="line"><span>Procrastination blocked you 3 times this week.</span></span>
<span class="line"><span>Consider:</span></span>
<span class="line"><span>  • Smaller task chunks</span></span>
<span class="line"><span>  • Pomodoro timer</span></span>
<span class="line"><span>  • Pre-work ritual (e.g., coffee + music)</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Tomorrow&#39;s focus goal: {{user_input}}</span></span></code></pre></div><h2 id="ritual-flow" tabindex="-1">Ritual Flow <a class="header-anchor" href="#ritual-flow" aria-label="Permalink to &quot;Ritual Flow&quot;">​</a></h2><ol><li><strong>Prompt</strong>: Evening Shutdown notification (default 10 PM)</li><li><strong>Audit</strong>: Review your time breakdown (1 min)</li><li><strong>Reflect</strong>: Answer 3–4 questions (3–4 min)</li><li><strong>Plan</strong>: Set tomorrow&#39;s intention (1 min)</li><li><strong>Lockdown</strong>: Optional: Enable evening focus mode (blocks social media, dimms screen)</li><li><strong>Log</strong>: All responses saved to audit chain</li></ol><h2 id="customization" tabindex="-1">Customization <a class="header-anchor" href="#customization" aria-label="Permalink to &quot;Customization&quot;">​</a></h2><h3 id="timing" tabindex="-1">Timing <a class="header-anchor" href="#timing" aria-label="Permalink to &quot;Timing&quot;">​</a></h3><ul><li><strong>Default</strong>: 10 PM (configurable)</li><li><strong>Flexible</strong>: Appear in notification; user taps when ready</li><li><strong>Strict</strong>: Mandatory before logging out (can override)</li></ul><h3 id="questions" tabindex="-1">Questions <a class="header-anchor" href="#questions" aria-label="Permalink to &quot;Questions&quot;">​</a></h3><p>Choose which questions to include:</p><div class="language- vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang"></span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>[ ] Time audit</span></span>
<span class="line"><span>[✓] Canvas progress</span></span>
<span class="line"><span>[✓] Focus reflection</span></span>
<span class="line"><span>[✓] Tomorrow&#39;s priority</span></span>
<span class="line"><span>[✓] Energy level</span></span>
<span class="line"><span>[ ] Sleep forecast</span></span></code></pre></div><h3 id="actions-after-shutdown" tabindex="-1">Actions After Shutdown <a class="header-anchor" href="#actions-after-shutdown" aria-label="Permalink to &quot;Actions After Shutdown&quot;">​</a></h3><p>Optionally trigger:</p><ul><li>Evening lockdown (block social apps)</li><li>Do-not-disturb until morning</li><li>Scheduled morning brief</li><li>Export audit chain</li></ul><h2 id="example-developer-evening" tabindex="-1">Example: Developer Evening <a class="header-anchor" href="#example-developer-evening" aria-label="Permalink to &quot;Example: Developer Evening&quot;">​</a></h2><div class="language- vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang"></span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>10 PM: Evening Shutdown notification appears</span></span>
<span class="line"><span></span></span>
<span class="line"><span>📊 TIME AUDIT</span></span>
<span class="line"><span>Deep work: 4h 30m (goal: 4h) ✓ +30m bonus</span></span>
<span class="line"><span>Meetings: 1h (goal: 1.5h) ✓</span></span>
<span class="line"><span>Email/admin: 45m (goal: &lt;1h) ✓</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Q: &quot;Hit your focus goals?&quot;</span></span>
<span class="line"><span>A: [Mostly]</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Q: &quot;What blocked focus?&quot;</span></span>
<span class="line"><span>A: [Slack notifications, context switching]</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Q: &quot;Tomorrow&#39;s priority?&quot;</span></span>
<span class="line"><span>A: &quot;Deploy backend feature&quot;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Q: &quot;Energy level?&quot;</span></span>
<span class="line"><span>A: [Medium]</span></span>
<span class="line"><span></span></span>
<span class="line"><span>💭 Coaching:</span></span>
<span class="line"><span>&quot;Strong day! You avoided context switches pretty well </span></span>
<span class="line"><span>in the afternoon. Watch Slack notifications tomorrow—</span></span>
<span class="line"><span>consider &#39;Focus hours&#39; from 10–12 AM.&quot;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>[Export audit] [Start evening lockdown] [Done]</span></span></code></pre></div><h2 id="integration-with-rules" tabindex="-1">Integration with Rules <a class="header-anchor" href="#integration-with-rules" aria-label="Permalink to &quot;Integration with Rules&quot;">​</a></h2><p>Evening shutdown can trigger rules:</p><div class="language-yaml vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">yaml</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span style="${ssrRenderStyle({ "--shiki-light": "#22863A", "--shiki-dark": "#85E89D" })}">trigger</span><span style="${ssrRenderStyle({ "--shiki-light": "#24292E", "--shiki-dark": "#E1E4E8" })}">:</span></span>
<span class="line"><span style="${ssrRenderStyle({ "--shiki-light": "#24292E", "--shiki-dark": "#E1E4E8" })}">  - </span><span style="${ssrRenderStyle({ "--shiki-light": "#22863A", "--shiki-dark": "#85E89D" })}">schedule</span><span style="${ssrRenderStyle({ "--shiki-light": "#24292E", "--shiki-dark": "#E1E4E8" })}">: </span><span style="${ssrRenderStyle({ "--shiki-light": "#032F62", "--shiki-dark": "#9ECBFF" })}">&quot;every day at 22:00&quot;</span></span>
<span class="line"></span>
<span class="line"><span style="${ssrRenderStyle({ "--shiki-light": "#22863A", "--shiki-dark": "#85E89D" })}">action</span><span style="${ssrRenderStyle({ "--shiki-light": "#24292E", "--shiki-dark": "#E1E4E8" })}">:</span></span>
<span class="line"><span style="${ssrRenderStyle({ "--shiki-light": "#24292E", "--shiki-dark": "#E1E4E8" })}">  - </span><span style="${ssrRenderStyle({ "--shiki-light": "#032F62", "--shiki-dark": "#9ECBFF" })}">show_evening_shutdown</span></span>
<span class="line"><span style="${ssrRenderStyle({ "--shiki-light": "#24292E", "--shiki-dark": "#E1E4E8" })}">  - </span><span style="${ssrRenderStyle({ "--shiki-light": "#22863A", "--shiki-dark": "#85E89D" })}">log_audit</span><span style="${ssrRenderStyle({ "--shiki-light": "#24292E", "--shiki-dark": "#E1E4E8" })}">: </span><span style="${ssrRenderStyle({ "--shiki-light": "#032F62", "--shiki-dark": "#9ECBFF" })}">&quot;Evening reflection completed&quot;</span></span>
<span class="line"><span style="${ssrRenderStyle({ "--shiki-light": "#24292E", "--shiki-dark": "#E1E4E8" })}">  - </span><span style="${ssrRenderStyle({ "--shiki-light": "#22863A", "--shiki-dark": "#85E89D" })}">optional</span><span style="${ssrRenderStyle({ "--shiki-light": "#24292E", "--shiki-dark": "#E1E4E8" })}">:</span></span>
<span class="line"><span style="${ssrRenderStyle({ "--shiki-light": "#24292E", "--shiki-dark": "#E1E4E8" })}">      - </span><span style="${ssrRenderStyle({ "--shiki-light": "#22863A", "--shiki-dark": "#85E89D" })}">enable_evening_lockdown</span><span style="${ssrRenderStyle({ "--shiki-light": "#24292E", "--shiki-dark": "#E1E4E8" })}">: </span><span style="${ssrRenderStyle({ "--shiki-light": "#005CC5", "--shiki-dark": "#79B8FF" })}">true</span></span>
<span class="line"><span style="${ssrRenderStyle({ "--shiki-light": "#24292E", "--shiki-dark": "#E1E4E8" })}">      - </span><span style="${ssrRenderStyle({ "--shiki-light": "#22863A", "--shiki-dark": "#85E89D" })}">mute_notifications</span><span style="${ssrRenderStyle({ "--shiki-light": "#24292E", "--shiki-dark": "#E1E4E8" })}">: </span><span style="${ssrRenderStyle({ "--shiki-light": "#005CC5", "--shiki-dark": "#79B8FF" })}">true</span></span></code></pre></div><h2 id="data-privacy" tabindex="-1">Data Privacy <a class="header-anchor" href="#data-privacy" aria-label="Permalink to &quot;Data Privacy&quot;">​</a></h2><ul><li>Responses stay local (no cloud upload unless user exports)</li><li>Responses feed audit chain (tamper-evident)</li><li>Users can delete old shutdowns from Settings → Privacy</li><li>Export data in JSON format for personal analysis</li></ul><h2 id="ritual-evolution" tabindex="-1">Ritual Evolution <a class="header-anchor" href="#ritual-evolution" aria-label="Permalink to &quot;Ritual Evolution&quot;">​</a></h2><p>Over time, Evening Shutdown learns:</p><ul><li>Your typical time spent on focus vs. distraction</li><li>Your most common blockers</li><li>Your energy patterns (when you tend to have low energy)</li><li>Your priority themes (academic, work, wellness, etc.)</li></ul><p>Coachy uses this to personalize suggestions.</p><p>See <strong><a href="./index">Rituals Overview</a></strong> for custom ritual design.</p></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("rituals/evening-shutdown.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const eveningShutdown = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  eveningShutdown as default
};
