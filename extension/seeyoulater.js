browser.runtime.onMessage.addListener(
  async ({ addBookmark: { title, description, url, tags } }) => {
    let notificationId = await browser.notifications.create({
      type: "basic",
      title: "See You Later",
      message: `Sending bookmark to server...`,
    });
    let { server_url } = await browser.storage.sync.get("server_url");
    try {
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
      await browser.notifications.clear(notificationId);
      notificationId = await browser.notifications.create({
        type: "basic",
        title: "See You Later",
        message: `Saved bookmark successfully`,
      });
    } catch (err) {
      await browser.notifications.clear(notificationId);
      notificationId = await browser.notifications.create({
        type: "basic",
        title: `Error saving bookmark: ${err}`,
        message: `Saved bookmark successfully`,
      });
    }
  }
);
