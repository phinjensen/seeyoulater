# See You Later!

See You Later is a bookmarking tool suite for personal collections and sharing. In its most basic form, it's a command line tool for mainting a database of web pages, but it also includes a simple server for bookmarking from multiple devices or sharing bookmarks through an API. There is also a browser extension for Firefox which allows adding bookmarks to any SeeYouLater service.

## Pre-Alpha

Some of the above statements are not true, and the project as a whole is in a very early stage. It's almost to the point where I'll start using it for my own personal use.

| Feature             | Supported? |
|---------------------|------------|
| Adding bookmarks    | Yes!       |
| - With tags         | Yes!       |
| Searching bookmarks | Yes!       |
| Deleting bookmarks  | Yes!       |
| Configuration       | No :(      |
| Server              | Partial    |
| - Add               | Yes!       |
|   - With tags       | Yes!       |
| - Search            | Yes!       |
| - Delete            | No :(      |
| - Sharing           | No :(      |
| - Authentication    | No :(      |
| Extension           | Partial    |
| - Add               | Yes!       |
|   - With tags       | No :(      |
| - Search\*          | No :(      |
| - Delete\*          | No :(      |
| - Configuration\*\* | Partial    |
| - Firefox           | Yes!       |
| - Chrome            | No :(      |
| - Edge              | No :(      |
| - Safari            | No :(      |
| - Other browsers    | No :(      |
| Import              | No :(      |
| - Browsers          | No :(      |
| - Buku              | No :(      |

\* These features may never be implemented. The focus is a good CLI interface for searching, while the extension just provides a quick and easy way to add bookmarks from your various browsers.
\*\* Currently, you can configure a target server in the browser. Other settings may or may not come.

## Rationale

I used Buku for a while but don't love its command line interface or the lack of native sync support. Setting up browser extensions also requires extra dependencies such as a native host, and have features I don't necessarily want or need. I wondered, can I do it better? Maybe, maybe not. At any rate, it was a good excuse to practice my Rust skills and learn how to write browser extensions.

## Contributing

This is (currently) a totally personal project and I already have a lengthy TODO list (see above), so I'm not terribly interested in new ideas right now. However, I would love any bug fixes, feature additions, etc. if you do want to contribute! Just open an issue or pull request.
