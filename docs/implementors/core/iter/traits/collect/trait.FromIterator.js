(function() {var implementors = {};
implementors["generic_array"] = [{"text":"impl&lt;T, N&gt; FromIterator&lt;T&gt; for <a class=\"struct\" href=\"generic_array/struct.GenericArray.html\" title=\"struct generic_array::GenericArray\">GenericArray</a>&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"generic_array/trait.ArrayLength.html\" title=\"trait generic_array::ArrayLength\">ArrayLength</a>&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":["generic_array::GenericArray"]}];
implementors["heapless"] = [{"text":"impl&lt;K, V, N, S&gt; FromIterator&lt;(K, V)&gt; for <a class=\"struct\" href=\"heapless/struct.IndexMap.html\" title=\"struct heapless::IndexMap\">IndexMap</a>&lt;K, V, N, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Eq + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a> + Default,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;Bucket&lt;K, V&gt;&gt; + <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;Option&lt;Pos&gt;&gt;,&nbsp;</span>","synthetic":false,"types":["heapless::indexmap::IndexMap"]},{"text":"impl&lt;T, N, S&gt; FromIterator&lt;T&gt; for <a class=\"struct\" href=\"heapless/struct.IndexSet.html\" title=\"struct heapless::IndexSet\">IndexSet</a>&lt;T, N, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a> + Default,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;Bucket&lt;T, ()&gt;&gt; + <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;Option&lt;Pos&gt;&gt;,&nbsp;</span>","synthetic":false,"types":["heapless::indexset::IndexSet"]},{"text":"impl&lt;K, V, N&gt; FromIterator&lt;(K, V)&gt; for <a class=\"struct\" href=\"heapless/struct.LinearMap.html\" title=\"struct heapless::LinearMap\">LinearMap</a>&lt;K, V, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;(K, V)&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Eq,&nbsp;</span>","synthetic":false,"types":["heapless::linear_map::LinearMap"]},{"text":"impl&lt;T, N&gt; FromIterator&lt;T&gt; for <a class=\"struct\" href=\"heapless/struct.Vec.html\" title=\"struct heapless::Vec\">Vec</a>&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":["heapless::vec::Vec"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()