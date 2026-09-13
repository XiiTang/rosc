# Bounded OSC codec patch

Base: upstream rosc 0.11.4, commit
`711945b5d75b24a544cb80d8aa0e3d6a3f9dd880`.

This branch adds the standard `S` Symbol type and rejects malformed tag strings,
unclosed arrays, nonzero padding, negative blob sizes and invalid/trailing bundle
element content. Decode traversal is bounded to 64 bundle/array levels and 65,536
packet/argument nodes. Node limits are checked before growing the decoded tree;
the caller remains responsible for bounding the input byte slice.

The upstream API returns unconsumed bytes. A caller accepting one datagram must
require an empty remainder. Runtime scheduling, network IO and DTOs are outside
this patch. Upstream history, source, test vectors and licenses are retained.

Validation: upstream test suite plus `tests/strict_codec_test.rs`.
