suite(["@tier2"], "fitler", () => {
  let pages = [
    {
      url: "/analytics/commands/user",
      filter_id: "#block-limit",
      expected_url_value: 1000,
      expected_url_key: "limit",
    },
    {
      url: "/analytics/snarks",
      filter_id: "#block-limit",
      expected_url_value: 1000,
      expected_url_key: "limit",
    },
    {
      url: "/analytics/staker-leaderboard",
      filter_id: "#epoch",
      expected_url_value: 0,
      expected_url_key: "epoch",
    },
    {
      url: "/analytics/snarker-leaderboard",
      filter_id: "#epoch",
      expected_url_value: 0,
      expected_url_key: "epoch",
    },
  ];
  pages.forEach(({ url, filter_id, expected_url_value, expected_url_key }) =>
    it(`has defaults for ${url}`, () => {
      cy.visit(url);
      cy.get(filter_id).should("have.value", expected_url_value);
      cy.url().should("include", `${expected_url_key}=${expected_url_value}`);
    }),
  );
});
