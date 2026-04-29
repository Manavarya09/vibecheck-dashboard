<script lang="ts">
  import { currentSession, liveUpdate, todaySummary } from "../lib/stores";
  import {
    addSessionNote,
    addSessionTag,
    getSessionNotes,
    getSessionTags,
    removeSessionTag,
    type SessionNote,
  } from "../lib/api";
  import { formatDuration, pct, relativeTime } from "../lib/utils";

  let notes = $state<SessionNote[]>([]);
  let tags = $state<string[]>([]);
  let noteDraft = $state("");
  let tagDraft = $state("");
  let savingNote = $state(false);
  let savingTag = $state(false);
  let loadedForSession = $state<number | null>(null);

  let session = $derived($currentSession);
  let update = $derived($liveUpdate);
  let totalSecs = $derived(update?.durationSecs ?? session?.durationSecs ?? 0);
  let aiSecs = $derived(update?.aiAssistedSecs ?? 0);
  let manualSecs = $derived(update?.manualCodingSecs ?? 0);
  let nonCodingSecs = $derived(update?.nonCodingSecs ?? 0);
  let totalTracked = $derived(aiSecs + manualSecs + nonCodingSecs);
  let aiShare = $derived(pct(aiSecs, totalTracked));
  let todayTotal = $derived($todaySummary?.totalSecs ?? 0);
  let workflowState = $derived(update?.workflowState ?? "settled");
  let contextSwitches = $derived(update?.contextSwitches10m ?? 0);
  let promptLoopScore = $derived(update?.promptLoopScore ?? 0);
  let aiStreakSecs = $derived(update?.aiStreakSecs ?? 0);

  let signals = $derived.by(() => {
    if (!session || session.status !== "active") {
      return [
        { label: "Idle", text: "No active coding loop right now." },
        { label: "Prep", text: "Use tags to mark session intent before starting." },
      ];
    }

    const derivedSignals = [];
    if (aiShare >= 75) {
      derivedSignals.push({
        label: "AI-heavy",
        text: "Model assistance is dominating the session. Watch for passive drift.",
      });
    }
    if (totalSecs >= 7200) {
      derivedSignals.push({
        label: "Long-run",
        text: "Two-hour threshold crossed. This is where time distortion gets sneaky.",
      });
    } else if (totalSecs >= 3600) {
      derivedSignals.push({
        label: "Extended",
        text: "You are past one hour. Add a checkpoint before the next big prompt.",
      });
    }
    if (todayTotal >= 14400) {
      derivedSignals.push({
        label: "Recovery debt",
        text: "Today is already heavy. Fresh judgment is probably getting expensive.",
      });
    }
    if (contextSwitches >= 8) {
      derivedSignals.push({
        label: "Switch storm",
        text: "You are bouncing tools fast enough to lose the plot.",
      });
    }
    if (workflowState === "agent_shepherding") {
      derivedSignals.push({
        label: "Agent shepherding",
        text: "Claude/aider/codex appears to be running while you supervise elsewhere.",
      });
    }
    if (workflowState === "prompt_edit_loop") {
      derivedSignals.push({
        label: "Prompt-edit loop",
        text: "You are cycling between AI output and code edits with rising loop intensity.",
      });
    }
    if (derivedSignals.length === 0) {
      derivedSignals.push({
        label: "Stable",
        text: "The session still looks intentional. Keep the scope narrow.",
      });
    }
    return derivedSignals;
  });

  async function loadSessionData(sessionId: number) {
    const [loadedNotes, loadedTags] = await Promise.all([
      getSessionNotes(sessionId),
      getSessionTags(sessionId),
    ]);
    notes = loadedNotes;
    tags = loadedTags;
    loadedForSession = sessionId;
  }

  $effect(() => {
    if (!session) {
      notes = [];
      tags = [];
      loadedForSession = null;
      return;
    }
    if (loadedForSession !== session.id) {
      loadSessionData(session.id);
    }
  });

  async function handleAddNote() {
    if (!session || !noteDraft.trim()) return;
    savingNote = true;
    await addSessionNote(session.id, noteDraft.trim());
    notes = await getSessionNotes(session.id);
    noteDraft = "";
    savingNote = false;
  }

  async function handleAddTag() {
    if (!session || !tagDraft.trim()) return;
    savingTag = true;
    await addSessionTag(session.id, tagDraft.trim().toLowerCase());
    tags = await getSessionTags(session.id);
    tagDraft = "";
    savingTag = false;
  }

  async function handleRemoveTag(tag: string) {
    if (!session) return;
    await removeSessionTag(session.id, tag);
    tags = await getSessionTags(session.id);
  }
</script>

