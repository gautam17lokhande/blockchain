# NFT-based Ticketing

## Project Title
NFT-based Ticketing

## Project Description
The NFT-based Ticketing system enables event organizers to issue event tickets as unique, non-fungible tokens (NFTs). These NFTs serve as verifiable and secure event tickets, allowing for the easy transfer and resale of tickets while eliminating the risk of counterfeiting or fraud. The tickets are stored on a blockchain, ensuring that each one is unique, traceable, and tamper-proof.

## Project Vision
The vision of this project is to revolutionize the ticketing industry by leveraging blockchain technology to create a transparent, secure, and scalable NFT ticketing platform. The use of NFTs ensures that each ticket is one-of-a-kind, allowing event organizers to easily track ticket sales and prevent fraudulent duplication of tickets.

## Key Features
- **NFT Ticket Creation**: Event organizers can create NFT tickets for their events.
- **Ticket Transfer**: NFTs can be transferred to other users, enabling the resale or gifting of tickets.
- **Ticket Validation**: The system ensures that only valid tickets (NFTs) can be used to enter events.
- **Secure and Transparent**: All tickets and transactions are stored on the blockchain, providing transparency and eliminating fraud.

## Contract Details

- Contract Address: CA72HFGFSRZI4GW3OPRBAELJ4Z24SEZB2RFYCYWAUPDDHALQNMJBDWK6

### Functions:

1. **create_ticket**
   - This function allows the event organizer to create a new NFT ticket for a specific event.
   - Parameters:
     - `event_name`: Name of the event.
     - `ticket_price`: Price of the ticket.
     - `ticket_quantity`: The number of tickets available for the event.
   - Returns: The unique ticket ID for the NFT created.

2. **transfer_ticket**
   - This function allows a ticket holder to transfer their NFT ticket to another user.
   - Parameters:
     - `ticket_id`: The unique ID of the ticket being transferred.
     - `recipient`: Address of the recipient to whom the ticket is being transferred.
   - Returns: A confirmation message that the ticket was successfully transferred.

3. **get_ticket_info**
   - This function retrieves details of a specific ticket.
   - Parameters:
     - `ticket_id`: The unique ID of the ticket.
   - Returns: The ticket's event name, ticket price, owner, and other relevant information.

4. **validate_ticket**
   - This function validates the ownership and authenticity of a ticket before it is used to enter an event.
   - Parameters:
     - `ticket_id`: The unique ID of the ticket being validated.
   - Returns: A boolean value indicating whether the ticket is valid or not.

---

This smart contract provides the foundation for creating and managing NFT-based tickets for events, ensuring that all tickets are verifiable, transferable, and secure.
