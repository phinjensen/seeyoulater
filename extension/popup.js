function save(bookmark) {
  browser.runtime.sendMessage({ addBookmark: bookmark });
  window.close();
}

document.addEventListener("DOMContentLoaded", async (event) => {
  // Get input fields
  const inputs = {
    title: document.querySelector("input[name='title']"),
    url: document.querySelector("input[name='url']"),
    description: document.querySelector("textarea[name='description']"),
    tags: document.querySelector("input[name='tags']"),
  };

  // Add expand functionality to URL edit button
  document.querySelectorAll(".expand").forEach((button) => {
    button.addEventListener("click", (event) => {
      const target = document.querySelector(`#${button.dataset.open}`);
      target.classList.remove("hidden");
      button.classList.add("hidden");
    });
  });

  // Get title, URL, and description from tag
  const tab = await browser.tabs.query({ active: true, currentWindow: true });
  const [{ title, url }] = tab;
  const [description] = await browser.tabs.executeScript({
    code: `
      (
        document.querySelector('meta[name="description"]') ||
        document.querySelector('meta[name="og:description"]')
      )?.attributes.getNamedItem("content")?.value || "";
    `,
  });

  inputs.title.value = title;
  inputs.url.value = url;
  inputs.description.value = description;

  // Helper function to generate object with data for POST request
  const getValues = () =>
    Object.fromEntries(
      Object.entries(inputs).map(([key, { value }]) => {
        if (key === "tags") {
          value = value
            .split(",")
            .map((tag) => tag.trim())
            .filter(Boolean);
        }
        return [key, value];
      })
    );

  // Set up form submission
  const form = document.querySelector("form");
  const onSubmit = (event) => {
    event.preventDefault();
    window.removeEventListener("unload", onUnload);
    save(getValues());
  };
  function onUnload(event) {
    event.preventDefault();
    onSubmit();
  }

  // Cancel button closes window without saving
  document.querySelector("#cancel").addEventListener("click", (event) => {
    event.preventDefault();
    window.removeEventListener("unload", onUnload);
    window.close();
  });

  // Submitting the form or closing the window without the cancel button saves
  form.addEventListener("submit", onSubmit);
  window.addEventListener("unload", onUnload);

  document.addEventListener("keydown", (event) => {
    if (event.key === "Enter" && event.target.type !== "textarea") {
      event.preventDefault();
      form.requestSubmit();
    }
  });

  inputs.title.focus();
});
