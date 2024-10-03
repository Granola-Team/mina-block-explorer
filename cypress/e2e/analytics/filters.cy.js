suite(["@tier2"], "fitler", () => {
  let pages = [
    {
      url: "/analytics/commands/user",
      filter_ids: ["#blockheight-gte", "#blockheight-lte"],
      expected_urls_values: [7000, 10000],
      expected_url_keys: ["q-blockheight-gte", "q-blockheight-lte"],
    },
    {
      url: "/analytics/snarks",
      filter_ids: ["#blockheight-gte", "#blockheight-lte"],
      expected_urls_values: [7000, 10000],
      expected_url_keys: ["q-blockheight-gte", "q-blockheight-lte"],
    },
    {
      url: "/analytics/staker-leaderboard",
      filter_ids: ["#epoch"],
      expected_url_values: [0],
      expected_url_keys: ["epoch"],
    },
    {
      url: "/analytics/snarker-leaderboard",
      filter_ids: ["#epoch"],
      expected_url_values: [0],
      expected_url_keys: ["epoch"],
    },
  ];
  pages.forEach(({ url, filter_ids, expected_url_values, expected_url_keys }) =>
    it(`has defaults for ${url}`, () => {
      cy.visit(url);
      filter_ids.forEach((filter_id, index) => {
        const expected_url_value = expected_url_values[index];
        const expected_url_key = expected_url_keys[index];

        cy.get(filter_id).should("have.value", expected_url_value);
        cy.url().should("include", `${expected_url_key}=${expected_url_value}`);
      });
    }),
  );
});
