browser.runtime.onMessage.addListener(
  async ({ addBookmark: { title, description, url, tags } }) => {
    let { server_url } = await browser.storage.sync.get("server_url");
    let response = await fetch(server_url + "/add", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        title,
        url,
        description,
        tags,
      }),
    });
    let body = await response.json();
  }
);
