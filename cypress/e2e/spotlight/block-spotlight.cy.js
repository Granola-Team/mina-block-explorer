describe('Block spotlight', () => {

    let expected_fields = ["State Hash",
        "Previous State Hash",
        "Staged Ledger Hash",
        "Snarked Ledger Hash",
        "Coinbase",
        "Coinbase Receiver",
        "Winning Account",
        "SNARK Fees",
        "Global Slot",
        "Slot",
        "Epoch",
        "Transaction Fees",
        "Blockchain Length"
    ];
    let block_id = "3NLn8e9HoJgvmbJubfX7wbhNzgcz9BSeS2LDNk4X7XngzhPgBfjf";

    it('displays complete information', () => {
        cy.visit(`/blocks/${block_id}`);
        cy.testSpotlight("Block Spotlight", block_id, expected_fields);
    });
});