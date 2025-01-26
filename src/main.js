const invoke = window.__TAURI__.core.invoke;

let searchInputEl;
let resultsGridEl;
let totalPagesEl;

async function search() {
  const searchText = searchInputEl.value;
  const currentPage = 1;

  try {
    resultsGridEl.innerHTML = '<p>Loading...</p>';
    totalPagesEl.textContent = '';

    const [result, totalPageCount] = await invoke('search_steam_apps', {
      searchText: searchText,
      currentPage: currentPage,
    });

    const results = JSON.parse(result);

    if (results.length > 0) {
      resultsGridEl.innerHTML = results
        .map(
          (app) => `
          <div class="result-card">
            <img src="${app.image}" alt="${app.title}" />
            <div class="card-content">
              <h4>${app.title}</h4>
            </div>
          </div>
        `
        )
        .join('');
    } else {
      resultsGridEl.innerHTML = '<p>No results found.</p>';
    }

    totalPagesEl.textContent = `Total Pages: ${totalPageCount}`;
  } catch (error) {
    resultsGridEl.innerHTML = '<p>Error fetching results. Please try again.</p>';
    console.error('Search error:', error);
  }
}

window.addEventListener('DOMContentLoaded', () => {
  searchInputEl = document.querySelector('#search-input');
  resultsGridEl = document.querySelector('#results-grid');
  totalPagesEl = document.querySelector('#total-pages');

  document.querySelector('#search-form').addEventListener('submit', (e) => {
    e.preventDefault();
    search();
  });
});