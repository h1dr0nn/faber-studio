<script lang="ts">
  import {
    User,
    LogOut,
    Settings as SettingsIcon,
    Shield,
    CreditCard,
    ExternalLink,
  } from "lucide-svelte";
  import Button from "$lib/components/ui/Button.svelte";

  let user = {
    name: "User Name",
    email: "user@example.com",
    plan: "Pro Plan",
    avatar: null,
  };
</script>

<div class="feature-container">
  <div class="profile-section">
    <div class="avatar-large">
      <User size={40} />
    </div>
    <div class="user-info">
      <h3>{user.name}</h3>
      <p>{user.email}</p>
    </div>
    <Button variant="outline" size="sm" class="w-full">Edit Profile</Button>
  </div>

  <div class="menu-list">
    <div class="menu-item-group">
      <span class="section-label">Account</span>
      <button class="menu-row">
        <Shield size={16} />
        <span>Security</span>
      </button>
      <button class="menu-row">
        <CreditCard size={16} />
        <span>Billing</span>
        <span class="badge">Pro</span>
      </button>
    </div>

    <div class="menu-item-group">
      <span class="section-label">Links</span>
      <button class="menu-row">
        <ExternalLink size={16} />
        <span>Documentation</span>
      </button>
      <button class="menu-row">
        <ExternalLink size={16} />
        <span>Community Forum</span>
      </button>
    </div>
  </div>

  <div class="footer-actions">
    <button class="logout-btn">
      <LogOut size={16} />
      <span>Sign Out</span>
    </button>
  </div>
</div>

<style>
  .feature-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    color: var(--fg-secondary);
  }

  .profile-section {
    padding: 24px 16px;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 12px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .avatar-large {
    width: 64px;
    height: 64px;
    background-color: var(--bg-hover);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--fg-tertiary);
  }

  .user-info h3 {
    margin: 0;
    font-size: 16px;
    color: var(--fg-primary);
  }

  .user-info p {
    margin: 4px 0 0;
    font-size: 12px;
    opacity: 0.6;
  }

  :global(.w-full) {
    width: 100%;
  }

  .menu-list {
    flex: 1;
    overflow-y: auto;
    padding: 12px 0;
  }

  .menu-item-group {
    display: flex;
    flex-direction: column;
    margin-bottom: 16px;
  }

  .section-label {
    padding: 0 16px 8px;
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    opacity: 0.5;
    letter-spacing: 0.5px;
  }

  .menu-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 16px;
    background: none;
    border: none;
    color: var(--fg-secondary);
    cursor: pointer;
    font-size: 13px;
    text-align: left;
  }

  .menu-row:hover {
    background-color: var(--bg-hover);
    color: var(--fg-primary);
  }

  .badge {
    margin-left: auto;
    background-color: var(--accent-primary);
    color: #fff;
    font-size: 10px;
    padding: 1px 6px;
    border-radius: 10px;
  }

  .footer-actions {
    padding: 12px;
    border-top: 1px solid var(--border-subtle);
  }

  .logout-btn {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 12px;
    width: 100%;
    background: none;
    border: none;
    color: #c74e61;
    cursor: pointer;
    font-size: 13px;
    border-radius: 4px;
  }

  .logout-btn:hover {
    background-color: rgba(199, 78, 97, 0.1);
  }
</style>
