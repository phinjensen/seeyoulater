sync_options = ["server_url"];
local_options = ["username", "password"];

document.addEventListener("DOMContentLoaded", (event) => {
  browser.storage.sync
    .get(sync_options)
    .then((result) =>
      sync_options.forEach(
        (option) =>
          (document.querySelector(`input[name="${option}"`).value =
            result[option] || "")
      )
    );
  browser.storage.local
    .get(local_options)
    .then((result) =>
      local_options.forEach(
        (option) =>
          (document.querySelector(`input[name="${option}"`).value =
            result[option] || "")
      )
    );

  document.querySelector("#save").addEventListener("click", (event) => {
    browser.storage.sync.set(
      Object.fromEntries(
        simple_options.map((option) => [
          option,
          document.querySelector(`input[name="${option}"`).value,
        ])
      )
    );
  });
});
