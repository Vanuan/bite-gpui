# Handoff: the `bite_*` branches

This repository carries one architecture — upstream GPUI split into layers —
transplanted onto several upstream bases. Each branch stands alone: pick the one
whose base matches the release you build.

## The architecture

```
gpui_types           primitives: geometry, colour, keystroke vocabulary
gpui_platform        the platform SPI: platform, window, text-system, executor traits
gpui_backend         renderer, atlas, scene and line-layout vocabulary
gpui_engine          text shaping and line layout
gpui_engine_default  the concrete engine wiring
gpui_runtime         app runtime and frame pipelines
gpui_authoring       the element, window and app layer authors write against
gpui_parley          the parley-backed text system
gpui                 the facade, re-exporting the layers
```

`gpui` re-exports the layer crates (`pub use gpui_authoring::*;`,
`pub use gpui_runtime::*;`), so upstream's public API — `gpui::App`,
`gpui::Window`, `gpui::div` — keeps working unchanged.

`gpui` and `gpui_authoring` each declare `extern crate self as gpui;`, so a
`gpui::` path *inside* those crates resolves to the crate itself and is correct
as written. That is the main reason a blanket rewrite of `gpui::` paths is
wrong in this tree.

## Branches

| branch | base | commits | state |
| --- | --- | --- | --- |
| `bite_master` | `4b47ceb9d3` (`upstream/main` at the time) | 14 | the reshaped stack, rebased onto main |
| `bite_v1.21.0-pre` | `808200ff7c` (tag `v1.21.0-pre`) | 13 | the same architecture on the pre-release tag |
| `bite_v1.20.2` | `7c451e694f` (tag `v1.20.2`) | 26 | back-port: 13 replay + 13 back-port commits |
| `bite_v1.19.x` | `a4a4c16576` (`upstream/v1.19.x`) | 22 | back-port: 13 replay + 9 back-port commits |

`bite_v1.19.x` is based on the live `upstream/v1.19.x` branch, not the `v1.19.2`
tag: the tag is 7 commits behind the branch.

The seven layer branches the fork also keeps (`gpui_types` … `gpui_textsystem`)
are not published here.

## Verifying a branch

```sh
git clone git@github.com:Vanuan/bite-gpui.git
cd bite-gpui
git checkout bite_v1.20.2
cargo check --workspace
```

The architecture closure — the crates the refactor touches — can also be linted
on its own, which is minutes rather than a full build:

```sh
cargo clippy --all-targets --all-features \
  -p gpui_types -p gpui_platform -p gpui_engine -p gpui_engine_default \
  -p gpui_runtime -p gpui_authoring -p gpui_parley -p gpui -p gpui_linux \
  -- -D warnings
```

## Maintaining a back-port

A back-port is two phases, kept as separate commits, and the branch history names
which is which:

1. **Replay** — the architecture commits applied onto the base. Pure moves carry
   the architecture; intermediate commits do not compile, and that is expected.
2. **Back-port commits** — everything the *base* needs that the architecture does
   not provide.

The classes that recur, in the order they were found:

* **A file that cannot compile where the refactor put it.** Take the fork's copy
  of it outright; the commit body names the file and the reason. Hand-merging
  that file is a forward-port done badly.
* **An API the base still calls that the refactor removed or moved out of
  reach** — `BackgroundExecutor::spawn_dedicated`, which `agent_servers` needs;
  `PriorityQueueReceiver::len`, which another crate calls. Restore the API rather
  than rewriting the caller.
* **Main-only code grafted into a base file** by a merge that took both sides of
  a hunk. It appears as `dead_code`, as `E0050`/`E0425` where a trait changed
  shape (`Element::request_layout` lost its `inspector_id` parameter to
  `Element::source_location`), or as a call to a method the base's other files do
  not have. Remove the graft: a back-port should not import a feature to satisfy
  a test.
* **`gpui_platform` names two different crates.** Upstream's `gpui_platform` was
  the platform *bundle*; the refactor folds that into `gpui` and gives the name
  to the platform SPI. A base caller's `gpui_platform::x()` was the bundle's
  re-export of `gpui::x()`, so it belongs on the facade.

Two instruments are required, not optional:

* `cargo check --workspace` — the closure's check cannot see base callers outside
  the closure, which is where most of the errors are.
* `cargo clippy --all-targets --all-features` — test-, bench- and
  feature-gated code that a library-only check never compiles.

Neither a lib-only `cargo check -p <closure>` nor a successful closure lint run
is sufficient: in the `v1.19.x` back-port, 11 of the 22 errors came from these two
and were invisible to the closure.

## Where the detail lives

The fork keeps an untracked `.tools/` directory holding the back-port tooling
(`probe_backport.py`, the per-target take-stack lists, the lint helpers), the
`gpui-use-merge` rebase driver, and `maintenance-report.md`, which records how
each branch was produced, which side each file took and why, and the pitfalls
found while doing it. Ask for it before running the next back-port.

## Known gaps

* The test suites compile but are not run on any `bite_*` branch. "Compiles and
  lints" is the whole of the evidence so far.
* `cargo clippy --workspace --all-targets --all-features` (everything, all
  targets and features at once) does not complete on a 691 GiB volume; the
  closure lint and the workspace check do.
* `script/clippy` (`--workspace --release`) has not been run to completion.
