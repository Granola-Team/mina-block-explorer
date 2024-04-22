import { DEFAULT_ACCOUNT_PK, DEFAULT_CANONICAL_BLOCK_HASH } from "../constants";

suite(["@CI"], "pagination", () => {
  function testTablePagination(alias) {
    cy.get(alias, { timeout: 30000 }).find("button").last().as("next");
    cy.get(alias, { timeout: 30000 }).find("button").first().as("prev");
    cy.get(alias, { timeout: 30000 }).find(".current-page").as("currentPage");

    // initial check
    cy.get("@prev").should("be.disabled");
    cy.get("@next").should("not.be.disabled");
    cy.get("@currentPage").should("contain", 1);

    // next page
    cy.get("@next").click();
    cy.wait(1000);
    cy.get("@prev").should("not.be.disabled");
    cy.get("@next").should("not.be.disabled");
    cy.get("@currentPage").should("contain", 2);

    // number click (last page)
    cy.get(alias, { timeout: 30000 })
      .find("button:nth-last-child(2)")
      .as("last-clickable-page");
    cy.get(alias, { timeout: 30000 })
      .find(".button-container > .page:nth-last-child(2)")
      .as("last-page");

    function clickLastPageUntil() {
      cy.get("@last-page").then(($pg) => {
        if (!$pg.attr("class").includes("current-page")) {
          cy.get("@last-clickable-page").click();
          clickLastPageUntil();
        }
      });
    }
    clickLastPageUntil();

    cy.wait(1000);
    cy.get("@prev").should("not.be.disabled");
    cy.get("@next").should("be.disabled");
    cy.get("@last-page")
      .invoke("text")
      .then((text) => {
        expect(parseInt(text)).to.be.gt(1);
      });

    // prev page
    cy.get("@prev").click();
    cy.wait(1000);
    cy.get("@prev").should("not.be.disabled");
    cy.get("@next").should("not.be.disabled");
    cy.get("@last-page")
      .invoke("text")
      .then((text) => {
        expect(parseInt(text)).to.be.gt(1);
      });
  }

  let account = `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`;
  let tables = ["User Commands", "SNARK Jobs", "Block Production"];
  tables.forEach((tableHeading) =>
    it(`works on table '${tableHeading}' on page ${account}`, () => {
      cy.visit(account);
      cy.aliasTablePagination(tableHeading, "pag");
      testTablePagination("@pag");
    }),
  );

  [
    `/blocks/${DEFAULT_CANONICAL_BLOCK_HASH}/user-commands`,
    `/blocks/${DEFAULT_CANONICAL_BLOCK_HASH}/internal-commands`,
    `/blocks/${DEFAULT_CANONICAL_BLOCK_HASH}/snark-jobs`,
    "/",
    "/summary",
    "/blocks",
    "/commands",
    "/snarks",
    "/stakes",
    "/next-stakes",
    "/transactions/internal-commands",
  ].forEach((page) =>
    it(`works on ${page}`, () => {
      cy.visit(page);
      cy.get(".pagination-controls").as("pag");
      testTablePagination("@pag");
    }),
  );
});
