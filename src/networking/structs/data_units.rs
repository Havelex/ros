struct Frame {
    preamble: [u8; 7],
    sfd: u8,
    dmac: [u8; 6],
    smac: [u8; 6],
    tag: [u8; 4],
    length: [u8; 2],
    payload: [u8; 1500],
    fcs: [u8; 4],
    ipg: [u8; 12],
}

struct Packet {
    version: [bool; 4],
    ihl: [bool; 4],
    tos: u8,
    total_length: u8,
    id: u16,
    flags: [bool; 3],
    offset: [bool; 13],
    ttl: u8,
    protocol: u8,
    checksum: u16,
    src: u32,
    dest: u32,
    options: [u8; 40],
    data: [u8; 65515],
}

struct Udp {
    sport: u16,
    dport: u16,
    length: u16,
    checksum: u16,
    data: [u8; 65507],
}
