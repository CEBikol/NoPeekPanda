<script>
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    import { themes, applyTheme } from "../utils/theme";
    import { onMount } from "svelte";

    export let onBack;

    let theme = "mocha";
    let vaultPath = "";
    let isLoading = false;

    onMount(async () => {
        await loadSettings();
    });

    async function loadSettings() {
        try {
            isLoading = true;
            const settings = await invoke("get_settings");
            theme = settings.theme || "mocha";
            vaultPath = settings.vault_folder_path || "";

            applyTheme(theme);
        } catch (error) {
            console.error("Ошибка загрузки настроек:", error);
        } finally {
            isLoading = false;
        }
    }

    async function selectFolder() {
        try {
            const selected = await open({
                directory: true,
                multiple: false,
                title: "Выберите папку с хранилищами",
            });

            if (selected) {
                vaultPath = selected;
            }
        } catch (error) {
            console.error("Ошибка выбора папки:", error);
        }
    }

    function handleThemeChange(event) {
        const newTheme = event.target.value;
        theme = newTheme;
        applyTheme(newTheme);
    }

    async function saveSettings() {
        try {
            isLoading = true;
            await invoke("update_settings", {
                theme: theme,
                vaultFolderPath: vaultPath,
            });

            if (onBack) onBack();
        } catch (error) {
            console.error("Ошибка сохранения настроек:", error);
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="settings-page">
    <div class="header">
        <button on:click={onBack} class="back-btn">← Назад</button>
        <h1>Настройки</h1>
    </div>

    {#if isLoading}
        <div class="loading">Загрузка...</div>
    {:else}
        <div class="settings-form">
            <div class="form-group">
                <label for="theme">Тема:</label>
                <select
                    id="theme"
                    bind:value={theme}
                    on:change={handleThemeChange}
                >
                    {#each Object.entries(themes) as [key, themeData]}
                        <option value={key}>{themeData.name}</option>
                    {/each}
                </select>
            </div>

            <div class="form-group">
                <label>Путь к хранилищам:</label>
                <div class="path-selector">
                    <input
                        type="text"
                        bind:value={vaultPath}
                        placeholder="Выберите папку с .vault файлами"
                        readonly
                        class="input-field"
                    />
                    <button on:click={selectFolder} class="browse-btn">
                        Обзор...
                    </button>
                </div>
            </div>

            <button on:click={saveSettings} class="save-btn">Сохранить</button>
        </div>
    {/if}
</div>

<style>
    .settings-page {
        color: var(--ctp-text);
        padding: 2rem;
        max-width: 600px;
        margin: 0 auto;
        background-color: var(--ctp-base);
        min-height: 100vh;
        box-sizing: border-box;
    }

    .header {
        display: flex;
        align-items: center;
        margin-bottom: 2rem;
        gap: 1rem;
    }

    .back-btn {
        background: var(--ctp-surface0);
        border: 1px solid var(--ctp-overlay0);
        color: var(--ctp-text);
        padding: 0.5rem 1rem;
        border-radius: 6px;
        cursor: pointer;
        font-family: inherit;
    }

    .back-btn:hover {
        background: var(--ctp-surface1);
    }

    h1 {
        margin: 0;
        font-size: 1.8rem;
        color: var(--ctp-text);
    }

    .settings-form {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .form-group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    label {
        font-weight: 500;
    }

    select,
    .input-field {
        background: var(--ctp-surface0);
        color: var(--ctp-text);
        border: 1px solid var(--ctp-overlay0);
        padding: 0.75rem;
        border-radius: 6px;
        font-family: inherit;
        font-size: 1rem;
    }

    select {
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%23cdd6f4' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='m6 9 6 6 6-6'/%3E%3C/svg%3E");
        background-repeat: no-repeat;
        background-position: right 0.75rem center;
        background-size: 16px;
        padding-right: 2.5rem;
    }

    .path-selector {
        display: flex;
        gap: 0.5rem;
    }

    .path-selector input {
        flex: 1;
    }

    .browse-btn,
    .save-btn {
        padding: 0.75rem 1rem;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        border: none;
        font-family: inherit;
        font-size: 1rem;
    }

    .browse-btn {
        background: var(--ctp-surface1);
        color: var(--ctp-text);
        white-space: nowrap;
    }

    .browse-btn:hover {
        background: var(--ctp-surface2);
    }

    .save-btn {
        background: var(--ctp-green);
        color: var(--ctp-base);
        margin-top: 1rem;
    }

    .save-btn:hover {
        background: var(--ctp-teal);
    }

    .loading {
        text-align: center;
        padding: 2rem;
        color: var(--ctp-text);
    }
</style>
