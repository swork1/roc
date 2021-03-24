pub const SEARCH_JS: &str = r#"
(() => {
  let sidebar = document.getElementById("sidebar-nav");
  let searchBox = document.getElementById("module-search");

  function search() {
    let text = searchBox.value.toLowerCase(); // Search is case-insensitive.

    if (text === "") {
      // Un-hide everything
      sidebar.querySelectorAll(".sidebar-entry a").forEach((entry) => entry.classList.remove("hidden"));

      // Re-hide all the sub-entries except for those of the current module
      let currentModuleName = document.querySelector('.module-name').textContent;

      sidebar.querySelectorAll(".sidebar-entry").forEach((entry) => {
        let entryName = entry.querySelector('.sidebar-module-link').textContent;
        if (currentModuleName === entryName) return;
        entry.querySelectorAll(".sidebar-sub-entries a").forEach((subEntry) => subEntry.classList.add("hidden"));
      })
    } else {
      // First, show/hide all the sub-entries within each module (top-level functions etc.)
      sidebar.querySelectorAll(".sidebar-sub-entries a").forEach((entry) => {
        if (entry.textContent.toLowerCase().includes(text)) {
          entry.classList.remove("hidden");
        } else {
          entry.classList.add("hidden");
        }
      });

      // Then, show/hide modules based on whether they match, or any of their sub-entries matched
      sidebar.querySelectorAll(".sidebar-module-link").forEach((entry) => {
        if (entry.textContent.toLowerCase().includes(text) || entry.parentNode.querySelectorAll(".sidebar-sub-entries a:not(.hidden)").length > 0) {
          entry.classList.remove("hidden");
        } else {
          entry.classList.add("hidden");
        }
      });
    }
  }

  searchBox.addEventListener("input", search);

  search();
})();
"#;

pub const STYLE_CSS: &str = r#"
:root {
  --link-color: #612bde;
  --code-link-color: #5721d4;
  --text-color: #333333;
  --code-color: #222222;
  --code-bg-color: #eeeeee;
  --body-bg-color: #fdfdfd;
  --border-color: #e9e9e9;
  --faded-color: #4c4c4c;
  --monospace-font;
  --font-sans: -apple-system, BlinkMacSystemFont, Roboto, Helvetica, Arial, sans-serif;
  --font-mono: SFMono-Regular, Consolas, "Liberation Mono", Menlo, Courier, monospace;
  --top-header-height: 67px;
  --sidebar-width: 280px;
  --top-bar-bg: #8257e5;
  --top-bar-fg: #ffffff;
  --nav-link-hover-color: #000000;
}

a {
  color: #972395;
}

.logo {
  padding: 2px 8px;
}

.logo svg {
  height: 48px;
  width: 48px;
  fill: var(--top-bar-fg);
}

.logo:hover {
  text-decoration: none;
}

.logo svg:hover {
  fill: var(--nav-link-hover-color);
}

.pkg-full-name {
  color: var(--text-color);
  display: flex;
  align-items: center;
  font-size: 32px;
  margin: 0 8px;
  font-weight: normal;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  height: 100%;
}

.pkg-full-name a {
  padding-top: 12px;
  padding-bottom: 16px;
}

a {
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}

.pkg-and-logo {
  min-width: 0;
  /* necessary for text-overflow: ellipsis to work in descendants */
  display: flex;
  align-items: center;
  height: 100%;
  background-color: var(--top-bar-bg);
}

.pkg-and-logo a,
.pkg-and-logo a:visited {
  color: var(--top-bar-fg);
}

.pkg-and-logo a:hover {
  color: var(--nav-link-hover-color);
  text-decoration: none;
}

.main-container {
  min-width: 0;
  /* necessary for text-overflow: ellipsis to work in descendants */
}

.search-button {
  flex-shrink: 0;
  /* always shrink the package name before these; they have a relatively constrained length */
  padding: 12px 18px;
  margin-right: 42px;
  display: none;
  /* only show this in the mobile view */
}

.version {
  padding: 18px 10px;
  min-width: 48px;
  margin-right: 8px;
}

