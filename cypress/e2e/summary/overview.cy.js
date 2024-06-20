import { parseFormattedNumber } from "../helpers";

suite(["@tier1"], "blockchain overview", () => {
  it("displays non-zero metrics", async () => {
    cy.visit("/blocks");
    cy.intercept("GET", "/summary").as("summaryData");
    await cy.wait("@summaryData");
    let summaryItems = [
      // {
      //   id: "#uniqueBlockProducers",
      //   label: "Unique Producers of last 1000 Blocks",
      // },
      { id: "#totalUserCommands", label: "Total User Commands" },
      { id: "#totalInternalCommands", label: "Total Internal Commands" },
      { id: "#blockchainLength", label: "Blockchain Length" },
    ];
    summaryItems.forEach(({ id, label }) => {
      cy.get(id).should("not.contain", 0);
      cy.get(id)
        .invoke("text")
        .then((text) => {
          let value = parseFormattedNumber(text);
          expect(value).to.be.gt(0);
        });
      cy.get(id).siblings("label").should("have.text", label);
    });
  });
});
