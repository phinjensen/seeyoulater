const OPTIONS = {
  sync: ["server_url"],
  local: ["username", "password"],
};

document.addEventListener("DOMContentLoaded", (event) => {
  for (let [area, keys] of Object.entries(OPTIONS)) {
    browser.storage[area].get(keys).then((result) => {
      keys.forEach(
        (option) =>
          (document.querySelector(`input[name="${option}"`).value =
            result[option] || "")
      );
    });
  }

  document.querySelector("#save").addEventListener("click", (event) => {
    for (let [area, keys] of Object.entries(OPTIONS)) {
      browser.storage[area].set(
        Object.fromEntries(
          keys.map((option) => [
            option,
            document.querySelector(`input[name="${option}"`).value,
          ])
        )
      );
    }
  });
});
