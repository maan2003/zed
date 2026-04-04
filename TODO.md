- fix remote + worktree
- reduce space taken by tools - collapse like codex - similar to fixed height thinking
- letting agent panel taken full screen [sidebar|agent-panel] - no editor (hidden center panel)
  - viewing terminal while agent mode is active?
  - ideally it should be one zoomed panel
  - different modes?
    - like agent mode, editor mode, agent + editor mode, agent + terminal mode

  - final decision:
    - no gui
    - only specific modes with keyboard shortcuts
    - modes will be:
      - normal - aka editor
      - terminal only - just sidebar and terminal
      - agent only
      - agent + editor - for review?
      - agent + terminal - note horizontal stacked [agent|terminal]

    - as a result these will terminal and agent will not in normal pane registerations

- reduce height of shell tool
- remove all permissions in codex-acp (--dangerously-allow-all)

- subagent rendering in codex-acp
- orcherestator


- reducing chrome:
  - remove status bar? - move stuff to titlebar
  - remove search from sidebar - only show on /

ISSUES:
  - tittle is gone
  - crash when switching to agent only mode
  - load_session -> shell commands not loading properly
  - load_session, new_session are slow
  - links inside codex don't work
  - hide status bar when in agent only/terminal only mode?
