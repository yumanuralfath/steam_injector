const invoke = window.__TAURI__.core.invoke;

let searchInputEl;
let resultsEl;
let currentPageEl;
let prevPageEl;
let nextPageEl;

let currentPage = 1;
let searchText = '';

async function search(page = 1) {
  try {
    resultsEl.innerHTML = '<p>Loading...</p>';

    const response = await invoke('search_steam_apps', {
      searchText: searchText,
      currentPage: page
    });

    if (!Array.isArray(response) || response.length < 2) {
      throw new Error('Unexpected response format');
    }

    // Parse JSON dari elemen pertama
    const result = JSON.parse(response[0]);
    const totalPageCount = response[1];

    renderResults(result);
    currentPageEl.textContent = `Page: ${page} / ${totalPageCount}`;
    prevPageEl.disabled = page === 1;
    nextPageEl.disabled = page === totalPageCount || (totalPageCount === 0);

    console.log(totalPageCount);
    currentPage = page;
  } catch (error) {
    resultsEl.innerHTML = '<p>Error fetching results. Please try again.</p>';
    console.error('Search error:', error);
  }
}


function renderResults(results) {

  if (!Array.isArray(results)) {
    console.error('Invalid results format:', results);
    resultsEl.innerHTML = '<p>Error: Invalid data format.</p>';
    return;
  }

  // Gunakan filter untuk hanya merender item yang memiliki data lengkap
  const validResults = results.filter(
    ({ appid, image, title }) => typeof appid === 'number' && image && title
  );

  if (validResults.length === 0) {
    resultsEl.innerHTML = '<p>No results found.</p>';
    return;
  }

  resultsEl.innerHTML = validResults
    .map(
      ({ appid, image, title }) => `
      <div class="result-item">
        <img src="${image}" alt="${title}" class="result-image" />
        <div class="result-info">
          <p class="result-title">${title}</p>
          <p class="result-appid">App ID: ${appid}</p>
        </div>
      </div>
    `
    )
    .join('');
}

window.addEventListener('DOMContentLoaded', () => {
  searchInputEl = document.querySelector('#search-input');
  resultsEl = document.querySelector('#results');
  currentPageEl = document.querySelector('#current-page');
  prevPageEl = document.querySelector('#prev-page');
  nextPageEl = document.querySelector('#next-page');

  document.querySelector('#search-form').addEventListener('submit', (e) => {
    e.preventDefault();
    searchText = searchInputEl.value;
    currentPage = 1;
    search(currentPage);
  });

  prevPageEl.addEventListener('click', () => {
    if (currentPage > 1) {
      search(currentPage - 1);
    }
  });

  nextPageEl.addEventListener('click', () => {
    search(currentPage + 1);
  });
});