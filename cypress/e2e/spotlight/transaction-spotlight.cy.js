const devices = require("../../devices.json");
const {
  FIRST_TXN_HASH,
  FIRST_NON_CANONICAL_TXN_HASH,
  STAKE_DELEGATION_HASH,
  ROMEK_MINA_NAMING_SERVICE_TXN_HASH,
  WHISPERIT_TXN_HASH,
  WHISPERIT_BLOCK_STATE_HASH,
  ROMEK_NAMING_MEMO,
  FIRST_INTERNAL_TXN_HASH,
} = require("../constants");

suite(["@tier2"], "transaction spotlight", () => {
  let expected_fields = [
    "Status",
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

  it("displays proper status", () => {
    cy.visit(
      `/commands/${FIRST_TXN_HASH}?q-state-hash=${FIRST_INTERNAL_TXN_HASH}`,
    );
    cy.testSpotlightValue("Status", "Failed");
    cy.visit(
      `/commands/${WHISPERIT_TXN_HASH}?q-state-hash=${WHISPERIT_BLOCK_STATE_HASH}`,
    );
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

  it("displays memo", () => {
    cy.visit(`/commands/${ROMEK_MINA_NAMING_SERVICE_TXN_HASH}`);
    cy.contains(ROMEK_NAMING_MEMO).should("exist");
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
    cy.visit(
      `/commands/${WHISPERIT_TXN_HASH}?q-state-hash=${WHISPERIT_BLOCK_STATE_HASH}`,
    );
    cy.get("section").contains("In Other Blocks").should("exist");
    cy.aliasTableRows("In Other Blocks", "table-rows");
    cy.get("@table-rows").should("have.lengthOf", 3);
  });
});
