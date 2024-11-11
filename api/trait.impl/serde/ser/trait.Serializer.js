(function() {
    var implementors = Object.fromEntries([["quick_xml",[["impl&lt;'w, 'r, W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Write.html\" title=\"trait core::fmt::Write\">Write</a>&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for <a class=\"struct\" href=\"quick_xml/se/struct.Serializer.html\" title=\"struct quick_xml::se::Serializer\">Serializer</a>&lt;'w, 'r, W&gt;"]]],["serde",[]],["serde_json",[["impl <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for <a class=\"struct\" href=\"serde_json/value/struct.Serializer.html\" title=\"struct serde_json::value::Serializer\">Serializer</a>"],["impl&lt;'a, W, F&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for &amp;'a mut <a class=\"struct\" href=\"serde_json/struct.Serializer.html\" title=\"struct serde_json::Serializer\">Serializer</a>&lt;W, F&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,\n    F: <a class=\"trait\" href=\"serde_json/ser/trait.Formatter.html\" title=\"trait serde_json::ser::Formatter\">Formatter</a>,</div>"]]],["serde_urlencoded",[["impl&lt;'input, 'output, Target&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for <a class=\"struct\" href=\"serde_urlencoded/struct.Serializer.html\" title=\"struct serde_urlencoded::Serializer\">Serializer</a>&lt;'input, 'output, Target&gt;<div class=\"where\">where\n    Target: 'output + <a class=\"trait\" href=\"form_urlencoded/trait.Target.html\" title=\"trait form_urlencoded::Target\">UrlEncodedTarget</a>,</div>"]]],["toml",[["impl&lt;'a, 'b&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for &amp;'b mut <a class=\"struct\" href=\"toml/ser/struct.Serializer.html\" title=\"struct toml::ser::Serializer\">Serializer</a>&lt;'a&gt;"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[430,13,857,521,289]}