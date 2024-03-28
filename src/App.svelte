<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let inputCipher = "";
  let password = "";
  let outputSeed = ""
  let passwordWarning = "";
  let showPassword = false;

  function togglePasswordVisibility() {
    showPassword = !showPassword;
  }

    async function handleFormSubmit() {
  try {
    outputSeed = await invoke('handle_data', { inputCipher: inputCipher, password: password });


    if (!outputSeed) {
      throw new Error("The ciphertext and password do not match");
    }

    passwordWarning = "";
  } catch (error: any) {

    outputSeed = "";
    passwordWarning = error.message || "The ciphertext and password do not match";
  }
}

    function copyToClipboard(text: string) {
    navigator.clipboard.writeText(text).then(
      function () {
        console.log("copyed!");
      },
      function (err) {
        console.error("error", err);
      },
    );
  }

   const handlePaste = (event: ClipboardEvent) => {
    event.preventDefault();
    const clipboardData = event.clipboardData;
    if (clipboardData) {
        const pastedData = clipboardData.getData('text');
        inputCipher = pastedData.replace(/\r\n/g, '\n').replace(/\n/g, ' ');
    }
  };

</script>

<main>
  <h1>Valut39: Decryption</h1>
  <h2>Decrypting the Seed Phrase</h2>
  <p>you can decrypt an encrypted seed phrase. Please enter the ciphertext and password, then click the "Decrypt" button.</p>
  <p>Use the password you set during encryption.</p>
  <form on:submit|preventDefault={handleFormSubmit}>
    <label>
      Ciphertext:
      <textarea bind:value={inputCipher} style="width: 100%; white-space: pre-wrap;" rows="2" on:paste={handlePaste}></textarea>
    </label>
    <label>
      Password:
      {#if showPassword}
    <input type="text" bind:value={password} style="width: 100%;" minlength="6" class="password-input" />
  {:else}
    <input type="password" bind:value={password} style="width: 100%;" minlength="6" class="password-input" />
  {/if}
  <button type="button" on:click={togglePasswordVisibility} class="toggle-button">{showPassword ? 'Hide' : 'Show password'}</button>
    </label>
    <button type="submit">Decrypt</button>
    {#if passwordWarning}
    <p style="color: red;">{passwordWarning}</p>
    {/if}
  </form>

    {#if outputSeed}
    <p>Seed Phrase: {outputSeed}</p>
  {/if}

</main>

<style>
  :global(body) {
    margin: 0;
    background: linear-gradient(to bottom right, #afeeee 33%, #f0e68c 66%);
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;

  }
  main {
    padding: 20px;
    font-family: sans-serif;
    color: #696969;
    width: 66%;
    box-sizing: border-box;
  }
  h1 {
    font-size: 24px;
    margin-bottom: 20px;
  }
  h2 {
    font-size: 18px;
    margin-bottom: 10px;
  }
  p {
    font-size: 16px;
    margin-bottom: 50px;
  }
  label {
    display: block;
    margin-bottom: 10px;
  }
  textarea{
    padding: 5px;
    font-size: 16px;
    width: 48vw;
    height: 12vw;
  }
  input {
    padding: 5px;
    font-size: 16px;
  }
  button {
    margin-top: 20px;
    padding: 5px 10px;
    font-size: 16px;
    cursor: pointer;
  }
</style>

