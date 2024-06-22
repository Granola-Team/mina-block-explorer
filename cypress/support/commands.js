// ***********************************************
// This example commands.js shows you how to
// create various custom commands and overwrite
// existing commands.
//
// For more comprehensive examples of custom
// commands please read more here:
// https://on.cypress.io/custom-commands
// ***********************************************
//
//
// -- This is a parent command --
// Cypress.Commands.add('login', (email, password) => { ... })
//
//
// -- This is a child command --
// Cypress.Commands.add('drag', { prevSubject: 'element'}, (subject, options) => { ... })
//
//
// -- This is a dual command --
// Cypress.Commands.add('dismiss', { prevSubject: 'optional'}, (subject, options) => { ... })
//
//
// -- This will overwrite an existing command --

import { parseFormattedNumber } from "../e2e/helpers";

// Cypress.Commands.overwrite('visit', (originalFn, url, options) => { ... })
Cypress.Commands.add(
  "aliasTableRows",
  (tableHeading, alias, tableHeaderEl = "h1") => {
    cy.get("section")
      .find(tableHeaderEl)
      .contains(tableHeading)
      .parents("section")
      .find("table tr:not(:has(th))")
      .as(alias);
  },
);

Cypress.Commands.add(
  "aliasTransposedTableRows",
  (tableHeading, alias, tableHeaderEl = "h1") => {
    cy.get("section")
      .find(tableHeaderEl)
      .contains(tableHeading)
      .parents("section")
      .find("table tr:has(th)")
      .as(alias);
  },
);

Cypress.Commands.add(
  "aliasTableHeaders",
  (tableHeading, alias, tableHeadingEl = "h1") => {
    cy.get("section")
      .find(tableHeadingEl)
      .contains(tableHeading)
      .parents("section")
      .find("table:first th")
      .as(alias);
  },
);

Cypress.Commands.add("closeAccountDialog", () => {
  cy.get("dialog button#closedialog a").click();
  cy.get("dialog").should("not.exist");
});

Cypress.Commands.add(
  "openAccountDialog",
  (nthRow, columnHeading, tableHeading) => {
    cy.clickLinkInTable(nthRow, columnHeading, tableHeading);
    cy.get("dialog").should("be.visible");
  },
);

Cypress.Commands.add("openMobileMenu", () => {
  cy.get("nav").should("not.be.visible");
  cy.get('label[for="nav-toggle"]').click();
  cy.get("nav").should("be.visible");
});

Cypress.Commands.add(
  "tableHasOrderedColumns",
  (tableHeading, columns, tableHeaderEl = "h1") => {
    cy.aliasTableHeaders(tableHeading, "columns", tableHeaderEl);
    cy.get("@columns").should("have.length", columns.length);
    columns.forEach((col, i) => {
      cy.get("@columns").eq(i).contains(col);
    });
  },
);

Cypress.Commands.add(
  "clickLinkInTransposedTable",
  (columnHeading, tableHeading, tableHeadingEl = "h1") => {
    cy.aliasTransposedTableRows(tableHeading, "table-rows", tableHeadingEl);
    cy.get("@table-rows")
      .contains(columnHeading)
      .siblings("td")
      .first()
      .find("a")
      .click({ force: true });
  },
);

Cypress.Commands.add(
  "clickLinkInTable",
  (nthRow, columnHeading, tableHeading, tableHeadingEl = "h1") => {
    cy.aliasTableHeaders(tableHeading, "columns", tableHeadingEl);
    cy.get("@columns")
      .contains(columnHeading)
      .parents("th")
      .invoke("index")
      .then((columnIndex) => {
        cy.aliasTableRows(tableHeading, "table-rows", tableHeadingEl);
        cy.get("@table-rows")
          .eq(nthRow)
          .find("td")
          .eq(columnIndex)
          .find("a")
          .click({ force: true });
      });
  },
);

Cypress.Commands.add("tableHasMoreThanNRows", (tableHeading, n) => {
  cy.aliasTableRows(tableHeading, "table-rows");
  cy.get(`@table-rows`).should(($tr) => {
    expect($tr).to.have.length.of.at.least(n);
  });
});

Cypress.Commands.add("testSpotlight", (heading, id, expected_fields) => {
  cy.get("section#spotlight-section h1").contains(heading);
  cy.get("#spotlight-id").contains(id);
  cy.get("section#spotlight-section table").within(() => {
    expected_fields.forEach((field) => {
      cy.get("th").contains(field);
    });
  });
});

Cypress.Commands.add("assertTableRecordsCorrect", (heading) => {
  cy.aliasTableRows(heading, "table-rows");
  cy.get("@table-rows").then(($rows) => {
    cy.get(".metadata")
      .invoke("text")
      .then((text) => {
        let [displaying, available, total] = text.split(" of ");
        displaying = displaying.replace(/\+/g, "");
        if (total == null) {
          total = available;
          displaying = parseFormattedNumber(displaying);
          total = parseFormattedNumber(total);
          expect(displaying).to.be.lte(total);
        } else {
          displaying = parseFormattedNumber(displaying);
          available = parseFormattedNumber(available);
          total = parseFormattedNumber(total);
          expect(displaying).to.be.lte(available);
          expect(available).to.be.lte(total);
        }
        expect(displaying).to.eq($rows.length);
      });
  });
});

Cypress.Commands.add(
  "assertForEachColumnValue",
  (heading, column, assertion, limit = 5) => {
    cy.aliasTableHeaders(heading, "columns");
    cy.get("@columns")
      .contains(column)
      .parents("th")
      .invoke("index")
      .then((columnIndex) => {
        cy.aliasTableRows(heading, "table-rows");
        cy.get("@table-rows").find(".loading-placeholder").should("not.exist");
        cy.get("@table-rows").should("have.length.gte", 1);
        cy.get("@table-rows").each(($row, index) => {
          if (index > limit) return;
          cy.wrap($row)
            .find("td")
            .eq(columnIndex)
            .invoke("text")
            .then(assertion);
        });
      });
  },
);

Cypress.Commands.add(
  "aliasTableColumnValue",
  (columnHeading, column, alias = "table-column-values") => {
    cy.aliasTableHeaders(columnHeading, "columns");
    cy.get("@columns")
      .contains(column)
      .parents("th")
      .invoke("index")
      .then((columnIndex) => {
        cy.aliasTableRows(columnHeading, "table-rows");
        cy.get("@table-rows").find("td").eq(columnIndex).as(alias);
      });
  },
);

Cypress.Commands.add(
  "tableColumnValuesEqual",
  (columnHeading, column, value) => {
    cy.aliasTableColumnValue(columnHeading, column, "table-column-values");
    cy.get("@table-column-values").should("contain", value);
  },
);
