(function() {var implementors = {
"futures_sink":[],
"tokio_util":[["impl&lt;'a, S&gt; <a class=\"trait\" href=\"futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;&amp;'a [<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>]&gt; for <a class=\"struct\" href=\"tokio_util/io/struct.CopyToBytes.html\" title=\"struct tokio_util::io::CopyToBytes\">CopyToBytes</a>&lt;S&gt;<span class=\"where fmt-newline\">where\n    S: <a class=\"trait\" href=\"futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;<a class=\"struct\" href=\"bytes/bytes/struct.Bytes.html\" title=\"struct bytes::bytes::Bytes\">Bytes</a>&gt;,</span>"],["impl&lt;S: <a class=\"trait\" href=\"futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;T, Error = E&gt;, E, T&gt; <a class=\"trait\" href=\"futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;T&gt; for <a class=\"struct\" href=\"tokio_util/io/struct.StreamReader.html\" title=\"struct tokio_util::io::StreamReader\">StreamReader</a>&lt;S, E&gt;"],["impl&lt;T, I, E&gt; <a class=\"trait\" href=\"futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;I&gt; for <a class=\"struct\" href=\"tokio_util/codec/struct.FramedWrite.html\" title=\"struct tokio_util::codec::FramedWrite\">FramedWrite</a>&lt;T, E&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"tokio/io/async_write/trait.AsyncWrite.html\" title=\"trait tokio::io::async_write::AsyncWrite\">AsyncWrite</a>,\n    E: <a class=\"trait\" href=\"tokio_util/codec/trait.Encoder.html\" title=\"trait tokio_util::codec::Encoder\">Encoder</a>&lt;I&gt;,\n    E::<a class=\"associatedtype\" href=\"tokio_util/codec/trait.Encoder.html#associatedtype.Error\" title=\"type tokio_util::codec::Encoder::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;,</span>"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static&gt; <a class=\"trait\" href=\"futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;T&gt; for <a class=\"struct\" href=\"tokio_util/sync/struct.PollSender.html\" title=\"struct tokio_util::sync::PollSender\">PollSender</a>&lt;T&gt;"],["impl&lt;T, I, D&gt; <a class=\"trait\" href=\"futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;I&gt; for <a class=\"struct\" href=\"tokio_util/codec/struct.FramedRead.html\" title=\"struct tokio_util::codec::FramedRead\">FramedRead</a>&lt;T, D&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;I&gt;,</span>"],["impl&lt;T, I, U&gt; <a class=\"trait\" href=\"futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;I&gt; for <a class=\"struct\" href=\"tokio_util/codec/struct.Framed.html\" title=\"struct tokio_util::codec::Framed\">Framed</a>&lt;T, U&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"tokio/io/async_write/trait.AsyncWrite.html\" title=\"trait tokio::io::async_write::AsyncWrite\">AsyncWrite</a>,\n    U: <a class=\"trait\" href=\"tokio_util/codec/trait.Encoder.html\" title=\"trait tokio_util::codec::Encoder\">Encoder</a>&lt;I&gt;,\n    U::<a class=\"associatedtype\" href=\"tokio_util/codec/trait.Encoder.html#associatedtype.Error\" title=\"type tokio_util::codec::Encoder::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()