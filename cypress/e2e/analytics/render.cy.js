suite(["@tier2"], "chart", () => {
  let analytics_pages = [
    {
      page: "/analytics/commands/user",
      chart_ids: [
        "#user-commands-volume",
        "#user-commands-top-recipients",
        "#user-commands-top-transfers",
        "#fee-spread",
        "#transfer-count",
      ],
    },
    { page: "/analytics/blocks", chart_ids: ["#rewards", "#blocks"] },
    { page: "/analytics/snarks", chart_ids: ["#avg-snark-fee"] },
  ];

  analytics_pages.forEach(({ page, chart_ids }) =>
    it(`renders on page ${page}`, () => {
      cy.visit(page);
      chart_ids.forEach((id) => {
        cy.get(id).find("canvas").should("be.visible");
      });
    }),
  );
});