<section class="journal">
  <div class="journal-header">
    <div>
      <p class="eyebrow">Session Intelligence</p>
      <h3>Notes, Tags, and Workflow Reads</h3>
    </div>
    {#if session}
      <span class="session-id">Session #{session.id}</span>
    {/if}
  </div>

  <div class="signal-list">
    {#each signals as signal}
      <div class="signal-card">
        <span class="signal-label">{signal.label}</span>
        <p>{signal.text}</p>
      </div>
    {/each}
  </div>

  <div class="metric-row">
    <span>Workflow: <strong>{workflowState.replaceAll("_", " ")}</strong></span>
    <span>Prompt loop score: <strong>{promptLoopScore}</strong></span>
    <span>Context switches / 10m: <strong>{contextSwitches}</strong></span>
    <span>AI streak: <strong>{formatDuration(aiStreakSecs)}</strong></span>
  </div>

  {#if session}
    <div class="grid">
      <div class="panel">
        <div class="panel-header">
          <span class="panel-title">Tags</span>
          <span class="panel-meta">{tags.length} attached</span>
        </div>
        <div class="tag-form">
          <input
            class="input"
            bind:value={tagDraft}
            maxlength="24"
            placeholder="deep-work, claude-loop, debugging"
            onkeydown={(e) => e.key === "Enter" && handleAddTag()}
          />
          <button class="button" onclick={handleAddTag} disabled={savingTag}>Add</button>
        </div>
        <div class="tag-list">
          {#if tags.length > 0}
            {#each tags as tag}
              <button class="tag-chip" onclick={() => handleRemoveTag(tag)}>
                #{tag} <span aria-hidden="true">x</span>
              </button>
            {/each}
          {:else}
            <p class="empty">No tags yet. Add one to mark the session mood or task.</p>
          {/if}
        </div>
      </div>

      <div class="panel">
        <div class="panel-header">
          <span class="panel-title">Research Notes</span>
          <span class="panel-meta">{notes.length} captured</span>
        </div>
        <div class="note-form">
          <textarea
            class="textarea"
            bind:value={noteDraft}
            rows="4"
            placeholder="What happened in this loop? What was useful, weird, or too easy to keep extending?"
          ></textarea>
          <button class="button" onclick={handleAddNote} disabled={savingNote}>Save Note</button>
        </div>
        <div class="note-list">
          {#if notes.length > 0}
            {#each notes as note}
              <article class="note-card">
                <p>{note.note}</p>
                <span>{relativeTime(note.createdAt)}</span>
              </article>
            {/each}
          {:else}
            <p class="empty">No notes yet. This is useful for your AIFL paper and your own debriefs.</p>
          {/if}
        </div>
      </div>
    </div>
  {:else}
    <p class="empty wide">Start a session to capture notes and tags for workflow analysis.</p>
  {/if}
</section>

<style>
  .journal {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 22px 24px;
    box-shadow: var(--shadow-sm);
    backdrop-filter: blur(10px);
  }
  .journal-header {
    display: flex;
    justify-content: space-between;
    gap: 16px;
    align-items: start;
    margin-bottom: 16px;
  }
  .eyebrow {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.16em;
    color: var(--text-tertiary);
    font-weight: 800;
    margin-bottom: 4px;
  }
  h3 {
    font-size: 22px;
    font-weight: 800;
    color: var(--ink);
    letter-spacing: -0.03em;
  }
  .session-id {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-tertiary);
    white-space: nowrap;
  }
  .signal-list {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 10px;
    margin-bottom: 16px;
  }
  .signal-card {
    padding: 12px 14px;
    border-radius: 18px;
    background: rgba(40, 31, 23, 0.05);
  }
  .signal-label {
    display: inline-block;
    margin-bottom: 6px;
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--primary);
    font-weight: 800;
  }
  .signal-card p {
    color: var(--text-secondary);
    font-size: 13px;
    line-height: 1.45;
  }
  .metric-row {
    display: flex;
    flex-wrap: wrap;
    gap: 14px;
    margin-bottom: 14px;
    font-size: 12px;
    color: var(--text-secondary);
  }
  .metric-row strong {
    color: var(--text);
    text-transform: capitalize;
  }
  .grid {
    display: grid;
    grid-template-columns: 0.95fr 1.25fr;
    gap: 14px;
  }
  .panel {
    padding: 16px;
    border-radius: 20px;
    background: rgba(255, 255, 255, 0.68);
    border: 1px solid rgba(116, 78, 55, 0.1);
  }
  .panel-header {
    display: flex;
    justify-content: space-between;
    gap: 12px;
    margin-bottom: 12px;
  }
  .panel-title {
    font-size: 14px;
    font-weight: 800;
    color: var(--ink);
  }
  .panel-meta {
    font-size: 11px;
    color: var(--text-tertiary);
    font-family: var(--font-mono);
  }
  .tag-form,
  .note-form {
    display: flex;
    gap: 8px;
    margin-bottom: 12px;
  }
  .note-form {
    flex-direction: column;
  }
  .input,
  .textarea {
    width: 100%;
    border: 1px solid var(--border);
    background: rgba(255, 255, 255, 0.82);
    border-radius: 14px;
    padding: 10px 12px;
    color: var(--text);
    font: inherit;
  }
  .textarea {
    resize: vertical;
    min-height: 100px;
  }
  .button {
    align-self: flex-start;
    padding: 10px 14px;
    border-radius: 14px;
    background: linear-gradient(135deg, var(--primary), #e69264);
    color: white;
    font-size: 12px;
    font-weight: 700;
  }
  .tag-list {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }
  .tag-chip {
    padding: 8px 10px;
    border-radius: 999px;
    background: rgba(219, 118, 85, 0.12);
    color: var(--text);
    font-size: 12px;
    font-weight: 700;
  }
  .note-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
    max-height: 240px;
    overflow-y: auto;
  }
  .note-card {
    padding: 12px 14px;
    border-radius: 16px;
    background: rgba(40, 31, 23, 0.05);
  }
  .note-card p {
    font-size: 13px;
    color: var(--text);
    line-height: 1.5;
    margin-bottom: 8px;
  }
  .note-card span {
    font-size: 11px;
    color: var(--text-tertiary);
    font-family: var(--font-mono);
  }
  .empty {
    font-size: 12px;
    color: var(--text-tertiary);
  }
  .empty.wide {
    padding-top: 6px;
  }
  @media (max-width: 1040px) {
    .signal-list,
    .grid {
      grid-template-columns: 1fr;
    }
  }
</style>
