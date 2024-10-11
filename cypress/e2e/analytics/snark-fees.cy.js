suite(["@tier2"], "snark fees", () => {
  it("are rendered", () => {
    cy.visit(
      "/analytics/snarks?q-blockheight-gte=8000&q-blockheight-lte=10000",
    );
    cy.aliasTableRows("SNARK Fees Overview", "table-rows");
    cy.get("@table-rows").should("have.lengthOf", 8);
    let metrics = [
      "Count",
      "Sum",
      "Mean",
      "Median",
      "Min",
      "Max",
      "25%",
      "75%",
    ];
    metrics.forEach((metric, i) => {
      cy.get("@table-rows").eq(i).should("contain.text", metric);
    });
  });
});
