<script lang="ts">
  import { autoPaused, currentSession, liveUpdate, todaySummary } from "../lib/stores";
  import { formatDuration, pct } from "../lib/utils";

  let update = $derived($liveUpdate);
  let status = $derived($currentSession?.status ?? "idle");

  let sessionSecs = $derived(update?.durationSecs ?? $currentSession?.durationSecs ?? 0);
  let aiSecs = $derived(update?.aiAssistedSecs ?? 0);
  let manualSecs = $derived(update?.manualCodingSecs ?? 0);
  let otherSecs = $derived(update?.nonCodingSecs ?? 0);
  let sessionTotal = $derived(aiSecs + manualSecs + otherSecs);
  let aiShare = $derived(pct(aiSecs, sessionTotal));
  let manualShare = $derived(pct(manualSecs, sessionTotal));
  let nonCodingShare = $derived(pct(otherSecs, sessionTotal));
  let todayTotal = $derived($todaySummary?.totalSecs ?? 0);
  let uninterruptedRisk = $derived(Math.min(100, Math.round(sessionSecs / 120)));
  let escalationRisk = $derived(
    Math.min(
      100,
      Math.round((update?.promptLoopScore ?? 0) * 0.65 + aiShare * 0.25 + sessionSecs / 240)
    )
  );
  let recoveryDebt = $derived(Math.min(100, Math.round(todayTotal / 240 + ($autoPaused ? 10 : 0))));
  let workflowState = $derived(update?.workflowState ?? "settled");
  let contextSwitches = $derived(update?.contextSwitches10m ?? 0);
  let aiStreak = $derived(update?.aiStreakSecs ?? 0);

  let riskBand = $derived.by(() => {
    if (status !== "active") return "calm";
    if (sessionSecs >= 7200 || aiShare >= 80 || workflowState === "context_switch_storm") return "critical";
    if (sessionSecs >= 3600 || aiShare >= 65 || (update?.promptLoopScore ?? 0) >= 55) return "elevated";
    return "steady";
  });

  let summary = $derived(
    {
      calm: "No active flow loop right now. Panko is just collecting baseline behavior.",
      steady: "The session is healthy so far. Friction still exists, which is good for judgment.",
      elevated: "AIFL signals are rising. Instant iteration is starting to outrun natural stopping cues.",
      critical: "The loop is self-reinforcing now. You are likely coding past your original intention boundary.",
    }[riskBand]
  );

  let recommendation = $derived(
    {
      calm: "Start with one narrow target so the session has a natural finish line.",
      steady: "Define the next checkpoint before asking the model for another major jump.",
      elevated: "Pause for 5 minutes, stand up, and decide whether the next prompt is actually necessary.",
      critical: "Hard stop. Step away from the keyboard before continuing this session.",
    }[riskBand]
  );
</script>

<section class="insights">
  <div class="header">
    <div>
      <p class="eyebrow">Research Monitor</p>
      <h3>AIFL Watch</h3>
    </div>
    <span class="risk-pill {riskBand}">
      {riskBand === "calm" ? "Baseline" : riskBand === "steady" ? "Steady" : riskBand === "elevated" ? "Elevated" : "Critical"}
    </span>
  </div>

  <p class="summary">{summary}</p>

  <div class="signal-grid">
    <div class="signal-card">
      <span class="label">Uninterrupted Session</span>
      <strong>{formatDuration(sessionSecs)}</strong>
      <span class="hint">{uninterruptedRisk}/100 time-distortion risk</span>
    </div>
    <div class="signal-card">
      <span class="label">AI Reliance</span>
      <strong>{aiShare}%</strong>
      <span class="hint">{manualShare}% manual, {nonCodingShare}% non-coding</span>
    </div>
    <div class="signal-card">
      <span class="label">Escalation Load</span>
      <strong>{escalationRisk}</strong>
      <span class="hint">{workflowState.replaceAll("_", " ")} pattern detected</span>
    </div>
    <div class="signal-card">
      <span class="label">Recovery Debt</span>
      <strong>{recoveryDebt}</strong>
      <span class="hint">{formatDuration(todayTotal)} tracked today</span>
    </div>
  </div>

  <div class="research-strip">
    <div class="research-note">
      <span class="note-label">Paper Lens</span>
      <p>
        The app should treat vibe coding as a possible flow loop, not just a timer. Long uninterrupted
        AI-assisted sessions reduce natural friction and make stopping cues easier to ignore.
      </p>
    </div>
    <div class="research-note action">
      <span class="note-label">Recommended Nudge</span>
      <p>{recommendation}</p>
    </div>
  </div>

  <div class="workflow-footer">
    <span>Workflow state: <strong>{workflowState.replaceAll("_", " ")}</strong></span>
    <span>Context switches / 10m: <strong>{contextSwitches}</strong></span>
    <span>AI streak: <strong>{formatDuration(aiStreak)}</strong></span>
  </div>
