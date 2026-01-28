<script>
    // @ts-nocheck
    const SearchIcon = "/icons/search.png";

    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    import { themes, applyTheme } from "../utils/theme";
    import { onMount } from "svelte";

    // @ts-ignore
    export let onBack;

    let theme = "mocha";
    let vaultPath = "";
    let isLoading = false;

     // Состояния обновлений
    let updateInfo = null;
    let showUpdateModal = false;
    let isUpdating = false;
    let isWindows = false;

    onMount(async () => {
        await loadSettings();
        try {
           const os = await invoke("get_os");
           isWindows = os === "windows";
           console.log("Определена ОС:", os);
        }  catch (e) {
           console.error("Ошибка определения ОС:", e);
           isWindows = false;
        }
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

    // @ts-ignore
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

            // @ts-ignore
            if (onBack) onBack();
        } catch (error) {
            console.error("Ошибка сохранения настроек:", error);
        } finally {
            isLoading = false;
        }
    }

    async function checkForUpdate() {
    try {
        const result = await invoke("check_update");
        updateInfo = result; // null если обновлений нет
        showUpdateModal = true;
    } catch (e) {
        alert(`Ошибка проверки: ${e.message || e}`);
    }
}

async function startInstall() {
    const message = isWindows
        ? "⚠️ После установки приложение будет ЗАКРЫТО.\nСохраните все данные!\n\nПродолжить?"
        : "После установки приложение перезапустится.\nУбедитесь, что данные сохранены.\n\nПродолжить?";
    
    if (!confirm(message)) return;

    isUpdating = true;
    try {
        await invoke("install_update");
        // На успех: приложение закроется (Win) или перезапустится (Linux/macOS)
        // Сюда дойдём ТОЛЬКО при ошибке
    } catch (e) {
        alert(`Ошибка установки: ${e.message || e}`);
        isUpdating = false; // Разблокируем кнопку для повторной попытки
    }
    // При успехе состояние не сбрасываем — приложение уже закрылось/перезапустилось
}

function closeUpdateModal() {
    showUpdateModal = false;
    updateInfo = null;
    isUpdating = false;
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
            <div class="action-buttons">
                <button 
                    on:click={checkForUpdate} 
                    title="Проверить обновления"
                    class="action-btn"
                    aria-label="Проверить обновления"
                >
                    <img 
                    src={SearchIcon}
                    alt="Проверить обновления" 
                    class="icon action-btn"
                    />
                </button>
            </div>

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
                <!-- svelte-ignore a11y_label_has_associated_control -->
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

{#if showUpdateModal}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-overlay" on:click={closeUpdateModal}>
    <div class="modal-content" on:click|stopPropagation>
      <div class="modal-header">
        <h2>{updateInfo ? 'Доступно обновление' : 'Обновления не найдены'}</h2>
        <button class="modal-close" on:click={closeUpdateModal}>×</button>
      </div>
      <div class="modal-body">
        {#if updateInfo}
          <p><strong>Версия:</strong> {updateInfo.version}</p>
          <p><strong>Описание:</strong> {updateInfo.body || 'Новое обновление'}</p>
          <div class="warning-box">
            <p>⚠️ После установки приложение будет {isWindows ? 'закрыто' : 'перезапущено'}.</p>
            <p>Убедитесь, что все данные сохранены!</p>
          </div>
        {:else}
          <p>Вы используете последнюю версию ✅</p>
        {/if}
      </div>
      <div class="modal-footer">
        {#if updateInfo}
          <button class="btn-secondary" on:click={closeUpdateModal}>Отмена</button>
          <button 
            class="btn-primary" 
            on:click={startInstall} 
            disabled={isUpdating}
          >
            {isUpdating ? '⏳ Установка...' : '⬇️ Установить'}
          </button>
        {:else}
          <button class="btn-primary" on:click={closeUpdateModal}>Закрыть</button>
        {/if}
      </div>
    </div>
  </div>
{/if}

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
    .modal-overlay {
        position: fixed;
        inset: 0;
        background-color: color-mix(in srgb, var(--ctp-crust) 60%, transparent);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
        backdrop-filter: blur(4px);
    }
    .modal-content {
        background-color: var(--ctp-mantle);
        border-radius: 12px;
        width: 90%;
        max-width: 500px;
        box-shadow: 0 20px 40px color-mix(in srgb, var(--ctp-crust) 30%, transparent);
    }
    .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1.5rem;
        border-bottom: 1px solid var(--ctp-surface0);
    }
    .modal-header h2 { margin: 0; color: var(--ctp-text); }
    .modal-close {
        background: none; border: none; font-size: 1.5rem; cursor: pointer;
        color: var(--ctp-subtext1); width: 32px; height: 32px; border-radius: 4px;
        display: flex; align-items: center; justify-content: center;
    }
    .modal-close:hover { background-color: var(--ctp-surface0); color: var(--ctp-text); }
    .modal-body { padding: 1.5rem; color: var(--ctp-text); }
    .warning-box {
        background-color: color-mix(in srgb, var(--ctp-yellow) 10%, transparent);
        border-left: 3px solid var(--ctp-yellow);
        padding: 1rem;
        border-radius: 0 6px 6px 0;
        margin-top: 1rem;
        font-weight: 500;
    }
    .modal-footer {
        display: flex;
        gap: 1rem;
        justify-content: flex-end;
        padding: 1.5rem;
        border-top: 1px solid var(--ctp-surface0);
    }
    .btn-primary, .btn-secondary {
        padding: 0.75rem 1.5rem;
        border-radius: 8px;
        font-family: inherit;
        font-weight: 500;
        cursor: pointer;
        border: none;
    }
    .btn-primary {
        background-color: var(--ctp-green);
        color: var(--ctp-base);
    }
    .btn-primary:hover:not(:disabled) { background-color: var(--ctp-teal); }
    .btn-primary:disabled { opacity: 0.6; cursor: not-allowed; }
    .btn-secondary {
        background-color: var(--ctp-surface0);
        color: var(--ctp-text);
    }
    .btn-secondary:hover { background-color: var(--ctp-surface1); }
    /* Контейнер для кнопок действий */
    .action-buttons {
        display: flex;
        flex-wrap: wrap;
        gap: 0.75rem;
        justify-content: center;
        margin-bottom: 1.25rem;
        padding: 0.5rem;
    }

    /* Единый стиль для всех кнопок в блоке */
    .action-btn {
        background: none;
        border: 1px solid var(--ctp-surface0);
        border-radius: 8px;
        width: 40px;
        height: 40px;
        padding: 0.4rem;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.2s ease;
        position: relative;
    }

    .action-btn:hover {
        background-color: var(--ctp-surface2);
        transform: translateY(-1px);
    }

    .action-btn:active {
        transform: translateY(0);
    }

    /* Иконка внутри кнопки (сохраняет реакцию на тему через .icon) */
    .action-icon {
        width: 20px;
        height: 20px;
        transition: transform 0.2s ease;
    }

    .action-btn:hover .action-icon {transform: scale(1.15)}

</style>
