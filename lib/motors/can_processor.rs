fn process_message(id: u16, data: &[u8; 8]) -> Message {
    Message.id = id;
    Message.index = data[1] << 8 | data[0];
    Message.command = data[3];
    Message.subindex = data[2]
    Message.data = data[7] << 24 | data[6] << 16 | data[5] << 8 | data[4];
}



