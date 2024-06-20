import { DEFAULT_LOCALE } from "../constants";
import { parseFormattedNumber } from "../helpers";

let pages = [
  {
    page: "/blocks",
    wait: () => {
      cy.intercept("GET", "/summary").as("summaryData");
      cy.wait("@summaryData");
      cy.wait(100);
    },
    tests: [
      {
        name: "overview",
        selector: () => cy.get("#blockchainLength"),
      },
      {
        name: "height column",
        selector: () => {
          cy.aliasTableRows("Blocks", "table-rows");
          return cy.get("@table-rows").first().find("td").first();
        },
      },
      {
        name: "slot column",
        selector: () => {
          cy.aliasTableRows("Blocks", "table-rows");
          return cy.get("@table-rows").first().find("td").eq(2);
        },
      },
    ],
  },
];

pages.forEach(({ tests, page, wait }) => {
  suite(["@tier1"], "number", () => {
    it(`on page ${page} is formatted correctly for '${tests.map((t) => t.name).join("', '")}'`, () => {
      cy.visit(page);
      wait();
      tests.forEach(({ selector }) => {
        selector()
          .invoke("text")
          .then((text) => {
            let number = parseFormattedNumber(text);
            expect(number).to.be.a("number");
            const formatter = new Intl.NumberFormat(DEFAULT_LOCALE, {});
            expect(text).to.equal(formatter.format(number));
          });
      });
    });
  });
});
