suite(["@tier2"], "blockchain overview", () => {
  it("displays valid metrics", () => {
    cy.visit("/blocks");
    cy.intercept("GET", "/summary").as("summaryData");
    cy.wait("@summaryData");
    let numericSummaryItems = [
      { id: "#epoch", label: "Epoch" },
      {
        id: "#uniqueBlockProducers",
        label: "Unique Producers of last 10000 blocks",
      },
      { id: "#globalSlot", label: "Global Slot" },
      { id: "#blockchainLength", label: "Blockchain Length" },
      { id: "#totalMina", label: "Total MINA" },
      { id: "#circulatingSupply", label: "Circulating Supply" },
      { id: "#totalNumBlocks", label: "Total Blocks" },
      { id: "#totalUserCommands", label: "Total User Commands" },
      { id: "#totalInternalCommands", label: "Total Internal Commands" },
      { id: "#totalSnarks", label: "Total SNARKs" },
    ];
    let stringSummaryItems = [
      { id: "#chainId", label: "Chain ID" },
      { id: "#genesisStateHash", label: "Genesis State Hash" },
    ];
    numericSummaryItems.forEach(({ id, label }) => {
      // cy.get(id).should("not.contain", 0);
      cy.get(id).checkNumeric();
      cy.get(id).siblings("label").should("have.text", label);
    });
    stringSummaryItems.forEach(({ id, label }) => {
      cy.get(id).should("not.be.empty");
      cy.get(id).siblings("label").should("have.text", label);
    });
  });
});