</section>

<style>
  .insights {
    position: relative;
    overflow: hidden;
    background:
      radial-gradient(circle at top right, rgba(217, 119, 87, 0.16), transparent 34%),
      linear-gradient(145deg, rgba(253, 248, 239, 0.96), rgba(242, 233, 221, 0.94));
    border: 1px solid rgba(153, 101, 73, 0.18);
    border-radius: 28px;
    padding: 24px;
    box-shadow: var(--shadow-lg);
  }
  .header {
    display: flex;
    justify-content: space-between;
    gap: 16px;
    align-items: start;
    margin-bottom: 12px;
  }
  .eyebrow {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.16em;
    color: var(--text-tertiary);
    margin-bottom: 6px;
    font-weight: 700;
  }
  h3 {
    font-size: 28px;
    line-height: 1;
    color: var(--ink);
    letter-spacing: -0.03em;
  }
  .risk-pill {
    border-radius: 999px;
    padding: 8px 12px;
    font-size: 11px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    background: rgba(43, 32, 24, 0.06);
  }
  .risk-pill.steady {
    color: var(--success);
  }
  .risk-pill.elevated {
    color: var(--warning-ink);
    background: rgba(212, 148, 58, 0.14);
  }
  .risk-pill.critical {
    color: var(--danger);
    background: rgba(196, 85, 77, 0.14);
  }
  .summary {
    max-width: 64ch;
    color: var(--text-secondary);
    margin-bottom: 18px;
    font-size: 14px;
  }
  .signal-grid {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 10px;
    margin-bottom: 14px;
  }
  .signal-card {
    padding: 14px;
    border-radius: 18px;
    background: rgba(255, 255, 255, 0.72);
    border: 1px solid rgba(153, 101, 73, 0.12);
  }
  .label {
    display: block;
    margin-bottom: 6px;
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--text-tertiary);
    font-weight: 700;
  }
  .signal-card strong {
    display: block;
    font-size: 24px;
    color: var(--ink);
    letter-spacing: -0.03em;
    margin-bottom: 4px;
  }
  .hint {
    font-size: 12px;
    color: var(--text-secondary);
    line-height: 1.4;
  }
  .research-strip {
    display: grid;
    grid-template-columns: 1.2fr 0.9fr;
    gap: 10px;
  }
  .research-note {
    padding: 14px 16px;
    border-radius: 18px;
    background: rgba(40, 31, 23, 0.06);
  }
  .research-note.action {
    background: rgba(217, 119, 87, 0.12);
  }
  .note-label {
    display: block;
    font-size: 10px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: var(--text-tertiary);
    margin-bottom: 6px;
  }
  .research-note p {
    color: var(--text);
    font-size: 13px;
    line-height: 1.5;
  }
  .workflow-footer {
    display: flex;
    flex-wrap: wrap;
    gap: 14px;
    margin-top: 12px;
    font-size: 12px;
    color: var(--text-secondary);
  }
  .workflow-footer strong {
    color: var(--text);
    text-transform: capitalize;
  }
  @media (max-width: 1024px) {
    .signal-grid,
    .research-strip {
      grid-template-columns: 1fr 1fr;
    }
  }
</style>
