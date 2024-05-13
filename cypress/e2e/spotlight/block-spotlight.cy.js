import {
  DEFAULT_CANONICAL_BLOCK_HASH,
  DEFAULT_NON_CANONICAL_BLOCK_HASH,
} from "../constants";

suite(["@CI"], "Block spotlight", () => {
  let expected_fields = [
    "State Hash",
    "Previous State Hash",
    "Staged Ledger Hash",
    "Snarked Ledger Hash",
    "Coinbase",
    "Coinbase Receiver",
    "SNARK Fees",
    "Global Slot",
    "Slot",
    "Epoch",
    "Transaction Fees",
    "Blockchain Length",
  ];

  function testForCompleteness(
    stateHash,
    minUserCommands,
    minSnarkJobs,
    minInternalCommands,
  ) {
    cy.visit(`/blocks/${stateHash}`);
    cy.testSpotlight("Block Spotlight", stateHash, expected_fields);

    cy.visit(`/blocks/${stateHash}/spotlight`);
    cy.testSpotlight("Block Spotlight", stateHash, expected_fields);

    cy.get(`a[href="/blocks/${stateHash}/commands/user"]`).click();
    cy.tableHasMoreThanNRows("User Commands", minUserCommands);

    cy.get(`a[href="/blocks/${stateHash}/snark-jobs"]`).click();
    cy.tableHasMoreThanNRows("SNARK Jobs", minSnarkJobs);
    cy.tableColumnValuesEqual("SNARK Jobs", "Hash", stateHash);

    cy.get(`a[href="/blocks/${stateHash}/commands/internal"]`).click();
    cy.tableHasMoreThanNRows("Internal Commands", minInternalCommands);
  }

  it("displays complete information for canonical block", () => {
    testForCompleteness(DEFAULT_CANONICAL_BLOCK_HASH, 7, 7, 7);
  });

  it("displays complete information for non-canonical block", () => {
    testForCompleteness(DEFAULT_NON_CANONICAL_BLOCK_HASH, 7, 7, 6);
  });
});
