browser.browserAction.onClicked.addListener(function ({ title, url }) {
  fetch("http://localhost/anything", {
    method: "POST",
    body: JSON.stringify({
      title,
      url,
    }),
  })
    .then((response) => response.json())
    .then((body) => console.log(body));
});
