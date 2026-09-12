<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  let loading = true;
  let vaultExists = false;
  let vaultUnlocked = false;

  let masterPassword = "";
  let confirmPassword = "";
  let loginPassword = "";

  let showMasterPassword = false;
  let showConfirmPassword = false;
  let showLoginPassword = false;

  let message = "";
  let messageType: "success" | "error" | "" = "";
  let submitting = false;

  onMount(async () => {
    await checkVault();
  });

  async function checkVault() {
    loading = true;
    message = "";
    messageType = "";

    try {
      vaultExists = await invoke<boolean>("check_vault_exists");
    } catch (error) {
      showError(String(error));
    } finally {
      loading = false;
    }
  }

  async function createVault() {
    message = "";
    messageType = "";

    if (!masterPassword.trim()) {
      showError("Please enter a master password.");
      return;
    }

    if (masterPassword.length < 8) {
      showError("Your master password must contain at least 8 characters.");
      return;
    }

    if (!confirmPassword) {
      showError("Please confirm your master password.");
      return;
    }

    if (masterPassword !== confirmPassword) {
      showError("The master passwords do not match.");
      return;
    }

    submitting = true;

    try {
      const result = await invoke<string>("initialise_vault", {
        masterPassword,
      });

      message = result;
      messageType = "success";

      masterPassword = "";
      confirmPassword = "";

      setTimeout(() => {
        vaultExists = true;
        message = "";
        messageType = "";
      }, 1000);
    } catch (error) {
      showError(String(error));
    } finally {
      submitting = false;
    }
  }

  async function unlockVault() {
    message = "";
    messageType = "";

    if (!loginPassword) {
      showError("Please enter your master password.");
      return;
    }

    submitting = true;

    try {
      const valid = await invoke<boolean>("verify_master_password", {
        masterPassword: loginPassword,
      });

      if (valid) {
        vaultUnlocked = true;
        loginPassword = "";
        message = "";
        messageType = "";
      } else {
        showError("Incorrect master password.");
      }
    } catch (error) {
      showError(String(error));
    } finally {
      submitting = false;
    }
  }

  async function lockVault() {
    try {
      await invoke("lock_vault");

      vaultUnlocked = false;
      loginPassword = "";
      message = "";
      messageType = "";
      showLoginPassword = false;
    } catch (error) {
      showError(String(error));
    }
  }

  function showError(text: string) {
    message = text;
    messageType = "error";
  }
</script>

<svelte:head>
  <title>VaultFive</title>
</svelte:head>

