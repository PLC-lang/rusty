(function() {var implementors = {
"chrono":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;<a class=\"struct\" href=\"chrono/struct.Duration.html\" title=\"struct chrono::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"chrono/struct.Duration.html\" title=\"struct chrono::Duration\">Duration</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;&amp;'a <a class=\"struct\" href=\"chrono/struct.Duration.html\" title=\"struct chrono::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"chrono/struct.Duration.html\" title=\"struct chrono::Duration\">Duration</a>"]],
"fraction":[["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"fraction/trait.Integer.html\" title=\"trait fraction::Integer\">Integer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;<a class=\"enum\" href=\"fraction/prelude/enum.GenericFraction.html\" title=\"enum fraction::prelude::GenericFraction\">GenericFraction</a>&lt;T&gt;&gt; for <a class=\"enum\" href=\"fraction/prelude/enum.GenericFraction.html\" title=\"enum fraction::prelude::GenericFraction\">GenericFraction</a>&lt;T&gt;"],["impl&lt;'a, T: 'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"fraction/trait.Integer.html\" title=\"trait fraction::Integer\">Integer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;&amp;'a <a class=\"enum\" href=\"fraction/prelude/enum.GenericFraction.html\" title=\"enum fraction::prelude::GenericFraction\">GenericFraction</a>&lt;T&gt;&gt; for <a class=\"enum\" href=\"fraction/prelude/enum.GenericFraction.html\" title=\"enum fraction::prelude::GenericFraction\">GenericFraction</a>&lt;T&gt;"]],
"jsonschema":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;<a class=\"enum\" href=\"jsonschema/output/enum.BasicOutput.html\" title=\"enum jsonschema::output::BasicOutput\">BasicOutput</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"jsonschema/output/enum.BasicOutput.html\" title=\"enum jsonschema::output::BasicOutput\">BasicOutput</a>&lt;'a&gt;"]],
"num_bigint":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;T&gt; for <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a><span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;T, Output = <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>&gt;,</span>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;T&gt; for <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a><span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;T, Output = <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>&gt;,</span>"]],
"num_complex":[["impl&lt;'a, T: 'a + <a class=\"trait\" href=\"num_traits/trait.Num.html\" title=\"trait num_traits::Num\">Num</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;&amp;'a <a class=\"struct\" href=\"num_complex/struct.Complex.html\" title=\"struct num_complex::Complex\">Complex</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"num_complex/struct.Complex.html\" title=\"struct num_complex::Complex\">Complex</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"num_traits/trait.Num.html\" title=\"trait num_traits::Num\">Num</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;<a class=\"struct\" href=\"num_complex/struct.Complex.html\" title=\"struct num_complex::Complex\">Complex</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"num_complex/struct.Complex.html\" title=\"struct num_complex::Complex\">Complex</a>&lt;T&gt;"]],
"num_rational":[["impl&lt;'a, T: <a class=\"trait\" href=\"num_integer/trait.Integer.html\" title=\"trait num_integer::Integer\">Integer</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;&amp;'a <a class=\"struct\" href=\"num_rational/struct.Ratio.html\" title=\"struct num_rational::Ratio\">Ratio</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"num_rational/struct.Ratio.html\" title=\"struct num_rational::Ratio\">Ratio</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"num_integer/trait.Integer.html\" title=\"trait num_integer::Integer\">Integer</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;<a class=\"struct\" href=\"num_rational/struct.Ratio.html\" title=\"struct num_rational::Ratio\">Ratio</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"num_rational/struct.Ratio.html\" title=\"struct num_rational::Ratio\">Ratio</a>&lt;T&gt;"]],
"time":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;&amp;'a <a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;<a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()