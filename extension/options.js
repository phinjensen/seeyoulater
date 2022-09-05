document.addEventListener("DOMContentLoaded", (event) => {
  browser.storage.sync
    .get("server_url")
    .then(
      ({ server_url }) =>
        (document.querySelector('input[name="server_url"').value = server_url)
    );

  document.querySelector("#save").addEventListener("click", (event) => {
    browser.storage.sync.set({
      server_url: document.querySelector('input[name="server_url"').value,
    });
  });
});
