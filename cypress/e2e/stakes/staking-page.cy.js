import { parseFormattedNumber } from "../helpers";
suite(["@tier2"], "staking ledger", () => {
  beforeEach(() => {
    cy.visit("/staking-ledgers?epoch=1");
    cy.intercept("GET", "/summary").as("summaryData");
    cy.wait("@summaryData");
  });
  it("displays a ledger hash", () => {
    cy.get(".ledger-hash").should("exist");
  });
  it("shows slot progress message", () => {
    cy.wait(500);
    cy.get(".staking-ledger-percent-complete").as("slot-info");
    cy.get("@slot-info")
      .invoke("text")
      .then((epochProgressText) => {
        const info = extractEpochProgress(epochProgressText);
        let calculated_percent = parseFloat(
          (
            (parseFormattedNumber(info.slot) /
              parseFormattedNumber(info.totalSlots)) *
            100
          ).toFixed(0),
        );
        expect(parseFormattedNumber(info.percent)).to.be.within(
          calculated_percent - 1,
          calculated_percent,
        );
      });
  });
  function extractEpochProgress(input) {
    let regex =
      /(\d{1,3}(?:,\d{3})*|\d+)(?:\.(\d+))?% complete \((\d{1,3}(?:,\d{3})*)\/(\d{1,3}(?:,\d{3})*) slots filled\)/;
    const match = input.match(regex);
    return match
      ? {
          percent: match[1] + (match[2] ? "." + match[2] : ""), // Append the decimal part if it exists
          slot: match[3],
          totalSlots: match[4],
        }
      : null;
  }
  it("defaults to current epoch", () => {
    cy.get("section").contains("Staking Ledger");
  });
  it("disables 'Previous' button appropriately", () => {
    cy.get("button.hover\\:cursor-not-allowed")
      .contains("Previous")
      .should("exist");
    cy.get("button.hover\\:cursor-not-allowed")
      .contains("Next")
      .should("not.exist");
  });
  it("contains buttons for epoch navigation", () => {
    cy.get("section").contains("Staking Ledger - Epoch 1");
    cy.get("section").contains("button", "Next").click();
    cy.wait(500);
    cy.get("section").contains("Staking Ledger - Epoch 2");
    cy.get("button").contains("Previous").click();
    cy.wait(500);
    cy.get("section").contains("Staking Ledger - Epoch 1");
  });
  it("disables 'Next' button appropriately", () => {
    cy.wait(500);
    cy.get("section").contains("button", "Next").click();
    cy.get("button.hover\\:cursor-not-allowed")
      .contains("Previous")
      .should("not.exist");
    cy.get("button.hover\\:cursor-not-allowed")
      .contains("Next")
      .should("exist");
  });
});
