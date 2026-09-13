use rosc::{decoder, encoder, OscMessage, OscPacket, OscType};

fn complete(bytes: &[u8]) -> bool {
    decoder::decode_udp(bytes).is_ok_and(|(rest, _)| rest.is_empty())
}

fn bundle(child: &[u8]) -> Vec<u8> {
    let mut bytes = b"#bundle\0\0\0\0\0\0\0\0\x01".to_vec();
    bytes.extend_from_slice(&(child.len() as u32).to_be_bytes());
    bytes.extend_from_slice(child);
    bytes
}

#[test]
fn symbol_uses_distinct_standard_tag_and_preserves_value() {
    let bytes = b"/a\0\0,S\0\0synth\0\0\0";
    let packet = OscPacket::Message(OscMessage {
        addr: "/a".into(),
        args: vec![OscType::Symbol("synth".into())],
    });
    assert_eq!(encoder::encode(&packet).unwrap(), bytes);
    assert_eq!(decoder::decode_udp(bytes).unwrap(), (&b""[..], packet));
}

#[test]
fn malformed_structure_is_not_accepted_as_a_complete_packet() {
    for bytes in [
        b"/a\0\0,[i\0\0\0\0\x01".as_slice(),
        b"/a\0X,\0\0\0",
        b"/a\0\0i\0\0\0",
        b"/a\0\0,]\0\0",
        b"/a\0\0,b\0\0\xff\xff\xff\xff",
        b"/a\0\0,b\0\0\0\0\0\x01\x01\0\0X",
    ] {
        assert!(!complete(bytes), "accepted {bytes:?}");
        assert!(!complete(&bundle(bytes)), "accepted nested {bytes:?}");
    }
    let trailing_child = b"/a\0\0,\0\0\0JUNK";
    assert!(!complete(&bundle(trailing_child)));
}

#[test]
fn bundle_and_array_depth_are_bounded_before_recursion() {
    let mut packet = b"/a\0\0,\0\0\0".to_vec();
    for _ in 0..64 {
        packet = bundle(&packet);
    }
    assert!(complete(&packet));
    assert!(!complete(&bundle(&packet)));
    for count in [64, 65] {
        let mut packet = b"/a\0\0,".to_vec();
        packet.extend(std::iter::repeat(b'[').take(count));
        packet.extend(std::iter::repeat(b']').take(count));
        packet.push(0);
        while packet.len() % 4 != 0 {
            packet.push(0);
        }
        assert_eq!(complete(&packet), count == 64);
    }
}

#[test]
fn zero_payload_tags_cannot_allocate_an_unbounded_argument_tree() {
    let mut packet = b"/a\0\0,".to_vec();
    packet.extend(std::iter::repeat(b'N').take(65_536));
    packet.push(0);
    while packet.len() % 4 != 0 {
        packet.push(0);
    }
    assert!(decoder::decode_udp(&packet).is_err());
}
