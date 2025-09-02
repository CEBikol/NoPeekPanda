<script>
    const ShowIcon = "/icons/visible.png";
    const HideIcon = "/icons/hide.png";
    const DeleteIcon = "/icons/delete.png";
    const CopyIcon = "/icons/copy.png";
    const SiteIcon = "/icons/site.png";
    const AddIcon = "/icons/add.png";
    const RefreshIcon = "/icons/refresh.png";
    const SettingsIcon = "/icons/settings.png";

    export let selectedFile = "";
    export let onLogout = () => {};
    
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let error = "";
    let newSite = "";
    let newLogin = "";
    let newPassword = "";
    let showNewPassword = false;
    let addingPassword = false;
    let services = [];
    let viewingPasswords = new Map();
    let copyingStates = new Map();
    let isLoading = true;

    // Загружаем сервисы при монтировании компонента
    onMount(async () => {
        console.log("MainInterface mounted, loading services...");
        await loadServices();
    });

    async function loadServices() {
        try {
            console.log("Загрузка сервисов...");
            services = await invoke("list_services");
            console.log("Сервисы загружены:", services);
            error = "";
            isLoading = false;
        } catch (e) {
            console.error("Ошибка загрузки сервисов:", e);
            error = "Не удалось загрузить список сервисов";
            isLoading = false;
        }
    }

    async function handlePasswordView(service) {
        try {
            // Если пароль уже показывается, скрываем его
            if (viewingPasswords.has(service.id)) {
                const { timer } = viewingPasswords.get(service.id);
                clearTimeout(timer);
                viewingPasswords.delete(service.id);
                viewingPasswords = new Map(viewingPasswords);
                return;
            }
            
            viewingPasswords.set(service.id, { loading: true });
            viewingPasswords = new Map(viewingPasswords);
            
            const passwordEntry = await invoke("get_password", { id: service.id });
            
            const timer = setTimeout(() => {
                viewingPasswords.delete(service.id);
                viewingPasswords = new Map(viewingPasswords);
            }, 10000);
            
            viewingPasswords.set(service.id, {
                password: passwordEntry.password,
                timer: timer
            });
            
            viewingPasswords = new Map(viewingPasswords);
        } catch (e) {
            error = "Не удалось получить пароль";
            console.error("Ошибка получения пароля:", e);
            viewingPasswords.delete(service.id);
            viewingPasswords = new Map(viewingPasswords);
        }
    }

    async function copyToClipboard(text, id) {
        copyingStates.set(id, true);
        setTimeout(() => copyingStates.delete(id), 2000);
        copyingStates = new Map(copyingStates);
        
        try {
            await navigator.clipboard.writeText(text);
        } catch (e) {
            error = "Ошибка копирования в буфер обмена";
            console.error("Ошибка копирования:", e);
        }
    }

    async function addNewPassword() {
        if (!newSite || !newLogin || !newPassword) {
            error = "Все поля должны быть заполнены";
            return;
        }

        try {
            addingPassword = true;
            
            await invoke("add_password", {
                site: newSite,
                login: newLogin,
                password: newPassword
            });
            
            // Немедленно очищаем пароль из памяти
            newPassword = "";
            showNewPassword = false;
            
            // Обновляем список сервисов
            await loadServices();
            
            // Сброс формы
            newSite = "";
            newLogin = "";
            error = "";
        } catch (e) {
            error = "Не удалось добавить пароль";
            console.error("Ошибка добавления пароля:", e);
        } finally {
            addingPassword = false;
        }
    }

    async function deleteService(service) {
        if (confirm("Вы уверены, что хотите удалить этот пароль?")) {
            try {
                await invoke("delete_password", { id: service.id });
                
                // Обновляем список сервисов
                services = services.filter(s => s.id !== service.id);
                
                // Очищаем состояние просмотра пароля
                if (viewingPasswords.has(service.id)) {
                    const { timer } = viewingPasswords.get(service.id);
                    clearTimeout(timer);
                    viewingPasswords.delete(service.id);
                    viewingPasswords = new Map(viewingPasswords);
                }
                
                error = "";
            } catch (e) {
                error = "Не удалось удалить пароль";
                console.error("Ошибка удаления пароля:", e);
            }
        }
    }

    function toggleNewPasswordVisibility() {
        showNewPassword = !showNewPassword;
    }
