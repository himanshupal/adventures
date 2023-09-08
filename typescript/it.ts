async function* fetchCommits(repo: string) {
  let url: string | undefined = `https://api.github.com/repos/${repo}/commits`;

  while (url) {
    const response = await fetch(url, {
      // (1)
      headers: { "User-Agent": "Our script" }, // github needs any user-agent header
    });

    const body: unknown[] = await response.json(); // (2) response is JSON (array of commits)

    // (3) the URL of the next page is in the headers, extract it
    const nextPage: RegExpMatchArray | string | null | undefined = response.headers.get("Link")?.match(/<(.*?)>; rel="next"/);

    url = nextPage?.[1];

    for (const commit of body) {
      // (4) yield commits one by one, until the page ends
      yield commit as { sha: string };
    }
  }
}

for await (const commit of fetchCommits("himanshupal/editors")) {
  console.log(commit?.sha);
}