<main class="app-shell">
  <section class="brand-panel">
    <div class="brand">
      <div class="logo">V</div>
      <span>VaultFive</span>
    </div>

    <div class="brand-content">
      <p class="eyebrow">SECURE DESKTOP VAULT</p>

      <h1>Your passwords.<br />Protected locally.</h1>

      <p class="brand-description">
        VaultFive keeps your credentials organised in a local desktop vault
        protected by one master password.
      </p>

      <div class="features">
        <div class="feature">
          <span class="feature-icon">✓</span>
          <span>Local desktop storage</span>
        </div>

        <div class="feature">
          <span class="feature-icon">✓</span>
          <span>Protected master password</span>
        </div>

        <div class="feature">
          <span class="feature-icon">✓</span>
          <span>Simple credential management</span>
        </div>
      </div>
    </div>

    <p class="team">Team 4 · Software Engineering</p>
  </section>

  <section class="form-panel">
    {#if loading}
      <div class="card loading-card">
        <div class="spinner"></div>
        <p>Opening VaultFive...</p>
      </div>
    {:else if !vaultExists}
      <div class="card">
        <div class="mobile-brand">
          <div class="logo small">V</div>
          <span>VaultFive</span>
        </div>

        <div class="status-badge">FIRST-TIME SETUP</div>

        <h2>Create your vault</h2>

        <p class="subtitle">
          Choose a master password to protect access to your VaultFive vault.
        </p>

        <form on:submit|preventDefault={createVault}>
          <div class="field">
            <label for="master-password">Master password</label>

            <div class="password-field">
              <input
                id="master-password"
                type={showMasterPassword ? "text" : "password"}
                bind:value={masterPassword}
                placeholder="Enter at least 8 characters"
                autocomplete="new-password"
                disabled={submitting}
              />

              <button
                class="visibility-button"
                type="button"
                on:click={() => (showMasterPassword = !showMasterPassword)}
                aria-label={showMasterPassword
                  ? "Hide master password"
                  : "Show master password"}
              >
                {showMasterPassword ? "Hide" : "Show"}
              </button>
            </div>

            <span class="hint">Minimum 8 characters</span>
          </div>

          <div class="field">
            <label for="confirm-password">Confirm master password</label>

            <div class="password-field">
              <input
                id="confirm-password"
                type={showConfirmPassword ? "text" : "password"}
                bind:value={confirmPassword}
                placeholder="Enter your master password again"
                autocomplete="new-password"
                disabled={submitting}
              />

              <button
                class="visibility-button"
                type="button"
                on:click={() => (showConfirmPassword = !showConfirmPassword)}
                aria-label={showConfirmPassword
                  ? "Hide confirmation password"
                  : "Show confirmation password"}
              >
                {showConfirmPassword ? "Hide" : "Show"}
              </button>
            </div>
          </div>

          {#if message}
            <div
              class:success={messageType === "success"}
              class:error={messageType === "error"}
              class="message"
              role="status"
            >
              {message}
            </div>
          {/if}

          <button class="primary-button" type="submit" disabled={submitting}>
            {submitting ? "Creating vault..." : "Create Vault"}
          </button>
        </form>

        <div class="security-note">
          <span class="shield">◆</span>

          <div>
            <strong>Remember your master password</strong>
            <p>
              VaultFive does not provide online password recovery. Keep your
              master password somewhere safe.
            </p>
          </div>
        </div>
      </div>
    {:else if !vaultUnlocked}
      <div class="card">
        <div class="mobile-brand">
          <div class="logo small">V</div>
          <span>VaultFive</span>
        </div>

        <div class="status-badge">VAULT LOCKED</div>

        <h2>Welcome back</h2>

        <p class="subtitle">
          Enter your master password to unlock your VaultFive vault.
        </p>

        <form on:submit|preventDefault={unlockVault}>
          <div class="field">
            <label for="login-password">Master password</label>

            <div class="password-field">
              <input
                id="login-password"
                type={showLoginPassword ? "text" : "password"}
                bind:value={loginPassword}
                placeholder="Enter your master password"
                autocomplete="current-password"
                disabled={submitting}
              />

              <button
                class="visibility-button"
                type="button"
                on:click={() => (showLoginPassword = !showLoginPassword)}
                aria-label={showLoginPassword
                  ? "Hide master password"
                  : "Show master password"}
              >
                {showLoginPassword ? "Hide" : "Show"}
              </button>
            </div>
          </div>

          {#if message}
            <div
              class:success={messageType === "success"}
              class:error={messageType === "error"}
              class="message"
              role="status"
            >
              {message}
            </div>
          {/if}

          <button class="primary-button" type="submit" disabled={submitting}>
            {submitting ? "Unlocking..." : "Unlock Vault"}
          </button>
        </form>

        <p class="local-note">Your vault is stored locally on this device.</p>
      </div>
    {:else}
      <div class="dashboard-card">
        <div class="dashboard-header">
          <div>
            <div class="status-badge">VAULT UNLOCKED</div>
            <h2>Your vault</h2>
            <p class="subtitle">
              Your VaultFive vault is unlocked and ready to use.
            </p>
          </div>

          <button class="lock-button" type="button" on:click={lockVault}>
            Lock Vault
          </button>

          
        </div>

        <div class="empty-state">
          <div class="empty-icon">V</div>

          <h3>No credentials yet</h3>

          <p>
            Your saved credentials will appear here once credential management
            is implemented.
          </p>
        </div>
      </div>
    {/if}
  </section>
</main>

<style>
  :global(*) {
    box-sizing: border-box;
  }

  :global(html) {
    background: #f4f6fb;
  }

  :global(body) {
    margin: 0;
    min-width: 320px;
    min-height: 100vh;
    font-family:
      Inter,
      ui-sans-serif,
      system-ui,
      -apple-system,
      BlinkMacSystemFont,
      "Segoe UI",
      sans-serif;
    color: #172033;
    background: #f4f6fb;
  }

  :global(button),
  :global(input) {
    font: inherit;
  }

  .app-shell {
    min-height: 100vh;
    display: grid;
    grid-template-columns: minmax(310px, 0.9fr) minmax(500px, 1.4fr);
  }

  .brand-panel {
    min-height: 100vh;
    padding: 42px 50px;
    display: flex;
    flex-direction: column;
    background: radial-gradient(
        circle at 20% 20%,
        rgba(79, 117, 255, 0.22),
        transparent 32%
      ),
      linear-gradient(145deg, #111a35 0%, #172652 55%, #1c3471 100%);
    color: white;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 13px;
    font-size: 21px;
    font-weight: 750;
    letter-spacing: -0.4px;
  }

  .logo {
    width: 42px;
    height: 42px;
    border-radius: 13px;
    display: grid;
    place-items: center;
    background: #5476ff;
    box-shadow: 0 8px 24px rgba(33, 65, 183, 0.4);
    font-size: 21px;
    font-weight: 800;
  }

  .logo.small {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    font-size: 18px;
  }

  .brand-content {
    margin: auto 0;
    max-width: 520px;
  }

  .eyebrow {
    margin: 0 0 18px;
    color: #9eb4ff;
    font-size: 12px;
    font-weight: 800;
    letter-spacing: 2px;
  }

  .brand-content h1 {
    margin: 0;
    font-size: clamp(40px, 4.2vw, 66px);
    line-height: 1.03;
    letter-spacing: -2.5px;
  }

  .brand-description {
    margin: 25px 0 32px;
    max-width: 450px;
    color: #c8d2ef;
    font-size: 16px;
    line-height: 1.7;
  }

  .features {
    display: grid;
    gap: 15px;
  }

  .feature {
    display: flex;
    align-items: center;
    gap: 12px;
    color: #e2e8fb;
    font-size: 14px;
  }

  .feature-icon {
    width: 25px;
    height: 25px;
    display: grid;
    place-items: center;
    border-radius: 50%;
    background: rgba(93, 124, 255, 0.18);
    color: #a9bcff;
    font-weight: 800;
  }

  .team {
    margin: 0;
    color: #8292bc;
    font-size: 12px;
  }

  .form-panel {
    min-height: 100vh;
    padding: 48px;
    display: grid;
    place-items: center;
    background: radial-gradient(
        circle at 80% 15%,
        rgba(92, 119, 255, 0.08),
        transparent 25%
      ),
      #f6f8fc;
  }

  .card,
  .dashboard-card {
    width: min(100%, 500px);
    padding: 44px;
    border: 1px solid #e5e9f2;
    border-radius: 24px;
    background: white;
    box-shadow: 0 22px 60px rgba(35, 48, 79, 0.09);
  }

  .dashboard-card {
    width: min(100%, 760px);
  }

  .mobile-brand {
    display: none;
    align-items: center;
    gap: 10px;
    margin-bottom: 28px;
    font-weight: 750;
  }

  .status-badge {
    display: inline-flex;
    padding: 7px 11px;
    margin-bottom: 17px;
    border-radius: 999px;
    background: #eef2ff;
    color: #4864d9;
    font-size: 10px;
    font-weight: 800;
    letter-spacing: 1px;
  }

  h2 {
    margin: 0 0 10px;
    color: #172033;
    font-size: 31px;
    letter-spacing: -1px;
  }

  .subtitle {
    margin: 0 0 31px;
    color: #6b7488;
    font-size: 14px;
    line-height: 1.6;
  }

  form {
    display: grid;
    gap: 21px;
  }

  .field {
    display: grid;
    gap: 8px;
  }

  label {
    color: #30394d;
    font-size: 13px;
    font-weight: 700;
  }

  .password-field {
    position: relative;
  }

  input {
    width: 100%;
    height: 50px;
    padding: 0 72px 0 15px;
    border: 1px solid #d9deea;
    border-radius: 11px;
    outline: none;
    background: #fbfcfe;
    color: #172033;
    transition:
      border-color 0.18s ease,
      box-shadow 0.18s ease,
      background 0.18s ease;
  }

  input::placeholder {
    color: #a1a9b8;
  }

  input:focus {
    border-color: #5574ed;
    background: white;
    box-shadow: 0 0 0 4px rgba(85, 116, 237, 0.11);
  }

  .visibility-button {
    position: absolute;
    top: 50%;
    right: 8px;
    min-width: 53px;
    padding: 7px 9px;
    transform: translateY(-50%);
    border: 0;
    border-radius: 7px;
    background: transparent;
    color: #53617a;
    cursor: pointer;
    font-size: 12px;
    font-weight: 700;
  }

  .visibility-button:hover {
    background: #edf1f8;
  }

  .visibility-button:focus-visible,
  .primary-button:focus-visible,
  .lock-button:focus-visible {
    outline: 3px solid rgba(75, 104, 221, 0.28);
    outline-offset: 2px;
  }

  .hint {
    color: #8992a4;
    font-size: 11px;
  }

  .primary-button {
    width: 100%;
    min-height: 50px;
    margin-top: 2px;
    border: 0;
    border-radius: 11px;
    background: #425fd4;
    color: white;
    cursor: pointer;
    font-size: 14px;
    font-weight: 750;
    box-shadow: 0 8px 18px rgba(66, 95, 212, 0.2);
    transition:
      transform 0.15s ease,
      background 0.15s ease;
  }

  .primary-button:hover:not(:disabled) {
    background: #3653c7;
    transform: translateY(-1px);
  }

  .primary-button:disabled {
    cursor: not-allowed;
    opacity: 0.65;
  }

  .message {
    padding: 12px 14px;
    border-radius: 9px;
    font-size: 12px;
    line-height: 1.5;
  }

  .message.error {
    border: 1px solid #f2cccc;
    background: #fff3f3;
    color: #a13737;
  }

  .message.success {
    border: 1px solid #c8e8d5;
    background: #effaf3;
    color: #287547;
  }

  .security-note {
    margin-top: 27px;
    padding: 15px;
    display: flex;
    gap: 12px;
    border: 1px solid #e2e7f1;
    border-radius: 12px;
    background: #f8f9fc;
  }

  .shield {
    color: #526edb;
    font-size: 12px;
    padding-top: 2px;
  }

  .security-note strong {
    display: block;
    margin-bottom: 4px;
    color: #3a4459;
    font-size: 11px;
  }

  .security-note p {
    margin: 0;
    color: #7a8394;
    font-size: 10px;
    line-height: 1.5;
  }

  .local-note {
    margin: 27px 0 0;
    text-align: center;
    color: #8992a4;
    font-size: 11px;
  }

  .loading-card {
    display: grid;
    justify-items: center;
    gap: 15px;
    color: #667085;
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid #e3e7ef;
    border-top-color: #4b68da;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  .dashboard-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 24px;
    margin-bottom: 34px;
  }

  .lock-button {
    min-height: 42px;
    padding: 0 18px;
    border: 1px solid #d8deeb;
    border-radius: 10px;
    background: white;
    color: #3c465b;
    cursor: pointer;
    font-size: 13px;
    font-weight: 700;
  }

  .lock-button:hover {
    background: #f5f7fb;
  }

  .empty-state {
    min-height: 330px;
    padding: 50px 25px;
    display: grid;
    place-items: center;
    align-content: center;
    text-align: center;
    border: 1px dashed #d9deea;
    border-radius: 18px;
    background: #fafbfe;
  }

  .empty-icon {
    width: 58px;
    height: 58px;
    margin-bottom: 18px;
    display: grid;
    place-items: center;
    border-radius: 16px;
    background: #edf1ff;
    color: #4b68da;
    font-size: 24px;
    font-weight: 800;
  }

  .empty-state h3 {
    margin: 0 0 9px;
    color: #283247;
    font-size: 20px;
  }

  .empty-state p {
    max-width: 390px;
    margin: 0;
    color: #7a8394;
    font-size: 13px;
    line-height: 1.6;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  @media (max-width: 850px) {
    .app-shell {
      display: block;
    }

    .brand-panel {
      display: none;
    }

    .form-panel {
      padding: 30px 20px;
    }

    .mobile-brand {
      display: flex;
    }
  }

  @media (max-width: 620px) {
    .dashboard-card {
      min-height: 100vh;
      padding: 30px 22px;
      border: 0;
      border-radius: 0;
      box-shadow: none;
    }

    .dashboard-header {
      display: grid;
    }

    .lock-button {
      width: 100%;
    }
  }

  @media (max-width: 520px) {
    .form-panel {
      padding: 0;
      background: white;
    }

    .card {
      min-height: 100vh;
      padding: 35px 24px;
      border: 0;
      border-radius: 0;
      box-shadow: none;
    }
  }
</style>