</script>



<div class="main-interface">
    <header>
        <h1>Мое хранилище</h1>
        <div class="header-actions">
            <span class="file-indicator">Файл: {selectedFile}</span>
            <button on:click={onLogout} class="logout-btn">Выйти</button>
        </div>
    </header>

    {#if error}
        <div class="global-error" on:click={() => error = ''}>
            {error}
        </div>
    {/if}

    <div class="content">
        <!-- Форма добавления пароля -->
        <div class="add-form-container permanent-form">
            <div class="form-header">
                <h2>Добавить новый пароль</h2>
            </div>
            
            <div class="form-grid">
                <div class="form-group">
                    <label for="site">Сайт или приложение</label>
                    <div class="input-with-icon">
                        <img src={SiteIcon} alt="Сайт" class="input-icon icon" />
                        <input 
                            id="site" 
                            type="text" 
                            bind:value={newSite}
                            placeholder="example.com"
                            class="input-field"
                            on:keypress={(e) => e.key === "Enter" && addNewPassword()}
                        />
                    </div>
                </div>
                
                <div class="form-group">
                    <label for="login">Логин</label>
                    <input 
                        id="login" 
                        type="text" 
                        bind:value={newLogin}
                        placeholder="username@example.com"
                        class="input-field"
                        on:keypress={(e) => e.key === "Enter" && addNewPassword()}
                    />
                </div>
                
                <div class="form-group">
                    <label for="password">Пароль</label>
                    <div class="password-wrapper">
                        <input 
                            id="password" 
                            type={showNewPassword ? "text" : "password"} 
                            bind:value={newPassword}
                            placeholder="••••••••••••••••"
                            class="input-field"
                            on:keypress={(e) => e.key === "Enter" && addNewPassword()}
                        />
                        <button 
                            type="button" 
                            on:click={toggleNewPasswordVisibility} 
                            class="toggle-password-btn"
                            title={showNewPassword ? "Скрыть пароль" : "Показать пароль"}
                        >
                            <img 
                                src={showNewPassword ? HideIcon : ShowIcon} 
                                alt={showNewPassword ? "Скрыть" : "Показать"} 
                                class="icon"
                            />
                        </button>
                    </div>
                </div>
            </div>
            
            <button 
                on:click={addNewPassword} 
                class="action-btn"
                disabled={addingPassword}
            >
                {#if addingPassword}
                    <div class="spinner small-spinner"></div> Добавление...
                {:else}
                    <img src={AddIcon} alt="Добавить" class="btn-icon" /> Добавить пароль
                {/if}
            </button>
        </div>

        {#if services.length === 0}
            <div class="empty-state">
                <h2>Нет сохраненных сервисов</h2>
                <p>Добавьте первый сервис с помощью формы выше</p>
            </div>
        {:else}
            <div class="password-grid">
                {#each services as service}
                    <div class="password-card">
                        <div class="card-header">
                            <div class="site-info">
                                <div class="site-icon">
                                    {service.site.charAt(0).toUpperCase()}
                                </div>
                                <div class="site-details">
                                    <h3 class="site-name">{service.site}</h3>
                                    <p class="login">{service.login}</p>
                                </div>
                            </div>
                            <div class="card-actions">
                                <button 
                                    on:click={() => deleteService(service)} 
                                    class="icon-btn delete-btn"
                                    title="Удалить сервис"
                                >
                                    <img src={DeleteIcon} alt="Удалить" class="icon" />
                                </button>
                            </div>
                        </div>
                        
                        <div class="card-body">
                            <div class="password-row">
                                <div class="password-display">
                                    {#if viewingPasswords.has(service.id)}
                                        {#if viewingPasswords.get(service.id).loading}
                                            <div class="spinner small-spinner"></div>
                                        {:else}
                                            <span class="password-text">{viewingPasswords.get(service.id).password}</span>
                                        {/if}
                                    {:else}
                                        <span class="password-masked">••••••••••</span>
                                    {/if}
                                </div>
                                
                                <div class="password-actions">
                                    <button 
                                        on:click={() => handlePasswordView(service)}
                                        class="icon-btn view-btn"
                                        title={viewingPasswords.has(service.id) ? "Скрыть пароль" : "Показать пароль"}
                                    >
                                        <img 
                                            src={viewingPasswords.has(service.id) ? HideIcon : ShowIcon} 
                                            alt={viewingPasswords.has(service.id) ? "Скрыть" : "Показать"} 
                                            class="icon" 
                                        />
                                    </button>
                                    
                                    {#if viewingPasswords.has(service.id) && !viewingPasswords.get(service.id).loading}
                                        <button 
                                            on:click={() => copyToClipboard(viewingPasswords.get(service.id).password, service.id)}
                                            class="icon-btn copy-btn"
                                            title="Скопировать пароль"
                                        >
                                            <img 
                                                src={CopyIcon} 
                                                alt="Копировать" 
                                                class="icon" 
                                            />
                                        </button>
                                    {/if}
                                </div>
                            </div>
                        </div>
                    </div>
                {/each}
            </div>
        {/if}
    </div>
</div>

<style>
    .main-interface {
        color: var(--ctp-text);
        padding: 2rem;
        max-width: 1200px;
        margin: 0 auto;
        background-color: var(--ctp-base);
        min-height: 100vh;
        box-sizing: border-box;
    }

    header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
        padding-bottom: 1rem;
        border-bottom: 1px solid var(--ctp-surface0);
    }

    .header-actions {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .file-indicator {
        background-color: var(--ctp-surface0);
        padding: 0.3rem 0.8rem;
        border-radius: 20px;
        font-size: 0.9rem;
        color: var(--ctp-subtext1);
    }

    .logout-btn {
        padding: 0.5rem 1rem;
        background-color: var(--ctp-red);
        color: var(--ctp-base);
        border: none;
        border-radius: 6px;
        cursor: pointer;
        font-family: inherit;
        transition: all 0.2s ease;
    }

    .logout-btn:hover {
        background-color: var(--ctp-maroon);
        transform: translateY(-2px);
        box-shadow: 0 4px 8px color-mix(in srgb, var(--ctp-crust) 15%, transparent);
    }

    .content {
        color: var(--ctp-text);
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
        box-shadow: 0 4px 12px color-mix(in srgb, var(--ctp-crust) 10%, transparent);
        animation: fadeIn 0.3s ease, slideDown 0.2s ease 0.3s forwards;
        z-index: 1000;
        backdrop-filter: blur(4px);
        transition: all 0.2s ease;
        cursor: pointer;
        text-align: center;
        user-select: none;
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

    .add-form-container {
        background-color: var(--ctp-mantle);
        border-radius: 12px;
        padding: 1.5rem;
        margin-bottom: 2rem;
        box-shadow: 0 4px 6px color-mix(in srgb, var(--ctp-crust) 5%, transparent);
    }

    .form-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1.5rem;
    }

    .form-grid {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 1.2rem;
        margin-bottom: 1.5rem;
    }

    @media (max-width: 768px) {
        .form-grid {
            grid-template-columns: 1fr;
        }
    }

    .form-group {
        margin-bottom: 0;
    }

    label {
        display: block;
        margin-bottom: 0.5rem;
        color: var(--ctp-subtext1);
        font-weight: 500;
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
        transition: all 0.25s;
        box-sizing: border-box;
        appearance: none;
    }

    .input-field:focus {
        outline: none;
        border-color: var(--ctp-blue);
        box-shadow: 0 0 0 2px color-mix(in srgb, var(--ctp-blue) 20%, transparent);
    }

    /* Стили для поля с иконкой */
    .input-with-icon {
        position: relative;
    }

    .input-icon {
        position: absolute;
        left: 10px;
        top: 50%;
        transform: translateY(-50%);
        width: 20px;
        height: 20px;
        filter: brightness(0.8);
    }

    .input-with-icon .input-field {
        padding-left: 40px;
    }

    /* Стили для поля пароля с кнопкой переключения видимости */
    .password-wrapper {
        position: relative;
        display: flex;
        align-items: center;
    }

    .password-wrapper .input-field {
        padding-right: 45px;
    }

    .toggle-password-btn {
        position: absolute;
        right: 10px;
        top: 50%;
        transform: translateY(-50%);
        background: none;
        border: none;
        cursor: pointer;
        padding: 5px;
        border-radius: 4px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--ctp-subtext0);
        transition: all 0.2s ease;
    }

    .toggle-password-btn:hover {
        background-color: var(--ctp-surface1);
        color: var(--ctp-text);
    }

    .icon {
        width: 20px;
        height: 20px;
        object-fit: contain;
    }

    .action-btn {
        width: 100%;
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.8rem 1.2rem;
        font-size: 1rem;
        font-weight: 500;
        font-family: inherit;
        transition: all 0.25s;
        cursor: pointer;
        color: var(--ctp-base);
        background-color: var(--ctp-blue);
        box-shadow: 0 2px 4px color-mix(in srgb, var(--ctp-crust) 5%, transparent);
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
    }

    .action-btn:hover {
        background-color: var(--ctp-lavender);
        transform: translateY(-1px);
        box-shadow: 0 4px 6px color-mix(in srgb, var(--ctp-crust) 10%, transparent);
    }

    .action-btn:disabled {
        background-color: var(--ctp-surface1);
        cursor: not-allowed;
        transform: none;
        box-shadow: none;
        color: var(--ctp-overlay1);
    }

    .btn-icon {
        width: 18px;
        height: 18px;
        object-fit: contain;
    }

    .password-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
        gap: 1.5rem;
        margin-top: 1rem;
    }

    .password-card {
        background-color: var(--ctp-mantle);
        border-radius: 12px;
        overflow: hidden;
        box-shadow: 0 4px 6px color-mix(in srgb, var(--ctp-crust) 5%, transparent);
        transition: all 0.3s ease;
        display: flex;
        flex-direction: column;
    }

    .password-card:hover {
        transform: translateY(-3px);
        box-shadow: 0 6px 12px color-mix(in srgb, var(--ctp-crust) 10%, transparent);
    }

    .card-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem;
        background-color: var(--ctp-surface0);
        border-bottom: 1px solid var(--ctp-surface1);
    }

    .site-info {
        display: flex;
        align-items: center;
        gap: 0.8rem;
    }

    .site-icon {
        width: 36px;
        height: 36px;
        background-color: var(--ctp-surface1);
        border-radius: 8px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-weight: bold;
        color: var(--ctp-text);
        font-size: 1.2rem;
    }

    .site-details {
        display: flex;
        flex-direction: column;
    }

    .site-name {
        margin: 0;
        font-size: 1.1rem;
        color: var(--ctp-text);
        font-weight: 500;
    }

    .login {
        margin: 0;
        font-size: 0.9rem;
        color: var(--ctp-subtext1);
        font-weight: 400;
    }

    .card-actions {
        display: flex;
        gap: 0.5rem;
    }

    .icon-btn {
        background: none;
        border: none;
        cursor: pointer;
        padding: 0.5rem;
        border-radius: 6px;
        color: var(--ctp-subtext1);
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.2s ease;
    }

    .icon-btn:hover {
        background-color: var(--ctp-surface1);
        color: var(--ctp-text);
    }

    .view-btn:hover {
        color: var(--ctp-mauve);
    }

    .copy-btn:hover {
        color: var(--ctp-green);
    }

    .delete-btn:hover {
        color: var(--ctp-red);
    }

    .card-body {
        padding: 1rem;
        flex-grow: 1;
    }

    .password-row {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .password-display {
        flex-grow: 1;
        overflow: hidden;
        min-height: 24px;
        display: flex;
        align-items: center;
    }

    .password-text {
        font-family: monospace;
        font-size: 0.95rem;
        color: var(--ctp-text);
        word-break: break-all;
    }

    .password-masked {
        font-family: monospace;
        font-size: 1.2rem;
        color: var(--ctp-overlay2);
        letter-spacing: 3px;
    }

    .password-actions {
        display: flex;
        gap: 0.5rem;
        margin-left: 0.8rem;
    }

    .spinner {
        width: 32px;
        height: 32px;
        border: 3px solid transparent;
        border-top: 3px solid var(--ctp-blue);
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    .small-spinner {
        width: 16px;
        height: 16px;
        border-width: 2px;
    }

    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }

    .empty-state {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        min-height: 300px;
        text-align: center;
        padding: 2rem;
        color: var(--ctp-subtext1);
    }

    .empty-state h2 {
        margin-bottom: 1rem;
        color: var(--ctp-text);
    }
</style>
