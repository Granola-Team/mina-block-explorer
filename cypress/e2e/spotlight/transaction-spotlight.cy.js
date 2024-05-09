const devices = require("../../devices.json");

suite(["@CI"], "transaction spotlight", () => {
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
  let transaction_id = "CkpYkHBnz3c7mxcKsPP7Y3m69HvFyU5CepLPjxuc1ynTsfQnAkYnT";
  let mobile = devices[0];

  it("displays complete information", () => {
    cy.viewport(mobile);
    cy.visit(`/commands/${transaction_id}`);
    cy.testSpotlight("Command Spotlight", transaction_id, expected_fields);
  });

  it("renders the tooltip for stake delegations", () => {
    cy.visit("/commands/CkpYpu7SoosTDXH1vTsL6ZpmCtASNyVPV1kub3FJ33ubSRqLCWaHK");
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
    cy.visit(`/commands/${transaction_id}`);
    cy.get("section#spotlight-section table").within(() => {
      cy.get("th").contains("Amount").as("amount");
      cy.get("@amount").parent("tr").as("row");
      cy.get("@row").within(() => {
        cy.get("td .tooltip").should("not.exist");
      });
    });
  });
});
