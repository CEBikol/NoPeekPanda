<script>
// @ts-nocheck

    const ShowIcon = "/icons/visible.png";
    const HideIcon = "/icons/hide.png";
    const SettingsIcon = "/icons/settings.png";
    const RefreshIcon = "/icons/refresh.png";
    const AddIcon = "/icons/add.png";
    const TauriIcon = "/icons/tauri-logo.png";
    const CatppuccinIcon = "/icons/catppuccin-logo.png";
    const RustIcon = "/icons/rust-logo.png";
    const FlutIcon = "/icons/flaticon.png"
    const ArrowDownIcon = "/icons/arrow-down-sign.png"

    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { loadTheme } from "../utils/theme.js";
    import { openUrl } from "@tauri-apps/plugin-opener";

    import Settings from "./settings.svelte";
    import MainInterface from "./main.svelte";

    let currentView = "login";
    let vaultFiles = [];
    let selectedFile = "";
    let error = "";
    let password = "";
    let showPassword = false;
    let isLoading = false;
    let vaultPath = "";
    let isVaultOpen = false;
    // Состояние модального окна создания хранилища
    let showCreateVaultModal = false;
    let newVaultName = "";
    let newVaultPassword = "";
    let showNewVaultPassword = false;
    let isCreatingVault = false;

    let vaultFolderExists = false;

    $: hasVaultPath = vaultPath !== "";
    
    $: if (hasVaultPath) {
        checkVaultFolder();
    }

    onMount(async () => {
        loadTheme();
        await loadAppSettings();
        await checkVaultFolder();
        await loadVaultFiles();
    });

    async function loadAppSettings() {
        try {
            const settings = await invoke("get_settings");
            vaultPath = settings.vault_folder_path || "";
            
            if (hasVaultPath) {
                await checkVaultFolder();
            } else {
              error= "Не задан путь до папки"
            }
        } catch (e) {
            // @ts-ignore
            error = ("Ошибка загрузки настроек:", e);
            vaultPath = "";
            vaultFolderExists = false;
        }
    }

    async function checkVaultFolder() {
        try {
            vaultFolderExists = await invoke("check_directory_exists", { path: vaultPath });
            if (!vaultFolderExists) {
                await invoke("create_directory", { path: vaultPath });
                vaultFolderExists = true;
            }
        } catch (e) {
            console.error("Ошибка проверки/создания папки:", e);
            vaultFolderExists = false;
        }
    }
    async function loadVaultFiles() {
        try {
            isLoading = true;
            vaultFiles = await invoke("populate_list");
            error = "";
        } catch (e) {
            error = "Не удалось загрузить список хранилищ";
            vaultFiles = [];
        } finally {
            isLoading = false;
        }
    }

    function togglePasswordVisibility() {
        showPassword = !showPassword;
    }

    function openSettings() {
        currentView = "settings";
    }

    async function closeSettings() {
        await loadAppSettings();
        await loadVaultFiles();
        currentView = "login";
    }

   async function handleLogin() {
        if (!selectedFile) {
            error = "Пожалуйста, выберите хранилище";
            return;
        }
        
        if (!password) {
            error = "Пожалуйста, введите пароль";
            return;
        }

        try {
            isLoading = true;
            
            // Формируем полный путь к хранилищу
            const fullPath = `${vaultPath}/${selectedFile}.db`;
            
            await invoke("open_vault", {
                path: fullPath,
                masterPassword: password
            });
            
            // Немедленно очищаем пароль из памяти
            password = "";
            showPassword = false;
            
            isVaultOpen = true;
            currentView = "main";
            error = "";
            
            console.log("Успешный вход, переключаемся на main view");
            
        } catch (e) {
            console.error("Ошибка входа:", e);
            if (e.includes("Invalid master password")) {
                error = "Неверный пароль. Попробуйте еще раз.";
            } else if (e.includes("Storage not found")) {
                error = "Хранилище не найдено. Обновите список.";
            } else {
                error = "Ошибка входа. Проверьте данные.";
            }
        } finally {
            isLoading = false;
        }
    }
    
    async function handleLogout() {
        try {
            await invoke("close_vault");
        } catch (e) {
            console.error("Ошибка закрытия хранилища:", e);
        } finally {
            isVaultOpen = false;
            currentView = "login";
            password = "";
        }
    }

    function openCreateVaultModal() {
    if (!hasVaultPath) return;
    showCreateVaultModal = true;
    newVaultName = "";
    newVaultPassword = "";
    showNewVaultPassword = false;
    error = "";
}

