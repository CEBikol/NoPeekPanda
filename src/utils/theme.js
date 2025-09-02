export const themes = {
  latte: {
    name: "Latte",
    colors: {
      base: "#eff1f5",
      mantle: "#e6e9ef",
      crust: "#dce0e8",
      text: "#4c4f69",
      subtext1: "#5c5f77",
      subtext0: "#6c6f85",
      overlay2: "#7c7f93",
      overlay1: "#8c8fa1",
      overlay0: "#9ca0b0",
      surface2: "#acb0be",
      surface1: "#bcc0cc",
      surface0: "#ccd0da",
      blue: "#1e66f5",
      lavender: "#7287fd",
      sapphire: "#209fb5",
      sky: "#04a5e5",
      teal: "#179299",
      green: "#40a02b",
      yellow: "#df8e1d",
      peach: "#fe640b",
      maroon: "#e64553",
      red: "#d20f39",
      mauve: "#8839ef",
      pink: "#ea76cb",
      flamingo: "#dd7878",
      rosewater: "#dc8a78",
    },
  },
  frappe: {
    name: "Frappé",
    colors: {
      base: "#303446",
      mantle: "#292c3c",
      crust: "#232634",
      text: "#c6d0f5",
      subtext1: "#b5bfe2",
      subtext0: "#a5adce",
      overlay2: "#949cbb",
      overlay1: "#838ba7",
      overlay0: "#737994",
      surface2: "#626880",
      surface1: "#51576d",
      surface0: "#414559",
      blue: "#8caaee",
      lavender: "#babbf1",
      sapphire: "#85c1dc",
      sky: "#99d1db",
      teal: "#81c8be",
      green: "#a6d189",
      yellow: "#e5c890",
      peach: "#ef9f76",
      maroon: "#ea999c",
      red: "#e78284",
      mauve: "#ca9ee6",
      pink: "#f4b8e4",
      flamingo: "#eebebe",
      rosewater: "#f2d5cf",
    },
  },
  macchiato: {
    name: "Macchiato",
    colors: {
      base: "#24273a",
      mantle: "#1e2030",
      crust: "#181926",
      text: "#cad3f5",
      subtext1: "#b8c0e0",
      subtext0: "#a5adcb",
      overlay2: "#939ab7",
      overlay1: "#8087a2",
      overlay0: "#6e738d",
      surface2: "#5b6078",
      surface1: "#494d64",
      surface0: "#363a4f",
      blue: "#8aadf4",
      lavender: "#b7bdf8",
      sapphire: "#7dc4e4",
      sky: "#91d7e3",
      teal: "#8bd5ca",
      green: "#a6da95",
      yellow: "#eed49f",
      peach: "#f5a97f",
      maroon: "#ee99a0",
      red: "#ed8796",
      mauve: "#c6a0f6",
      pink: "#f5bde6",
      flamingo: "#f0c6c6",
      rosewater: "#f4dbd6",
    },
  },
  mocha: {
    name: "Mocha",
    colors: {
      base: "#1e1e2e",
      mantle: "#181825",
      crust: "#11111b",
      text: "#cdd6f4",
      subtext1: "#bac2de",
      subtext0: "#a6adc8",
      overlay2: "#9399b2",
      overlay1: "#7f849c",
      overlay0: "#6c7086",
      surface2: "#585b70",
      surface1: "#45475a",
      surface0: "#313244",
      blue: "#89b4fa",
      lavender: "#b4befe",
      sapphire: "#74c7ec",
      sky: "#89dceb",
      teal: "#94e2d5",
      green: "#a6e3a1",
      yellow: "#f9e2af",
      peach: "#fab387",
      maroon: "#eba0ac",
      red: "#f38ba8",
      mauve: "#cba6f7",
      pink: "#f5c2e7",
      flamingo: "#f2cdcd",
      rosewater: "#f5e0dc",
    },
  },
};

export function applyTheme(themeName) {
  const theme = themes[themeName] || themes.mocha;

  for (const [colorName, colorValue] of Object.entries(theme.colors)) {
    document.documentElement.style.setProperty(
      `--ctp-${colorName}`,
      colorValue,
    );
  }

  document.documentElement.setAttribute("data-theme", themeName);

  localStorage.setItem("selectedTheme", themeName);
}

// Загрузка темы из настроек при запуске приложения
export function loadTheme() {
  // Сначала пробуем загрузить из localStorage для быстрой загрузки
  const savedTheme = localStorage.getItem("selectedTheme");
  if (savedTheme && themes[savedTheme]) {
    applyTheme(savedTheme);
    return savedTheme;
  }

  // Если в localStorage нет, используем тему по умолчанию
  applyTheme("mocha");
  return "mocha";
}
