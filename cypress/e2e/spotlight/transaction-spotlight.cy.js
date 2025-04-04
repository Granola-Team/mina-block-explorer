let devices = ["iphone-xr", "macbook-11"];
import {
  FIRST_TXN_HASH,
  FIRST_NON_CANONICAL_TXN_HASH,
  STAKE_DELEGATION_HASH,
  APPLIED_TXN_HASH,
  APPLIED_TXN_BLOCK_STATE_HASH,
  FAILED_TXN_HASH,
  TXN_HASH_IN_OTHER_BLOCKS,
} from "../constants.js";
suite(["@tier2"], "transaction spotlight", () => {
  let expected_fields = [
    "Status",
    "Date",
    "Canonical",
    "Amount",
    "From/Fee Payer",
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
  it("displays proper status", () => {
    cy.visit(`/commands/${FAILED_TXN_HASH}`);
    cy.wait(100);
    cy.testSpotlightValue("Status", "Failed");
    cy.visit(
      `/commands/${APPLIED_TXN_HASH}?q-state-hash=${APPLIED_TXN_BLOCK_STATE_HASH}`,
    );
    cy.wait(100);
    cy.testSpotlightValue("Status", "Applied");
  });
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
  it("displays other blocks containing the same txn", () => {
    cy.visit(`/commands/${TXN_HASH_IN_OTHER_BLOCKS}`);
    cy.get("section").contains("In Other Blocks").should("exist");
    cy.aliasTableRows("In Other Blocks", "table-rows");
    cy.get("@table-rows").should("have.lengthOf", 1);
    cy.tableHasOrderedColumns("In Other Blocks", [
      "Height",
      "Block State Hash",
    ]);
  });
});
