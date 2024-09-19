suite(["@tier2"], "analytics filter", () => {
  let analytics_pages = [
    {
      page: "/analytics/snarks",
      filters: [{ id: "#block-limit", url_key: "limit", test_val: 100 }],
    },
  ];

  analytics_pages.forEach(({ page, filters }) =>
    it(`is URL integrated on ${page}`, () => {
      cy.visit(page);
      filters.forEach(({ id, url_key, test_val }) => {
        cy.get(id).type(test_val);
        cy.url().contains(`${url_key}=${test_val}`);
      });
    }),
  );
});
