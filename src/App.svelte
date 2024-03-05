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
      passwordWarning = "";
    } catch (error: any) {
      // console.error(error);
      outputSeed = "";
      passwordWarning = error.message || "暗号文とパスワードが一致していません";
    }
  }

    function copyToClipboard(text: string) {
    navigator.clipboard.writeText(text).then(
      function () {
        console.log("コピー成功");
      },
      function (err) {
        console.error("コピー失敗", err);
      },
    );
  }

   const handlePaste = (event: ClipboardEvent) => {
    event.preventDefault();
    const clipboardData = event.clipboardData;
    if (clipboardData) {
        const pastedData = clipboardData.getData('text');
        // CRLFをLFに置換して、不要な改行を除去
        inputCipher = pastedData.replace(/\r\n/g, '\n').replace(/\n/g, ' '); // 修正箇所
    }
  };

</script>

<main>
  <h1>Nil Wallet: 暗号復号</h1>
  <h3>シードフレーズの復号</h3>
  <p>こちらでは暗号化されたシードフレーズを復号することができます。暗号文とパスワードを入力して「復号する」ボタンをクリックしてください。</p>
  <p>パスワードは暗号化時に設定したものを使用してください。</p>
  <form on:submit|preventDefault={handleFormSubmit}>
    <label>
      暗号文:
      <textarea bind:value={inputCipher} style="width: 100%; white-space: pre-wrap;" rows="2" on:paste={handlePaste}></textarea>
    </label>
    <label>
      パスワード:
      {#if showPassword}
    <input type="text" bind:value={password} style="width: 100%;" minlength="6" class="password-input" />
  {:else}
    <input type="password" bind:value={password} style="width: 100%;" minlength="6" class="password-input" />
  {/if}
  <button type="button" on:click={togglePasswordVisibility} class="toggle-button">{showPassword ? '非表示にする' : 'パスワードを表示する'}</button>
    </label>
    <button type="submit">復号する</button>
    {#if passwordWarning}
    <p style="color: red;">{passwordWarning}</p>
    {/if}
  </form>

    {#if outputSeed}
    <p>シードフレーズ: {outputSeed}</p>
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
  h3 {
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