function closeCreateVaultModal() {
    showCreateVaultModal = false;
    newVaultName = "";
    newVaultPassword = "";
    showNewVaultPassword = false;
    error = "";
}

async function handleCreateVault() {
    if (!newVaultName.trim()) {
        error = "Введите имя хранилища";
        return;
    }
    if (!newVaultPassword) {
        error = "Введите пароль";
        return;
    }

    try {
        isCreatingVault = true;
        await invoke("create_vault", {
            storageName: newVaultName.trim(),
            password: newVaultPassword
        });

        await loadVaultFiles();
        selectedFile = newVaultName.trim();
        closeCreateVaultModal();
        error = "";
    } catch (e) {
        console.error("Ошибка создания хранилища:", e);
        if (e.includes("Storage with this name already exists")) {
            error = "Хранилище с таким именем уже существует";
        } else {
            error = "Не удалось создать хранилище";
        }
    } finally {
        isCreatingVault = false;
    }
}

function toggleNewVaultPasswordVisibility() {
    showNewVaultPassword = !showNewVaultPassword;
}

    function openTauriWebsite() {
        openUrl("https://tauri.app/");
    }

    function openCatppuccinWebsite() {
        openUrl("https://catppuccin.com/");
    }

    function openRustWebsite() {
        openUrl("https://www.rust-lang.org/");
    }

    function openFlutIconWebsite() {
        openUrl("https://www.flaticon.com/")
    }
</script>

<svelte:head>
    <link rel="stylesheet" href="/themes/app.css" />