body {
  display: grid;
  grid-template-columns: [before-sidebar] 1fr [sidebar] var(--sidebar-width) [main-content] fit-content(calc(1280px - var(--sidebar-width))) [end] 1fr;
  grid-template-rows: [top-header] var(--top-header-height) [above-footer] auto [footer] auto;
  box-sizing: border-box;
  margin: 0;
  padding: 0;
  font-family: var(--font-sans);
  color: var(--text-color);
  background-color: var(--body-bg-color);
}

main {
  grid-column-start: main-content;
  grid-column-end: main-content;
  grid-row-start: above-footer;
  grid-row-end: above-footer;
  box-sizing: border-box;
  position: relative;
  font-size: 18px;
  line-height: 1.85em;
  margin-top: 2px;
  padding: 48px;
}

#sidebar-nav {
  grid-column-start: sidebar;
  grid-column-end: sidebar;
  grid-row-start: above-footer;
  grid-row-end: above-footer;
  position: relative;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
  padding-left: 56px;
  padding-top: 6px;
  width: 100%;
}

.top-header-extension {
  grid-column-start: before-sidebar;
  grid-column-end: sidebar;
  grid-row-start: top-header;
  grid-row-end: top-header;
  background-color: var(--top-bar-bg);
}

.top-header {
  grid-column-start: sidebar;
  grid-column-end: end;
  grid-row-start: top-header;
  grid-row-end: top-header;
  display: flex;
  flex-direction: row;
  align-items: center;
  flex-wrap: nowrap;
  flex-grow: 1;
  box-sizing: border-box;
  font-family: var(--font-sans);
  font-size: 24px;
  height: 100%;
  min-width: 0;
  /* necessary for text-overflow: ellipsis to work in descendants */
}

.top-header-triangle {
  /* This used to be a clip-path, but Firefox on Android (at least as of early 2020)
   * rendered the page extremely slowly in that version. With this approach it's super fast.
   */
  width: 0;
  height: 0;
  border-style: solid;
  border-width: var(--top-header-height) 0 0 48px;
  border-color: transparent transparent transparent var(--top-bar-bg);
}

p {
  overflow-wrap: break-word;
  margin: 24px 0;
}

footer {
  grid-column-start: main-content;
  grid-column-end: main-content;
  grid-row-start: footer;
  grid-row-end: footer;
  max-width: var(--main-content-max-width);
  font-size: 14px;
  box-sizing: border-box;
  padding: 16px;
}

footer p {
  display: inline-block;
  margin-top: 0;
  margin-bottom: 8px;
}

.content {
  box-sizing: border-box;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
}

.sidebar-entry ul {
  list-style-type: none;
  margin: 0;
}

.sidebar-entry a {
  box-sizing: border-box;
  min-height: 48px;
  min-width: 48px;
  padding: 12px 16px;
  font-family: var(--font-mono);
}

.sidebar-sub-entries a {
  display: block;
  line-height: 24px;
  width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  padding-left: 36px;
}

.module-name {
  font-size: 56px;
  line-height: 1em;
  font-family: var(--font-mono);
  font-weight: bold;
  margin-top: 18px;
  margin-bottom: 48px;
}

.module-name a,
.module-name a:visited {
  color: inherit;
}

.sidebar-module-link {
  box-sizing: border-box;
  font-size: 18px;
  line-height: 24px;
  font-family: var(--font-mono);
  font-weight: bold;
  display: block;
  width: 100%;
  padding: 8px 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

a,
a:visited {
  color: var(--link-color);
}

h3 {
  font-size: 32px;
  margin: 48px 0 24px 0;
}

h4 {
  font-size: 24px;
}

.type-def {
  font-size: 24px;
  color: var(--link-color);
}

.code-snippet {
  padding: 12px 16px;
  display: block;
  box-sizing: border-box;
  font-family: var(--font-mono);
  background-color: var(--code-bg-color);
}

code {
  font-family: var(--font-mono);
  color: var(--code-color);
  background-color: var(--code-bg-color);
  padding: 2px 8px;
  display: inline-block;
}

code a {
  color: var(--code-link-color);
}

code a:visited {
  color: var(--code-link-color);
}

pre {
  margin: 36px 0;
  padding: 8px;
  box-sizing: border-box;
  background-color: var(--code-bg-color);
  overflow-x: auto;
}

pre code {
  padding: 6px 8px;
}

.hidden {
  /* Use !important to win all specificity fights. */
  display: none !important;
}

#module-search:placeholder-shown {
  padding: 0;
  opacity: 0;
  height: 0;
}

