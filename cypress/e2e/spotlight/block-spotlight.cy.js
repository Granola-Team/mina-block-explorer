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
    "Winning Account",
    "SNARK Fees",
    "Global Slot",
    "Slot",
    "Epoch",
    "Transaction Fees",
    "Blockchain Length",
  ];

  function testForCompleteness(
    stateHash,
    expectedUserCommands,
    expectedSnarkJobs,
    expectedInternalCommands,
  ) {
    cy.visit(`/blocks/${stateHash}`);
    cy.testSpotlight("Block Spotlight", stateHash, expected_fields);

    cy.visit(`/blocks/${stateHash}/spotlight`);
    cy.testSpotlight("Block Spotlight", stateHash, expected_fields);

    cy.get(`a[href="/blocks/${stateHash}/user-commands"]`).click();
    cy.tableHasNRows("User Commands", expectedUserCommands);

    cy.get(`a[href="/blocks/${stateHash}/snark-jobs"]`).click();
    cy.tableHasNRows("SNARK Jobs", expectedSnarkJobs);
    cy.tableColumnValuesEqual("SNARK Jobs", "Hash", stateHash);

    cy.get(`a[href="/blocks/${stateHash}/internal-commands"]`).click();
    cy.tableHasNRows("Internal Commands", expectedInternalCommands);
  }

  it("displays complete information for canonical block", () => {
    testForCompleteness(DEFAULT_CANONICAL_BLOCK_HASH, 10, 10, 10);
  });

  it("displays complete information for non-canonical block", () => {
    testForCompleteness(DEFAULT_NON_CANONICAL_BLOCK_HASH, 10, 10, 6);
  });
});
