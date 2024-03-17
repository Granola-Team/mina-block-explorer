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
// Cypress.Commands.overwrite('visit', (originalFn, url, options) => { ... })

Cypress.Commands.add('aliasTableRows', (tableHeading, alias) => {
  cy.contains('section',tableHeading)
    .find('table tr:not(:has(th))', {timeout: 60000})
    .as(alias);
});

Cypress.Commands.add('aliasTableHeaders', (tableHeading, alias) => {
  cy.contains('section',tableHeading)
    .find('table th', {timeout: 60000})
    .as(alias);
})

Cypress.Commands.add('closeAccountDialog', () => {
  cy.get('dialog button#closedialog a').click();
  cy.get('dialog').should('not.exist');
});

Cypress.Commands.add('openAccountDialog', (nthRow, columnHeading, tableHeading) => {
  cy.clickLinkInTable(nthRow, columnHeading, tableHeading);
  cy.get('dialog', { timeout: 60000 }).should('be.visible');
});

Cypress.Commands.add('accountDialogToAccount', () => {
  cy.get('dialog button#viewmore a').click();
  cy.get('dialog').should('not.exist');

  cy.url().should('contain', '/addresses/accounts')
});

Cypress.Commands.add('openMobileMenu', () => {
  cy.get('nav').should('not.be.visible');
  cy.get('label[for="nav-toggle"]').click();
  cy.get('nav').should('be.visible');
});

Cypress.Commands.add('tableHasOrderedColumns', (tableHeading, columns) => {
  cy.aliasTableHeaders(tableHeading, 'columns');
  cy.get('@columns').should('have.length',columns.length);
  columns.forEach((col,i) => {
      cy.get('@columns').eq(i).contains(col);    
  });
});

Cypress.Commands.add('clickLinkInTable', (nthRow, columnHeading, tableHeading) => {
  cy.aliasTableHeaders(tableHeading, 'columns');
  cy.get('@columns')
    .contains(columnHeading, { timeout: 60000 }) 
    .invoke('index')
    .then(columnIndex => {
      cy.contains('section',tableHeading)
        .find('table tr', {timeout: 60000}) 
        .eq(nthRow) 
        .find('td')
        .eq(columnIndex)
        .find('a') 
        .click({force: true});            
    });
});

Cypress.Commands.add('tableHasNRows', (tableHeading, n) => {
  cy.aliasTableRows(tableHeading, 'table-rows');
  cy.get('@table-rows')
    .should(($tr) => {
      expect($tr).to.have.length(n);
    })
});

Cypress.Commands.add('spotlightData', (label) => {
  cy.get('table:first tr th').contains(label).siblings('td');
});

Cypress.Commands.add('aliasTableValue', (nthRow, columnHeading, tableHeading, alias) => {
  cy.aliasTableHeaders(tableHeading, 'columns');
  cy.get('@columns')
    .contains(columnHeading, { timeout: 60000 }) 
    .invoke('index')
    .then(columnIndex => {
      cy.aliasTableRows(tableHeading, 'table-rows');
      cy.get('@table-rows')
        .eq(nthRow) 
        .find('td')
        .eq(columnIndex)
        .as(alias)
    });
});

Cypress.Commands.add('tableHasLessThanNRows', (tableHeading, n) => {
  cy.aliasTableRows(tableHeading, 'table-rows');
  cy.get(`@table-rows`)
    .should(($tr) => {
      expect($tr).to.have.length.of.at.most(n);
    })
});

Cypress.Commands.add('testSpotlight',(heading, id, expected_fields) => {
  cy.get("section#spotlight-section h1").contains(heading);
  cy.get("#spotlight-id").contains(id);
  cy.get("section#spotlight-section table").within(() => {
    expected_fields.forEach(field => {
      cy.get('th').contains(field)
    });
  });
  
})

Cypress.Commands.add('tableColumnValuesEqual', (heading, column, value) => {
  cy.aliasTableHeaders(heading, 'columns');
  cy.get('@columns')
    .contains(column, { timeout: 60000 }) 
    .invoke('index')
    .then(columnIndex => {
      cy.contains('section',heading)
        .find('table tr td', {timeout: 60000}) 
        .eq(columnIndex)
        .find('a', { timeout: 60000 }) 
        .should('have.text', value)         
    });
});

Cypress.Commands.add('prepareSnapshotTest',() => {
  /** 
   * Make the header static so that it doesn't get in the way.
   * We are unable to scroll to top and take snapshots without 
   * the header getting in the way.
   */
  cy.get('header').invoke('css', 'position', 'static');
})


