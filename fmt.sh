# `format_code_in_doc_comments` is nightly

sed -I "" 's/# format_code_in_doc_comments = true/format_code_in_doc_comments = true/' rustfmt.toml

cargo +nightly fmt

sed -I "" 's/^format_code_in_doc_comments = true/# format_code_in_doc_comments = true/' rustfmt.toml

