<script>

    const STORAGE_KEY = "theme";
    const DARK_PREFERENCE = '(prefers-color-scheme: dark)';

    const THEMES = {
        DARK: 'dark',
        LIGHT: 'light',
    };

    function preferesDarkThemes() {
        window.matchMedia(DARK_PREFERENCE).matches;
    }

	function toggleTheme() {
        const storedTheme = localStorage.getItem(STORAGE_KEY);

        if(storedTheme){
            localStorage.removeItem(STORAGE_KEY)
        }
        
        localStorage.setItem(STORAGE_KEY)
        
    }
</script>

<input type="checkbox" name="Dark Mode Toggle" id="DarkModeToggle" on:click={toggleTheme} />