#module-search,
#module-search:focus {
  opacity: 1;
  padding: 12px 16px;
  height: 48px;
}

/* Show the "Search" label link when the text input has a placeholder */
#module-search:placeholder-shown+#search-link {
  display: flex;
}

/* Hide the "Search" label link when the text input has focus */
#module-search:focus+#search-link {
  display: none;
}

#module-search {
  display: block;
  box-sizing: border-box;
  background-color: var(--code-bg-color);
  width: 100%;
  box-sizing: border-box;
  font-size: 18px;
  line-height: 18px;
  margin-top: 6px;
  border: none;
  color: var(--faded-color);
  background-color: var(--code-bg-color);
  font-family: var(--font-serif);
}

#module-search::placeholder {
  color: var(--faded-color);
  opacity: 1;
}

#search-link {
  box-sizing: border-box;
  display: none;
  align-items: center;
  font-size: 18px;
  line-height: 18px;
  padding: 12px 16px;
  height: 48px;
  cursor: pointer;
  color: var(--link-color);
}

#search-link:hover {
  text-decoration: underline;
}

@media (prefers-color-scheme: dark) {
  :root {
    --body-bg-color: #303030;
    --code-bg-color: #393939;
    --border-color: #555555;
    --code-color: #eeeeee;
    --text-color: #cccccc;
    --logo-solid: #777777;
    --faded-color: #bbbbbb;
    --link-color: #c5a8ff;
    --code-link-color: #b894ff;
    --top-bar-bg: #6845b9;
    --top-bar-fg: #eeeeee;
  }

  html {
    scrollbar-color: #444444 #2f2f2f;
  }
}

@media only screen and (max-device-width: 480px) {
  .search-button {
    display: block;
    /* This is only visible in mobile. */
  }

  .top-header {
    width: auto;
  }

  .pkg-full-name {
    margin-left: 8px;
    margin-right: 12px;
    font-size: 24px;
    padding-bottom: 14px;
  }

  .pkg-full-name a {
    vertical-align: middle;
    padding: 18px 0;
  }

  .logo {
    padding-left: 2px;
    width: 50px;
    height: 54px;
  }

  .version {
    margin: 0;
    font-weight: normal;
    font-size: 18px;
    padding-bottom: 16px;
  }

  .module-name {
    font-size: 36px;
    margin-top: 8px;
    margin-bottom: 8px;
    max-width: calc(100% - 18px);
    overflow: hidden;
    text-overflow: ellipsis;
  }

  main {
    padding: 18px;
    font-size: 16px;
  }

  .container {
    margin: 0;
    min-width: 320px;
    max-width: 100%;
  }

  .content {
    flex-direction: column;
  }

  .sidebar {
    margin-top: 0;
    padding-left: 0;
    width: auto;
  }

  #sidebar-heading {
    font-size: 24px;
    margin: 16px;
  }

  h3 {
    font-size: 18px;
    margin: 0;
    padding: 0;
  }

  h4 {
    font-size: 16px;
  }

  .top-header {
    justify-content: space-between;
  }

  .content {
    /* Display the sidebar below <main> without affecting tab index */
    flex-direction: column-reverse;
  }
}
"#;

pub const FAVICON: &str = r#"
<svg viewBox="0 0 52 53" xmlns="http://www.w3.org/2000/svg">
  <style>polygon {fill: #5c0bff;}@media (prefers-color-scheme: dark) {polygon {fill: #7733ff;}} </style>
  <polygon points="0,0 23.8834,3.21052 37.2438,19.0101 45.9665,16.6324 50.5,22 45,22 44.0315,26.3689 26.4673,39.3424 27.4527,45.2132 17.655,53 23.6751,22.7086"/>
</svg>
"#;
