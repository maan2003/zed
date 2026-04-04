- codexacp gets stuck sometimes? maybe just the api?
  - actually no- latest progress message is stick? so it appears as if nothing is happening when new tool calls are hapening

- Internal error: {
  "message": "Selected model is at capacity. Please try a different model.",
  "codex_error_info": "server_overloaded"
} - not retrying
- steering
- show context length / remaining context
- require double esc to cancel
- list the time taken "Worked for X minutes"
- reduce space taken by tools - collapse like codex - similar to fixed height thinking
  - maybe it should be similar to thinking
  - done!
  - no padding after the shell tool in semi expanded state

- multiple commentary stream updates just concat without new lines

- rename threads (title) in codex-acp
- reduce height of shell tool
- subagent rendering in codex-acp
- editting past messages in codex-acp
- orcherestator
- fix remote + worktree
- long env variable list when launching acp in zed itself
  - also happens in terminal: /run/current-system/sw/bin/fish: Argument list too long
- load_session -> shell commands not loading properly
  - even ls, rg etc are not loading to normal tools
- load_session, new_session are slow
- links inside codex don't work
- tittle is gone
- session restore doesn't respect remote projects when in multi workspace - opens them as separate windows
  - upstream is workong on this?
- workspace modes still don't feel great

- remove all permissions in codex-acp (--dangerously-allow-all)
  - looks like full access is good enough?

- reducing chrome:
  - remove status bar? - move stuff to titlebar
  - remove search from sidebar - only show on /
  - hide status bar when in agent only/terminal only mode?
