export function runTestSuite(testSuiteData) {
  const {
    disabled = false,
    tag,
    url,
    table: { heading, filter_tests, columns, sorting_columns = [] },
    tests,
  } = testSuiteData;
  suite([tag], `table on ${url}`, () => {
    if (disabled) {
      xit("has standard functionality", () => {});
    } else {
      it("has standard functionality", () => {
        cy.visit(url);
        cy.viewport(768, 2000);
        cy.intercept("GET", "/summary").as("summaryData");
        cy.wait("@summaryData").then(() => {
          cy.tableHasOrderedColumns(heading, columns);
          if (columns.filter((c) => c === "Date").length > 0) {
            cy.assertForEachColumnValue(heading, "Date", (dateText) => {
              // Attempt to parse the date string
              const parsedDate = new Date(dateText);
              // Assert that the date is valid
              expect(parsedDate.toString()).not.to.equal("Invalid Date");
            });
          }
          sorting_columns.forEach(({ column, type, sort_options }) => {
            sort_options.forEach((sort, i) => {
              if (sort != null) {
                cy.log("Testing Sort Order: " + sort);
                cy.assertSortOrder(
                  heading,
                  column,
                  sort.includes("DESC"),
                  type,
                );
                // we don't necessarily expect the url to indicate
                // sort direction on the first page load
                if (i !== 0) {
                  cy.url().should("include", `sort-dir=${sort}`);
                }
              }
              cy.get("th").contains(column).click("top");
            });
          });
          filter_tests.forEach(({ column, input, assertion }) => {
            cy.get("th").contains(column).find("input").as("input");
            cy.get("@input").type(input, { delay: 0 });
            cy.intercept("POST", "/graphql").as("graphql");
            cy.wait("@graphql", { timeout: 15000 });
            cy.wait(1000);
            assertion();
            if (heading != "Staking Ledger - Epoch 1") {
              cy.assertTableRecordsCorrect(heading);
            }
            cy.get("@input").clear();
          });
          tests.forEach((test) => test());
        });
      });
    }
  });
}
