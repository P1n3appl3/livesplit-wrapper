(function() {var implementors = {
"bytemuck":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"bytemuck/checked/enum.CheckedCastError.html\" title=\"enum bytemuck::checked::CheckedCastError\">CheckedCastError</a>",1,["bytemuck::checked::CheckedCastError"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"bytemuck/enum.PodCastError.html\" title=\"enum bytemuck::PodCastError\">PodCastError</a>",1,["bytemuck::PodCastError"]]],
"livesplit_wrapper":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"livesplit_wrapper/enum.Error.html\" title=\"enum livesplit_wrapper::Error\">Error</a>",1,["livesplit_wrapper::process::Error"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"livesplit_wrapper/struct.Process.html\" title=\"struct livesplit_wrapper::Process\">Process</a>",1,["livesplit_wrapper::process::Process"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"livesplit_wrapper/struct.Logger.html\" title=\"struct livesplit_wrapper::Logger\">Logger</a>",1,["livesplit_wrapper::Logger"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"livesplit_wrapper/enum.TimerState.html\" title=\"enum livesplit_wrapper::TimerState\">TimerState</a>",1,["livesplit_wrapper::TimerState"]]],
"log":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"log/enum.Level.html\" title=\"enum log::Level\">Level</a>",1,["log::Level"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"log/enum.LevelFilter.html\" title=\"enum log::LevelFilter\">LevelFilter</a>",1,["log::LevelFilter"]],["impl&lt;'a&gt; !<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"log/struct.Record.html\" title=\"struct log::Record\">Record</a>&lt;'a&gt;",1,["log::Record"]],["impl&lt;'a&gt; !<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"log/struct.RecordBuilder.html\" title=\"struct log::RecordBuilder\">RecordBuilder</a>&lt;'a&gt;",1,["log::RecordBuilder"]],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"log/struct.Metadata.html\" title=\"struct log::Metadata\">Metadata</a>&lt;'a&gt;",1,["log::Metadata"]],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"log/struct.MetadataBuilder.html\" title=\"struct log::MetadataBuilder\">MetadataBuilder</a>&lt;'a&gt;",1,["log::MetadataBuilder"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"log/struct.SetLoggerError.html\" title=\"struct log::SetLoggerError\">SetLoggerError</a>",1,["log::SetLoggerError"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"log/struct.ParseLevelError.html\" title=\"struct log::ParseLevelError\">ParseLevelError</a>",1,["log::ParseLevelError"]]],
"once_cell":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"once_cell/unsync/struct.OnceCell.html\" title=\"struct once_cell::unsync::OnceCell\">OnceCell</a>&lt;T&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,</span>",1,["once_cell::unsync::OnceCell"]],["impl&lt;T, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"once_cell/unsync/struct.Lazy.html\" title=\"struct once_cell::unsync::Lazy\">Lazy</a>&lt;T, F&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,</span>",1,["once_cell::unsync::Lazy"]],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"once_cell/sync/struct.OnceCell.html\" title=\"struct once_cell::sync::OnceCell\">OnceCell</a>&lt;T&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,</span>",1,["once_cell::sync::OnceCell"]],["impl&lt;T, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"once_cell/sync/struct.Lazy.html\" title=\"struct once_cell::sync::Lazy\">Lazy</a>&lt;T, F&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,</span>",1,["once_cell::sync::Lazy"]],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"once_cell/race/struct.OnceBox.html\" title=\"struct once_cell::race::OnceBox\">OnceBox</a>&lt;T&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,</span>",1,["once_cell::race::once_box::OnceBox"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"once_cell/race/struct.OnceNonZeroUsize.html\" title=\"struct once_cell::race::OnceNonZeroUsize\">OnceNonZeroUsize</a>",1,["once_cell::race::OnceNonZeroUsize"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"once_cell/race/struct.OnceBool.html\" title=\"struct once_cell::race::OnceBool\">OnceBool</a>",1,["once_cell::race::OnceBool"]]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()