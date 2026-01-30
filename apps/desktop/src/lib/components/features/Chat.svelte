<script lang="ts">
  import {
    MessageSquare,
    Send,
    Trash2,
    Bot,
    User,
    Loader2,
    History,
    Zap,
    ZapOff,
  } from "lucide-svelte";
  import { uiState } from "$lib/ui-state.svelte";
  import { tick } from "svelte";
  import { marked } from "marked";

  // Configure marked for security and typical IDE needs
  marked.setOptions({
    gfm: true,
    breaks: true,
  });

  let inputMessage = $state("");
  let chatViewport: HTMLElement;

  async function handleSend() {
    if (!inputMessage.trim() || uiState.isSendingChat) return;
    const msg = inputMessage;
    inputMessage = "";
    await uiState.sendChatMessage(msg);
    scrollToBottom();
  }

  function scrollToBottom() {
    if (chatViewport) {
      setTimeout(() => {
        chatViewport.scrollTo({
          top: chatViewport.scrollHeight,
          behavior: "smooth",
        });
      }, 50);
    }
  }

  $effect(() => {
    if (uiState.chatMessages.length > 0) {
      scrollToBottom();
    }
  });
</script>

<div class="chat-container">
  <div class="chat-header">
    <div class="title">
      <MessageSquare size={16} />
      <span>AI CHAT</span>
    </div>
    <div class="actions">
      <button
        class="toggle-btn {uiState.isContinuousChat ? 'active' : ''}"
        onclick={() => (uiState.isContinuousChat = !uiState.isContinuousChat)}
        title={uiState.isContinuousChat
          ? "Continuous Chat (History enabled)"
          : "Single Message Mode"}
      >
        {#if uiState.isContinuousChat}
          <Zap size={14} />
        {:else}
          <ZapOff size={14} />
        {/if}
      </button>
      <button
        class="icon-btn"
        onclick={() => uiState.clearChat()}
        title="Clear Chat"
      >
        <Trash2 size={14} />
      </button>
    </div>
  </div>

  <div class="chat-messages" bind:this={chatViewport}>
    {#if uiState.chatMessages.length === 0}
      <div class="empty-state">
        <Bot size={48} />
        <p>Chat with AI assistant...</p>
      </div>
    {/if}

    {#each uiState.chatMessages as msg}
      <div class="message {msg.role}">
        <div class="avatar">
          {#if msg.role === "assistant"}
            <Bot size={14} />
          {:else}
            <User size={14} />
          {/if}
        </div>
        <div class="bubble">
          <div class="role-label">
            {msg.role === "assistant" ? "Assistant" : "You"}
          </div>
          <div class="content markdown-body">
            {@html marked.parse(msg.content)}
          </div>
        </div>
      </div>
    {/each}

    {#if uiState.isSendingChat}
      <div class="message assistant typing">
        <div class="avatar"><Bot size={14} /></div>
        <div class="bubble">
          <Loader2 size={14} class="spin" />
        </div>
      </div>
    {/if}
  </div>

  <div class="chat-input-area">
    <div class="input-wrapper">
      <textarea
        placeholder="Ask anything... (Shift+Enter for new line)"
        bind:value={inputMessage}
        onkeydown={(e) => {
          if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault();
            handleSend();
          }
        }}
      ></textarea>
      <button
        class="send-btn"
        onclick={handleSend}
        disabled={!inputMessage.trim() || uiState.isSendingChat}
      >
        <Send size={16} />
      </button>
    </div>
    <div class="input-footer">
      <span
        >{uiState.isContinuousChat
          ? "Continuous Context"
          : "Single Command"}</span
      >
    </div>
  </div>
</div>

<style>
  .chat-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-sidebar);
  }

  .chat-header {
    height: 35px;
    padding: 0 12px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid var(--border-subtle);
    background-color: var(--bg-tab-inactive);
  }

  .chat-header .title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11px;
    font-weight: 700;
    color: var(--fg-secondary);
  }

  .chat-header .actions {
    display: flex;
    gap: 4px;
  }

  .icon-btn,
  .toggle-btn {
    background: transparent;
    border: none;
    color: var(--fg-tertiary);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .icon-btn:hover,
  .toggle-btn:hover {
    background-color: var(--bg-hover);
    color: var(--fg-primary);
  }

  .toggle-btn.active {
    color: var(--accent-primary);
  }

  .chat-messages {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    user-select: text !important;
    -webkit-user-select: text !important;
  }

  .empty-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    opacity: 0.5;
    text-align: center;
  }

  .empty-state p {
    margin-top: 12px;
    font-size: 14px;
  }

  .message {
    display: flex;
    gap: 12px;
    max-width: 90%;
  }

  .message.user {
    align-self: flex-end;
    flex-direction: row-reverse;
  }

  .avatar {
    width: 24px;
    height: 24px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--bg-active);
    color: var(--fg-secondary);
    flex-shrink: 0;
  }

  .message.assistant .avatar {
    background-color: var(--accent-primary);
    color: white;
  }

  .bubble {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .message.user .bubble {
    align-items: flex-end;
  }

  .role-label {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    opacity: 0.5;
  }

  .content {
    background-color: var(--bg-active);
    padding: 8px 12px;
    border-radius: 8px;
    font-size: 13px;
    line-height: 1.5;
    word-break: break-word;
    color: var(--fg-primary);
  }

  .message.assistant .content {
    background-color: var(--bg-panel);
    border: 1px solid var(--border-subtle);
  }

  /* Markdown Styles */
  .markdown-body :global(p) {
    margin: 8px 0;
  }
  .markdown-body :global(p:first-child) {
    margin-top: 0;
  }
  .markdown-body :global(p:last-child) {
    margin-bottom: 0;
  }

  .markdown-body :global(h1),
  .markdown-body :global(h2),
  .markdown-body :global(h3),
  .markdown-body :global(h4) {
    margin: 16px 0 8px 0;
    font-weight: 600;
    line-height: 1.25;
    color: var(--fg-primary);
  }

  .markdown-body :global(h1) {
    font-size: 1.4em;
  }
  .markdown-body :global(h2) {
    font-size: 1.2em;
  }
  .markdown-body :global(h3) {
    font-size: 1.1em;
  }

  .markdown-body :global(ul),
  .markdown-body :global(ol) {
    padding-left: 20px;
    margin: 8px 0;
  }

  .markdown-body :global(li) {
    margin: 4px 0;
  }

  .markdown-body :global(code) {
    font-family: var(--font-mono, monospace);
    background-color: var(--bg-active);
    padding: 0.2em 0.4em;
    border-radius: 3px;
    font-size: 0.9em;
  }

  .markdown-body :global(pre) {
    background-color: var(--bg-app);
    padding: 12px;
    border-radius: 6px;
    overflow-x: auto;
    margin: 12px 0;
    border: 1px solid var(--border-subtle);
  }

  .markdown-body :global(pre code) {
    background-color: transparent;
    padding: 0;
    display: block;
    font-size: 12px;
    line-height: 1.4;
  }

  .markdown-body :global(blockquote) {
    margin: 8px 0;
    padding-left: 12px;
    border-left: 3px solid var(--border-subtle);
    color: var(--fg-secondary);
  }

  .markdown-body :global(a) {
    color: var(--accent-primary);
    text-decoration: none;
  }

  .markdown-body :global(a:hover) {
    text-decoration: underline;
  }

  .markdown-body :global(table) {
    border-collapse: collapse;
    width: 100%;
    margin: 8px 0;
  }

  .markdown-body :global(th),
  .markdown-body :global(td) {
    border: 1px solid var(--border-subtle);
    padding: 6px 12px;
    text-align: left;
  }

  .markdown-body :global(th) {
    background-color: var(--bg-active);
  }

  .chat-input-area {
    padding: 12px;
    border-top: 1px solid var(--border-subtle);
    background-color: var(--bg-sidebar);
  }

  .input-wrapper {
    position: relative;
    display: flex;
    align-items: flex-end;
    background-color: var(--bg-input);
    border: 1px solid var(--border-subtle);
    border-radius: 6px;
    padding: 8px;
    gap: 8px;
  }

  .input-wrapper:focus-within {
    border-color: var(--accent-primary);
  }

  textarea {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--fg-primary);
    padding: 0;
    font-family: inherit;
    font-size: 13px;
    resize: none;
    max-height: 150px;
    min-height: 20px;
    outline: none;
    line-height: 1.5;
  }

  .send-btn {
    background: var(--accent-primary);
    color: white;
    border: none;
    border-radius: 4px;
    width: 26px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: transform 0.1s;
  }

  .send-btn:hover {
    transform: scale(1.05);
  }

  .send-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .input-footer {
    display: flex;
    justify-content: center;
    margin-top: 6px;
  }

  .input-footer span {
    font-size: 10px;
    text-transform: uppercase;
    font-weight: 700;
    opacity: 0.4;
    letter-spacing: 0.5px;
  }

  :global(.spin) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
