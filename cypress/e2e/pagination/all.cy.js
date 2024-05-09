import { DEFAULT_ACCOUNT_PK, DEFAULT_CANONICAL_BLOCK_HASH } from "../constants";

suite(["@CI"], "pagination", () => {
  function testTablePagination(alias) {
    cy.get(alias, { timeout: 30000 })
      .find("button.go_to_first")
      .last()
      .as("first");
    cy.get(alias, { timeout: 30000 })
      .find("button.go_to_prev")
      .first()
      .as("prev");
    cy.get(alias, { timeout: 30000 })
      .find("button.go_to_next")
      .last()
      .as("next");
    cy.get(alias, { timeout: 30000 })
      .find("button.go_to_last")
      .first()
      .as("last");
    cy.get(alias, { timeout: 30000 }).find(".current-page").as("currentPage");

    // initial check
    cy.get("@first").should("be.disabled");
    cy.get("@prev").should("be.disabled");
    cy.get("@next").should("not.be.disabled");
    cy.get("@last").should("not.be.disabled");
    cy.get("@currentPage").should("contain", 1);

    // next page
    cy.get("@next").click();
    cy.wait(1000);
    cy.get("@first").should("not.be.disabled");
    cy.get("@prev").should("not.be.disabled");
    cy.get("@next").should("not.be.disabled");
    cy.get("@last").should("not.be.disabled");
    cy.get("@currentPage").should("contain", 2);

    cy.get("@last").click();
    cy.wait(1000);
    cy.get("@first").should("not.be.disabled");
    cy.get("@prev").should("not.be.disabled");
    cy.get("@next").should("be.disabled");
    cy.get("@last").should("be.disabled");

    // prev page
    cy.get("@prev").click();
    cy.wait(1000);
    cy.get("@first").should("not.be.disabled");
    cy.get("@prev").should("not.be.disabled");
    cy.get("@next").should("not.be.disabled");
    cy.get("@last").should("not.be.disabled");

    // first page
    cy.get("@first").click();
    cy.get("@first").should("be.disabled");
    cy.get("@prev").should("be.disabled");
    cy.get("@next").should("not.be.disabled");
    cy.get("@last").should("not.be.disabled");
    cy.get("@currentPage").should("contain", 1);
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
    "/commands/internal-commands",
    "/commands/user-commands",
    `/blocks/${DEFAULT_CANONICAL_BLOCK_HASH}/user-commands`,
    `/blocks/${DEFAULT_CANONICAL_BLOCK_HASH}/internal-commands`,
    `/blocks/${DEFAULT_CANONICAL_BLOCK_HASH}/snark-jobs`,
    "/",
    "/summary",
    "/blocks",
  ].forEach((page) =>
    it(`works on ${page}`, () => {
      cy.visit(page);
      cy.get(".pagination-controls").as("pag");
      testTablePagination("@pag");
    }),
  );
});
