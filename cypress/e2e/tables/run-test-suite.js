export function runTestSuite(testSuiteData) {
  const {
    url,
    table: { heading, filter_tests, columns, sorting_columns = [] },
    tests,
  } = testSuiteData;

  describe(`table on ${url}`, () => {
    beforeEach(() => {
      cy.visit(url);
      cy.waitUntilTableLoads(heading);
    });

    it("has ordered columns", () => {
      cy.tableHasOrderedColumns(heading, columns);
    });

    if (columns.filter((c) => c === "Date").length > 0) {
      it("has valid dates", () => {
        cy.assertForEachColumnValue(heading, "Date", (dateText) => {
          // Attempt to parse the date string
          const parsedDate = new Date(dateText);
          // Assert that the date is valid
          expect(parsedDate.toString()).not.to.equal("Invalid Date");
        });
      });
    }

    sorting_columns.forEach(({ column, type, sort_options }) => {
      it(`has sortable column '${column}'`, () => {
        sort_options.forEach((sort, i) => {
          if (sort != null) {
            cy.assertSortOrder(heading, column, sort.includes("DESC"), type);
            // we don't necessarily expect the url to indicate
            // sort direction on the first page load
            if (i !== 0) {
              cy.url().should("include", `sort-dir=${sort}`);
            }
          }
          cy.get("th").contains(column).click("top");
        });
      });
    });

    filter_tests.forEach(({ column, input, assertion, filter_type }) => {
      it(`has working filter for column '${column}' and input '${input}'`, () => {
        if (filter_type == "select") {
          cy.get("th").contains(column).find("select").as("input");
          cy.get("@input").select(input);
        } else {
          cy.get("th").contains(column).find("input").as("input");
          cy.get("@input").type(input, { delay: 0 });
        }
        cy.wait(1000); // wait for user input to trigger table load
        cy.waitUntilTableLoads(heading);
        assertion();
        cy.assertTableRecordsCorrect(heading);
      });
    });

    tests.forEach(([desc, test]) => it(desc, test));
  });
}
