import {
  FIRST_BLOCK_WITH_SNARK_WORK,
  FIRST_NON_CANONICAL_BLOCK_WITH_SNARK_WORK,
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
    "Canonical",
  ];

  function testForCompleteness(stateHash) {
    cy.visit(`/blocks/${stateHash}`);
    cy.testSpotlight("Block Spotlight", stateHash, expected_fields);

    cy.visit(`/blocks/${stateHash}/spotlight`);
    cy.testSpotlight("Block Spotlight", stateHash, expected_fields);

    cy.get(`a[href="/blocks/${stateHash}/commands/user"]`).click();
    cy.tableHasMoreThanNRows("User Commands", 0);

    cy.get(`a[href="/blocks/${stateHash}/snark-jobs"]`).click();
    cy.tableHasMoreThanNRows("SNARK Jobs", 0);
    cy.tableColumnValuesEqual("SNARK Jobs", "Hash", stateHash);

    cy.get(`a[href="/blocks/${stateHash}/commands/internal"]`).click();
    cy.tableHasMoreThanNRows("Internal Commands", 0);
  }

  it("displays complete information for canonical block", () => {
    testForCompleteness(FIRST_BLOCK_WITH_SNARK_WORK);
  });

  it("displays complete information for non-canonical block", () => {
    testForCompleteness(FIRST_NON_CANONICAL_BLOCK_WITH_SNARK_WORK);
  });
});
