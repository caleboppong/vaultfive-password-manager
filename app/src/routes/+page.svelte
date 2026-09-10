<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = "";
  let greetMsg = "";
  let dbMsg = "";

  async function greet() {
    greetMsg = await invoke<string>("greet", { name });
  }

  async function testDatabase() {
    try {
      dbMsg = await invoke<string>("test_database");
    } catch (error) {
      dbMsg = `Database test failed: ${error}`;
    }
  }
</script>

<main class="container">
  <h1>VaultFive Technical Proof of Concept</h1>

  <section>
    <h2>Rust Command Test</h2>

    <div class="row">
      <input
        bind:value={name}
        placeholder="Enter a name..."
      />

      <button on:click={greet}>
        Greet
      </button>
    </div>

    {#if greetMsg}
      <p>{greetMsg}</p>
    {/if}
  </section>

  <section>
    <h2>SQLite Test</h2>

    <button on:click={testDatabase}>
      Test SQLite Connection
    </button>

    {#if dbMsg}
      <p>{dbMsg}</p>
    {/if}
  </section>
</main>

<style>
  .container {
    max-width: 700px;
    margin: 0 auto;
    padding: 3rem 2rem;
    font-family: Arial, sans-serif;
  }

  section {
    margin-top: 2rem;
    padding: 1.5rem;
    border: 1px solid #ccc;
    border-radius: 8px;
  }

  .row {
    display: flex;
    gap: 0.5rem;
  }

  input {
    flex: 1;
    padding: 0.75rem;
  }


  button {
    padding: 0.75rem 1rem;
    cursor: pointer;
  }
  
</style>
