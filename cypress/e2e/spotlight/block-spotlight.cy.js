import { DEFAULT_CANONICAL_BLOCK_HASH } from "../constants";

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

    it('displays complete information', () => {
        cy.visit(`/blocks/${DEFAULT_CANONICAL_BLOCK_HASH}`);
        cy.testSpotlight("Block Spotlight", DEFAULT_CANONICAL_BLOCK_HASH, expected_fields);

        cy.visit(`/blocks/${DEFAULT_CANONICAL_BLOCK_HASH}/spotlight`);
        cy.testSpotlight("Block Spotlight", DEFAULT_CANONICAL_BLOCK_HASH, expected_fields);

        cy.get(`a[href="/blocks/${DEFAULT_CANONICAL_BLOCK_HASH}/user-commands"]`).click();
        cy.tableHasNRows("User Commands", 10);

        cy.get(`a[href="/blocks/${DEFAULT_CANONICAL_BLOCK_HASH}/snark-jobs"]`).click();
        cy.tableHasNRows("SNARK Jobs", 10);
        cy.tableColumnValuesEqual("SNARK Jobs", "Hash", DEFAULT_CANONICAL_BLOCK_HASH);

        cy.get(`a[href="/blocks/${DEFAULT_CANONICAL_BLOCK_HASH}/internal-commands"]`).click();
        cy.tableHasNRows("Internal Commands", 10);
    });
});