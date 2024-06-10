const devices = require("../../devices.json");
const {
  FIRST_TXN_HASH,
  FIRST_NON_CANONICAL_TXN_HASH,
  STAKE_DELEGATION_HASH,
} = require("../constants");

suite(["tier1"], "transaction spotlight", () => {
  let expected_fields = [
    "Date",
    "Canonical",
    "Amount",
    "From",
    "Nonce",
    "Kind",
    "Txn Hash",
    "Block Height",
    "Block State Hash",
    "Fee",
    "To",
    "Memo",
  ];
  let mobile = devices[0];

  it("displays complete information", () => {
    cy.viewport(mobile);
    cy.visit(`/commands/${FIRST_TXN_HASH}`);
    cy.testSpotlight("Command Spotlight", FIRST_TXN_HASH, expected_fields);
  });

  it("displays non-canonical command", () => {
    cy.visit(`/commands/${FIRST_NON_CANONICAL_TXN_HASH}`);
    cy.testSpotlight(
      "Command Spotlight",
      FIRST_NON_CANONICAL_TXN_HASH,
      expected_fields,
    );
  });

  it("renders the tooltip for stake delegations", () => {
    cy.visit(`/commands/${STAKE_DELEGATION_HASH}`);
    cy.get("section#spotlight-section table").within(() => {
      cy.get("th").contains("Amount").as("amount");
      cy.get("@amount").parent("tr").as("row");
      cy.get("@row").within(() => {
        cy.get("td .tooltip").should(
          "have.attr",
          "title",
          "Stake delegations have no transacted amount",
        );
      });
    });
  });

  it("does not render the tooltip for regular payments", () => {
    cy.visit(`/commands/${FIRST_TXN_HASH}`);
    cy.get("section#spotlight-section table").within(() => {
      cy.get("th").contains("Amount").as("amount");
      cy.get("@amount").parent("tr").as("row");
      cy.get("@row").within(() => {
        cy.get("td .tooltip").should("not.exist");
      });
    });
  });
});
