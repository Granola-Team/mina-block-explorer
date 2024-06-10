suite(["tier1"], "chart", () => {
  let analytics_pages = [
    { page: "/analytics/commands/internal", chart_ids: ["#chart"] },
    { page: "/analytics/commands/user", chart_ids: ["#chart"] },
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
