suite(["@tier2"], "filters", () => {
  let analytics_pages = [
    {
      page: "/analytics/snarks",
      filters: [{ id: "block-limit", url_key: "limit", test_val: 100 }],
    },
  ];

  analytics_pages.forEach(({ page, filters }) =>
    it(`has URL integrated filters ${page}`, () => {
      cy.visit(page);
      filters.forEach(({ id, url_key, test_val }) => {
        cy.get(id).type(test_val);
        cy.url().contains(`${url_key}=${test_val}`);
      });
    }),
  );
});