</svelte:head>


    {#if error}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="global-error" on:click={() => error = ''}>
            {error}
        </div>
    {/if}
    
    {#if currentView === "login"}
    <main class="container">
        <div class="card">
            <div class="login-header">
                <h1>Выберите хранилище</h1>
                <div class="header-buttons">
                    <button
                        on:click={openCreateVaultModal}
                        class="add-btn"
                        title="Создать новое хранилище"
                        disabled={!hasVaultPath}
                    >
                        <img
                            src={AddIcon}
                            alt="Создать хранилище"
                            class="icon"
                        />
                    </button>
                    <button
                        on:click={openSettings}
                        class="settings-btn"
                        title="Настройки"
                    >
                        <img 
                            src={SettingsIcon} 
                            alt="Настройки" 
                            class="icon"
                         />
                    </button>
                </div>
            </div>

            <div class="form-group">
                <div class="select-with-refresh">
                    <div class="select-wrapper">
                    <select bind:value={selectedFile} class="input-field">
                        <option value="" disabled selected>Выберите хранилище</option>
                        {#each vaultFiles as file}
                        <option value={file}>{file}</option>
                        {/each}
                    </select>
                    <img src={ArrowDownIcon} alt="" class="select-arrow icon" />
                    </div>
                    <button
                    on:click={loadVaultFiles}
                    class="refresh-btn"
                    title="Обновить список"
                    disabled={isLoading}
                    >
                    {#if isLoading}
                        <div class="spinner"></div>
                    {:else}
                        <img src={RefreshIcon} alt="Обновить" class="icon" />
                    {/if}
                    </button>
                </div>
            </div>
            <div class="form-group">
                <div class="password-wrapper">
                    <input
                        id="password"
                        type={showPassword ? "text" : "password"}
                        bind:value={password}
                        placeholder="Введите пароль"
                        class="input-field"
                        on:keypress={(e) => e.key === "Enter" && handleLogin()}
                    />
                    <button
                        type="button"
                        on:click={togglePasswordVisibility}
                        class="toggle-btn"
                        title={showPassword
                            ? "Скрыть пароль"
                            : "Показать пароль"}
                    >
                        {#if showPassword}
                            <img
                                src={HideIcon}
                                alt="Скрыть пароль"
                                class="icon"
                            />
                        {:else}
                            <img
                                src={ShowIcon}
                                alt="Показать пароль"
                                class="icon"
                            />
                        {/if}
                    </button>
                </div>
            </div>
            <button on:click={handleLogin} class="action-btn">Войти</button>
        </div>
        <div class="logos-container">
            <div class="logos">
                <button on:click={openTauriWebsite} class="logo-btn" title="Tauri">
                    <img src={TauriIcon} alt="Tauri" class="logo-icon" />
                </button>
                <button on:click={openCatppuccinWebsite} class="logo-btn" title="Catppuccin">
                    <img src={CatppuccinIcon} alt="Catppuccin" class="logo-icon" />
                </button>
                <button on:click={openRustWebsite} class="logo-btn" title="Rust">
                    <img src={RustIcon} alt="Rust" class="logo-icon" />
                </button>
                <button on:click={openFlutIconWebsite} class="logo-btn" title="Fluticon">
                    <img src={FlutIcon} alt="FlutIcon" class="logo-icon">
                </button>    
            </div>
        </div>
    </main>
    {:else if currentView === "settings"}
        <Settings onBack={closeSettings} />
    {:else if currentView === "main"}
        <MainInterface {selectedFile} {password} onLogout={handleLogout} />
    {/if}

    {#if showCreateVaultModal}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="modal-overlay" on:click={closeCreateVaultModal}>
            <div class="modal-content" on:click|stopPropagation>
                <div class="modal-header">
                    <h2>Создать новое хранилище</h2>
                    <button class="modal-close" on:click={closeCreateVaultModal}>×</button>
                </div>
                
                <div class="modal-body">
                    <div class="form-group">
                        <label for="new-vault-name">Имя хранилища</label>
                        <input 
                            id="new-vault-name"
                            type="text"
                            bind:value={newVaultName}
                            placeholder="Моё главное хранилище"
                            class="input-field"
                        />
                    </div>
                    
                    <div class="form-group">
                        <label for="new-vault-password">Пароль</label>
                        <div class="password-wrapper">
                            <input 
                                id="new-vault-password"
                                type={showNewVaultPassword ? "text" : "password"}
                                bind:value={newVaultPassword}
                                placeholder="••••••••••••••••"
                                class="input-field"
                            />
                            <button 
                                type="button"
                                on:click={toggleNewVaultPasswordVisibility}
                                class="toggle-password-btn"
                                title={showNewVaultPassword ? "Скрыть пароль" : "Показать пароль"}
                            >
                                <img 
                                    src={showNewVaultPassword ? HideIcon : ShowIcon} 
                                    alt={showNewVaultPassword ? "Скрыть" : "Показать"} 
                                    class="icon"
                                />
                            </button>
                        </div>
                    </div>
                </div>
                
                <div class="modal-footer">
                    <button class="btn-secondary" on:click={closeCreateVaultModal}>Отмена</button>
                    <button 
                        on:click={handleCreateVault}
                        class="btn-primary"
                        disabled={isCreatingVault}
                    >
                        {#if isCreatingVault}
                            <div class="spinner small-spinner"></div> Создание...
                        {:else}
                            Создать хранилище
                        {/if}
                    </button>
                </div>
            </div>
        </div>
    {/if}


<style>
    .container {
        margin: 0;
        padding: 0;
        min-height: 100vh;
        box-sizing: border-box;
        background-color: var(--ctp-base);
        display: flex;
        align-items: center;
        justify-content: center;
    }
    .global-error {
        position: fixed;
        top: 1rem;
        left: 50%;
        transform: translateX(-50%);
        max-width: calc(100% - 2rem);
        width: max-content;
        padding: 0.75rem 1.5rem;
        color: var(--ctp-red);
        background-color: color-mix(in srgb, var(--ctp-red) 10%, transparent);
        border-radius: 12px;
        font-size: 0.95rem;
        font-weight: 500;
        border-left: 4px solid var(--ctp-red);
        box-shadow: 0 4px 12px color-mix(in srgb, var(--ctp-crust) 20%, transparent);
        animation: fadeIn 0.3s ease, slideDown 0.2s ease 0.3s forwards;
        z-index: 1000;
        backdrop-filter: blur(4px);
        transition: all 0.2s ease;
        cursor: default;
        text-align: center;
    }

    @keyframes fadeIn {
        from { 
            opacity: 0; 
            transform: translate(-50%, -10px);
        }
        to { 
            opacity: 1; 
            transform: translate(-50%, 0);
        }
    }

    @keyframes slideDown {
        from { margin-top: -10px; }
        to { margin-top: 0; }
    }

    .global-error:hover {
        box-shadow: 0 6px 16px color-mix(in srgb, var(--ctp-crust) 30%, transparent);
        transform: translateX(-50%) scale(1.02);
    }

    .card {
        background-color: var(--ctp-mantle);
        border-radius: 12px;
        padding: 2rem;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
        width: 100%;
        max-width: 400px;
        display: flex;
        flex-direction: column;
        gap: 1.2rem;
        position: absolute; 
    }

    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
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
        max-height: 90vh;
        overflow-y: auto;
        box-shadow: 0 20px 40px color-mix(in srgb, var(--ctp-crust) 30%, transparent);
    }

    .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1.5rem;
        border-bottom: 1px solid var(--ctp-surface0);
    }

    .modal-header h2 {
        margin: 0;
        color: var(--ctp-text);
    }

    .modal-close {
        background: none;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
        color: var(--ctp-subtext1);
        padding: 0;
        width: 32px;
        height: 32px;
        border-radius: 4px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .modal-close:hover {
        background-color: var(--ctp-surface0);
        color: var(--ctp-text);
    }

    .modal-body {
        padding: 1.5rem;
    }

    .modal-footer {
        display: flex;
        gap: 1rem;
        justify-content: flex-end;
        padding: 1.5rem;
        border-top: 1px solid var(--ctp-surface0);
    }

    .logos-container {
        display: flex;
        justify-content: center;
        width: 100%;
        max-width: 400px;
        margin-top: auto;
        padding: 1rem 0;
    }

    .logos {
        display: flex;
        gap: 0.8rem;
        opacity: 0.7;
        transition: opacity 0.3s ease;
        justify-content: center;
        flex-wrap: wrap;
    }

    .logos:hover {
        opacity: 1;
    }

    .logo-btn {
        background: none;
        border: none;
        cursor: pointer;
        padding: 0.5rem;
        border-radius: 8px;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: background-color 0.2s ease;
    }

    .logo-btn:hover {
        background-color: var(--ctp-surface0);
    }

    .logo-icon {
        width: 32px;
        height: 32px;
        object-fit: contain;
    }

    .login-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 0.5rem;
    }

    .header-buttons {
        display: flex;
        gap: 0.5rem;
    }

    h1 {
        margin: 0;
        font-size: 1.8rem;
        color: var(--ctp-text);
    }

    .settings-btn,
    .add-btn {
        background: none;
        border: none;
        cursor: pointer;
        padding: 0.5rem;
        border-radius: 50%;
        width: 2.5rem;
        height: 2.5rem;
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--ctp-text);
    }

    .settings-btn:hover,
    .add-btn:hover:not(:disabled) {
        background-color: var(--ctp-surface0);
    }

    .add-btn:disabled {
        cursor: not-allowed;
        opacity: 0.5;
    }

    .form-group {
        width: 100%;
        position: relative;
    }

    .select-with-refresh {
        display: flex;
        gap: 0.5rem;
        align-items: stretch;
    }

    .select-with-refresh .input-field {
        flex: 1;
    }

    .select-wrapper {
        position: relative;
        display: inline-flex;
        align-items: center;
        width: 100%;
        }

        /* Скрываем стандартную стрелку */
        select.input-field {
        -webkit-appearance: none;
        -moz-appearance: none;
        appearance: none;

        /* Занимаем всё пространство */
        width: 100%;
        padding: flex;
        padding-right: 32px; /* оставляем место для иконки */
        background-color: var(--ctp-surface0);
        color: var(--ctp-text);
        border: 1px solid var(--ctp-overlay0);
        border-radius: 6px;
        outline: none;
        cursor: pointer;
        }

        select.input-field:hover {
        border-color: var(--ctp-overlay0);
        }

        select.input-field:focus {
        border-color: var(--ctp-blue);
        box-shadow: 0 0 0 2px rgba(137, 180, 250, 0.2);
        }

        /* Иконка стрелки */
        .select-arrow {
        position: absolute;
        right: 12px;
        top: 50%;
        transform: translateY(-50%);
        pointer-events: none; /* чтобы не мешала кликам по select */
        width: 16px;
        height: 16px;
        /* Класс .icon уже должен задавать нужный цвет через filter или opacity */
    }

    .refresh-btn {
        background: var(--ctp-surface0);
        border: 1px solid var(--ctp-overlay0);
        border-radius: 8px;
        cursor: pointer;
        padding: 0.75rem;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 3rem;
        transition: background-color 0.25s;
    }

    .refresh-btn:hover:not(:disabled) {
        background: var(--ctp-surface1);
    }

    .refresh-btn:disabled {
        cursor: not-allowed;
        opacity: 0.7;
    }

    .spinner {
        width: 16px;
        height: 16px;
        border: 2px solid transparent;
        border-top: 2px solid var(--ctp-text);
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }

    .input-field {
        width: 100%;
        border-radius: 8px;
        border: 1px solid var(--ctp-overlay0);
        padding: 0.8rem 1rem;
        font-size: 1rem;
        font-weight: 500;
        font-family: inherit;
        color: var(--ctp-text);
        background-color: var(--ctp-surface0);
        transition: border-color 0.25s;
        box-sizing: border-box;
        appearance: none;
    }

    select.input-field {
        padding-right: 2.5rem;
    }

    .input-field:focus {
        outline: none;
        border-color: var(--ctp-blue);
        box-shadow: 0 0 0 2px rgba(136, 57, 239, 0.2);
    }

    .password-wrapper {
        position: relative;
        display: flex;
        align-items: center;
    }

    .toggle-btn {
        position: absolute;
        right: 0.8rem;
        background: none;
        border: none;
        cursor: pointer;
        padding: 0;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .toggle-btn:hover {
        color: var(--ctp-blue);
    }

    .action-btn {
        width: 100%;
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.8rem 1.2rem;
        font-size: 1rem;
        font-weight: 500;
        font-family: inherit;
        transition: background-color 0.25s;
        cursor: pointer;
        color: var(--ctp-base);
        background-color: var(--ctp-blue);
    }

    .action-btn:hover {
        background-color: var(--ctp-lavender);
    }

    .logos {
        position: absolute;
        bottom: 0.5rem;
        left: 0.5rem;
        display: flex;
        gap: 0.5rem;
        opacity: 0.7;
        transition: opacity 0.3s ease;
    }
    .icon {
        width: 20px;
        height: 20px;
        object-fit: contain;
    }

</style>
