use anchor_lang::prelude::*;
declare_id!("8rPKhJ5i9NiAEpLmCx2ETEJryM6fbHjrZwJtxihU6aek");

#[program]
pub mod crosschain {
    use super::*;


    pub fn send(ctx:Context<Initialize>) -> Result<()>{


        let operator: [u8; 20] = [
            0x7a, 0x6f, 0x8c, 0xe6, 0xc7, 0x53, 0x42, 0x64, 0x20, 0x2c,
            0xd6, 0xe4, 0xa8, 0x58, 0x7e, 0x94, 0x0b, 0x8a, 0x18, 0x65
        ];
        let amount: u64 = 123456789;
        let message = "Hello, world!".to_string();

        // Create a new Packet instance
        let packet = Packet {
            operator,
            amount,
            message,
        };


        emit!(SendEvent{
        payload:packet.encode(),}
        );

        msg!("encoded {:?}",packet.encode());
        Ok(())
    }

    pub fn receive(ctx: Context<Initialize>, payload: Vec<u8>) -> Result<()>{
        let packet = Packet::decode(&payload);
        
        emit!(
            ReceiveEvent{
           addr: packet.operator,amount: packet.amount,message: packet.message,}
           
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[event]
struct SendEvent {
    payload:Vec<u8>
}

#[event]
struct ReceiveEvent{
    addr: [u8; 20],
    amount:u64,
    message:String,
}



pub const OPERATOR_OFFSET: usize = 0;
pub const AMOUNT_OFFSET: usize = 20;
pub const MESSAGE_OFFSET: usize = 52;
#[derive(Debug)]
struct Packet {
    pub operator: [u8; 20],
    pub amount: u64,
    pub message: String,
}

impl Packet {

    pub fn new(operator: [u8; 20], amount: u64, message: String) -> Self {
        Packet {
            operator,
            amount,
            message,
        }
    }

    pub fn decode(encoded_packet: &[u8]) -> Self {
        let operator = Self::decode_operator(encoded_packet);
        let amount = Self::decode_amount(encoded_packet);
        let message = Self::decode_message(encoded_packet);
        Packet::new(operator, amount, message)
    }
    pub fn encode(&self) -> Vec<u8> {
        [
            &self.operator[..],
            &u64_to_32byte_vec(self.amount)[..],
            self.message.as_bytes(),
        ]
            .concat()
    }

    fn decode_operator(encoded_packet: &[u8]) -> [u8; 20] {
        let mut operator = [0u8; 20];
        operator.copy_from_slice(&encoded_packet[OPERATOR_OFFSET..AMOUNT_OFFSET]);
        operator
    }

    fn decode_amount(encoded_packet: &[u8]) -> u64 {
        let mut amount_bytes = [0u8; 32];
        amount_bytes.copy_from_slice(&encoded_packet[AMOUNT_OFFSET..MESSAGE_OFFSET]);
        vec_to_u64(amount_bytes).unwrap()
    }

    fn decode_message(encoded_packet: &[u8]) -> String {
        String::from_utf8(encoded_packet[MESSAGE_OFFSET..].to_vec()).expect("Invalid UTF-8")
    }

}

fn u64_to_32byte_vec(input: u64) -> Vec<u8> {
    // Convert the u64 to an 8-byte array
    let mut bytes = input.to_le_bytes().to_vec();
    // Extend the byte array to 32 bytes by adding zeros
    bytes.resize(32, 0);

    bytes
}
fn vec_to_u64(bytes: [u8; 32]) -> std::result::Result<u64, &'static str> {
    // Ensure the vector has at least 8 bytes
    if bytes.len() < 8 {
        return Err("Vector is too short to convert to u64");
    }
    // Extract the first 8 bytes
    let mut array = [0u8; 8];
    array.copy_from_slice(&bytes[..8]);

    // Convert the 8-byte array to u64
    Ok(u64::from_le_bytes(array))
}


