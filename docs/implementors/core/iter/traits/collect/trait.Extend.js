(function() {var implementors = {};
implementors["heapless"] = [{"text":"impl&lt;T, N&gt; Extend&lt;T&gt; for <a class=\"struct\" href=\"heapless/struct.HistoryBuffer.html\" title=\"struct heapless::HistoryBuffer\">HistoryBuffer</a>&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":["heapless::histbuf::HistoryBuffer"]},{"text":"impl&lt;'a, T, N&gt; Extend&lt;&amp;'a T&gt; for <a class=\"struct\" href=\"heapless/struct.HistoryBuffer.html\" title=\"struct heapless::HistoryBuffer\">HistoryBuffer</a>&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":["heapless::histbuf::HistoryBuffer"]},{"text":"impl&lt;K, V, N, S&gt; Extend&lt;(K, V)&gt; for <a class=\"struct\" href=\"heapless/struct.IndexMap.html\" title=\"struct heapless::IndexMap\">IndexMap</a>&lt;K, V, N, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Eq + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;Bucket&lt;K, V&gt;&gt; + <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;Option&lt;Pos&gt;&gt;,&nbsp;</span>","synthetic":false,"types":["heapless::indexmap::IndexMap"]},{"text":"impl&lt;'a, K, V, N, S&gt; Extend&lt;(&amp;'a K, &amp;'a V)&gt; for <a class=\"struct\" href=\"heapless/struct.IndexMap.html\" title=\"struct heapless::IndexMap\">IndexMap</a>&lt;K, V, N, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Eq + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a> + Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;Bucket&lt;K, V&gt;&gt; + <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;Option&lt;Pos&gt;&gt;,&nbsp;</span>","synthetic":false,"types":["heapless::indexmap::IndexMap"]},{"text":"impl&lt;T, N, S&gt; Extend&lt;T&gt; for <a class=\"struct\" href=\"heapless/struct.IndexSet.html\" title=\"struct heapless::IndexSet\">IndexSet</a>&lt;T, N, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;Bucket&lt;T, ()&gt;&gt; + <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;Option&lt;Pos&gt;&gt;,&nbsp;</span>","synthetic":false,"types":["heapless::indexset::IndexSet"]},{"text":"impl&lt;'a, T, N, S&gt; Extend&lt;&amp;'a T&gt; for <a class=\"struct\" href=\"heapless/struct.IndexSet.html\" title=\"struct heapless::IndexSet\">IndexSet</a>&lt;T, N, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + Eq + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a> + Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;Bucket&lt;T, ()&gt;&gt; + <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;Option&lt;Pos&gt;&gt;,&nbsp;</span>","synthetic":false,"types":["heapless::indexset::IndexSet"]},{"text":"impl&lt;T, N&gt; Extend&lt;T&gt; for <a class=\"struct\" href=\"heapless/struct.Vec.html\" title=\"struct heapless::Vec\">Vec</a>&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":["heapless::vec::Vec"]},{"text":"impl&lt;'a, T, N&gt; Extend&lt;&amp;'a T&gt; for <a class=\"struct\" href=\"heapless/struct.Vec.html\" title=\"struct heapless::Vec\">Vec</a>&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"heapless/trait.ArrayLength.html\" title=\"trait heapless::ArrayLength\">ArrayLength</a>&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":["heapless::vec::Vec"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()