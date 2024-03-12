suite(["@CI"],'Block spotlight', () => {

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

        cy.visit(`/blocks/${block_id}/spotlight`);
        cy.testSpotlight("Block Spotlight", block_id, expected_fields);

        cy.get(`a[href="/blocks/${block_id}/user-commands"]`).click();
        cy.tableHasNRows("User Commands", 10);

        cy.get(`a[href="/blocks/${block_id}/snark-jobs"]`).click();
        cy.tableHasNRows("SNARK Jobs", 10);
        cy.tableColumnValuesEqual("SNARK Jobs", "Hash", block_id);

        cy.get(`a[href="/blocks/${block_id}/fee-transfers"]`).click();
        cy.tableHasNRows("Fee Transfers", 10);
    });
});