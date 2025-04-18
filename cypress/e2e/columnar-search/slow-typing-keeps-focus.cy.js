import { kebabCase } from "../helpers";
describe("input", () => {
  let slow_input_searches = [
    {
      origin: "/addresses/accounts",
      input: "B62",
      column: "Public Key",
    },
    {
      origin: "/blocks",
      input: "253134",
      column: "Height",
    },
    {
      origin: "/staking-ledgers?epoch=1",
      input: "B62",
      column: "Key",
    },
    {
      origin: "/commands/internal",
      input: "253134",
      column: "Height",
    },
  ];
  slow_input_searches.forEach(({ origin, input, column }) =>
    it(`remains focused as user types slowly into ${column} on page ${origin}`, () => {
      cy.visit(origin);
      cy.intercept("GET", "/summary").as("summaryData");
      cy.wait("@summaryData");
      cy.wait(1000);
      let cssSelector = "#q-" + kebabCase(column);
      cy.get(cssSelector).type(input, { delay: 750 });
      cy.get(cssSelector).should("have.focus");
    }),
  );
});
