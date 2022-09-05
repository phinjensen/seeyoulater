browser.browserAction.onClicked.addListener(function ({ title, url }) {
  browser.storage.sync.get("server_url").then(({ server_url }) => {
    fetch(server_url + "/add", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        title,
        url,
      }),
    })
      .then((response) => response.json())
      .then((body) => console.log(body));
  });
});
