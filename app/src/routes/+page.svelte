<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  type Credential = {
    id: number;
    service: string;
    username: string;
    password: string;
    created_at: string;
    updated_at: string;
  };

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

  let credentials: Credential[] = [];
  let searchQuery = "";

  let showCredentialForm = false;
  let editingId: number | null = null;

  let service = "";
  let username = "";
  let password = "";
  let showCredentialPassword = false;

  let revealedPasswords: Record<number, boolean> = {};

  let credentialLoading = false;
  let savingCredential = false;

  $: filteredCredentials = credentials.filter((credential) => {
    const query = searchQuery.trim().toLowerCase();

    if (!query) {
      return true;
    }

    return (
      credential.service.toLowerCase().includes(query) ||
      credential.username.toLowerCase().includes(query)
    );
  });

  $: credentialCountText =
    credentials.length === 1
      ? "1 credential"
      : `${credentials.length} credentials`;

  onMount(async () => {
    await checkVault();
  });

  function showSuccess(text: string) {
    message = text;
    messageType = "success";
  }

  function showError(error: unknown) {
    message = String(error);
    messageType = "error";
  }

  function clearMessage() {
    message = "";
    messageType = "";
  }

  async function checkVault() {
    loading = true;
    clearMessage();

    try {
      vaultExists = await invoke<boolean>("check_vault_exists");
    } catch (error) {
      showError(error);
    } finally {
      loading = false;
    }
  }

  async function createVault() {
    clearMessage();

    if (!masterPassword) {
      showError("Master password is required.");
      return;
    }

    if (masterPassword.length < 8) {
      showError("Master password must contain at least 8 characters.");
      return;
    }

    if (masterPassword !== confirmPassword) {
      showError("The master passwords do not match.");
      return;
    }

    loading = true;

    try {
      await invoke<string>("initialise_vault", {
        masterPassword
      });

      vaultExists = true;
      masterPassword = "";
      confirmPassword = "";
      showMasterPassword = false;
      showConfirmPassword = false;

      showSuccess(
        "Vault created successfully. Unlock it with your master password."
      );
    } catch (error) {
      showError(error);
    } finally {
      loading = false;
    }
  }

  async function unlockVault() {
    clearMessage();

    if (!loginPassword) {
      showError("Enter your master password.");
      return;
    }

    loading = true;

    try {
      const valid = await invoke<boolean>("verify_master_password", {
        masterPassword: loginPassword
      });

      if (!valid) {
        showError("Incorrect master password.");
        return;
      }

      vaultUnlocked = true;
      loginPassword = "";
      showLoginPassword = false;

      await loadCredentials();

      showSuccess("Vault unlocked successfully.");
    } catch (error) {
      vaultUnlocked = false;
      showError(error);
    } finally {
      loading = false;
    }
  }

  async function lockVault() {
    try {
      await invoke("lock_vault");
    } catch (error) {
      showError(error);
      return;
    }

    vaultUnlocked = false;

    credentials = [];
    searchQuery = "";
    loginPassword = "";

    revealedPasswords = {};

    resetCredentialForm();

    clearMessage();
  }

  async function loadCredentials() {
    credentialLoading = true;

    try {
      credentials = await invoke<Credential[]>("get_credentials");
    } catch (error) {
      credentials = [];
      showError(error);
    } finally {
      credentialLoading = false;
    }
  }

  function openAddCredential() {
    editingId = null;
    service = "";
    username = "";
    password = "";
    showCredentialPassword = false;
    showCredentialForm = true;
    clearMessage();
  }

  function openEditCredential(credential: Credential) {
    editingId = credential.id;
    service = credential.service;
    username = credential.username;
    password = credential.password;
    showCredentialPassword = false;
    showCredentialForm = true;
    clearMessage();
  }

  function resetCredentialForm() {
    showCredentialForm = false;
    editingId = null;

    service = "";
    username = "";
    password = "";

    showCredentialPassword = false;
  }

  async function saveCredential() {
    clearMessage();

    if (!service.trim()) {
      showError("Service name is required.");
      return;
    }

    if (!username.trim()) {
      showError("Username or email is required.");
      return;
    }

    if (!password) {
      showError("Password is required.");
      return;
    }

    savingCredential = true;

    try {
      if (editingId !== null) {
        await invoke("update_credential", {
          id: editingId,
          service,
          username,
          password
        });

        showSuccess("Credential updated successfully.");
      } else {
        await invoke<number>("add_credential", {
          service,
          username,
          password
        });

        showSuccess("Credential added successfully.");
      }

      resetCredentialForm();
      await loadCredentials();
    } catch (error) {
      showError(error);
    } finally {
      savingCredential = false;
    }
  }

  async function deleteCredential(credential: Credential) {
    const confirmed = window.confirm(
      `Delete the credential for ${credential.service}?`
    );

    if (!confirmed) {
      return;
    }

    clearMessage();

    try {
      await invoke("delete_credential", {
        id: credential.id
      });

      revealedPasswords[credential.id] = false;
      revealedPasswords = { ...revealedPasswords };

      await loadCredentials();

      showSuccess("Credential deleted successfully.");
    } catch (error) {
      showError(error);
    }
  }

  function togglePasswordVisibility(id: number) {
    revealedPasswords[id] = !revealedPasswords[id];

    revealedPasswords = {
      ...revealedPasswords
    };
  }

  function maskPassword(password: string) {
    return "•".repeat(Math.max(8, Math.min(password.length, 16)));
  }
