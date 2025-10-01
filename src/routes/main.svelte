<script>
// @ts-nocheck

    const ShowIcon = "/icons/visible.png";
    const HideIcon = "/icons/hide.png";
    const DeleteIcon = "/icons/delete.png";
    const CopyIcon = "/icons/copy.png";
    const SiteIcon = "/icons/site.png";
    const AddIcon = "/icons/add.png";
    const RefreshIcon = "/icons/refresh.png";
    const SettingsIcon = "/icons/settings.png";
    const OtpIcon = "/icons/otp.png"
    const PasswordIcon = "/icons/password.png"

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
    
    // Новые состояния для навигации и модальных окон
    let activeSection = "passwords"; // 'passwords' или 'totp'
    let showAddModal = false;

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

    function switchSection(section) {
        if (section === 'totp') {
            alert('Функция TOTP ещё не реализована');
            return;
        }
        activeSection = section;
    }

    function openAddModal() {
        showAddModal = true;
    }

    function closeAddModal() {
        showAddModal = false;
        // Сбрасываем форму
        newSite = "";
        newLogin = "";
        newPassword = "";
        showNewPassword = false;
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
            
            // Закрываем модальное окно
            closeAddModal();
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
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="global-error" on:click={() => error = ''}>
            {error}
        </div>
    {/if}

    <div class="content-layout">
        <nav class="sidebar">
            <div class="nav-section">
                <h3>Разделы</h3>
                <div class="nav-items">
                    <button 
                        class:nav-item-active={activeSection === 'passwords'}
                        class="nav-item"
                        on:click={() => switchSection('passwords')}
                    >
                        <img src={PasswordIcon} alt="Пароли" class="nav-icon icon" />
                        <span>Пароли</span>
                    </button>
                    <button 
                        class:nav-item-active={activeSection === 'totp'}
                        class="nav-item"
                        on:click={() => switchSection('totp')}
                    >
                        <img src={OtpIcon} alt="TOTP" class="nav-icon icon" />
                        <span>TOTP</span>
                    </button>
                </div>
            </div>
            
            <div class="sidebar-footer">
                <div class="stats">
                    <span class="stat-count">{services.length}</span>
                    <span>сервисов</span>
                </div>
            </div>
        </nav>

        <main class="main-content">
            {#if activeSection === 'passwords'}
                <div class="content-header">
                    <h2>Сохраненные пароли</h2>
                    <div class="content-actions">
                        <button 
                            on:click={loadServices} 
                            class="icon-btn refresh-btn"
                            title="Обновить список"
                            disabled={isLoading}
                        >
                            {#if isLoading}
                                <div class="spinner small-spinner"></div>
                            {:else}
                                <img src={RefreshIcon} alt="Обновить" class="icon" />
                            {/if}
                        </button>
                    </div>
                </div>

                <div class="cards-grid">
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <div class="add-card" on:click={openAddModal}>
                        <div class="add-card-content">
                            <div class="add-icon">
                                <img src={AddIcon} alt="Добавить" />
                            </div>
                            <span class="add-text">Добавить пароль</span>
                        </div>
                    </div>

                    <!-- Карточки существующих сервисов -->
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

            {:else if activeSection === 'totp'}
                <div class="empty-section">
                    <div class="empty-icon">⏱️</div>
                    <h2>TOTP пока не доступен</h2>
                    <p>Эта функция находится в разработке и будет добавлена в ближайшем обновлении.</p>
                </div>
            {/if}
        </main>
    </div>
</div>

{#if showAddModal}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="modal-overlay" on:click={closeAddModal}>
        <div class="modal-content" on:click|stopPropagation>
            <div class="modal-header">
                <h2>Добавить новый пароль</h2>
                <button class="modal-close" on:click={closeAddModal}>×</button>
            </div>
            
            <div class="modal-body">
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
            
            <div class="modal-footer">
                <button class="btn-secondary" on:click={closeAddModal}>Отмена</button>
                <button 
                    on:click={addNewPassword} 
                    class="btn-primary"
                    disabled={addingPassword}
                >
                    {#if addingPassword}
                        <div class="spinner small-spinner"></div> Добавление...
                    {:else}
                        Добавить пароль
                    {/if}
                </button>
            </div>
        </div>
    </div>
{/if}

<style>
    .main-interface {

        margin: 0 !important;
        padding: 0 !important;
        min-height: 100vh !important;
        box-sizing: border-box !important;
        background-color: var(--ctp-base) !important;
        display: block !important;
        align-items: normal !important;
        justify-content: normal !important;
        

        color: var(--ctp-text);
        padding: 0; 
        max-width: none;
        width: 100%;
    }

    .content-layout {
        display: flex;
        flex: 1;
        overflow: hidden;
        width: 100%;
        height: calc(100vh - 80px);
    }

    header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1.5rem 2rem;
        background-color: var(--ctp-mantle);
        border-bottom: 1px solid var(--ctp-surface0);
    }

    .header-actions {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .file-indicator {
        background-color: var(--ctp-surface0);
        padding: 0.4rem 1rem;
        border-radius: 20px;
        font-size: 0.9rem;
        color: var(--ctp-subtext1);
    }

    .logout-btn {
        padding: 0.6rem 1.2rem;
        background-color: var(--ctp-red);
        color: var(--ctp-base);
        border: none;
        border-radius: 8px;
        cursor: pointer;
        font-family: inherit;
        font-weight: 500;
        transition: all 0.2s ease;
    }

    .logout-btn:hover {
        background-color: var(--ctp-maroon);
        transform: translateY(-1px);
        box-shadow: 0 4px 8px color-mix(in srgb, var(--ctp-crust) 15%, transparent);
    }

    /* Sidebar */
    .sidebar {
        width: 250px;
        background-color: var(--ctp-mantle);
        border-right: 1px solid var(--ctp-surface0);
        padding: 1.5rem;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }

    .nav-section h3 {
        margin: 0 0 1rem 0;
        font-size: 0.9rem;
        color: var(--ctp-subtext1);
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .nav-items {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .nav-item {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.75rem 1rem;
        background: none;
        border: none;
        border-radius: 8px;
        color: var(--ctp-subtext1);
        cursor: pointer;
        transition: all 0.2s ease;
        font-family: inherit;
        font-size: 1rem;
        text-align: left;
    }

    .nav-item:hover {
        background-color: var(--ctp-surface0);
        color: var(--ctp-text);
    }

    .nav-item-active {
        background-color: var(--ctp-blue);
        color: var(--ctp-base) !important;
    }

    .nav-icon {
        font-size: 1.2rem;
        width: 24px;
        text-align: center;
    }

    .sidebar-footer {
        margin-top: auto;
    }

    .stats {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 1rem;
        background-color: var(--ctp-surface0);
        border-radius: 8px;
        font-size: 0.9rem;
        color: var(--ctp-subtext1);
    }

    .stat-count {
        font-weight: bold;
        color: var(--ctp-blue);
        font-size: 1.1rem;
    }

    /* Main Content */
    .main-content {
        flex: 1;
        padding: 2rem;
        overflow-y: auto;
        background-color: var(--ctp-base);
    }

    .content-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
    }

    .content-header h2 {
        margin: 0;
        color: var(--ctp-text);
        font-size: 1.5rem;
    }

    .content-actions {
        display: flex;
        gap: 0.5rem;
    }

    /* Cards Grid */
    .cards-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 1.5rem;
        align-items: start;
    }

    .add-card {
        background-color: var(--ctp-mantle);
        border: 2px dashed var(--ctp-overlay0);
        border-radius: 12px;
        padding: 2rem;
        cursor: pointer;
        transition: all 0.3s ease;
        display: flex;
        align-items: center;
        justify-content: center;
        min-height: 180px;
    }

    .add-card:hover {
        border-color: var(--ctp-blue);
        background-color: var(--ctp-surface0);
        transform: translateY(-2px);
        box-shadow: 0 4px 12px color-mix(in srgb, var(--ctp-crust) 10%, transparent);
    }

    .add-card-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        color: var(--ctp-subtext1);
    }

    .add-icon {
        width: 48px;
        height: 48px;
        background-color: var(--ctp-surface0);
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.2s ease;
    }

    .add-card:hover .add-icon {
        background-color: var(--ctp-blue);
    }

    .add-icon img {
        width: 24px;
        height: 24px;
        filter: brightness(0.8);
    }

    .add-card:hover .add-icon img {
        filter: brightness(1);
    }

    .add-text {
        font-weight: 500;
        font-size: 1.1rem;
    }

    .password-card {
        background-color: var(--ctp-mantle);
        border-radius: 12px;
        overflow: hidden;
        box-shadow: 0 4px 6px color-mix(in srgb, var(--ctp-crust) 5%, transparent);
        transition: all 0.3s ease;
        display: flex;
        flex-direction: column;
        min-height: 180px;
    }

    .password-card:hover {
        transform: translateY(-2px);
        box-shadow: 0 6px 12px color-mix(in srgb, var(--ctp-crust) 10%, transparent);
    }

    .card-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1.25rem;
        background-color: var(--ctp-surface0);
        border-bottom: 1px solid var(--ctp-surface1);
    }

    .site-info {
        display: flex;
        align-items: center;
        gap: 0.8rem;
    }

    .site-icon {
        width: 40px;
        height: 40px;
        background-color: var(--ctp-surface1);
        border-radius: 10px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-weight: bold;
        color: var(--ctp-text);
        font-size: 1.2rem;
        flex-shrink: 0;
    }

    .site-details {
        display: flex;
        flex-direction: column;
        min-width: 0;
        flex: 1;
    }

    .site-name {
        margin: 0;
        font-size: 1.1rem;
        color: var(--ctp-text);
        font-weight: 600;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .login {
        margin: 0.25rem 0 0 0;
        font-size: 0.9rem;
        color: var(--ctp-subtext1);
        font-weight: 400;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .card-actions {
        display: flex;
        gap: 0.25rem;
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

    .refresh-btn:hover {
        color: var(--ctp-blue);
    }

    .card-body {
        padding: 1.25rem;
        flex-grow: 1;
        display: flex;
        align-items: center;
    }

    .password-row {
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 100%;
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
        flex-shrink: 0;
    }

    .empty-section {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 60vh;
        text-align: center;
        color: var(--ctp-subtext1);
    }

    .empty-icon {
        font-size: 4rem;
        margin-bottom: 1.5rem;
        opacity: 0.5;
    }

    .empty-section h2 {
        margin-bottom: 1rem;
        color: var(--ctp-text);
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

    .btn-primary {
        padding: 0.75rem 1.5rem;
        background-color: var(--ctp-blue);
        color: var(--ctp-base);
        border: none;
        border-radius: 8px;
        cursor: pointer;
        font-family: inherit;
        font-weight: 500;
        transition: all 0.2s ease;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .btn-primary:hover:not(:disabled) {
        background-color: var(--ctp-lavender);
        transform: translateY(-1px);
    }

    .btn-primary:disabled {
        opacity: 0.6;
        cursor: not-allowed;
        transform: none;
    }

    .btn-secondary {
        padding: 0.75rem 1.5rem;
        background-color: var(--ctp-surface0);
        color: var(--ctp-text);
        border: none;
        border-radius: 8px;
        cursor: pointer;
        font-family: inherit;
        font-weight: 500;
        transition: all 0.2s ease;
    }

    .btn-secondary:hover {
        background-color: var(--ctp-surface1);
    }

    /* Form Styles */
    .form-group {
        margin-bottom: 1.5rem;
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

    /* Spinner */
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

    /* Responsive */
    @media (max-width: 768px) {
        .content-layout {
            flex-direction: column;
        }
        
        .sidebar {
            width: 100%;
            border-right: none;
            border-bottom: 1px solid var(--ctp-surface0);
        }
        
        /* Стили для иконок навигации */
        .nav-icon {
            width: 20px;
            height: 20px;
            object-fit: contain;
            filter: brightness(0.8);
            transition: all 0.2s ease;
        }

        .nav-item-active .nav-icon {
            filter: brightness(1);
        }

        .empty-section-icon {
            width: 64px;
            height: 64px;
            object-fit: contain;
            opacity: 0.5;
        }

        .nav-item {
            display: flex;
            align-items: center;
            gap: 0.75rem;
            padding: 0.75rem 1rem;
            background: none;
            border: none;
            border-radius: 8px;
            color: var(--ctp-subtext1);
            cursor: pointer;
            transition: all 0.2s ease;
            font-family: inherit;
            font-size: 1rem;
            text-align: left;
        }

        .nav-item:hover .nav-icon {
            filter: brightness(1);
            transform: scale(1.05);
        }

        .nav-item-active {
            background-color: var(--ctp-blue);
            color: var(--ctp-base) !important;
        }

        .nav-item-active .nav-icon {
            filter: brightness(1);
        }
        
        .cards-grid {
            grid-template-columns: 1fr;
        }
        
        .main-content {
            padding: 1rem;
        }
        
        header {
            padding: 1rem;
        }
    }
</style>