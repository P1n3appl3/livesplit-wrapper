<div align="center">
  <h1><strong>LiveSplit Wrapper</strong></h1>
  <p> <strong>A safe wrapper library for writing LiveSplit One autosplitters</strong> </p>
  <p>
    <a href="https://github.com/p1n3appl3/livesplit-wrapper/actions/workflows/ci.yml">
        <img src="https://github.com/P1n3appl3/livesplit-wrapper/actions/workflows/ci.yml/badge.svg" alt="build status" />
    </a>
    <a href="https://p1n3appl3.github.io/livesplit-wrapper/livesplit_wrapper/index.html">
        <img src="https://img.shields.io/github/workflow/status/p1n3appl3/livesplit-wrapper/Deploy%20docs?label=docs" alt="docs" />
    </a>
    <a href="https://choosealicense.com/licenses/mit/">
        <img src="https://img.shields.io/github/license/p1n3appl3/livesplit-wrapper" alt="license" />
    </a>
  </p>
</div>

[LiveSplit One](https://github.com/LiveSplit/livesplit-core) uses dynamically
loaded WASM modules for plugins, which means they have to communicate over a C
api. This crate contains ergonomic wrappers and helpers to write those plugins
in safe rust. At the moment only autosplitters are supported, but there will
eventually be support for more general plugins.

# Implementing an autosplitter

To write an autosplitter you need to implement the
[`Splitter`](https://p1n3appl3.github.io/livesplit-wrapper/livesplit_wrapper/trait.Splitter.html)
trait and invoke the
[`register_autosplitter!`](https://p1n3appl3.github.io/livesplit-wrapper/livesplit_wrapper/macro.register_autosplitter.html)
macro on your splitter.

Here's a full (nonsensical) example. Build this with
`--target wasm32-unknown-unknown` and it can be loaded by frontends such as
[`livesplit-one-desktop`](https://github.com/CryZe/livesplit-one-desktop) or
[`obs-livesplit`](https://github.com/P1n3appl3/obs-livesplit):

```rust
use livesplit_wrapper::{Splitter, Process, TimerState, HostFunctions};

#[derive(Default)]
struct MySplitter {
    process: Option<Process>,
}

livesplit_wrapper::register_autosplitter!(MySplitter);
impl Splitter for MySplitter {
    fn new() -> Self {
        let mut s = MySplitter::default();
        s.process = s.attach("CoolGame.exe");
        if s.process.is_none() {
            log::warn!("failed to connect to process, is the game running?");
        }
        s.set_tick_rate(120.0);
        s.set_variable("items collected", "0");
        s
    }

    fn update(&mut self) {
        if let Some(p) = &self.process {
            match (self.state(), p.read::<i16>(0xD1DAC71C)) {
                (TimerState::Paused, Ok(314)) => self.unpause(),
                (TimerState::Running, Ok(42)) => self.pause(),
                _ => {}
            }
        }
    }
}
```

For a real-world example, check out
[this Celeste autosplitter](https://github.com/P1n3appl3/climb/tree/main/auto-splitter).