</script>

<svelte:head>
  <title>VaultFive Password Manager</title>
  <meta
    name="description"
    content="VaultFive secure local password manager"
  />
</svelte:head>

{#if loading && !vaultUnlocked}
  <main class="loading-screen">
    <div class="loader"></div>
    <p>Preparing VaultFive...</p>
  </main>
{:else if !vaultExists}
  <main class="auth-layout">
    <section class="brand-panel">
      <div class="brand-content">
        <div class="logo-mark">V5</div>

        <p class="brand-label">VAULTFIVE</p>

        <h1>Your passwords.<br />Your device.<br />Your control.</h1>

        <p class="brand-description">
          A local desktop password manager designed to protect your
          credentials without relying on cloud storage.
        </p>

        <div class="security-points">
          <div>
            <span>01</span>
            <p>Local encrypted storage</p>
          </div>

          <div>
            <span>02</span>
            <p>Master password protection</p>
          </div>

          <div>
            <span>03</span>
            <p>Offline-first design</p>
          </div>
        </div>
      </div>
    </section>

    <section class="auth-panel">
      <div class="auth-card">
        <p class="eyebrow">FIRST-TIME SETUP</p>

        <h2>Create your vault</h2>

        <p class="subtitle">
          Choose a master password to protect your VaultFive vault.
        </p>

        {#if message}
          <div class:success={messageType === "success"} class:error={messageType === "error"} class="message">
            {message}
          </div>
        {/if}

        <form on:submit|preventDefault={createVault}>
          <label for="master-password">Master password</label>

          <div class="password-field">
            <input
              id="master-password"
              type={showMasterPassword ? "text" : "password"}
              bind:value={masterPassword}
              autocomplete="new-password"
              placeholder="Minimum 8 characters"
            />

            <button
              type="button"
              class="visibility-button"
              on:click={() => (showMasterPassword = !showMasterPassword)}
              aria-label="Show or hide master password"
            >
              {showMasterPassword ? "Hide" : "Show"}
            </button>
          </div>

          <label for="confirm-password">Confirm master password</label>

          <div class="password-field">
            <input
              id="confirm-password"
              type={showConfirmPassword ? "text" : "password"}
              bind:value={confirmPassword}
              autocomplete="new-password"
              placeholder="Enter the password again"
            />

            <button
              type="button"
              class="visibility-button"
              on:click={() => (showConfirmPassword = !showConfirmPassword)}
              aria-label="Show or hide confirmation password"
            >
              {showConfirmPassword ? "Hide" : "Show"}
            </button>
          </div>

          <button class="primary-button" type="submit">
            Create Vault
          </button>
        </form>

        <p class="auth-note">
          VaultFive does not provide online password recovery. Keep your
          master password safe.
        </p>
      </div>
    </section>
  </main>
{:else if !vaultUnlocked}
  <main class="auth-layout">
    <section class="brand-panel">
      <div class="brand-content">
        <div class="logo-mark">V5</div>

        <p class="brand-label">VAULTFIVE</p>

        <h1>Your passwords.<br />Your device.<br />Your control.</h1>

        <p class="brand-description">
          Unlock your encrypted local vault to access your saved credentials.
        </p>

        <div class="security-points">
          <div>
            <span>01</span>
            <p>Encrypted locally</p>
          </div>

          <div>
            <span>02</span>
            <p>Protected by your master password</p>
          </div>

          <div>
            <span>03</span>
            <p>Locked when you finish</p>
          </div>
        </div>
      </div>
    </section>

    <section class="auth-panel">
      <div class="auth-card">
        <p class="eyebrow">VAULT LOCKED</p>

        <h2>Welcome back</h2>

        <p class="subtitle">
          Enter your master password to unlock VaultFive.
        </p>

        {#if message}
          <div class:success={messageType === "success"} class:error={messageType === "error"} class="message">
            {message}
          </div>
        {/if}

        <form on:submit|preventDefault={unlockVault}>
          <label for="login-password">Master password</label>

          <div class="password-field">
            <input
              id="login-password"
              type={showLoginPassword ? "text" : "password"}
              bind:value={loginPassword}
              autocomplete="current-password"
              placeholder="Enter your master password"
            />

            <button
              type="button"
              class="visibility-button"
              on:click={() => (showLoginPassword = !showLoginPassword)}
              aria-label="Show or hide master password"
            >
              {showLoginPassword ? "Hide" : "Show"}
            </button>
          </div>

          <button class="primary-button" type="submit">
            Unlock Vault
          </button>
        </form>

        <div class="locked-note">
          <span>●</span>
          <p>
            Your credential data remains encrypted until the vault is
            successfully unlocked.
          </p>
        </div>
      </div>
    </section>
  </main>
{:else}
  <main class="vault-app">
    <aside class="sidebar">
      <div>
        <div class="sidebar-brand">
          <div class="small-logo">V5</div>

          <div>
            <strong>VaultFive</strong>
            <span>Password Manager</span>
          </div>
        </div>

        <nav>
          <button class="nav-item active" type="button">
            <span class="nav-icon">▦</span>
            Vault
          </button>
        </nav>
      </div>

      <div class="sidebar-bottom">
        <div class="security-status">
          <span class="status-dot"></span>

          <div>
            <strong>Vault unlocked</strong>
            <small>Encryption active</small>
          </div>
        </div>

        <button
          class="lock-sidebar-button"
          type="button"
          on:click={lockVault}
        >
          Lock Vault
        </button>
      </div>
    </aside>

    <section class="vault-content">
      <header class="vault-header">
        <div>
          <p class="eyebrow">VAULT UNLOCKED</p>
          <h1>Your vault</h1>
          <p>
            Securely manage the credentials stored on this device.
          </p>
        </div>

        <button
          class="header-lock-button"
          type="button"
          on:click={lockVault}
        >
          Lock Vault
        </button>
      </header>

      {#if message}
        <div
          class:success={messageType === "success"}
          class:error={messageType === "error"}
          class="message dashboard-message"
        >
          {message}
        </div>
      {/if}

      <section class="vault-toolbar">
        <div class="search-wrapper">
          <span class="search-icon">⌕</span>

          <input
            type="search"
            bind:value={searchQuery}
            placeholder="Search by service, username or email..."
            aria-label="Search credentials"
          />
        </div>

        <button
          type="button"
          class="add-button"
          on:click={openAddCredential}
        >
          + Add Credential
        </button>
      </section>

      <div class="vault-summary">
        <div>
          <strong>{credentialCountText}</strong>
          <span>stored securely</span>
        </div>

        {#if searchQuery}
          <p>
            {filteredCredentials.length}
            {filteredCredentials.length === 1 ? " result" : " results"}
            for "{searchQuery}"
          </p>
        {/if}
      </div>

      {#if showCredentialForm}
        <section class="credential-form-card">
          <div class="form-header">
            <div>
              <p class="eyebrow">
                {editingId === null ? "NEW CREDENTIAL" : "EDIT CREDENTIAL"}
              </p>

              <h2>
                {editingId === null
                  ? "Add a credential"
                  : "Update credential"}
              </h2>
            </div>

            <button
              type="button"
              class="close-button"
              on:click={resetCredentialForm}
              aria-label="Close credential form"
            >
              ×
            </button>
          </div>

          <form on:submit|preventDefault={saveCredential}>
            <div class="form-grid">
              <div class="field-group">
                <label for="service">Service or website</label>

                <input
                  id="service"
                  type="text"
                  bind:value={service}
                  maxlength="100"
                  placeholder="e.g. GitHub"
                />
              </div>

              <div class="field-group">
                <label for="username">Username or email</label>

                <input
                  id="username"
                  type="text"
                  bind:value={username}
                  maxlength="200"
                  placeholder="e.g. caleb@example.com"
                />
              </div>

              <div class="field-group full-width">
                <label for="credential-password">Password</label>

                <div class="password-field credential-password-field">
                  <input
                    id="credential-password"
                    type={showCredentialPassword ? "text" : "password"}
                    bind:value={password}
                    maxlength="500"
                    autocomplete="off"
                    placeholder="Enter the account password"
                  />

                  <button
                    type="button"
                    class="visibility-button"
                    on:click={() =>
                      (showCredentialPassword = !showCredentialPassword)}
                    aria-label="Show or hide credential password"
                  >
                    {showCredentialPassword ? "Hide" : "Show"}
                  </button>
                </div>
              </div>
            </div>

            <div class="form-actions">
              <button
                type="button"
                class="secondary-button"
                on:click={resetCredentialForm}
              >
                Cancel
              </button>

              <button
                type="submit"
                class="primary-action-button"
                disabled={savingCredential}
              >
                {#if savingCredential}
                  Saving...
                {:else if editingId === null}
                  Save Credential
                {:else}
                  Update Credential
                {/if}
              </button>
            </div>
          </form>
        </section>
      {/if}

      {#if credentialLoading}
        <section class="credential-loading">
          <div class="loader small"></div>
          <p>Loading credentials...</p>
        </section>
      {:else if credentials.length === 0}
        <section class="empty-state">
          <div class="empty-icon">▣</div>

          <h2>No credentials yet</h2>

          <p>
            Add your first login to begin building your encrypted VaultFive
            vault.
          </p>

          <button
            type="button"
            class="primary-action-button"
            on:click={openAddCredential}
          >
            Add Your First Credential
          </button>
        </section>
      {:else if filteredCredentials.length === 0}
        <section class="empty-state compact">
          <div class="empty-icon">⌕</div>

          <h2>No matching credentials</h2>

          <p>
            No saved services or usernames match "{searchQuery}".
          </p>
        </section>
      {:else}
        <section class="credential-list">
          {#each filteredCredentials as credential (credential.id)}
            <article class="credential-card">
              <div class="credential-identity">
                <div class="service-avatar">
                  {credential.service.slice(0, 1).toUpperCase()}
                </div>

                <div>
                  <h3>{credential.service}</h3>
                  <p>{credential.username}</p>
                </div>
              </div>

              <div class="credential-password">
                <span class="password-label">Password</span>

                <div class="password-display">
                  <code>
                    {revealedPasswords[credential.id]
                      ? credential.password
                      : maskPassword(credential.password)}
                  </code>

                  <button
                    type="button"
                    class="reveal-button"
                    on:click={() =>
                      togglePasswordVisibility(credential.id)}
                  >
                    {revealedPasswords[credential.id] ? "Hide" : "Reveal"}
                  </button>
                </div>
              </div>

              <div class="credential-actions">
                <button
                  type="button"
                  class="edit-button"
                  on:click={() => openEditCredential(credential)}
                >
                  Edit
                </button>

                <button
                  type="button"
                  class="delete-button"
                  on:click={() => deleteCredential(credential)}
                >
                  Delete
                </button>
              </div>
            </article>
          {/each}
        </section>
      {/if}

      <footer class="vault-footer">
        <p>
          VaultFive stores credential records locally and protects them while
          the vault is locked.
        </p>
      </footer>
    </section>
  </main>
{/if}

<style>
  :global(*) {
    box-sizing: border-box;
  }

  :global(html) {
    background: #f4f7fb;
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
    color: #162033;
    background: #f4f7fb;
  }

  :global(button),
  :global(input) {
    font: inherit;
  }

  :global(button) {
    cursor: pointer;
  }

  .loading-screen {
    min-height: 100vh;
    display: grid;
    place-content: center;
    justify-items: center;
    gap: 18px;
    background: #f4f7fb;
    color: #61708b;
  }

  .loader {
    width: 42px;
    height: 42px;
    border: 4px solid #dce4f0;
    border-top-color: #3158d4;
    border-radius: 50%;
    animation: spin 0.75s linear infinite;
  }

  .loader.small {
    width: 30px;
    height: 30px;
    border-width: 3px;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .auth-layout {
    min-height: 100vh;
    display: grid;
    grid-template-columns: minmax(330px, 0.95fr) minmax(460px, 1.05fr);
  }

  .brand-panel {
    position: relative;
    display: flex;
    align-items: center;
    overflow: hidden;
    padding: 70px clamp(40px, 6vw, 100px);
    color: white;
    background:
      radial-gradient(
        circle at 15% 20%,
        rgba(90, 126, 255, 0.42),
        transparent 33%
      ),
      linear-gradient(145deg, #111d43 0%, #172969 58%, #2446a6 100%);
  }

  .brand-panel::after {
    content: "";
    position: absolute;
    width: 440px;
    height: 440px;
    right: -190px;
    bottom: -160px;
    border: 1px solid rgba(255, 255, 255, 0.13);
    border-radius: 50%;
    box-shadow:
      0 0 0 70px rgba(255, 255, 255, 0.025),
      0 0 0 140px rgba(255, 255, 255, 0.018);
  }

  .brand-content {
    position: relative;
    z-index: 1;
    max-width: 570px;
  }

  .logo-mark,
  .small-logo {
    display: grid;
    place-items: center;
    font-weight: 800;
    letter-spacing: -0.04em;
    color: white;
    background: linear-gradient(145deg, #557aff, #3158d4);
    box-shadow: 0 15px 35px rgba(17, 34, 92, 0.26);
  }

  .logo-mark {
    width: 64px;
    height: 64px;
    border-radius: 18px;
    font-size: 21px;
    margin-bottom: 28px;
  }

  .small-logo {
    width: 42px;
    height: 42px;
    border-radius: 12px;
    font-size: 15px;
  }

  .brand-label,
  .eyebrow {
    margin: 0 0 12px;
    font-size: 12px;
    font-weight: 800;
    letter-spacing: 0.14em;
  }

  .brand-label {
    color: #adc0ff;
  }

  .eyebrow {
    color: #3158d4;
  }

  .brand-panel h1 {
    margin: 0;
    max-width: 580px;
    font-size: clamp(38px, 5vw, 68px);
    line-height: 1.02;
    letter-spacing: -0.05em;
  }

  .brand-description {
    max-width: 500px;
    margin: 28px 0 36px;
    color: #c5d0eb;
    font-size: 16px;
    line-height: 1.7;
  }

  .security-points {
    display: grid;
    gap: 14px;
    max-width: 440px;
  }

  .security-points div {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 14px 0;
    border-top: 1px solid rgba(255, 255, 255, 0.11);
  }

  .security-points span {
    color: #7fa0ff;
    font-size: 11px;
    font-weight: 800;
  }

  .security-points p {
    margin: 0;
    color: #e4eaff;
    font-size: 14px;
  }

  .auth-panel {
    display: grid;
    place-items: center;
    padding: 60px;
    background: #f8faff;
  }

  .auth-card {
    width: min(100%, 500px);
  }

  .auth-card h2 {
    margin: 0;
    font-size: clamp(34px, 4vw, 48px);
    letter-spacing: -0.04em;
  }

  .subtitle {
    margin: 14px 0 32px;
    color: #69768d;
    line-height: 1.65;
  }

  form label,
  .field-group label {
    display: block;
    margin: 0 0 8px;
    color: #344158;
    font-size: 13px;
    font-weight: 700;
  }

  form > label:not(:first-child) {
    margin-top: 21px;
  }

  input {
    width: 100%;
    height: 50px;
    border: 1px solid #d7dfeb;
    border-radius: 11px;
    padding: 0 15px;
    outline: none;
    color: #162033;
    background: white;
    transition:
      border-color 0.2s,
      box-shadow 0.2s;
  }

  input:focus {
    border-color: #557aff;
    box-shadow: 0 0 0 4px rgba(85, 122, 255, 0.11);
  }

  .password-field {
    position: relative;
  }

  .password-field input {
    padding-right: 76px;
  }

  .visibility-button {
    position: absolute;
    top: 50%;
    right: 9px;
    transform: translateY(-50%);
    padding: 7px 10px;
    border: 0;
    border-radius: 7px;
    color: #3158d4;
    background: #eef2ff;
    font-size: 12px;
    font-weight: 750;
  }

  .primary-button {
    width: 100%;
    height: 52px;
    margin-top: 28px;
    border: 0;
    border-radius: 11px;
    color: white;
    background: #3158d4;
    font-weight: 750;
    box-shadow: 0 12px 24px rgba(49, 88, 212, 0.2);
  }

  .primary-button:hover,
  .add-button:hover,
  .primary-action-button:hover {
    background: #2749bc;
  }

  .auth-note {
    margin: 24px 0 0;
    color: #8a94a7;
    font-size: 12px;
    line-height: 1.6;
    text-align: center;
  }

  .locked-note {
    display: flex;
    align-items: flex-start;
    gap: 11px;
    margin-top: 26px;
    padding: 16px;
    border-radius: 11px;
    color: #66748c;
    background: #eef3fb;
    font-size: 12px;
    line-height: 1.6;
  }

  .locked-note span {
    color: #22a36a;
  }

  .locked-note p {
    margin: 0;
  }

  .message {
    margin: 0 0 22px;
    padding: 13px 15px;
    border-radius: 10px;
    font-size: 13px;
    line-height: 1.45;
  }

  .message.success {
    border: 1px solid #b7e7cf;
    color: #176944;
    background: #ecfaf3;
  }

  .message.error {
    border: 1px solid #f3c0c7;
    color: #a12839;
    background: #fff0f2;
  }

  .vault-app {
    min-height: 100vh;
    display: grid;
    grid-template-columns: 245px minmax(0, 1fr);
    background: #f5f7fb;
  }

  .sidebar {
    position: sticky;
    top: 0;
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 25px 18px;
    color: white;
    background: #14224a;
  }

  .sidebar-brand {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 0 5px 29px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.09);
  }

  .sidebar-brand strong,
  .sidebar-brand span {
    display: block;
  }

  .sidebar-brand strong {
    font-size: 16px;
  }

  .sidebar-brand span {
    margin-top: 3px;
    color: #99a9cf;
    font-size: 10px;
  }

  nav {
    margin-top: 25px;
  }

  .nav-item {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 11px;
    padding: 12px 13px;
    border: 0;
    border-radius: 10px;
    color: #b9c6e4;
    background: transparent;
    text-align: left;
  }

  .nav-item.active {
    color: white;
    background: rgba(84, 117, 224, 0.21);
  }

  .nav-icon {
    font-size: 17px;
  }

  .security-status {
    display: flex;
    gap: 10px;
    padding: 14px 10px;
    margin-bottom: 13px;
    border-radius: 10px;
    background: rgba(255, 255, 255, 0.055);
  }

  .status-dot {
    width: 9px;
    height: 9px;
    margin-top: 4px;
    border-radius: 50%;
    background: #35d889;
    box-shadow: 0 0 0 5px rgba(53, 216, 137, 0.1);
  }

  .security-status strong,
  .security-status small {
    display: block;
  }

  .security-status strong {
    font-size: 12px;
  }

  .security-status small {
    margin-top: 3px;
    color: #9aabd0;
    font-size: 10px;
  }

  .lock-sidebar-button {
    width: 100%;
    padding: 10px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 9px;
    color: white;
    background: transparent;
    font-size: 12px;
    font-weight: 700;
  }

  .vault-content {
    width: 100%;
    max-width: 1440px;
    margin: 0 auto;
    padding: 40px clamp(28px, 4vw, 60px);
  }

  .vault-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 30px;
    padding-bottom: 28px;
    border-bottom: 1px solid #e0e6ef;
  }

  .vault-header h1 {
    margin: 0;
    font-size: clamp(32px, 4vw, 46px);
    letter-spacing: -0.04em;
  }

  .vault-header > div > p:last-child {
    margin: 9px 0 0;
    color: #718097;
  }

  .header-lock-button {
    flex-shrink: 0;
    padding: 10px 16px;
    border: 1px solid #ccd5e4;
    border-radius: 9px;
    color: #43516a;
    background: white;
    font-weight: 700;
  }

  .dashboard-message {
    margin-top: 22px;
    margin-bottom: 0;
  }

  .vault-toolbar {
    display: flex;
    gap: 14px;
    margin-top: 28px;
  }

  .search-wrapper {
    position: relative;
    flex: 1;
  }

  .search-wrapper input {
    padding-left: 42px;
    background: white;
  }

  .search-icon {
    position: absolute;
    z-index: 1;
    top: 50%;
    left: 15px;
    transform: translateY(-50%);
    color: #8491a7;
    font-size: 20px;
  }

  .add-button,
  .primary-action-button {
    border: 0;
    border-radius: 10px;
    color: white;
    background: #3158d4;
    font-weight: 750;
  }

  .add-button {
    padding: 0 21px;
  }

  .vault-summary {
    min-height: 55px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 20px;
  }

  .vault-summary div {
    display: flex;
    align-items: baseline;
    gap: 8px;
  }

  .vault-summary strong {
    font-size: 13px;
  }

  .vault-summary span,
  .vault-summary p {
    color: #8793a6;
    font-size: 12px;
  }

  .credential-form-card {
    padding: 25px;
    margin-bottom: 24px;
    border: 1px solid #dce3ee;
    border-radius: 15px;
    background: white;
    box-shadow: 0 8px 28px rgba(25, 41, 80, 0.05);
  }

  .form-header {
    display: flex;
    justify-content: space-between;
    gap: 20px;
    margin-bottom: 23px;
  }

  .form-header h2 {
    margin: 0;
    font-size: 24px;
  }

  .close-button {
    width: 36px;
    height: 36px;
    border: 0;
    border-radius: 9px;
    color: #69758a;
    background: #f0f3f8;
    font-size: 24px;
  }

  .form-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 19px;
  }

  .field-group.full-width {
    grid-column: 1 / -1;
  }

  .credential-password-field input {
    height: 48px;
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 23px;
  }

  .secondary-button {
    padding: 11px 19px;
    border: 1px solid #d3dbe8;
    border-radius: 9px;
    color: #58657b;
    background: white;
    font-weight: 700;
  }

  .primary-action-button {
    padding: 12px 20px;
  }

  .primary-action-button:disabled {
    opacity: 0.55;
    cursor: not-allowed;
  }

  .credential-loading {
    min-height: 260px;
    display: grid;
    place-content: center;
    justify-items: center;
    gap: 12px;
    color: #7b879c;
  }

  .empty-state {
    min-height: 360px;
    display: grid;
    place-content: center;
    justify-items: center;
    padding: 45px;
    border: 1px dashed #ced7e5;
    border-radius: 15px;
    background: rgba(255, 255, 255, 0.63);
    text-align: center;
  }

  .empty-state.compact {
    min-height: 270px;
  }

  .empty-icon {
    width: 62px;
    height: 62px;
    display: grid;
    place-items: center;
    margin-bottom: 18px;
    border-radius: 17px;
    color: #3158d4;
    background: #edf1ff;
    font-size: 28px;
  }

  .empty-state h2 {
    margin: 0;
    font-size: 22px;
  }

  .empty-state p {
    max-width: 450px;
    margin: 10px 0 23px;
    color: #7c899e;
    line-height: 1.6;
  }

  .credential-list {
    display: grid;
    gap: 12px;
  }

  .credential-card {
    display: grid;
    grid-template-columns:
      minmax(190px, 1.2fr)
      minmax(210px, 1fr)
      auto;
    align-items: center;
    gap: 25px;
    padding: 19px 20px;
    border: 1px solid #dfe5ef;
    border-radius: 13px;
    background: white;
    transition:
      transform 0.15s,
      box-shadow 0.15s;
  }

  .credential-card:hover {
    transform: translateY(-1px);
    box-shadow: 0 9px 26px rgba(28, 42, 72, 0.06);
  }

  .credential-identity {
    min-width: 0;
    display: flex;
    align-items: center;
    gap: 13px;
  }

  .service-avatar {
    flex: 0 0 43px;
    width: 43px;
    height: 43px;
    display: grid;
    place-items: center;
    border-radius: 11px;
    color: #3158d4;
    background: #eaf0ff;
    font-weight: 850;
  }

  .credential-identity h3 {
    margin: 0;
    font-size: 15px;
  }

  .credential-identity p {
    max-width: 300px;
    margin: 5px 0 0;
    overflow: hidden;
    color: #7c889d;
    font-size: 12px;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .credential-password {
    min-width: 0;
  }

  .password-label {
    display: block;
    margin-bottom: 7px;
    color: #8a96a8;
    font-size: 10px;
    font-weight: 800;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  .password-display {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .password-display code {
    max-width: 240px;
    overflow: hidden;
    color: #344056;
    font-family:
      "SFMono-Regular",
      Consolas,
      monospace;
    font-size: 13px;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .reveal-button {
    padding: 5px 8px;
    border: 0;
    border-radius: 6px;
    color: #3158d4;
    background: #edf1ff;
    font-size: 10px;
    font-weight: 750;
  }

  .credential-actions {
    display: flex;
    justify-content: flex-end;
    gap: 7px;
  }

  .edit-button,
  .delete-button {
    padding: 8px 12px;
    border-radius: 8px;
    font-size: 11px;
    font-weight: 750;
  }

  .edit-button {
    border: 1px solid #cfd8e7;
    color: #45536a;
    background: white;
  }

  .delete-button {
    border: 1px solid #f0c8ce;
    color: #a93141;
    background: #fff6f7;
  }

  .vault-footer {
    margin-top: 30px;
    padding-top: 20px;
    border-top: 1px solid #e0e6ef;
    color: #929caf;
    font-size: 11px;
  }

  @media (max-width: 1000px) {
    .auth-layout {
      grid-template-columns: 1fr;
    }

    .brand-panel {
      min-height: auto;
      padding-top: 48px;
      padding-bottom: 48px;
    }

    .brand-panel h1 {
      font-size: 43px;
    }

    .security-points {
      display: none;
    }

    .auth-panel {
      padding: 50px 28px;
    }

    .vault-app {
      grid-template-columns: 1fr;
    }

    .sidebar {
      position: static;
      height: auto;
      flex-direction: row;
      align-items: center;
      padding: 15px 20px;
    }

    .sidebar-brand {
      padding: 0;
      border: 0;
    }

    .sidebar nav,
    .security-status {
      display: none;
    }

    .sidebar-bottom {
      display: flex;
      align-items: center;
    }

    .lock-sidebar-button {
      width: auto;
      padding: 9px 14px;
    }

    .header-lock-button {
      display: none;
    }
  }

  @media (max-width: 760px) {
    .vault-content {
      padding: 30px 18px;
    }

    .vault-toolbar {
      flex-direction: column;
    }

    .add-button {
      min-height: 48px;
    }

    .form-grid {
      grid-template-columns: 1fr;
    }

    .field-group.full-width {
      grid-column: auto;
    }

    .credential-card {
      grid-template-columns: 1fr;
      gap: 18px;
    }

    .credential-actions {
      justify-content: flex-start;
    }

    .vault-summary {
      align-items: flex-start;
      flex-direction: column;
      gap: 4px;
      padding: 14px 0;
    }
  }

  @media (max-width: 520px) {
    .auth-panel {
      padding: 40px 20px;
    }

    .brand-panel {
      padding: 38px 25px;
    }

    .brand-panel h1 {
      font-size: 34px;
    }

    .brand-description {
      margin-bottom: 0;
    }

    .vault-header h1 {
      font-size: 31px;
    }

    .sidebar-brand span {
      display: none;
    }

    .form-actions {
      flex-direction: column-reverse;
    }

    .form-actions button {
      width: 100%;
    }
  }
</style>
