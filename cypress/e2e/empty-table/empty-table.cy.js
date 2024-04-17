suite(["@CI"], "empty table", () => {
  let pages = [
    "/blocks?query=fake",
    "/commands?query=fake",
    "/snarks?query=fake",
    "/stakes?query=fake",
    "/next-stakes?query=fake",
  ];

  pages.forEach((page) =>
    it(`on ${page} shows as having zero records`, () => {
      cy.visit(page);
      cy.contains("Showing 0 to 0 of 0 records");
    }),
  );
});
